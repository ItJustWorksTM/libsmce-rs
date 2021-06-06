/*
 *  uuid.rs
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

use crate::ffi::Uuid;
use crate::ffi::{uuid_generate, uuid_to_hex};

impl Uuid {
    pub fn generate() -> Uuid {
        unsafe { uuid_generate() }
    }

    pub fn to_hex(&self) -> String {
        unsafe { uuid_to_hex(self) }
    }
}

#[test]
fn test() {
    let uuid = Uuid::generate();

    println!("{:x?}", uuid.bytes);
}
