/*
 *  build.rs
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

use std::fs;
use std::path::Path;

use cxx_build;

fn main() {
    let files = vec![
        "src/ffi/sketch.cxx",
        "src/ffi/uuid.cxx",
        "src/ffi/board_config.cxx",
        "src/ffi/board.cxx",
        "src/ffi/toolchain.cxx",
        "src/ffi/board_view.cxx",
    ];
    cxx_build::bridge("src/ffi/definitions.rs")
        .include("libsmce/include")
        .include("src/ffi")
        .files(&files)
        .flag_if_supported("-std=c++20")
        .compile("libsmce-rs");

    for path in fs::read_dir(&Path::new("src/ffi")).unwrap() {
        let path = path.unwrap();
        println!("cargo:rerun-if-changed={}", path.path().display());
    }

    println!("cargo:rustc-link-search=native=libsmce/lib");
    println!("cargo:rustc-link-search=native=/usr/lib");

    println!("cargo:rustc-link-lib=static=SMCE_static");
    println!("cargo:rustc-link-lib=static=boost_filesystem");
}
