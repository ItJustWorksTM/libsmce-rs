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

use std::path::PathBuf;
use std::ptr::null;

use cxx::UniquePtr;

use crate::ffi::{
    board_config_new, FrameBufferV, GpioDriverV, OpaqueBoardConfig, SecureDigitalStorageV,
    UartChannelV,
};
pub use crate::ffi::{AnalogDriver, DigitalDriver};

#[derive(Debug, Default, Clone)]
pub struct GpioDriver {
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
    In,
    Out,
}

#[derive(Debug, Clone)]
pub struct FrameBuffer {
    pub key: usize,
    pub direction: FrameBufferDirection,
}

impl Default for FrameBuffer {
    fn default() -> Self {
        Self {
            key: 0,
            direction: FrameBufferDirection::In,
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct BoardConfig {
    pub gpio_drivers: Vec<GpioDriver>,
    pub uart_channels: Vec<UartChannel>,
    pub sd_cards: Vec<SecureDigitalStorage>,
    pub frame_buffers: Vec<FrameBuffer>,
}

impl BoardConfig {
    pub(crate) fn as_native(&self) -> UniquePtr<OpaqueBoardConfig> {
        unsafe {
            board_config_new(
                &self.gpio_drivers.iter().map(|a| a.pin_id).collect(),
                self.gpio_drivers
                    .iter()
                    .map(|t| GpioDriverV {
                        pin_id: t.pin_id,
                        digital_driver: t.digital_driver.as_ref().map_or(null(), |t| t),
                        analog_driver: t.analog_driver.as_ref().map_or(null(), |t| t),
                    })
                    .collect(),
                self.uart_channels
                    .iter()
                    .map(|t| UartChannelV {
                        rx_pin_override: t.rx_pin_override.as_ref().map_or(null(), |t| t),
                        tx_pin_override: t.tx_pin_override.as_ref().map_or(null(), |t| t),
                        baud_rate: t.baud_rate,
                        rx_buffer_length: t.rx_buffer_length,
                        tx_buffer_length: t.tx_buffer_length,
                        flushing_threshold: t.flushing_threshold,
                    })
                    .collect(),
                self.sd_cards
                    .iter()
                    .map(|t| SecureDigitalStorageV {
                        cspin: t.cspin,
                        root_dir: t.root_dir.to_str().unwrap(),
                    })
                    .collect(),
                self.frame_buffers
                    .iter()
                    .map(|t| FrameBufferV {
                        key: t.key,
                        direction: match t.direction {
                            FrameBufferDirection::In => true,
                            FrameBufferDirection::Out => false,
                        },
                    })
                    .collect(),
            )
        }
    }
}

impl Into<UniquePtr<OpaqueBoardConfig>> for BoardConfig {
    // TODO: Actually implement
    fn into(self) -> UniquePtr<OpaqueBoardConfig> {
        UniquePtr::null()
    }
}

impl Default for DigitalDriver {
    fn default() -> Self {
        Self {
            read: true,
            write: true,
        }
    }
}

impl Default for AnalogDriver {
    fn default() -> Self {
        Self {
            read: true,
            write: true,
        }
    }
}

#[test]
fn build() {}
