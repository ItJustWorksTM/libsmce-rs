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

use std::fmt;
use std::fmt::{Debug, Formatter};
use std::path::Path;

use cxx::UniquePtr;

use crate::ffi::{toolchain_new, OpaqueToolchain, ToolchainResult};
use crate::sketch::Sketch;

pub struct Toolchain {
    internal: UniquePtr<OpaqueToolchain>,
}

impl Toolchain {
    pub fn new(resource_dir: &Path) -> Result<Self, ToolchainResult> {
        let mut internal = unsafe { toolchain_new(resource_dir.to_str().unwrap_or("")) };
        let res = unsafe { internal.pin_mut().check_suitable_environment() };
        match res {
            ToolchainResult::Ok => Ok(Toolchain { internal }),
            _ => Err(res),
        }
    }

    pub fn compile(&mut self, sketch: &mut Sketch) -> Result<(), ToolchainResult> {
        let res = unsafe { self.internal.pin_mut().compile(&mut sketch.internal) };
        match res {
            ToolchainResult::Ok => Ok(()),
            _ => Err(res),
        }
    }

    pub fn resource_dir(&self) -> &Path {
        unsafe { Path::new(self.internal.resource_dir()) }
    }

    pub fn cmake_path(&self) -> &Path {
        unsafe { Path::new(self.internal.cmake_path()) }
    }
}

impl Debug for Toolchain {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("OpaqueToolchain")
            .field("resource_dir", &self.resource_dir().to_path_buf())
            .field("cmake_path", &self.cmake_path().to_path_buf())
            .finish()
    }
}
