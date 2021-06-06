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

    pub(crate) enum BoardStatus {
        Clean,
        Configured,
        Running,
        Suspended,
        Stopped,
    }

    #[derive(Debug)]
    pub(crate) enum ToolchainResult {
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
    pub(crate) struct FreestandingLibraryV<'a> {
        pub include_dir: &'a str,
        pub archive_path: &'a str,
        pub compile_defs: &'a Vec<String>,
    }

    pub(crate) struct RemoteArduinoLibraryV<'a> {
        pub name: &'a String,
        pub version: &'a String,
    }

    pub(crate) struct LocalArduinoLibraryV<'a> {
        pub root_dir: &'a str,
        pub patch_for: &'a String,
    }

    #[derive(Default)]
    pub(crate) struct LibraryV<'a> {
        pub free: Vec<FreestandingLibraryV<'a>>,
        pub remote: Vec<RemoteArduinoLibraryV<'a>>,
        pub local: Vec<LocalArduinoLibraryV<'a>>,
    }

    #[derive(Debug, Clone)]
    pub struct DigitalDriver {
        pub read: bool,
        pub write: bool,
    }

    #[derive(Debug, Clone)]
    pub struct AnalogDriver {
        pub read: bool,
        pub write: bool,
    }

    // Board config
    pub(crate) struct GpioDriverV {
        pub pin_id: u16,
        pub digital_driver: *const DigitalDriver,
        pub analog_driver: *const AnalogDriver,
    }

    pub(crate) struct UartChannelV {
        pub rx_pin_override: *const u16,
        pub tx_pin_override: *const u16,
        pub baud_rate: u16,
        pub rx_buffer_length: usize,
        pub tx_buffer_length: usize,
        pub flushing_threshold: usize,
    }

    pub(crate) struct SecureDigitalStorageV<'a> {
        pub(crate) cspin: u16,
        pub(crate) root_dir: &'a str,
    }

    pub(crate) struct FrameBufferV {
        pub(crate) key: usize,
        pub(crate) direction: bool, // true = in false = out
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
            fqbn: &String,
            extra_board_uris: &Vec<String>,
            preproc_libs: LibraryV,
            complink_libs: LibraryV,
            extra_compile_defs: &Vec<String>,
            extra_compile_opts: &Vec<String>,
        ) -> UniquePtr<OpaqueSketchConfig>;

        include!("uuid.hxx");

        pub(crate) unsafe fn uuid_generate() -> Uuid;
        pub(crate) unsafe fn uuid_to_hex(uuid: &Uuid) -> String;

        include!("toolchain.hxx");

        pub(crate) type OpaqueToolchain;
        pub(crate) type OpaqueLockedLog;
        pub(crate) unsafe fn toolchain_new(resource_dir: &str) -> UniquePtr<OpaqueToolchain>;
        pub(crate) unsafe fn resource_dir<'a>(self: &'a OpaqueToolchain) -> &'a str;
        pub(crate) unsafe fn cmake_path<'a>(self: &'a OpaqueToolchain) -> &'a str;
        pub(crate) unsafe fn check_suitable_environment(
            self: Pin<&mut OpaqueToolchain>,
        ) -> ToolchainResult;
        pub(crate) unsafe fn compile(
            self: Pin<&mut OpaqueToolchain>,
            sketch: &mut UniquePtr<OpaqueSketch>,
        ) -> ToolchainResult;
        pub(crate) unsafe fn read_build_log(self: Pin<&mut OpaqueToolchain>) -> String;

        include!("board_config.hxx");

        type OpaqueBoardConfig;
        pub(crate) unsafe fn board_config_new(
            pins: &Vec<u16>,
            gpio_drivers: Vec<GpioDriverV>,
            uart_channels: Vec<UartChannelV>,
            sd_cards: Vec<SecureDigitalStorageV>,
            frame_buffers: Vec<FrameBufferV>,
        ) -> UniquePtr<OpaqueBoardConfig>;

        include!("board.hxx");

        pub(crate) type OpaqueBoard;
        pub(crate) unsafe fn board_new() -> UniquePtr<OpaqueBoard>;
        pub(crate) unsafe fn tick(self: Pin<&mut OpaqueBoard>) -> ExitInfo;
        pub(crate) unsafe fn status(self: &OpaqueBoard) -> BoardStatus;
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

        pub(crate) type OpaqueFramebuffer;
        pub(crate) unsafe fn needs_horizontal_flip(self: Pin<&mut OpaqueFramebuffer>) -> bool;
        pub(crate) unsafe fn needs_vertical_flip(self: Pin<&mut OpaqueFramebuffer>) -> bool;
        pub(crate) unsafe fn width(self: Pin<&mut OpaqueFramebuffer>) -> u16;
        pub(crate) unsafe fn height(self: Pin<&mut OpaqueFramebuffer>) -> u16;
        pub(crate) unsafe fn freq(self: Pin<&mut OpaqueFramebuffer>) -> u8;
        pub(crate) unsafe fn write_rgb888(self: Pin<&mut OpaqueFramebuffer>, buf: &[u8]) -> usize;
        pub(crate) unsafe fn write_rgb444(self: Pin<&mut OpaqueFramebuffer>, buf: &[u8]) -> usize;

    }
}
