/*
 *  board.cxx
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

#include <functional>
#include "libsmce-rs/src/ffi/definitions.rs"
#include "board.hxx"
#include "board_config.hxx"
#include "sketch.hxx"

OpaqueBoard::OpaqueBoard()
    : internal{[&](int exit_code) {
          this->exit_code = exit_code;
          this->exited = true;
      }} {}

auto board_new() -> std::unique_ptr<OpaqueBoard> { return std::make_unique<OpaqueBoard>(); }

auto OpaqueBoard::configure(const std::unique_ptr<OpaqueBoardConfig>& config) -> bool {
    return internal.configure(*config);
}

auto OpaqueBoard::attach_sketch(const std::unique_ptr<OpaqueSketch>& sketch) -> bool {
    return internal.attach_sketch(*sketch);
}

auto OpaqueBoard::tick() -> ExitInfo {
    internal.tick();
    return {exited, exited};
}

auto OpaqueBoard::status() const -> BoardStatus { return static_cast<BoardStatus>(internal.status()); }

auto OpaqueBoard::start() -> bool {
    exited = false;
    exit_code = 0;
    return internal.start();
}

auto OpaqueBoard::suspend() -> bool { return internal.suspend(); }

auto OpaqueBoard::resume() -> bool { return internal.resume(); }

auto OpaqueBoard::terminate() -> bool { return internal.terminate(); }

auto OpaqueBoard::reset() -> bool { return internal.reset(); }

auto OpaqueBoard::view() -> std::unique_ptr<OpaqueBoardView> {
    return std::make_unique<OpaqueBoardView>(OpaqueBoardView{internal.view()});
}
