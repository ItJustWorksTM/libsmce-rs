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

use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::from_utf8;

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    let mut cmake_out = out_dir.clone();
    cmake_out.push("smce-rs/cmake");

    fs::create_dir_all(&out_dir).unwrap();

    let configure_output = Command::new("cmake")
        .args(&["-B", cmake_out.to_str().unwrap()])
        .envs(std::env::vars())
        .current_dir("build")
        .output()
        .unwrap();

    if !configure_output.status.success() {
        println!("stdout: {}", from_utf8(&configure_output.stdout).unwrap());
        println!("stderr: {}", from_utf8(&configure_output.stderr).unwrap());

        return Err("Failed to find native libSMCE".into());
    }

    let stdout = from_utf8(&configure_output.stderr).unwrap();

    let mut include_dirs = vec!["src/ffi"];
    let mut cargo_directives = vec![];
    for line in stdout.lines() {
        if line.starts_with("cargo:") {
            cargo_directives.push(line);
        } else if let Some(path) = line.strip_prefix("header:") {
            include_dirs.push(path);
        }
    }

    assert!(!cargo_directives.is_empty());

    let source_files = vec![
        "src/ffi/sketch.cxx",
        "src/ffi/uuid.cxx",
        "src/ffi/board_config.cxx",
        "src/ffi/board.cxx",
        "src/ffi/toolchain.cxx",
        "src/ffi/board_view.cxx",
        "src/ffi/sketch_config.cxx",
    ];
    cxx_build::bridge("src/ffi/definitions.rs")
        .includes(&include_dirs)
        .files(&source_files)
        .flag_if_supported("-std=c++2a")
        .flag_if_supported("/std:c++2a")
        .flag_if_supported("-std=c++20")
        .flag_if_supported("/std:c++20")
        .compile("smce-rs");

    for directive in cargo_directives {
        println!("{}", directive);
    }

    println!("cargo:rustc-env=SMCE_TEST_HOME={}", out_dir.display());

    for path in fs::read_dir(&Path::new("src/ffi")).unwrap() {
        let path = path.unwrap();
        println!("cargo:rerun-if-changed={}", path.path().display());
    }

    println!("cargo:rerun-if-changed=build/CMakeLists.txt");

    Ok(())
}
