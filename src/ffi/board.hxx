/*
 *  board.hxx
 *  Copyright 2021 ItJustWorksTM
 *
 *  Licensed under the Apache LicenseVersion 2.0 (the "License");
 *  you may not use this file except in compliance with the License.
 *  You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 *  Unless required by applicable law or agreed to in writingsoftware
 *  distributed under the License is distributed on an "AS IS" BASIS,
 *  WITHOUT WARRANTIES OR CONDITIONS OF ANY KINDeither express or implied.
 *  See the License for the specific language governing permissions and
 *  limitations under the License.
 *
 */

#ifndef LIBSMCE_RS_BOARD_HXX
#define LIBSMCE_RS_BOARD_HXX

#include <memory>
#include <SMCE/Board.hpp>
#include <rust/cxx.h>
#include "board_config.hxx"
#include "board_view.hxx"
#include "sketch.hxx"
#include <iostream>

enum class OpaqueBoardStatus : uint8_t;

struct ExitInfo;

struct OpaqueBoard {
    smce::Board internal;
    bool exited = false;
    int exit_code = 0;

    OpaqueBoard();

    ~OpaqueBoard() { std::cout << "OPAQUE BOARD DIES \n"; }

    auto configure(const std::unique_ptr<OpaqueBoardConfig>& config) -> bool;
    auto attach_sketch(const std::unique_ptr<OpaqueSketch>& sketch) -> bool;
    auto tick() -> ExitInfo;
    auto status() const -> OpaqueBoardStatus;
    auto start() -> bool;
    auto suspend() -> bool;
    auto resume() -> bool;
    auto terminate() -> bool;
    auto reset() -> bool;
    auto view() -> std::unique_ptr<OpaqueBoardView>;
};

auto board_new() -> std::unique_ptr<OpaqueBoard>;

#endif // LIBSMCE_RS_BOARD_HXX
