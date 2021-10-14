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
#include "smce-rs/src/ffi/definitions.rs"
#include "sketch_config.hxx"

template <class T> auto conf(const T& vec) {
    auto ret = std::vector<std::string>{};
    std::transform(vec.begin(), vec.end(), std::back_inserter(ret),
                   [](const auto& str) { return std::string{str}; });
    return ret;
}

auto sketch_config_new(const SketchConfig& config) -> std::unique_ptr<OpaqueSketchConfig> {
    auto ret = smce::SketchConfig{};
    ret.fqbn = "fuckyou";
    ret.extra_board_uris = conf(config.extra_board_uris);

    std::transform(config.legacy_libs.begin(), config.legacy_libs.end(),
                   std::back_inserter(ret.legacy_preproc_libs), [](const auto& lib) {
                       return smce::SketchConfig::ArduinoLibrary{.name = std::string{lib}, .version = ""};
                   });

    ret.plugins = [&] {
        auto re = std::vector<smce::PluginManifest>{};
        for (const auto& plugin : config.plugins) {
            re.push_back(
                smce::PluginManifest{.name = std::string{plugin.name},
                                     .version = std::string{plugin.version},
                                     .depends = conf(plugin.depends),
                                     .needs_devices = conf(plugin.depends),
                                     .uri = std::string{plugin.uri},
                                     .patch_uri = std::string{plugin.patch_uri},
                                     .defaults = static_cast<smce::PluginManifest::Defaults>(plugin.defaults),
                                     .incdirs = conf(plugin.incdirs),
                                     .sources = conf(plugin.sources),
                                     .linkdirs = conf(plugin.linkdirs),
                                     .linklibs = conf(plugin.linklibs)});
        }
        return re;
    }();

    return std::make_unique<OpaqueSketchConfig>(ret);
}
