/*
 *  sketch_config.rs
 *  Copyright 2021 ItJustWorksTM
 *
 *  Licensed under the Apache License, Version 2.0 (the "License");
 *  you may not use this file except in compliance with the License.
 *  You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 *  Unless required by applicable law or agreed to in writing, software
 *  distributed under the License is distributed on an "AS IS" BASIS,
 *  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *  See the License for the specific language governing permissions and
 *  limitations under the License.
 *
 */

use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;
use std::ops::{Deref, Index};
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, LockResult, Mutex, MutexGuard, RwLock, RwLockReadGuard};

use cxx::UniquePtr;

use crate::board_config::{BoardConfig, GpioDriver};
// use crate::board_view::BoardView;
use crate::board_view::{
    AnalogPin, AnalogPins, BoardView, DigitalPin, DigitalPins, FrameBuffer, FrameBuffers,
    UartChannel, UartChannels,
};
use crate::ffi::{
    board_new, BoardStatus, ExitInfo, OpaqueBoard, OpaqueBoardView, OpaqueVirtualPin,
};
use crate::sketch::Sketch;

unsafe impl Send for OpaqueBoard {}

pub struct Board {
    internal: Option<(UniquePtr<OpaqueBoard>, BoardView)>,
}

pub struct BoardHandle<'a> {
    board: &'a mut Board,
}

type ExitCode = i32;

impl Board {
    pub fn new() -> Self {
        Self { internal: None }
    }

    pub fn start(
        &mut self,
        config: &BoardConfig,
        sketch: &Sketch,
    ) -> Result<BoardHandle<'_>, BoardError> {
        if self.internal.is_some() {
            return Err(BoardError::AlreadyRunning);
        }
        if !sketch.compiled() {
            return Err(BoardError::SketchNotCompiled);
        }

        let mut board: UniquePtr<OpaqueBoard> = unsafe { board_new() };
        let native_config = config.as_native();
        assert!(!board.is_null() && !native_config.is_null());

        // configure
        assert!(unsafe { board.pin_mut().configure(&native_config) });

        // attach
        assert!(unsafe { board.pin_mut().attach_sketch(&sketch.internal) });

        // start
        assert!(unsafe { board.pin_mut().start() });

        let mut bv: UniquePtr<OpaqueBoardView> = unsafe { board.pin_mut().view() };

        let bvstr = BoardView {
            digital_pins: DigitalPins {
                inner: {
                    config
                        .gpio_drivers
                        .iter()
                        .filter_map(|a| {
                            a.digital_driver.as_ref().map(|_| {
                                (
                                    a.pin_id as usize,
                                    DigitalPin {
                                        inner: UnsafeCell::new(unsafe {
                                            bv.pin_mut().get_pin(a.pin_id as usize)
                                        }),
                                        info: a.digital_driver.as_ref().unwrap().clone(),
                                    },
                                )
                            })
                        })
                        .collect()
                },
            },
            analog_pins: AnalogPins {
                inner: {
                    config
                        .gpio_drivers
                        .iter()
                        .filter_map(|a| {
                            a.analog_driver.as_ref().map(|_| {
                                (
                                    a.pin_id as usize,
                                    AnalogPin {
                                        inner: UnsafeCell::new(unsafe {
                                            bv.pin_mut().get_pin(a.pin_id as usize)
                                        }),
                                        info: a.analog_driver.as_ref().unwrap().clone(),
                                    },
                                )
                            })
                        })
                        .collect()
                },
            },
            uart_channels: UartChannels {
                inner: config
                    .uart_channels
                    .iter()
                    .enumerate()
                    .map(|(i, info)| UartChannel {
                        inner: UnsafeCell::new(unsafe { bv.pin_mut().get_uart(i) }),
                        info: info.clone(),
                    })
                    .collect(),
            },
            frame_buffers: FrameBuffers {
                inner: config
                    .frame_buffers
                    .iter()
                    .map(|fb| {
                        (
                            fb.key,
                            FrameBuffer {
                                inner: UnsafeCell::new(unsafe {
                                    bv.pin_mut().get_framebuffer(fb.key)
                                }),
                                info: fb.clone(),
                            },
                        )
                    })
                    .collect(),
            },
        };

        self.internal = Some((board, bvstr));
        Ok(self.handle().unwrap())
    }

    pub fn handle(&mut self) -> Option<BoardHandle<'_>> {
        if self.internal.is_some() {
            Some(BoardHandle { board: self })
        } else {
            None
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Self { internal: None }
    }
}

pub enum BoardHandleStatus {
    Running,
    Suspended,
}

impl BoardHandle<'_> {
    // unwrap is safe as we only exist when active
    #[doc(hidden)]
    fn internal(&mut self) -> &mut (UniquePtr<OpaqueBoard>, BoardView) {
        self.board.internal.as_mut().unwrap()
    }

    pub fn status() -> BoardHandleStatus {
        todo!()
    }

    pub fn suspend(&mut self) -> bool {
        unsafe { self.internal().0.pin_mut().suspend() }
    }

    pub fn resume(&mut self) -> bool {
        unsafe { self.internal().0.pin_mut().resume() }
    }

    pub fn view(&mut self) -> &BoardView {
        &self.internal().1
    }

    // Calls tick() once, if the sketch is still running we explicitly terminate
    pub fn stop(self) -> ExitCode {
        match self.tick() {
            Ok(mut handle) => {
                assert!(unsafe { handle.internal().0.pin_mut().terminate() });
                handle.board.internal = None;
                0
            }
            Err(exit_code) => exit_code,
        }
    }

    // Checks whether the sketch has died, returning the exit code if it has,
    pub fn tick(mut self) -> Result<Self, ExitCode> {
        match unsafe {
            let x = self.internal().0.pin_mut().tick();
            x
        } {
            ExitInfo {
                exit_code,
                exited: true,
            } => {
                self.board.internal = None;
                Err(exit_code)
            }
            _ => Ok(self),
        }
    }
}

#[derive(Debug)]
pub enum BoardError {
    SketchNotCompiled,
    AlreadyRunning,
}

impl Display for BoardError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for BoardError {}
