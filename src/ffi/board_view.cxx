/*
 *  board_view.cxx
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

#include "board_view.hxx"

#include <iostream>

using smce::BoardView;

template<class T>
auto reset_if(T&& ptr) {
    if (!ptr->exists())
        ptr.reset();
    return std::forward<T>(ptr);
}

auto OpaqueBoardView::get_framebuffer(size_t id) -> std::unique_ptr<OpaqueFramebuffer> {
    return reset_if(std::make_unique<OpaqueFramebuffer>(OpaqueFramebuffer{frame_buffers[id]}));;
}

auto OpaqueBoardView::get_uart(size_t id) -> std::unique_ptr<OpaqueVirtualUart> {
    return reset_if(std::make_unique<OpaqueVirtualUart>(OpaqueVirtualUart{uart_channels[id]}));
}

auto OpaqueBoardView::get_pin(size_t id) -> std::unique_ptr<OpaqueVirtualPin> {
    return std::make_unique<OpaqueVirtualPin>(OpaqueVirtualPin{pins[id]});
}
auto OpaqueBoardView::clone() -> std::unique_ptr<OpaqueBoardView> { return std::make_unique<OpaqueBoardView>(*this); }

auto OpaqueVirtualPin::is_digital() -> bool { return digital().exists(); }
auto OpaqueVirtualPin::is_analog() -> bool { return analog().exists(); }
auto OpaqueVirtualPin::analog_write(uint16_t val) -> void { analog().write(val); }
auto OpaqueVirtualPin::analog_read() -> uint16_t { return analog().read(); }
auto OpaqueVirtualPin::digital_write(bool val) -> void { digital().write(val); }
auto OpaqueVirtualPin::digital_read() -> bool { return digital().read(); }
auto OpaqueVirtualPin::clone() -> std::unique_ptr<OpaqueVirtualPin> { return std::make_unique<OpaqueVirtualPin>(*this); }

auto OpaqueVirtualUart::readable() -> size_t { return tx().size(); }
auto OpaqueVirtualUart::max_read() -> size_t { return tx().max_size(); }
auto OpaqueVirtualUart::max_write() -> size_t { return rx().max_size(); }
auto OpaqueVirtualUart::read(rust::Slice<uint8_t> buf) -> size_t {
    std::cout << "buf size: " << buf.size() << "\n";
    auto tx = smce::VirtualUart::tx();
    std::cout << "a\n";
    std::cout << "exists: " << tx.exists() << std::endl;

    *buf.data() = 69;
    // ??

    std::span buf_span = {reinterpret_cast<char*>(buf.data()), buf.size() - 1};
    std::cout << "[";
    for (auto x : buf_span)
        std::cout << static_cast<int>(x) << ", ";
    std::cout << "] ! CPP " << buf_span.size() << std::endl;
    static_assert(sizeof(uint8_t) == sizeof(char));
    std::cout << "wtf: " << tx.exists() << std::endl;
    auto tmp_buf = std::array<char, 4>{};
    auto read = tx.read(tmp_buf);
    std::cout << "cpp read: " << read << std::endl;
    return read;
}
auto OpaqueVirtualUart::write(rust::Slice<const uint8_t> buf) -> size_t {
    return rx().write({reinterpret_cast<const char*>(buf.data()), buf.size()});
}
auto OpaqueVirtualUart::front() -> uint8_t { return tx().front(); }
auto OpaqueVirtualUart::clone() -> std::unique_ptr<OpaqueVirtualUart> { return std::make_unique<OpaqueVirtualUart>(*this); }

auto OpaqueFramebuffer::needs_horizontal_flip() -> bool { return needs_horizontal_flip(); }
auto OpaqueFramebuffer::needs_vertical_flip() -> bool { return needs_vertical_flip(); }
auto OpaqueFramebuffer::width() -> uint16_t { return width(); }
auto OpaqueFramebuffer::height() -> uint16_t { return height(); }
auto OpaqueFramebuffer::freq() -> uint8_t { return freq(); }
auto OpaqueFramebuffer::write_rgb888(rust::Slice<const uint8_t> buf) -> bool {
    return smce::FrameBuffer::write_rgb888({reinterpret_cast<const std::byte*>(buf.data()), buf.size()});
}
auto OpaqueFramebuffer::write_rgb444(rust::Slice<const uint8_t> buf) -> bool {
    return smce::FrameBuffer::write_rgb444({reinterpret_cast<const std::byte*>(buf.data()), buf.size()});
}
