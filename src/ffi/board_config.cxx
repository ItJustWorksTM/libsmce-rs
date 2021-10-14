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
#include "smce-rs/src/ffi/definitions.rs"
#include "board_config.hxx"

#include <iostream>

auto board_config_new(const BoardConfig& config) -> std::unique_ptr<OpaqueBoardConfig> {
    auto ret = smce::BoardConfig{};

    std::transform(config.gpio_drivers.begin(), config.gpio_drivers.end(),
                   std::back_inserter(ret.gpio_drivers), [&](const auto& gpio) {
                       ret.pins.push_back(gpio.pin_id);
                       return smce::BoardConfig::GpioDrivers{
                           .pin_id = gpio.pin_id,
                           .digital_driver = smce::BoardConfig::GpioDrivers::DigitalDriver{gpio.allow_read,
                                                                                           gpio.allow_write},
                           .analog_driver = smce::BoardConfig::GpioDrivers::AnalogDriver{gpio.allow_read,
                                                                                         gpio.allow_write}};
                   });

    std::transform(config.uart_channels.begin(), config.uart_channels.end(),
                   std::back_inserter(ret.uart_channels), [](const auto& uart) {
                       return smce::BoardConfig::UartChannel{.rx_pin_override = std::nullopt,
                                                             .tx_pin_override = std::nullopt,
                                                             .baud_rate = uart.baud_rate,
                                                             .rx_buffer_length = uart.rx_buffer_length,
                                                             .tx_buffer_length = uart.tx_buffer_length,
                                                             .flushing_threshold = uart.flushing_threshold};
                   });

    std::transform(config.sd_cards.begin(), config.sd_cards.end(), std::back_inserter(ret.sd_cards),
                   [](const auto& sd) {
                       auto ret = smce::BoardConfig::SecureDigitalStorage{};
                       ret.cspin = sd.cspin;
                       ret.root_dir = std::string{sd.root_dir};
                       return ret;
                   });

    std::transform(config.frame_buffers.begin(), config.frame_buffers.end(),
                   std::back_inserter(ret.frame_buffers), [](const auto& fb) {
                       auto ret = smce::BoardConfig::FrameBuffer{};
                       ret.key = fb.key;
                       ret.direction = fb.allow_write ? smce::BoardConfig::FrameBuffer::Direction::in
                                                      : smce::BoardConfig::FrameBuffer::Direction::out;
                       return ret;
                   });

    return std::make_unique<OpaqueBoardConfig>(ret);
}
