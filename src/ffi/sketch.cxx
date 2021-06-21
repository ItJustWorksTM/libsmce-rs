/*
 *  sketch.cxx
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

#include <iostream>
#include <memory>
#include <string_view>
#include <SMCE/SMCE_fs.hpp>
#include "smce-rs/src/ffi/definitions.rs"
#include "sketch.hxx"

using smce::Sketch;

auto sketch_new(rust::Str source, const OpaqueSketchConfig& config) -> std::unique_ptr<OpaqueSketch> {
    auto src_sv = std::string_view{source.data(), source.size()};
    auto sk = OpaqueSketch{smce::stdfs::path{src_sv}, config};

    return std::make_unique<OpaqueSketch>(sk);
}

auto OpaqueSketch::get_source() const -> rust::Str { return {Sketch::get_source().c_str()}; }

auto OpaqueSketch::is_compiled() const -> bool { return Sketch::is_compiled(); }

auto OpaqueSketch::get_uuid() const -> Uuid { return into(Sketch::get_uuid()); }
