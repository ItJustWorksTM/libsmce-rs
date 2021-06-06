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

use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;
use std::pin::Pin;

use cxx::UniquePtr;

use crate::board_config::BoardConfig;
// use crate::board_view::BoardView;
use crate::board_view::BoardView;
use crate::ffi::{board_new, BoardStatus, ExitInfo, OpaqueBoard};
use crate::sketch::Sketch;

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

pub struct Board {
    internal: Option<UniquePtr<OpaqueBoard>>,
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
        let config = config.as_native();
        assert!(!board.is_null() && !config.is_null());

        // configure
        assert!(unsafe { board.pin_mut().configure(&config) });

        // attach
        assert!(unsafe { board.pin_mut().attach_sketch(&sketch.internal) });

        // start
        assert!(unsafe { board.pin_mut().start() });

        self.internal = Some(board);
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
    fn internal(&mut self) -> Pin<&mut OpaqueBoard> {
        self.board.internal.as_mut().unwrap().pin_mut()
    }

    pub fn status() -> BoardHandleStatus {
        todo!()
    }

    pub fn suspend(&mut self) -> bool {
        unsafe { self.internal().suspend() }
    }

    pub fn resume(&mut self) -> bool {
        unsafe { self.internal().resume() }
    }

    pub fn view(&mut self) -> BoardView<'_> {
        BoardView {
            view: unsafe { self.internal().view() },
            board: PhantomData,
        }
    }

    // Calls tick() once, if the sketch is still running we explicitly terminate
    pub fn stop(self) -> ExitCode {
        match self.tick() {
            Ok(mut handle) => {
                assert!(unsafe { handle.internal().terminate() });
                handle.board.internal = None;
                0
            }
            Err(exit_code) => exit_code,
        }
    }

    // Checks whether the sketch has died, returning the exit code if it has,
    pub fn tick(mut self) -> Result<Self, ExitCode> {
        match unsafe { self.internal().tick() } {
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
