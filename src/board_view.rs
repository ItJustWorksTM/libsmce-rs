/*
 *  board_view.rs
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

use std::error::Error;
use std::io;
use std::io::{BufReader, ErrorKind, Read, Write};
use std::marker::PhantomData;

use cxx::UniquePtr;

use crate::board::Board;
use crate::ffi::OpaqueBoardView;
use crate::ffi::OpaqueFramebuffer as OpaqueFrameBuffer;
use crate::ffi::OpaqueVirtualPin;
use crate::ffi::OpaqueVirtualUart;
use std::pin::Pin;

pub enum Link {
    Uart,
    Spi,
    I2c,
}

pub struct BoardView<'a, 'b, 'c> {
    pub(crate) view: UniquePtr<OpaqueBoardView>,
    pub(crate) board: PhantomData<&'a mut Board<'b, 'c>>,
}

impl<'a, 'b, 'c> BoardView<'a, 'b, 'c> {
    pub fn pin(&mut self, id: usize) -> VirtualPin<'a, 'b, 'c> {
        VirtualPin {
            vp: unsafe { self.view.pin_mut().get_pin(id) },
            board: PhantomData,
        }
    }

    pub fn uart(&mut self, id: usize) -> Option<VirtualUart<'a, 'b, 'c>> {
        let vu = unsafe { self.view.pin_mut().get_uart(id) };
        match !vu.is_null() {
            true => Some(VirtualUart {
                vu,
                board: PhantomData,
            }),
            false => None,
        }
    }

    pub fn framebuffer(&mut self, id: usize) -> Option<FrameBuffer<'a, 'b, 'c>> {
        let fb = unsafe { self.view.pin_mut().get_framebuffer(id) };
        match fb.is_null() {
            true => Some(FrameBuffer {
                fb,
                board: PhantomData,
            }),
            false => None,
        }
    }
}

pub struct VirtualPin<'a, 'b, 'c> {
    vp: UniquePtr<OpaqueVirtualPin>,
    pub(crate) board: PhantomData<&'a mut Board<'b, 'c>>,
}

impl<'a, 'b, 'c> VirtualPin<'a, 'b, 'c> {
    pub fn digital(mut self) -> Option<VirtualDigitalPin<'a, 'b, 'c>> {
        match unsafe { self.vp.pin_mut().is_digital() } {
            true => Some(VirtualDigitalPin { bv: self }),
            false => None,
        }
    }
    pub fn analog(mut self) -> Option<VirtualAnalogPin<'a, 'b, 'c>> {
        match unsafe { self.vp.pin_mut().is_analog() } {
            true => Some(VirtualAnalogPin { bv: self }),
            false => None,
        }
    }
}

pub struct VirtualDigitalPin<'a, 'b, 'c> {
    bv: VirtualPin<'a, 'b, 'c>,
}

impl VirtualDigitalPin<'_, '_, '_> {
    pub fn write(&mut self, val: bool) {
        unsafe { self.bv.vp.pin_mut().digital_write(val) }
    }
    pub fn read(&mut self) -> bool {
        unsafe { self.bv.vp.pin_mut().digital_read() }
    }
}

pub struct VirtualAnalogPin<'a, 'b, 'c> {
    bv: VirtualPin<'a, 'b, 'c>,
}

impl VirtualAnalogPin<'_, '_, '_> {
    pub fn write(&mut self, val: u16) {
        unsafe { self.bv.vp.pin_mut().analog_write(val) }
    }
    pub fn read(&mut self) -> u16 {
        unsafe { self.bv.vp.pin_mut().analog_read() }
    }
}

pub struct VirtualUart<'a, 'b, 'c> {
    vu: UniquePtr<OpaqueVirtualUart>,
    board: PhantomData<&'a mut Board<'b, 'c>>,
}

impl VirtualUart<'_, '_, '_> {
    pub fn available(&mut self) -> usize {
        unsafe { self.vu.pin_mut().readable() }
    }

    pub fn max_read(&mut self) -> usize {
        unsafe { self.vu.pin_mut().max_read() }
    }

    pub fn max_write(&mut self) -> usize {
        unsafe { self.vu.pin_mut().max_write() }
    }

    pub fn front(&mut self) -> u8 {
        unsafe { self.vu.pin_mut().front() }
    }
}

impl Write for VirtualUart<'_, '_, '_> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        unsafe { Ok(self.vu.pin_mut().write(buf)) }
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Read for VirtualUart<'_, '_, '_> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        unsafe { Ok(self.vu.pin_mut().read(buf)) }
    }
}

pub struct FrameBuffer<'a, 'b, 'c> {
    fb: UniquePtr<OpaqueFrameBuffer>,
    board: PhantomData<&'a mut Board<'b, 'c>>,
}

impl FrameBuffer<'_, '_, '_> {
    fn inner(&mut self) -> Pin<&mut OpaqueFrameBuffer> {
        unsafe { self.fb.pin_mut() }
    }

    pub fn needs_horizontal_flip(&mut self) -> bool {
        unsafe { self.inner().needs_horizontal_flip() }
    }

    pub fn needs_vertical_flip(&mut self) -> bool {
        unsafe { self.inner().needs_vertical_flip() }
    }

    pub fn width(&mut self) -> u16 {
        unsafe { self.inner().width() }
    }

    pub fn height(&mut self) -> u16 {
        unsafe { self.inner().height() }
    }

    pub fn freq(&mut self) -> u8 {
        unsafe { self.inner().freq() }
    }
}

impl Write for FrameBuffer<'_, '_, '_> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        unsafe { Ok(self.inner().write_rgb888(buf)) }
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[test]
fn test() {}
