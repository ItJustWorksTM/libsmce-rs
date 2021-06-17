/*
 *  board_view.hxx
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

#ifndef LIBSMCE_RS_BOARD_VIEW_HXX
#define LIBSMCE_RS_BOARD_VIEW_HXX

#include <memory>
#include <SMCE/BoardView.hpp>
#include <rust/cxx.h>

struct OpaqueVirtualPin : smce::VirtualPin {
    auto is_digital() -> bool;
    auto is_analog() -> bool;
    auto digital_write(bool) -> void;
    auto digital_read() -> bool;
    auto analog_write(uint16_t) -> void;
    auto analog_read() -> uint16_t;
    auto clone() -> std::unique_ptr<OpaqueVirtualPin>;
};

struct OpaqueVirtualUart : smce::VirtualUart {
    auto readable() -> size_t;
    auto max_read() -> size_t;
    auto max_write() -> size_t;
    auto read(rust::Slice<uint8_t> buf) -> size_t;
    auto write(rust::Slice<const uint8_t> buf) -> size_t;
    auto front() -> uint8_t;
    auto clone() -> std::unique_ptr<OpaqueVirtualUart>;
};

struct OpaqueFramebuffer : smce::FrameBuffer {
    auto needs_horizontal_flip() -> bool;
    auto needs_vertical_flip() -> bool;
    auto width() -> uint16_t;
    auto height() -> uint16_t;
    auto freq() -> uint8_t;
    auto write_rgb888(rust::Slice<const uint8_t> buf) -> bool;
    auto write_rgb444(rust::Slice<const uint8_t> buf) -> bool;
};

struct OpaqueBoardView : smce::BoardView {
    auto get_framebuffer(size_t id) -> std::unique_ptr<OpaqueFramebuffer>;
    auto get_uart(size_t id) -> std::unique_ptr<OpaqueVirtualUart>;
    auto get_pin(size_t id) -> std::unique_ptr<OpaqueVirtualPin>;
    auto clone() -> std::unique_ptr<OpaqueBoardView>;
};

#endif // LIBSMCE_RS_BOARD_VIEW_HXX
