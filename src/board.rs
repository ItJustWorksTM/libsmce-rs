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

use std::{cell::UnsafeCell, io, io::Read};

use cxx::UniquePtr;
use thiserror::Error;

use crate::board_config::BoardConfig;
use crate::board_view::{
    BoardView, FrameBuffer, FrameBuffers, GpioPin, Pins, UartChannel, UartChannels,
};
use crate::ffi::{board_new, ExitInfo, OpaqueBoard, OpaqueBoardStatus, OpaqueBoardView};
use crate::sketch::Sketch;

#[derive(Default)]
pub struct Board {
    internal: Option<(UnsafeCell<UniquePtr<OpaqueBoard>>, BoardView)>,
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
            pins: Pins {
                inner: {
                    config
                        .gpio_drivers
                        .iter()
                        .map(|a| {
                            (
                                a.pin_id as usize,
                                GpioPin {
                                    inner: UnsafeCell::new(unsafe {
                                        bv.pin_mut().get_pin(a.pin_id as usize)
                                    }),
                                    info: a.clone(),
                                },
                            )
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
                        inner: UnsafeCell::new({
                            let ret = unsafe { bv.pin_mut().get_uart(i) };
                            assert!(!ret.is_null());
                            ret
                        }),
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

        self.internal = Some((UnsafeCell::new(board), bvstr));
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

#[derive(Debug, Copy, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Status {
    Running,
    Suspended,
    Stopped,
}

impl BoardHandle<'_> {
    // unwrap is safe as we only exist when active
    #[doc(hidden)]
    fn internal(&self) -> &(UnsafeCell<UniquePtr<OpaqueBoard>>, BoardView) {
        self.board.internal.as_ref().unwrap()
    }

    pub fn status(&self) -> Status {
        match unsafe { (*self.internal().0.get()).pin_mut().status() } {
            OpaqueBoardStatus::Running => Status::Running,
            OpaqueBoardStatus::Suspended => Status::Suspended,
            _ => Status::Stopped,
        }
    }

    pub fn suspend(&self) -> bool {
        unsafe { (*self.internal().0.get()).pin_mut().suspend() }
    }

    pub fn resume(&self) -> bool {
        unsafe { (*self.internal().0.get()).pin_mut().resume() }
    }

    pub fn view(&self) -> &BoardView {
        &self.internal().1
    }

    pub fn log(&self) -> BoardLogReader {
        BoardLogReader { handle: self }
    }

    // Calls tick() once, if the sketch is still running we explicitly terminate
    pub fn stop(self) -> ExitCode {
        let exit_code = match self.tick() {
            Err(exit_code) => exit_code,
            _ => {
                unsafe { (*self.internal().0.get()).pin_mut().terminate() };
                0
            }
        };
        self.board.internal = None;
        exit_code
    }

    // Checks whether the sketch has died, returning the exit code if it has,
    // handle will still be valid, but in unstable state.
    pub fn tick(&self) -> Result<(), ExitCode> {
        match unsafe { (*self.internal().0.get()).pin_mut().tick() } {
            ExitInfo {
                exit_code,
                exited: true,
            } => Err(exit_code),
            _ => Ok(()),
        }
    }
}

#[derive(Clone, Copy, Error, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BoardError {
    #[error("Passed sketch is not compiled")]
    SketchNotCompiled,
    #[error("Board is already running a sketch")]
    AlreadyRunning,
}

pub struct BoardLogReader<'a> {
    handle: &'a BoardHandle<'a>,
}

impl Read for BoardLogReader<'_> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        Ok(unsafe { (*self.handle.internal().0.get()).pin_mut().runtime_log(buf) })
    }
}
