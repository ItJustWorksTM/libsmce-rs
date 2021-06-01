/*
 *  uuid.cxx
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

#include <cstring>
#include <filesystem>
#include <iostream>
#include <memory>
#include <string_view>
#include "libsmce-rs/src/ffi/definitions.rs"

auto uuid_generate() -> Uuid {
    const auto generated = smce::Uuid::generate();
    return into(generated);
}

auto uuid_to_hex(const Uuid& uuid) -> rust::String { return into(uuid).to_hex(); }

auto into(const smce::Uuid& native) -> Uuid {
    auto ret = Uuid{};

    std::memcpy(ret.bytes.data(), native.bytes.data(), native.bytes.size());

    return ret;
}

auto into(const Uuid& native) -> smce::Uuid {
    auto ret = smce::Uuid{};

    std::memcpy(ret.bytes.data(), native.bytes.data(), native.bytes.size());

    return ret;
}
