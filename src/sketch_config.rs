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

use std::path::PathBuf;

use cxx::UniquePtr;

use crate::ffi::{
    sketch_config_new, FreestandingLibraryV, LibraryV, LocalArduinoLibraryV, OpaqueSketchConfig,
    RemoteArduinoLibraryV,
};

#[derive(Debug)]
pub enum Library {
    FreestandingLibrary {
        include_dir: PathBuf,
        archive_path: PathBuf,
        compile_defs: Vec<String>,
    },
    RemoteArduinoLibrary {
        name: String,
        version: String,
    },
    LocalArduinoLibrary {
        root_dir: PathBuf,
        patch_for: String,
    },
}

fn as_simple(libs: &[Library]) -> LibraryV {
    let mut ret = LibraryV::default();
    for lib in libs {
        match lib {
            Library::FreestandingLibrary {
                include_dir,
                archive_path,
                compile_defs,
            } => {
                ret.free.push(FreestandingLibraryV {
                    include_dir: include_dir.to_str().unwrap(),
                    archive_path: archive_path.to_str().unwrap(),
                    compile_defs,
                });
            }
            Library::RemoteArduinoLibrary { name, version } => {
                ret.remote.push(RemoteArduinoLibraryV { name, version });
            }
            Library::LocalArduinoLibrary {
                root_dir,
                patch_for,
            } => {
                ret.local.push(LocalArduinoLibraryV {
                    root_dir: root_dir.to_str().unwrap(),
                    patch_for,
                });
            }
        }
    }
    ret
}

#[derive(Debug)]
pub struct SketchConfig {
    pub fqbn: String,
    pub extra_board_uris: Vec<String>,
    pub preproc_libs: Vec<Library>,
    pub complink_libs: Vec<Library>,
    pub extra_compile_defs: Vec<String>,
    pub extra_compile_opts: Vec<String>,
}

impl Default for SketchConfig {
    fn default() -> Self {
        Self {
            fqbn: "arduino:avr:nano".to_string(),
            extra_board_uris: Default::default(),
            preproc_libs: Default::default(),
            complink_libs: Default::default(),
            extra_compile_defs: Default::default(),
            extra_compile_opts: Default::default(),
        }
    }
}

impl SketchConfig {
    pub(crate) fn as_opaque(&self) -> UniquePtr<OpaqueSketchConfig> {
        unsafe {
            sketch_config_new(
                &self.fqbn,
                &self.extra_board_uris,
                as_simple(&self.preproc_libs),
                as_simple(&self.complink_libs),
                &self.extra_compile_defs,
                &self.extra_compile_opts,
            )
        }
    }
}
