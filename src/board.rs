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

use std::convert::{TryFrom, TryInto};
use std::marker::PhantomData;
use std::pin::Pin;

use cxx::UniquePtr;

use crate::board_config::BoardConfig;
use crate::board_view::BoardView;
use crate::ffi::{board_new, BoardStatus, OpaqueBoard};
use crate::sketch::Sketch;

pub struct BoardVendor {
    internal: UniquePtr<OpaqueBoard>,
    config: BoardConfig,
}

pub struct Board<'a, 'b> {
    board: &'a mut BoardVendor,
    sketch: &'b Sketch,
}

impl BoardVendor {
    fn inner_mut(&mut self) -> Pin<&mut OpaqueBoard> {
        unsafe { self.internal.pin_mut() }
    }

    pub fn new(config: BoardConfig) -> Self {
        unsafe {
            let mut board = board_new();
            board.pin_mut().configure(&config.clone().into());
            BoardVendor {
                internal: board,
                config,
            }
        }
    }

    pub fn use_sketch<'a, 'b>(&'a mut self, sketch: &'b Sketch) -> Option<Board<'a, 'b>> {
        if sketch.is_compiled() && unsafe { self.inner_mut().attach_sketch(&sketch.internal) } {
            Some(Board {
                board: self,
                sketch,
            })
        } else {
            None
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Status {
    Ready,
    Running,
    Suspended,
}

type ExitCode = i32;

impl TryFrom<BoardStatus> for Status {
    type Error = ();

    fn try_from(value: BoardStatus) -> Result<Self, Self::Error> {
        match value {
            BoardStatus::Running => Ok(Status::Running),
            BoardStatus::Suspended => Ok(Status::Suspended),
            BoardStatus::Stopped | BoardStatus::Configured => Ok(Status::Ready),
            _ => Err(()),
        }
    }
}

impl<'a, 'b> Board<'a, 'b> {
    fn inner_mut(&mut self) -> Pin<&mut OpaqueBoard> {
        self.board.inner_mut()
    }

    fn inner(&self) -> &UniquePtr<OpaqueBoard> {
        &self.board.internal
    }

    pub fn status(&self) -> Status {
        unsafe { self.inner().status() }
            .try_into()
            .expect("Invalid state")
    }

    pub fn tick(&mut self) -> Result<(), ExitCode> {
        let exit_info = unsafe { self.inner_mut().tick() };
        if !exit_info.exited {
            Ok(())
        } else {
            Err(exit_info.exit_code)
        }
    }

    pub fn start(&mut self) -> bool {
        unsafe { self.inner_mut().start() }
    }

    pub fn suspend(&mut self) -> bool {
        unsafe { self.inner_mut().suspend() }
    }

    pub fn resume(&mut self) -> bool {
        unsafe { self.inner_mut().resume() }
    }

    pub fn terminate(&mut self) -> bool {
        unsafe {
            let ret = self.inner_mut().terminate();
            // Workaround for libSMCE crash when restarting a sketch.
            if ret {
                assert!(self.inner_mut().reset());
                let new_config = self.board.config.clone().into();
                assert!(self.inner_mut().configure(&new_config));
                let sketch = &self.sketch.internal;
                assert!(self.inner_mut().attach_sketch(sketch));
            }
            ret
        }
    }

    pub fn view<'s>(&'s mut self) -> Option<BoardView<'s, 'a, 'b>> {
        match self.status() {
            Status::Running | Status::Suspended => Some(BoardView {
                view: unsafe { self.inner_mut().view() },
                board: PhantomData,
            }),
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use crate::board::BoardVendor;
    use crate::board_config::BoardConfig;
    use crate::sketch::Sketch;

    #[test]
    fn basics() {
        let mut vendor = BoardVendor::new(BoardConfig::default());
        let sketch = Sketch::new(Path::new(""));
        assert!(sketch.is_some());
        let sketch = sketch.unwrap();
        let board = vendor.use_sketch(&sketch);
        assert!(board.is_none());
    }
}
