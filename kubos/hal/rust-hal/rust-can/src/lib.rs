/*
 * Copyright (C) 2018 Kubos Corporation
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#![deny(missing_docs)]

//! A generalized HAL for communicating over Linux SocketCAN interfaces.

mod error;
pub mod mock;
#[cfg(test)]
mod tests;

pub use crate::error::*;

use socketcan::{
    CanFrame as SocketCanFrame, CanSocket, EmbeddedFrame, ExtendedId, Id, Socket, StandardId,
};
use std::cell::RefCell;
use std::convert::TryFrom;
use std::process::Command;
use std::time::{Duration, Instant};

/// CAN frame data used by the HAL.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanFrame {
    /// CAN frame identifier without SocketCAN flags.
    pub id: u32,
    /// Whether the frame uses the 29-bit extended ID format.
    pub extended: bool,
    /// CAN frame payload bytes.
    pub data: Vec<u8>,
}

impl CanFrame {
    /// Creates a new CAN frame with a standard 11-bit identifier.
    ///
    /// # Arguments
    ///
    /// `id` - Standard CAN identifier
    /// `data` - Frame payload bytes, up to 8 bytes for classic CAN
    pub fn standard(id: u16, data: &[u8]) -> Self {
        Self {
            id: u32::from(id),
            extended: false,
            data: data.to_vec(),
        }
    }

    /// Creates a new CAN frame with an extended 29-bit identifier.
    ///
    /// # Arguments
    ///
    /// `id` - Extended CAN identifier
    /// `data` - Frame payload bytes, up to 8 bytes for classic CAN
    pub fn extended(id: u32, data: &[u8]) -> Self {
        Self {
            id,
            extended: true,
            data: data.to_vec(),
        }
    }

    fn to_socketcan(&self) -> CanResult<SocketCanFrame> {
        let id = if self.extended {
            Id::Extended(
                ExtendedId::new(self.id).ok_or_else(|| CanError::InvalidFrame {
                    description: format!("extended CAN ID 0x{:X} is out of range", self.id),
                })?,
            )
        } else {
            let id = u16::try_from(self.id).map_err(|_| CanError::InvalidFrame {
                description: format!("standard CAN ID 0x{:X} is out of range", self.id),
            })?;
            Id::Standard(StandardId::new(id).ok_or_else(|| CanError::InvalidFrame {
                description: format!("standard CAN ID 0x{:X} is out of range", self.id),
            })?)
        };

        SocketCanFrame::new(id, self.data.as_slice()).ok_or_else(|| CanError::InvalidFrame {
            description: "classic CAN payloads must be 8 bytes or fewer".to_owned(),
        })
    }
}

impl From<SocketCanFrame> for CanFrame {
    fn from(frame: SocketCanFrame) -> Self {
        let (id, extended) = match frame.id() {
            Id::Standard(id) => (u32::from(id.as_raw()), false),
            Id::Extended(id) => (id.as_raw(), true),
        };

        Self {
            id,
            extended,
            data: frame.data().to_vec(),
        }
    }
}

/// Optional receive filter for multi-frame reads.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrameFilter {
    /// CAN frame identifier without SocketCAN flags.
    pub id: u32,
    /// Whether to match the 29-bit extended ID format.
    pub extended: bool,
}

impl FrameFilter {
    /// Creates a standard-ID filter.
    ///
    /// # Arguments
    ///
    /// `id` - Standard CAN identifier to match
    pub fn standard(id: u16) -> Self {
        Self {
            id: u32::from(id),
            extended: false,
        }
    }

    /// Creates an extended-ID filter.
    ///
    /// # Arguments
    ///
    /// `id` - Extended CAN identifier to match
    pub fn extended(id: u32) -> Self {
        Self { id, extended: true }
    }

    fn matches(&self, frame: &CanFrame) -> bool {
        self.id == frame.id && self.extended == frame.extended
    }
}

/// This trait is used to represent CAN streams and allows mocking in tests.
pub trait Stream: Send {
    /// Write one CAN frame to the stream.
    fn write(&self, frame: CanFrame) -> CanResult<()>;

    /// Read one CAN frame from the stream.
    fn read(&self, timeout: Duration) -> CanResult<CanFrame>;

    /// Read a specific number of CAN frames from the stream.
    fn read_frames(&self, count: usize, timeout: Duration) -> CanResult<Vec<CanFrame>>;

    /// Read payload bytes reassembled from multiple CAN frames.
    fn read_payload(
        &self,
        expected_len: usize,
        timeout: Duration,
        filter: Option<FrameFilter>,
    ) -> CanResult<Vec<u8>>;
}

/// Wrapper for CAN streams.
pub struct Connection {
    /// Any boxed stream that allows for communication over CAN bus interfaces.
    pub stream: Box<dyn Stream>,
}

impl Connection {
    /// Constructor to create a connection with a provided stream.
    pub fn new(stream: Box<dyn Stream>) -> Connection {
        Connection { stream }
    }

    /// Convenience constructor to create a SocketCAN connection from an interface name.
    ///
    /// # Arguments
    ///
    /// `interface` - CAN network interface name, for example `can0`
    pub fn from_interface(interface: &str) -> CanResult<Connection> {
        Ok(Connection {
            stream: Box::new(SocketCanStream::new(interface)?),
        })
    }

    /// Writes one CAN frame to the stream.
    pub fn write(&self, frame: CanFrame) -> CanResult<()> {
        self.stream.write(frame)
    }

    /// Reads one CAN frame from the stream.
    pub fn read(&self, timeout: Duration) -> CanResult<CanFrame> {
        self.stream.read(timeout)
    }

    /// Reads a specific number of CAN frames from the stream.
    pub fn read_frames(&self, count: usize, timeout: Duration) -> CanResult<Vec<CanFrame>> {
        self.stream.read_frames(count, timeout)
    }

    /// Reads payload bytes reassembled from multiple matching CAN frames.
    pub fn read_payload(
        &self,
        expected_len: usize,
        timeout: Duration,
        filter: Option<FrameFilter>,
    ) -> CanResult<Vec<u8>> {
        self.stream.read_payload(expected_len, timeout, filter)
    }

    /// Sets a CAN interface up with a bitrate.
    ///
    /// This runs `ip link set <interface> up type can bitrate <bitrate>`.
    pub fn set_interface_up(interface: &str, bitrate: u32) -> CanResult<()> {
        run_ip_link(&[
            "link",
            "set",
            interface,
            "up",
            "type",
            "can",
            "bitrate",
            &bitrate.to_string(),
        ])
    }

    /// Sets a CAN interface down.
    ///
    /// This runs `ip link set <interface> down`.
    pub fn set_interface_down(interface: &str) -> CanResult<()> {
        run_ip_link(&["link", "set", interface, "down"])
    }

    /// Resets a CAN interface by setting it down and then up with a bitrate.
    pub fn reset_interface(interface: &str, bitrate: u32) -> CanResult<()> {
        Self::set_interface_down(interface)?;
        Self::set_interface_up(interface, bitrate)
    }
}

fn run_ip_link(args: &[&str]) -> CanResult<()> {
    let output = Command::new("ip").args(args).output()?;

    if output.status.success() {
        Ok(())
    } else {
        let description = String::from_utf8_lossy(&output.stderr).trim().to_owned();
        Err(CanError::InterfaceError {
            description: if description.is_empty() {
                format!("ip {:?} exited with {}", args, output.status)
            } else {
                description
            },
        })
    }
}

struct SocketCanStream {
    socket: RefCell<CanSocket>,
}

impl SocketCanStream {
    fn new(interface: &str) -> CanResult<Self> {
        Ok(Self {
            socket: RefCell::new(CanSocket::open(interface)?),
        })
    }
}

impl Stream for SocketCanStream {
    fn write(&self, frame: CanFrame) -> CanResult<()> {
        let socket = self.socket.try_borrow().map_err(|_| CanError::PortBusy)?;
        socket.write_frame(&frame.to_socketcan()?)?;
        Ok(())
    }

    fn read(&self, timeout: Duration) -> CanResult<CanFrame> {
        let socket = self.socket.try_borrow().map_err(|_| CanError::PortBusy)?;
        Ok(socket.read_frame_timeout(timeout)?.into())
    }

    fn read_frames(&self, count: usize, timeout: Duration) -> CanResult<Vec<CanFrame>> {
        let start = Instant::now();
        let mut frames = Vec::with_capacity(count);

        while frames.len() < count {
            let remaining = timeout
                .checked_sub(Instant::now() - start)
                .ok_or(CanError::Timeout)?;

            frames.push(self.read(remaining)?);
        }

        Ok(frames)
    }

    fn read_payload(
        &self,
        expected_len: usize,
        timeout: Duration,
        filter: Option<FrameFilter>,
    ) -> CanResult<Vec<u8>> {
        let start = Instant::now();
        let mut payload = Vec::with_capacity(expected_len);

        while payload.len() < expected_len {
            let remaining = timeout
                .checked_sub(Instant::now() - start)
                .ok_or(CanError::Timeout)?;
            let frame = self.read(remaining)?;

            if filter
                .as_ref()
                .map_or(true, |filter| filter.matches(&frame))
            {
                payload.extend_from_slice(&frame.data);
            }
        }

        payload.truncate(expected_len);
        Ok(payload)
    }
}
