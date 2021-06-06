/*
 *  sketch_config.cxx
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

#include <algorithm>
#include "libsmce-rs/src/ffi/definitions.rs"
#include "sketch_config.hxx"

auto conf(const rust::Vec<rust::String>& vec) {
    auto ret = std::vector<std::string>{};
    std::transform(vec.begin(), vec.end(), std::back_inserter(ret),
                   [](const auto& str) { return std::string{str}; });
    return ret;
}

auto conf(const LibraryV& libs) {
    auto ret = std::vector<smce::SketchConfig::Library>{};
    std::transform(libs.free.begin(), libs.free.end(), std::back_inserter(ret), [](const auto& free) {
        return smce::SketchConfig::FreestandingLibrary{
            std::string{free.include_dir}, std::string{free.include_dir}, conf(free.compile_defs)};
    });
    std::transform(libs.remote.begin(), libs.remote.end(), std::back_inserter(ret), [](const auto& remote) {
        return smce::SketchConfig::RemoteArduinoLibrary{std::string{remote.name},
                                                        std::string{remote.version}};
    });
    std::transform(libs.local.begin(), libs.local.end(), std::back_inserter(ret), [](const auto& local) {
        return smce::SketchConfig::LocalArduinoLibrary{
            std::string{local.root_dir.data(), local.root_dir.size()}, std::string{local.patch_for}};
    });
    return ret;
}

auto sketch_config_new(const rust::String& fqbn, const rust::Vec<rust::String>& extra_board_uris,
                       LibraryV preproc_libs, LibraryV complink_libs,
                       const rust::Vec<rust::String>& extra_compile_defs,
                       const rust::Vec<rust::String>& extra_compile_opts)
    -> std::unique_ptr<OpaqueSketchConfig> {
    auto ret = smce::SketchConfig{};
    ret.fqbn = std::string{fqbn};
    ret.extra_board_uris = conf(extra_board_uris);
    ret.preproc_libs = conf(preproc_libs);
    ret.complink_libs = conf(complink_libs);
    ret.extra_compile_defs = conf(extra_compile_defs);
    ret.extra_compile_opts = conf(extra_compile_opts);
    return std::make_unique<OpaqueSketchConfig>(ret);
}
