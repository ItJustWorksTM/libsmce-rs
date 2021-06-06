/*
 *  sketch_config.rs
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

use crate::ffi::{board_config_new, OpaqueBoardConfig};
use cxx::UniquePtr;
use std::path::PathBuf;

#[derive(Debug, Default, Clone)]
pub struct DigitalDriver {
    pub read: bool,
    pub write: bool,
}

#[derive(Debug, Default, Clone)]
pub struct AnalogDriver {
    pub read: bool,
    pub write: bool,
    // size: usize,
}

#[derive(Debug, Default, Clone)]
pub struct GpioDrivers {
    pub pin_id: u16,
    pub digital_driver: Option<DigitalDriver>,
    pub analog_driver: Option<AnalogDriver>,
}

#[derive(Debug, Clone)]
pub struct UartChannel {
    pub rx_pin_override: Option<u16>,
    pub tx_pin_override: Option<u16>,
    pub baud_rate: u16,
    pub rx_buffer_length: usize,
    pub tx_buffer_length: usize,
    pub flushing_threshold: usize,
}

impl Default for UartChannel {
    fn default() -> Self {
        UartChannel {
            rx_pin_override: None,
            tx_pin_override: None,
            baud_rate: 9600,
            rx_buffer_length: 64,
            tx_buffer_length: 64,
            flushing_threshold: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SecureDigitalStorage {
    pub cspin: u16,
    pub root_dir: PathBuf,
}

#[derive(Debug, Clone)]
pub enum FrameBufferDirection {
    IN,
    OUT,
}

#[derive(Debug, Clone)]
pub struct FrameBuffer {
    pub key: usize,
    pub direction: FrameBufferDirection,
}

#[derive(Default, Clone)]
pub struct BoardConfig {
    pub pins: Vec<u16>,
    pub gpio_drivers: Vec<GpioDrivers>,
    pub uart_channels: Vec<UartChannel>,
    pub sd_cards: Vec<SecureDigitalStorage>,
    pub frame_buffers: Vec<FrameBuffer>,
}

impl BoardConfig {
    pub(crate) fn as_native(&self) -> UniquePtr<OpaqueBoardConfig> {
        unsafe { board_config_new() }
    }
}

impl Into<UniquePtr<OpaqueBoardConfig>> for BoardConfig {
    // TODO: Actually implement
    fn into(self) -> UniquePtr<OpaqueBoardConfig> {
        unsafe { board_config_new() }
    }
}
