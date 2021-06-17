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
use std::fmt::{Debug, Formatter};
use std::io::{ErrorKind, Read};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{fmt, io};

use cxx::UniquePtr;

use crate::ffi::{toolchain_new, OpaqueToolchain, ToolchainResult};
use crate::sketch::Sketch;

unsafe impl Send for OpaqueToolchain {}

// This is slightly dangerous, but is safe if the log reader only uses `read_build_log` as that is explicitly thread safe.
unsafe impl Sync for ToolchainInternal {}

pub fn toolchain(resource_dir: &Path) -> (Toolchain, BuildLogReader) {
    let internal = Arc::new(ToolchainInternal {
        internal: UnsafeCell::new(unsafe { toolchain_new(resource_dir.to_str().unwrap_or("")) }),
        finished: AtomicBool::new(false),
    });

    (
        Toolchain {
            internal: internal.clone(),
        },
        BuildLogReader {
            internal: internal.clone(),
        },
    )
}

struct ToolchainInternal {
    internal: UnsafeCell<UniquePtr<OpaqueToolchain>>,
    finished: AtomicBool,
}

pub struct Toolchain {
    internal: Arc<ToolchainInternal>,
}

pub struct BuildLogReader {
    internal: Arc<ToolchainInternal>,
}

impl Read for BuildLogReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let read = unsafe {
            (*self.internal.internal.get())
                .pin_mut()
                .read_build_log(buf)
        };
        // println!("read {}", read);

        // Only bail if we have no bytes to read and the compile has finished
        if read == 0 && self.internal.finished.load(Ordering::SeqCst) {
            Err(io::Error::new(
                ErrorKind::ConnectionAborted,
                "Compile finished",
            ))
        } else {
            Ok(read)
        }
    }
}

impl Toolchain {
    pub fn compile(self, sketch: &mut Sketch) -> Result<(), ToolchainResult> {
        let ret = match unsafe {
            (*self.internal.internal.get())
                .pin_mut()
                .check_suitable_environment()
        } {
            ToolchainResult::Ok => {
                match unsafe {
                    (*self.internal.internal.get())
                        .pin_mut()
                        .compile(&mut sketch.internal)
                } {
                    ToolchainResult::Ok => Ok(()),
                    err => Err(err),
                }
            }
            e => Err(e),
        };
        self.internal.finished.store(true, Ordering::SeqCst);
        ret
    }
}
