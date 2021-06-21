/*
 *  toolchain.rs
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
use std::fmt::Debug;
use std::io;
use std::io::Read;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use cxx::UniquePtr;
use thiserror::Error;

use crate::ffi::{toolchain_new, OpaqueToolchain, OpaqueToolchainResult};
use crate::sketch::Sketch;
use std::marker::PhantomData;

#[derive(Clone, Copy, Error, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum ToolchainError {
    #[error("Resource directory does not exist")]
    ResdirAbsent,
    #[error("Resource directory is a file")]
    ResdirFile,
    #[error("Resource directory empty")]
    ResdirEmpty,
    #[error("CMake not found in PATH")]
    CmakeNotFound,
    #[error("CMake unknown output")]
    CmakeUnknownOutput,
    #[error("CMake returned non 0 exit code")]
    CmakeFailing,
    #[error("Sketch path is invalid")]
    SketchInvalid,
    #[error("CMake configure failed")]
    ConfigureFailed,
    #[error("CMake build failed")]
    BuildFailed,
    #[error("Generic failure")]
    Generic,
}

impl Into<Result<(), ToolchainError>> for OpaqueToolchainResult {
    fn into(self) -> Result<(), ToolchainError> {
        Err(match self {
            OpaqueToolchainResult::Ok => return Ok(()),
            OpaqueToolchainResult::ResdirAbsent => ToolchainError::ResdirAbsent,
            OpaqueToolchainResult::ResdirFile => ToolchainError::ResdirFile,
            OpaqueToolchainResult::ResdirEmpty => ToolchainError::ResdirEmpty,
            OpaqueToolchainResult::CmakeNotFound => ToolchainError::CmakeNotFound,
            OpaqueToolchainResult::CmakeUnknownOutput => ToolchainError::CmakeUnknownOutput,
            OpaqueToolchainResult::CmakeFailing => ToolchainError::CmakeFailing,
            OpaqueToolchainResult::SketchInvalid => ToolchainError::SketchInvalid,
            OpaqueToolchainResult::ConfigureFailed => ToolchainError::ConfigureFailed,
            OpaqueToolchainResult::BuildFailed => ToolchainError::BuildFailed,
            _ => ToolchainError::Generic,
        })
    }
}

struct ToolchainInternal {
    internal: UnsafeCell<UniquePtr<OpaqueToolchain>>,
    finished: AtomicBool,
}

unsafe impl Sync for ToolchainInternal {}

pub struct Toolchain {
    internal: Arc<ToolchainInternal>,
    // Toolchain is not intended to be thread safe so explicitly block Sync
    _unsync: PhantomData<*const ()>,
}

// Toolchain is Send since it just stores an Arc
unsafe impl Send for Toolchain {}

pub struct BuildLogReader {
    internal: Arc<ToolchainInternal>,
}

impl BuildLogReader {
    pub fn disconnected(&self) -> bool {
        self.internal.finished.load(Ordering::SeqCst)
    }
}

impl Read for BuildLogReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let read = unsafe {
            (*self.internal.internal.get())
                .pin_mut()
                .read_build_log(buf)
        };

        Ok(read)
    }
}

impl Toolchain {
    pub fn new(resource_dir: &Path) -> (Toolchain, BuildLogReader) {
        let internal = Arc::new(ToolchainInternal {
            internal: UnsafeCell::new(unsafe {
                toolchain_new(resource_dir.to_str().unwrap_or(""))
            }),
            finished: AtomicBool::new(false),
        });

        (
            Toolchain {
                internal: internal.clone(),
                _unsync: PhantomData,
            },
            BuildLogReader { internal },
        )
    }

    fn check_suitable_env(&self) -> Result<(), ToolchainError> {
        unsafe {
            (*self.internal.internal.get())
                .pin_mut()
                .check_suitable_environment()
        }
        .into()
    }

    pub fn compile(self, sketch: &mut Sketch) -> Result<(), ToolchainError> {
        let ret = self.check_suitable_env().and_then(|_| {
            unsafe {
                (*self.internal.internal.get())
                    .pin_mut()
                    .compile(&mut sketch.internal)
            }
            .into()
        });

        self.internal.finished.store(true, Ordering::SeqCst);

        ret
    }
}
