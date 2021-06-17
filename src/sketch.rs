/*
 *  sketch.rs
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

use crate::ffi::{sketch_new, OpaqueSketch, Uuid};
use crate::sketch_config::SketchConfig;

unsafe impl Send for OpaqueSketch {}

pub struct Sketch {
    pub(crate) internal: UniquePtr<OpaqueSketch>,
    pub(crate) config: SketchConfig,
}

impl Sketch {
    // Takes path a sketch .ino or a folder containing a .ino file
    pub fn new(source: &Path, config: SketchConfig) -> Option<Sketch> {
        source.to_str().map(|source| Sketch {
            internal: unsafe { sketch_new(source, &config.as_opaque().as_ref().unwrap()) },
            config,
        })
    }

    pub fn source(&self) -> &Path {
        Path::new(unsafe { self.internal.get_source() })
    }

    pub fn compiled(&self) -> bool {
        unsafe { self.internal.is_compiled() }
    }

    pub fn uuid(&self) -> Uuid {
        unsafe { self.internal.get_uuid() }
    }

    pub fn config(&self) -> &SketchConfig {
        &self.config
    }
}

impl Debug for Sketch {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Sketch")
            .field("id", &self.uuid().to_hex())
            .field("path", &self.source().to_path_buf())
            .field("compiled", &self.compiled())
            .field("config", &self.config)
            .finish()
    }
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use crate::sketch::Sketch;
    use crate::sketch_config::SketchConfig;

    fn make_sketch() -> (Sketch, &'static Path) {
        // TODO: NOO
        let path = Path::new("/home/ruthgerd/nonexistent.ino");
        let sketch = Sketch::new(&path, SketchConfig::default());
        assert!(sketch.is_some());

        let sketch = sketch.unwrap();
        (sketch, path)
    }

    #[test]
    fn source_eq() {
        let (sketch, path) = make_sketch();

        assert_eq!(sketch.source(), path);
    }

    #[test]
    fn not_compiled() {
        let (sketch, _) = make_sketch();

        assert!(!sketch.compiled());
    }
}
