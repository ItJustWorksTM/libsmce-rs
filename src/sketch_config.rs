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

#[derive(Debug, Default)]
pub struct SketchConfig {
    pub fqbn: String,
    pub extra_board_uris: Vec<String>,
    pub preproc_libs: Vec<Library>,
    pub complink_libs: Vec<Library>,
    pub extra_compile_defs: Vec<String>,
    pub extra_compile_opts: Vec<String>,
}
