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

use crate::ffi::{sketch_new, OpaqueSketch};
use crate::uuid::Uuid;

pub struct Sketch {
    pub(crate) internal: UniquePtr<OpaqueSketch>,
}

impl Sketch {
    pub fn new(source: &Path) -> Option<Sketch> {
        match if source.is_file() {
            source.parent()?
        } else {
            source
        }
        .to_str()
        {
            Some(source) => Some(Sketch {
                internal: unsafe { sketch_new(source) },
            }),
            _ => None,
        }
    }

    pub fn get_source(&self) -> &Path {
        Path::new(unsafe { self.internal.get_source() })
    }

    pub fn is_compiled(&self) -> bool {
        unsafe { self.internal.is_compiled() }
    }

    pub fn get_uuid(&self) -> Uuid {
        unsafe { self.internal.get_uuid() }
    }
}

impl Debug for Sketch {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("OpaqueSketch")
            .field("id", &self.get_uuid().to_hex())
            .field("path", &self.get_source().to_path_buf())
            .field("compiled", &self.is_compiled())
            .finish()
    }
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use crate::sketch::Sketch;

    fn make_sketch() -> (Sketch, &'static Path) {
        let path = Path::new("/home/ruthgerd/nonexistent.ino");
        let sketch = Sketch::new(&path);
        assert!(sketch.is_some());

        let sketch = sketch.unwrap();
        (sketch, path)
    }

    #[test]
    fn source_eq() {
        let (sketch, path) = make_sketch();

        assert_eq!(sketch.get_source(), path);
    }

    #[test]
    fn not_compiled() {
        let (sketch, _) = make_sketch();

        assert!(!sketch.is_compiled());
    }
}
