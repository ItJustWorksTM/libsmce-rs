/*
 *  definitions.rs
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

#![allow(clippy::missing_safety_doc, clippy::needless_lifetimes)]

pub use ffi::*;

#[cxx::bridge]
pub mod ffi {

    pub(crate) struct ExitInfo {
        pub(crate) exited: bool,
        pub(crate) exit_code: i32,
    }

    pub(crate) struct Uuid {
        pub(crate) bytes: [u8; 16],
    }

    pub(crate) enum OpaqueBoardStatus {
        Clean,
        Configured,
        Running,
        Suspended,
        Stopped,
    }

    #[derive(Debug)]
    pub(crate) enum OpaqueToolchainResult {
        Ok,
        ResdirAbsent,
        ResdirFile,
        ResdirEmpty,
        CmakeNotFound,
        CmakeUnknownOutput,
        CmakeFailing,

        SketchInvalid,
        ConfigureFailed,
        BuildFailed,

        Generic = 255,
    }

    // Sketch Config
    #[derive(Debug, Clone, Default)]
    pub struct PluginManifest {
        pub name: String,
        pub version: String,
        pub depends: String,
        pub needs_devices: String,
        pub uri: String,
        pub patch_uri: String,
        pub defaults: u8,
        pub incdirs: Vec<String>,
        pub sources: Vec<String>,
        pub linkdirs: Vec<String>,
        pub linklibs: Vec<String>,
    }

    #[derive(Debug, Clone, Default)]
    pub struct SketchConfig {
        pub extra_board_uris: Vec<String>,
        pub legacy_libs: Vec<String>,
        pub plugins: Vec<PluginManifest>,
        pub extra_compile_defs: Vec<String>,
        pub extra_compile_opts: Vec<String>,
    }

    // Board config
    #[derive(Debug, Default, Clone)]
    pub struct GpioDriver {
        pub pin_id: u16,
        pub allow_read: bool,
        pub allow_write: bool,
    }

    #[derive(Debug, Clone)]
    pub struct UartChannel {
        pub baud_rate: u16,
        pub rx_buffer_length: usize,
        pub tx_buffer_length: usize,
        pub flushing_threshold: usize,
    }

    #[derive(Debug, Clone)]
    pub struct SecureDigitalStorage {
        pub cspin: u16,
        pub root_dir: String,
    }

    #[derive(Debug, Clone, Default)]
    pub struct FrameBuffer {
        pub key: usize,
        pub allow_write: bool,
    }

    #[derive(Debug, Clone, Default)]
    pub struct BoardConfig {
        pub gpio_drivers: Vec<GpioDriver>,
        pub uart_channels: Vec<UartChannel>,
        pub sd_cards: Vec<SecureDigitalStorage>,
        pub frame_buffers: Vec<FrameBuffer>,
    }

    unsafe extern "C++" {
        include!("sketch.hxx");

        pub(crate) type OpaqueSketch;
        pub(crate) unsafe fn sketch_new(
            source: &str,
            config: &OpaqueSketchConfig,
        ) -> UniquePtr<OpaqueSketch>;
        pub(crate) unsafe fn get_source<'a>(self: &'a OpaqueSketch) -> &'a str;
        pub(crate) unsafe fn is_compiled(self: &OpaqueSketch) -> bool;
        pub(crate) unsafe fn get_uuid(self: &OpaqueSketch) -> Uuid;

        include!("sketch_config.hxx");
        pub(crate) type OpaqueSketchConfig;

        pub(crate) unsafe fn sketch_config_new(
            config: &SketchConfig,
        ) -> UniquePtr<OpaqueSketchConfig>;

        include!("uuid.hxx");

        pub(crate) unsafe fn uuid_generate() -> Uuid;
        pub(crate) unsafe fn uuid_to_hex(uuid: &Uuid) -> String;

        include!("toolchain.hxx");

        pub(crate) type OpaqueToolchain;
        pub(crate) unsafe fn toolchain_new(resource_dir: &str) -> UniquePtr<OpaqueToolchain>;
        pub(crate) unsafe fn resource_dir<'a>(self: &'a OpaqueToolchain) -> &'a str;
        pub(crate) unsafe fn cmake_path<'a>(self: &'a OpaqueToolchain) -> &'a str;
        pub(crate) unsafe fn check_suitable_environment(
            self: Pin<&mut OpaqueToolchain>,
        ) -> OpaqueToolchainResult;
        pub(crate) unsafe fn compile(
            self: Pin<&mut OpaqueToolchain>,
            sketch: &mut UniquePtr<OpaqueSketch>,
        ) -> OpaqueToolchainResult;
        pub(crate) unsafe fn read_build_log(
            self: Pin<&mut OpaqueToolchain>,
            buf: &mut [u8],
        ) -> usize;

        include!("board_config.hxx");

        type OpaqueBoardConfig;
        pub(crate) unsafe fn board_config_new(config: &BoardConfig)
            -> UniquePtr<OpaqueBoardConfig>;

        include!("board.hxx");

        pub(crate) type OpaqueBoard;
        pub(crate) unsafe fn board_new() -> UniquePtr<OpaqueBoard>;
        pub(crate) unsafe fn tick(self: Pin<&mut OpaqueBoard>) -> ExitInfo;
        pub(crate) unsafe fn status(self: &OpaqueBoard) -> OpaqueBoardStatus;
        pub(crate) unsafe fn start(self: Pin<&mut OpaqueBoard>) -> bool;
        pub(crate) unsafe fn resume(self: Pin<&mut OpaqueBoard>) -> bool;
        pub(crate) unsafe fn suspend(self: Pin<&mut OpaqueBoard>) -> bool;
        pub(crate) unsafe fn terminate(self: Pin<&mut OpaqueBoard>) -> bool;
        pub(crate) unsafe fn reset(self: Pin<&mut OpaqueBoard>) -> bool;
        pub(crate) unsafe fn attach_sketch(
            self: Pin<&mut OpaqueBoard>,
            sketch: &UniquePtr<OpaqueSketch>,
        ) -> bool;
        pub(crate) unsafe fn configure(
            self: Pin<&mut OpaqueBoard>,
            conf: &UniquePtr<OpaqueBoardConfig>,
        ) -> bool;
        pub(crate) unsafe fn view(self: Pin<&mut OpaqueBoard>) -> UniquePtr<OpaqueBoardView>;
        pub(crate) unsafe fn runtime_log(self: Pin<&mut OpaqueBoard>, buf: &mut [u8]) -> usize;

        include!("board_view.hxx");

        pub(crate) type OpaqueBoardView;
        pub(crate) unsafe fn get_pin(
            self: Pin<&mut OpaqueBoardView>,
            id: usize,
        ) -> UniquePtr<OpaqueVirtualPin>;
        pub(crate) unsafe fn get_uart(
            self: Pin<&mut OpaqueBoardView>,
            id: usize,
        ) -> UniquePtr<OpaqueVirtualUart>;
        pub(crate) unsafe fn get_framebuffer(
            self: Pin<&mut OpaqueBoardView>,
            id: usize,
        ) -> UniquePtr<OpaqueFramebuffer>;
        pub(crate) unsafe fn clone(self: Pin<&mut OpaqueBoardView>) -> UniquePtr<OpaqueBoardView>;

        pub(crate) type OpaqueVirtualPin;
        pub(crate) unsafe fn is_digital(self: Pin<&mut OpaqueVirtualPin>) -> bool;
        pub(crate) unsafe fn is_analog(self: Pin<&mut OpaqueVirtualPin>) -> bool;
        pub(crate) unsafe fn digital_write(self: Pin<&mut OpaqueVirtualPin>, val: bool);
        pub(crate) unsafe fn digital_read(self: Pin<&mut OpaqueVirtualPin>) -> bool;
        pub(crate) unsafe fn analog_write(self: Pin<&mut OpaqueVirtualPin>, val: u16);
        pub(crate) unsafe fn analog_read(self: Pin<&mut OpaqueVirtualPin>) -> u16;

        pub(crate) type OpaqueVirtualUart;
        pub(crate) unsafe fn readable(self: Pin<&mut OpaqueVirtualUart>) -> usize;
        pub(crate) unsafe fn max_read(self: Pin<&mut OpaqueVirtualUart>) -> usize;
        pub(crate) unsafe fn max_write(self: Pin<&mut OpaqueVirtualUart>) -> usize;
        pub(crate) unsafe fn write(self: Pin<&mut OpaqueVirtualUart>, buf: &[u8]) -> usize;
        pub(crate) unsafe fn read(self: Pin<&mut OpaqueVirtualUart>, buf: &mut [u8]) -> usize;
        pub(crate) unsafe fn front(self: Pin<&mut OpaqueVirtualUart>) -> u8;
        pub(crate) unsafe fn clone(
            self: Pin<&mut OpaqueVirtualUart>,
        ) -> UniquePtr<OpaqueVirtualUart>;

        pub(crate) type OpaqueFramebuffer;
        pub(crate) unsafe fn needs_horizontal_flip(self: Pin<&mut OpaqueFramebuffer>) -> bool;
        pub(crate) unsafe fn needs_vertical_flip(self: Pin<&mut OpaqueFramebuffer>) -> bool;
        pub(crate) unsafe fn width(self: Pin<&mut OpaqueFramebuffer>) -> u16;
        pub(crate) unsafe fn height(self: Pin<&mut OpaqueFramebuffer>) -> u16;
        pub(crate) unsafe fn freq(self: Pin<&mut OpaqueFramebuffer>) -> u8;
        pub(crate) unsafe fn write_rgb888(self: Pin<&mut OpaqueFramebuffer>, buf: &[u8]) -> bool;
        pub(crate) unsafe fn write_rgb444(self: Pin<&mut OpaqueFramebuffer>, buf: &[u8]) -> bool;

    }
}

unsafe impl Send for OpaqueBoard {}

unsafe impl Send for OpaqueToolchain {}
// Warning: only `read_build_log` is thread safe
unsafe impl Sync for OpaqueToolchain {}

unsafe impl Send for OpaqueBoardView {}
unsafe impl Send for OpaqueFramebuffer {}
unsafe impl Send for OpaqueVirtualPin {}
unsafe impl Send for OpaqueVirtualUart {}

unsafe impl Send for OpaqueSketch {}
