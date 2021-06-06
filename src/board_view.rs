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

use std::io;
use std::io::{Read, Write};
use std::marker::PhantomData;
use std::pin::Pin;

use cxx::UniquePtr;

use crate::board::Board;
use crate::ffi::OpaqueBoardView;
use crate::ffi::OpaqueFramebuffer as OpaqueFrameBuffer;
use crate::ffi::OpaqueVirtualPin;
use crate::ffi::OpaqueVirtualUart;

pub enum Link {
    Uart,
    Spi,
    I2c,
}

pub struct BoardView<'a> {
    pub(crate) view: UniquePtr<OpaqueBoardView>,
    pub(crate) board: PhantomData<&'a mut Board>,
}

impl<'a> BoardView<'a> {
    // TODO: make these Results for easier ? syntax
    pub fn digital_pin(&mut self, id: usize) -> Option<VirtualDigitalPin<'a>> {
        let mut pin: UniquePtr<OpaqueVirtualPin> = unsafe { self.view.pin_mut().get_pin(id) };
        unsafe { pin.pin_mut().is_digital() }.then(move || VirtualDigitalPin {
            vp: pin,
            board: PhantomData,
        })
    }

    pub fn analog_pin(&mut self, id: usize) -> Option<VirtualAnalogPin<'a>> {
        let mut pin: UniquePtr<OpaqueVirtualPin> = unsafe { self.view.pin_mut().get_pin(id) };
        unsafe { pin.pin_mut().is_analog() }.then(move || VirtualAnalogPin {
            vp: pin,
            board: PhantomData,
        })
    }

    pub fn uart(&mut self, id: usize) -> Option<VirtualUart<'a>> {
        let vu = unsafe { self.view.pin_mut().get_uart(id) };
        match !vu.is_null() {
            true => Some(VirtualUart {
                vu,
                board: PhantomData,
            }),
            false => None,
        }
    }

    pub fn framebuffer(&mut self, id: usize) -> Option<FrameBuffer<'a>> {
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

pub struct VirtualDigitalPin<'a> {
    vp: UniquePtr<OpaqueVirtualPin>,
    board: PhantomData<&'a mut Board>,
}

impl VirtualDigitalPin<'_> {
    pub fn write(&mut self, val: bool) {
        unsafe { self.vp.pin_mut().digital_write(val) }
    }
    pub fn read(&mut self) -> bool {
        unsafe { self.vp.pin_mut().digital_read() }
    }
}

pub struct VirtualAnalogPin<'a> {
    vp: UniquePtr<OpaqueVirtualPin>,
    board: PhantomData<&'a mut Board>,
}

impl VirtualAnalogPin<'_> {
    pub fn write(&mut self, val: u16) {
        unsafe { self.vp.pin_mut().analog_write(val) }
    }
    pub fn read(&mut self) -> u16 {
        unsafe { self.vp.pin_mut().analog_read() }
    }
}

pub struct VirtualUart<'a> {
    vu: UniquePtr<OpaqueVirtualUart>,
    board: PhantomData<&'a mut Board>,
}

impl VirtualUart<'_> {
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

impl Write for VirtualUart<'_> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        unsafe { Ok(self.vu.pin_mut().write(buf)) }
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Read for VirtualUart<'_> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        unsafe { Ok(self.vu.pin_mut().read(buf)) }
    }
}

pub struct FrameBuffer<'a> {
    fb: UniquePtr<OpaqueFrameBuffer>,
    board: PhantomData<&'a mut Board>,
}

impl FrameBuffer<'_> {
    fn inner(&mut self) -> Pin<&mut OpaqueFrameBuffer> {
        self.fb.pin_mut()
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

impl Write for FrameBuffer<'_> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        unsafe { Ok(self.inner().write_rgb888(buf)) }
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[test]
fn test() {}
