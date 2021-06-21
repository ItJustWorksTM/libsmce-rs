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

use std::cell::UnsafeCell;
use std::collections::hash_map::Iter as MapIter;
use std::collections::HashMap;
use std::io;
use std::io::{Read, Write};
use std::ops::Index;
use std::pin::Pin;
use std::slice::Iter as VecIter;

use cxx::UniquePtr;

use crate::board_config::{
    FrameBuffer as FrameBufferInfo, GpioDriver as GpioDriverInfo, UartChannel as UartChannelInfo,
};
use crate::ffi::{OpaqueFramebuffer, OpaqueVirtualPin, OpaqueVirtualUart};

pub struct BoardView {
    pub pins: Pins,
    pub uart_channels: UartChannels,
    pub frame_buffers: FrameBuffers,
}

pub struct Pins {
    pub(crate) inner: HashMap<usize, GpioPin>,
}

impl Pins {
    pub fn iter(&self) -> AnalogPinIterator {
        AnalogPinIterator {
            inner_iter: self.inner.iter(),
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, pin: usize) -> Option<&GpioPin> {
        self.inner.get(&pin)
    }
}

impl Index<usize> for Pins {
    type Output = GpioPin;

    fn index(&self, index: usize) -> &Self::Output {
        self.inner.index(&index)
    }
}

pub struct AnalogPinIterator<'a> {
    inner_iter: MapIter<'a, usize, GpioPin>,
}

impl<'a> Iterator for AnalogPinIterator<'a> {
    type Item = (&'a usize, &'a GpioPin);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner_iter.next()
    }
}

pub struct UartChannels {
    pub(crate) inner: Vec<UartChannel>,
}

impl UartChannels {
    pub fn iter(&self) -> UartChannelIterator {
        UartChannelIterator {
            inner_iter: self.inner.iter(),
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Index<usize> for UartChannels {
    type Output = UartChannel;

    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

pub struct UartChannelIterator<'a> {
    inner_iter: VecIter<'a, UartChannel>,
}

impl<'a> Iterator for UartChannelIterator<'a> {
    type Item = &'a UartChannel;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner_iter.next()
    }
}

pub struct FrameBuffers {
    pub(crate) inner: HashMap<usize, FrameBuffer>,
}

impl FrameBuffers {
    pub fn iter(&self) -> FrameBuffersIterator {
        FrameBuffersIterator {
            inner_iter: self.inner.iter(),
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, key: usize) -> Option<&FrameBuffer> {
        self.inner.get(&key)
    }
}

impl Index<usize> for FrameBuffers {
    type Output = FrameBuffer;

    fn index(&self, index: usize) -> &Self::Output {
        self.inner.index(&index)
    }
}

pub struct FrameBuffersIterator<'a> {
    inner_iter: MapIter<'a, usize, FrameBuffer>,
}

impl<'a> Iterator for FrameBuffersIterator<'a> {
    type Item = (&'a usize, &'a FrameBuffer);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner_iter.next()
    }
}

pub struct GpioPin {
    pub(crate) inner: UnsafeCell<UniquePtr<OpaqueVirtualPin>>,
    pub(crate) info: GpioDriverInfo,
}

impl GpioPin {
    pub fn info(&self) -> &GpioDriverInfo {
        &self.info
    }

    pub fn analog_read(&self) -> u16 {
        unsafe { (*self.inner.get()).pin_mut().analog_read() }
    }

    pub fn analog_write(&self, val: u16) {
        unsafe { (*self.inner.get()).pin_mut().analog_write(val) }
    }

    pub fn digital_read(&self) -> bool {
        unsafe { (*self.inner.get()).pin_mut().digital_read() }
    }

    pub fn digital_write(&self, val: bool) {
        unsafe { (*self.inner.get()).pin_mut().digital_write(val) }
    }
}

pub struct UartChannel {
    pub(crate) inner: UnsafeCell<UniquePtr<OpaqueVirtualUart>>,
    pub(crate) info: UartChannelInfo,
}

// TODO: consider a split tx and rx read / writer
impl UartChannel {
    // Returns original BoardConfig::UartChannel
    pub fn info(&self) -> &UartChannelInfo {
        &self.info
    }
}

impl Read for &UartChannel {
    // Will never fail, expect 0 size reads.
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        Ok(unsafe { (*self.inner.get()).pin_mut().read(buf) })
    }
}

impl Write for &UartChannel {
    // Will fail with an WriteZero error if the buffer is full.
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let written = unsafe { (*self.inner.get()).pin_mut().write(buf) };
        if written > 0 {
            Ok(written)
        } else {
            Err(io::Error::new(
                io::ErrorKind::WriteZero,
                "Uart buffer is full, increase the max buffer size or try again",
            ))
        }
    }

    // TODO: decide if this will block for arduino land to read all the bytes
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[derive(Debug)]
pub enum FrameBufferFormat {
    Rgb888,
    Rgb444,
}

impl Default for FrameBufferFormat {
    fn default() -> Self {
        Self::Rgb888
    }
}

pub struct FrameBuffer {
    pub(crate) inner: UnsafeCell<UniquePtr<OpaqueFramebuffer>>,
    pub(crate) info: FrameBufferInfo,
}

impl FrameBuffer {
    fn inner(&self) -> Pin<&mut OpaqueFramebuffer> {
        unsafe { (*self.inner.get()).pin_mut() }
    }

    pub fn info(&self) -> &FrameBufferInfo {
        &self.info
    }

    pub fn needs_horizontal_flip(&self) -> bool {
        unsafe { self.inner().needs_horizontal_flip() }
    }

    pub fn needs_vertical_flip(&self) -> bool {
        unsafe { self.inner().needs_vertical_flip() }
    }

    pub fn width(&self) -> u16 {
        unsafe { self.inner().width() }
    }

    pub fn height(&self) -> u16 {
        unsafe { self.inner().height() }
    }

    pub fn freq(&self) -> u8 {
        unsafe { self.inner().freq() }
    }

    pub fn expected_buf_size(&self) -> usize {
        (self.width() * self.height() * 3) as usize
    }

    pub fn write(&self, buf: &[u8], format: FrameBufferFormat) -> bool {
        match format {
            FrameBufferFormat::Rgb888 => unsafe { self.inner().write_rgb888(buf) },
            FrameBufferFormat::Rgb444 => unsafe { self.inner().write_rgb444(buf) },
        }
    }
}
