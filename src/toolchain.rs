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

use std::fmt::Debug;
use std::{cell::UnsafeCell, process::Command};
use std::{
    fs::DirBuilder,
    sync::atomic::{AtomicBool, Ordering},
};
use std::{fs::File, io::Read};
use std::{io, path::PathBuf};
use std::{io::Write, sync::Arc};

use cxx::UniquePtr;
use thiserror::Error;

use crate::ffi::{toolchain_new, OpaqueToolchain, OpaqueToolchainResult};
use crate::sketch::Sketch;
use std::marker::PhantomData;

#[derive(Clone, Copy, Error, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum ToolchainError {
    #[error("Failed to extract Resources")]
    ResDirExtract,
    #[error("Resource directory does not exist")]
    ResdirAbsent,
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
    home_dir: PathBuf,
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
    pub fn new<S: Into<PathBuf>>(
        home_dir: S,
    ) -> Result<(Toolchain, BuildLogReader), ToolchainError> {
        let home_dir = home_dir.into();

        let internal = Arc::new(ToolchainInternal {
            internal: UnsafeCell::new(unsafe { toolchain_new(home_dir.to_str().unwrap_or("")) }),
            finished: AtomicBool::new(false),
        });

        Ok((
            Toolchain {
                internal: internal.clone(),
                home_dir,
                _unsync: PhantomData,
            },
            BuildLogReader { internal },
        ))
    }

    pub fn compile(self, sketch: &mut Sketch) -> Result<(), ToolchainError> {
        let resource_bytes = include_bytes!(concat!(env!("OUT_DIR"), "/SMCE_Resources.zip"));

        unsafe {
            (&mut *(self.internal.internal.get()))
                .pin_mut()
                .check_suitable_environment();
        }

        if !self.home_dir.exists() {
            DirBuilder::new()
                .recursive(true)
                .create(&self.home_dir)
                .map_err(|_| ToolchainError::ResdirAbsent)?;
        }

        let mut zip = self.home_dir.clone();
        zip.push("SMCE_Resources.zip");

        let mut file = File::create(&zip).map_err(|_| ToolchainError::ResDirExtract)?;

        file.write_all(resource_bytes)
            .map_err(|_| ToolchainError::ResDirExtract)?;

        Command::new("cmake")
            .arg("-E")
            .arg("tar")
            .arg("xf")
            .arg(&zip)
            .current_dir(&self.home_dir)
            .output()
            .map_err(|err| match err.kind() {
                std::io::ErrorKind::NotFound => ToolchainError::CmakeNotFound,
                _ => ToolchainError::ResDirExtract,
            })?;

        let ret = unsafe {
            (*self.internal.internal.get())
                .pin_mut()
                .compile(&mut sketch.internal)
        }
        .into();

        self.internal.finished.store(true, Ordering::SeqCst);

        ret
    }
}
