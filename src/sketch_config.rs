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

use cxx::UniquePtr;

use crate::ffi::{sketch_config_new, OpaqueSketchConfig};

pub use crate::ffi::PluginManifest;
pub use crate::ffi::SketchConfig;

impl SketchConfig {
    pub(crate) fn as_opaque(&self) -> UniquePtr<OpaqueSketchConfig> {
        unsafe { sketch_config_new(self) }
    }
}
