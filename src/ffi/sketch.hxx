/*
 *  sketch.hxx
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

#ifndef LIBSMCE_RS_SKETCH_HXX
#define LIBSMCE_RS_SKETCH_HXX

#include <rust/cxx.h>
#include <SMCE/Sketch.hpp>

struct Uuid;

struct OpaqueSketch : public smce::Sketch {
    using smce::Sketch::Sketch;

    auto get_source() const -> rust::Str;

    auto is_compiled() const -> bool;

    auto get_uuid() const -> Uuid;
};

auto sketch_new(rust::Str source) -> std::unique_ptr<OpaqueSketch>;

#endif // LIBSMCE_RS_SKETCH_HXX
