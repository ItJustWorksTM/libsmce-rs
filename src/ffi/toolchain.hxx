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

#ifndef LIBSMCE_RS_TOOLCHAIN_HXX
#define LIBSMCE_RS_TOOLCHAIN_HXX

#include <memory>
#include <SMCE/Toolchain.hpp>
#include <rust/cxx.h>
#include "sketch.hxx"

enum class ToolchainResult : uint8_t;

using OpaqueLockedLog = smce::Toolchain::LockedLog;

struct OpaqueToolchain : public smce::Toolchain {
    using smce::Toolchain::Toolchain;

    auto resource_dir() const -> rust::Str;
    auto cmake_path() const -> rust::Str;
    auto check_suitable_environment() -> ToolchainResult;
    auto compile(std::unique_ptr<OpaqueSketch>& sketch) -> ToolchainResult;
    auto read_build_log() -> rust::String;
};

auto toolchain_new(rust::Str resource_dir) -> std::unique_ptr<OpaqueToolchain>;

#endif // LIBSMCE_RS_TOOLCHAIN_HXX
