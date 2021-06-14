/*
 *  board_config.cxx
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

#include <optional>
#include <vector>
#include "libsmce-rs/src/ffi/definitions.rs"
#include "board_config.hxx"

#include <iostream>

auto board_config_new(const rust::Vec<uint16_t>& pins, rust::Vec<GpioDriverV> gpio_drivers,
                      rust::Vec<UartChannelV> uart_channels, rust::Vec<SecureDigitalStorageV> sd_cards,
                      rust::Vec<FrameBufferV> frame_buffers) -> std::unique_ptr<OpaqueBoardConfig> {
    auto ret = smce::BoardConfig{};
    std::copy(pins.begin(), pins.end(), std::back_inserter(ret.pins));

    std::transform(gpio_drivers.begin(), gpio_drivers.end(), std::back_inserter(ret.gpio_drivers),
                   [](const auto& gpio) {
                       auto ret = smce::BoardConfig::GpioDrivers{};

                       ret.pin_id = gpio.pin_id;

                       if (gpio.digital_driver) {
                           ret.digital_driver = {gpio.digital_driver->read, gpio.digital_driver->write};
                       }

                       if (gpio.analog_driver) {
                           ret.analog_driver = {gpio.analog_driver->read, gpio.analog_driver->write};
                       }

                       return ret;
                   });

    std::transform(uart_channels.begin(), uart_channels.end(), std::back_inserter(ret.uart_channels),
                   [](const auto& uart) {
                       auto ret = smce::BoardConfig::UartChannel{};

                       if (uart.rx_pin_override)
                           ret.rx_pin_override = *uart.rx_pin_override;

                       if (uart.tx_pin_override)
                           ret.tx_pin_override = *uart.tx_pin_override;

                       ret.baud_rate = uart.baud_rate;
                       ret.rx_buffer_length = uart.rx_buffer_length;
                       ret.tx_buffer_length = uart.tx_buffer_length;
                       ret.flushing_threshold = uart.flushing_threshold;

                     return ret;
                   });

    std::transform(sd_cards.begin(), sd_cards.end(), std::back_inserter(ret.sd_cards), [](const auto& sd) {
        auto ret = smce::BoardConfig::SecureDigitalStorage{};
        ret.cspin = sd.cspin;
        ret.root_dir = std::string{sd.root_dir.data(), sd.root_dir.size()};
        return ret;
    });

    std::transform(frame_buffers.begin(), frame_buffers.end(), std::back_inserter(ret.frame_buffers),
                   [](const auto& fb) {
                       auto ret = smce::BoardConfig::FrameBuffer{};
                       ret.key = fb.key;
                       ret.direction = fb.direction ? smce::BoardConfig::FrameBuffer::Direction::in
                                                    : smce::BoardConfig::FrameBuffer::Direction::out;
                       return ret;
                   });

    return std::make_unique<OpaqueBoardConfig>(ret);
}
