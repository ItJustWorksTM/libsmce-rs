/*
 *  toolchain.cxx
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

#include <SMCE/SMCE_fs.hpp>
#include "libsmce-rs/src/ffi/definitions.rs"
#include "toolchain.hxx"

#include <iostream>

using smce::Toolchain;

auto toolchain_new(rust::Str resource_dir) -> std::unique_ptr<OpaqueToolchain> {
    const auto res_sv = std::string_view{resource_dir.data(), resource_dir.size()};
    return std::make_unique<OpaqueToolchain>(smce::stdfs::path{res_sv});
}

auto OpaqueToolchain::resource_dir() const -> rust::Str { return {Toolchain::resource_dir().c_str()}; }
auto OpaqueToolchain::cmake_path() const -> rust::Str { return {Toolchain::cmake_path().c_str()}; }
auto OpaqueToolchain::check_suitable_environment() -> ToolchainResult {
    return static_cast<ToolchainResult>(Toolchain::check_suitable_environment().value());
}

auto OpaqueToolchain::compile(std::unique_ptr<OpaqueSketch>& sketch) -> ToolchainResult {
    const auto ret = Toolchain::compile(*sketch);
    return static_cast<ToolchainResult>(ret.value());
}
