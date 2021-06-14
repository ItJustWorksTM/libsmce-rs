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
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Formatter};
use std::io::{ErrorKind, Read, Write};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut, Index};
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, PoisonError, RwLock, RwLockReadGuard, Weak};
use std::{fmt, io};

use cxx::UniquePtr;

use crate::board::BoardHandle;
use crate::ffi::OpaqueFramebuffer;
use crate::ffi::OpaqueVirtualPin;
use crate::ffi::OpaqueVirtualUart;
use crate::ffi::{OpaqueBoard, OpaqueBoardView};

use crate::board_config::{
    AnalogDriver, DigitalDriver, FrameBuffer as FrameBufferInfo, UartChannel as UartChannelInfo,
};

use std::collections::hash_map::Iter as MapIter;
use std::slice::Iter as VecIter;

// TODO: put this in definitions.rs

unsafe impl Send for OpaqueBoardView {}
unsafe impl Send for OpaqueFramebuffer {}
unsafe impl Send for OpaqueVirtualPin {}
unsafe impl Send for OpaqueVirtualUart {}

// BoardView

pub struct BoardView {
    pub digital_pins: DigitalPins,
    pub analog_pins: AnalogPins,
    pub uart_channels: UartChannels,
    pub frame_buffers: FrameBuffers,
}

// DigitalPins

pub struct DigitalPins {
    pub(crate) inner: HashMap<usize, DigitalPin>,
}

impl DigitalPins {
    pub fn iter(&self) -> AnalogPinIterator {
        todo!()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn get(&self, pin: usize) -> Option<&DigitalPin> {
        self.inner.get(&pin)
    }
}

// AnalogPins
pub struct AnalogPins {
    pub(crate) inner: HashMap<usize, AnalogPin>,
}

impl AnalogPins {
    pub fn iter(&self) -> AnalogPinIterator {
        AnalogPinIterator {
            inner_iter: self.inner.iter(),
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn get(&self, pin: usize) -> Option<&AnalogPin> {
        self.inner.get(&pin)
    }
}

impl Index<usize> for AnalogPins {
    type Output = AnalogPin;

    fn index(&self, index: usize) -> &Self::Output {
        self.inner.index(&index)
    }
}

pub struct AnalogPinIterator<'a> {
    inner_iter: MapIter<'a, usize, AnalogPin>,
}

impl<'a> Iterator for AnalogPinIterator<'a> {
    type Item = (&'a usize, &'a AnalogPin);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner_iter.next()
    }
}

// UartChannels

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

// Framebuffers

pub struct FrameBuffers {
    pub(crate) inner: HashMap<usize, FrameBuffer>,
}

impl FrameBuffers {
    pub fn iter(&self) -> FrameBuffersIterator {
        todo!()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
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

pub struct FrameBuffersIterator {}

// Represents an digital pin, reads and writes can be considered atomic
pub struct DigitalPin {
    pub(crate) inner: UnsafeCell<UniquePtr<OpaqueVirtualPin>>,
    pub(crate) info: DigitalDriver,
}

impl DigitalPin {
    pub fn info(&self) -> &DigitalDriver {
        todo!()
    }

    pub fn read(&self) -> bool {
        unsafe { (*self.inner.get()).pin_mut().digital_read() }
    }

    pub fn write(&self, val: bool) {
        unsafe { (*self.inner.get()).pin_mut().digital_write(val) }
    }
}

// Represents an analog pin, reads and writes can be considered atomic
pub struct AnalogPin {
    pub(crate) inner: UnsafeCell<UniquePtr<OpaqueVirtualPin>>,
    pub(crate) info: AnalogDriver,
}

impl AnalogPin {
    pub fn info(&self) -> &AnalogDriver {
        &self.info
    }

    pub fn read(&self) -> u16 {
        unsafe { (*self.inner.get()).pin_mut().analog_read() }
    }

    pub fn write(&self, val: u16) {
        unsafe { (*self.inner.get()).pin_mut().analog_write(val) }
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
        unsafe { Ok((*self.inner.get()).pin_mut().read(buf)) }
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

pub struct FrameBuffer {
    pub(crate) inner: UnsafeCell<UniquePtr<OpaqueFramebuffer>>,
    pub(crate) info: FrameBufferInfo,
}

impl FrameBuffer {
    pub fn info(&self) -> &FrameBufferInfo {
        &self.info
    }

    pub fn needs_horizontal_flip(&self) -> bool {
        todo!()
    }

    pub fn needs_vertical_flip(&self) -> bool {
        todo!()
    }

    pub fn width(&self) -> u16 {
        todo!()
    }

    pub fn height(&self) -> u16 {
        todo!()
    }

    pub fn freq(&self) -> u8 {
        todo!()
    }
}

// TODO: Framebuffer is not real io so to say, consider not implementing the Write trait
// TODO: consider different Writer objects that do different encodings
impl Write for FrameBuffer {
    // Buffer needs to be exact sized, e.g. height * width * 4
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        todo!()
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
