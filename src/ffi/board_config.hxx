/*
 *  board_config.hxx
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

#ifndef LIBSMCE_RS_BOARD_CONFIG_HXX
#define LIBSMCE_RS_BOARD_CONFIG_HXX

#include <memory>
#include <SMCE/BoardConf.hpp>
#include <rust/cxx.h>

using OpaqueBoardConfig = smce::BoardConfig;

auto board_config_new() -> std::unique_ptr<OpaqueBoardConfig>;

#endif // LIBSMCE_RS_BOARD_CONFIG_HXX
