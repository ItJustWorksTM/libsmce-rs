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
mod ffi {

    pub struct ExitInfo {
        pub exited: bool,
        pub exit_code: i32,
    }

    pub struct Uuid {
        pub(crate) bytes: [u8; 16],
    }

    pub enum BoardStatus {
        Clean,
        Configured,
        Running,
        Suspended,
        Stopped,
    }

    #[derive(Debug)]
    pub enum ToolchainResult {
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

    unsafe extern "C++" {
        include!("sketch.hxx");

        pub type OpaqueSketch;
        pub unsafe fn sketch_new(source: &str) -> UniquePtr<OpaqueSketch>;
        pub unsafe fn get_source<'a>(self: &'a OpaqueSketch) -> &'a str;
        pub unsafe fn is_compiled(self: &OpaqueSketch) -> bool;
        pub unsafe fn get_uuid(self: &OpaqueSketch) -> Uuid;

        include!("uuid.hxx");

        pub unsafe fn uuid_generate() -> Uuid;
        pub unsafe fn uuid_to_hex(uuid: &Uuid) -> String;

        include!("toolchain.hxx");

        pub type OpaqueToolchain;
        pub unsafe fn toolchain_new(resource_dir: &str) -> UniquePtr<OpaqueToolchain>;
        pub unsafe fn resource_dir<'a>(self: &'a OpaqueToolchain) -> &'a str;
        pub unsafe fn cmake_path<'a>(self: &'a OpaqueToolchain) -> &'a str;
        pub unsafe fn check_suitable_environment(
            self: Pin<&mut OpaqueToolchain>,
        ) -> ToolchainResult;
        pub unsafe fn compile(
            self: Pin<&mut OpaqueToolchain>,
            sketch: &mut UniquePtr<OpaqueSketch>,
        ) -> ToolchainResult;

        include!("board_config.hxx");

        type OpaqueBoardConfig;
        pub unsafe fn board_config_new() -> UniquePtr<OpaqueBoardConfig>;

        include!("board.hxx");

        pub type OpaqueBoard;
        pub unsafe fn board_new() -> UniquePtr<OpaqueBoard>;
        pub unsafe fn tick(self: Pin<&mut OpaqueBoard>) -> ExitInfo;
        pub unsafe fn status(self: &OpaqueBoard) -> BoardStatus;
        pub unsafe fn start(self: Pin<&mut OpaqueBoard>) -> bool;
        pub unsafe fn suspend(self: Pin<&mut OpaqueBoard>) -> bool;
        pub unsafe fn resume(self: Pin<&mut OpaqueBoard>) -> bool;
        pub unsafe fn terminate(self: Pin<&mut OpaqueBoard>) -> bool;
        pub unsafe fn reset(self: Pin<&mut OpaqueBoard>) -> bool;
        pub unsafe fn attach_sketch(
            self: Pin<&mut OpaqueBoard>,
            sketch: &UniquePtr<OpaqueSketch>,
        ) -> bool;
        pub unsafe fn configure(
            self: Pin<&mut OpaqueBoard>,
            conf: &UniquePtr<OpaqueBoardConfig>,
        ) -> bool;
        pub unsafe fn view(self: Pin<&mut OpaqueBoard>) -> UniquePtr<OpaqueBoardView>;

        include!("board_view.hxx");

        pub type OpaqueBoardView;
        pub unsafe fn get_pin(
            self: Pin<&mut OpaqueBoardView>,
            id: usize,
        ) -> UniquePtr<OpaqueVirtualPin>;
        pub unsafe fn get_uart(
            self: Pin<&mut OpaqueBoardView>,
            id: usize,
        ) -> UniquePtr<OpaqueVirtualUart>;
        pub unsafe fn get_framebuffer(
            self: Pin<&mut OpaqueBoardView>,
            id: usize,
        ) -> UniquePtr<OpaqueFramebuffer>;

        pub type OpaqueVirtualPin;
        pub unsafe fn is_digital(self: Pin<&mut OpaqueVirtualPin>) -> bool;
        pub unsafe fn is_analog(self: Pin<&mut OpaqueVirtualPin>) -> bool;
        pub unsafe fn digital_write(self: Pin<&mut OpaqueVirtualPin>, val: bool);
        pub unsafe fn digital_read(self: Pin<&mut OpaqueVirtualPin>) -> bool;
        pub unsafe fn analog_write(self: Pin<&mut OpaqueVirtualPin>, val: u16);
        pub unsafe fn analog_read(self: Pin<&mut OpaqueVirtualPin>) -> u16;

        pub type OpaqueVirtualUart;
        pub unsafe fn readable(self: Pin<&mut OpaqueVirtualUart>) -> usize;
        pub unsafe fn max_read(self: Pin<&mut OpaqueVirtualUart>) -> usize;
        pub unsafe fn max_write(self: Pin<&mut OpaqueVirtualUart>) -> usize;
        pub unsafe fn write(self: Pin<&mut OpaqueVirtualUart>, buf: &[u8]) -> usize;
        pub unsafe fn read(self: Pin<&mut OpaqueVirtualUart>, buf: &mut [u8]) -> usize;
        pub unsafe fn front(self: Pin<&mut OpaqueVirtualUart>) -> u8;

        pub type OpaqueFramebuffer;
        pub unsafe fn needs_horizontal_flip(self: Pin<&mut OpaqueFramebuffer>) -> bool;
        pub unsafe fn needs_vertical_flip(self: Pin<&mut OpaqueFramebuffer>) -> bool;
        pub unsafe fn width(self: Pin<&mut OpaqueFramebuffer>) -> u16;
        pub unsafe fn height(self: Pin<&mut OpaqueFramebuffer>) -> u16;
        pub unsafe fn freq(self: Pin<&mut OpaqueFramebuffer>) -> u8;
        pub unsafe fn write_rgb888(self: Pin<&mut OpaqueFramebuffer>, buf: &[u8]) -> usize;
        pub unsafe fn write_rgb444(self: Pin<&mut OpaqueFramebuffer>, buf: &[u8]) -> usize;

    }
}

#[test]
fn source_path() {
    unsafe {
        let sketch = sketch_new("1234");
        assert!(!sketch.is_null());
    }
}
