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

use cxx::UniquePtr;

use crate::ffi::{board_config_new, OpaqueBoardConfig};

pub use crate::ffi::{BoardConfig, FrameBuffer, GpioDriver, SecureDigitalStorage, UartChannel};

impl Default for UartChannel {
    fn default() -> Self {
        UartChannel {
            baud_rate: 9600,
            rx_buffer_length: 64,
            tx_buffer_length: 64,
            flushing_threshold: 0,
        }
    }
}

impl BoardConfig {
    pub(crate) fn as_native(&self) -> UniquePtr<OpaqueBoardConfig> {
        unsafe { board_config_new(self) }
    }
}

impl Into<UniquePtr<OpaqueBoardConfig>> for BoardConfig {
    // TODO: Actually implement
    fn into(self) -> UniquePtr<OpaqueBoardConfig> {
        UniquePtr::null()
    }
}
