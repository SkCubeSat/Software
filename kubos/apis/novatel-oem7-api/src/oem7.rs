//
// Copyright (C) 2025 SkCubeSat
//
// Licensed under the Apache License, Version 2.0 (the "License")
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

//! Core OEM7 serial communication handler.
//!
//! Implements the hybrid ASCII/binary protocol:
//! - Commands sent as ASCII text with `\r\n` terminator
//! - Acknowledgments received as abbreviated ASCII (`<OK` or `<ERROR`)
//! - Log data received as binary frames (`AA 44 12` sync, header, body, CRC32)

use std::io::{Read, Write};
use std::time::Duration;

use log::{debug, error};
use serialport::SerialPort;

use crate::crc32::calc_crc;
use crate::messages::*;
use crate::{OEM7Error, OEM7Result};

/// Default serial read timeout in milliseconds
const DEFAULT_TIMEOUT_MS: u64 = 2000;

/// Maximum number of bytes to scan for sync pattern before giving up
const MAX_SYNC_SCAN: usize = 4096;

/// OEM7 serial communication handler.
///
/// Provides methods for sending ASCII commands and receiving
/// binary log data from a NovAtel OEM7 GNSS receiver.
pub struct OEM7 {
    /// Serial port handle
    port: Box<dyn SerialPort>,
}

impl OEM7 {
    /// Open a serial connection to an OEM7 receiver.
    ///
    /// # Arguments
    /// * `port_path` - Serial device path (e.g., `/dev/ttyS4`)
    /// * `baud_rate` - Baud rate (e.g., 9600)
    pub fn new(port_path: &str, baud_rate: u32) -> OEM7Result<Self> {
        let port = serialport::new(port_path, baud_rate)
            .timeout(Duration::from_millis(DEFAULT_TIMEOUT_MS))
            .open()?;

        debug!(
            "OEM7: Opened {} at {} baud",
            port_path, baud_rate
        );

        Ok(OEM7 { port })
    }

    /// Create an OEM7 instance from an existing serial port.
    /// Useful for testing with mock serial ports.
    pub fn from_port(port: Box<dyn SerialPort>) -> Self {
        OEM7 { port }
    }

    // ──────── ASCII Command Sending ────────

    /// Write an ASCII command to the serial port, appending `\r\n`.
    fn write_ascii(&mut self, cmd: &str) -> OEM7Result<()> {
        let full = format!("{}\r\n", cmd);
        debug!("OEM7 TX: {}", cmd);
        self.port.write_all(full.as_bytes())?;
        self.port.flush()?;
        Ok(())
    }

    /// Read bytes until we see `<OK` or `<ERROR` in the accumulated buffer.
    ///
    /// Returns `Ok(())` on `<OK`, or `Err(CommandRejected)` on `<ERROR`.
    fn read_ack(&mut self) -> OEM7Result<()> {
        let mut buf = Vec::with_capacity(256);
        let mut byte = [0u8; 1];

        loop {
            match self.port.read(&mut byte) {
                Ok(1) => {
                    buf.push(byte[0]);

                    // Check for <OK at end of buffer
                    if buf.len() >= 3 {
                        let tail = &buf[buf.len() - 3..];
                        if tail == b"<OK" || tail == b"\nOK" {
                            // Consume any trailing bytes (port identifier, CRLF)
                            self.drain_ascii_tail();
                            debug!("OEM7 ACK: OK");
                            return Ok(());
                        }
                    }

                    // Check for error responses
                    let s = String::from_utf8_lossy(&buf);
                    if s.contains("<ERROR") || s.contains("\nERROR") {
                        // Try to read to end of line for full error message
                        self.drain_ascii_tail();
                        let msg = s.trim().to_string();
                        error!("OEM7 ACK: {}", msg);
                        return Err(OEM7Error::CommandRejected(msg));
                    }

                    // Safety: don't accumulate forever
                    if buf.len() > 1024 {
                        return Err(OEM7Error::ParseError(
                            "Ack buffer overflow — no OK or ERROR found".to_string(),
                        ));
                    }
                }
                Ok(_) => continue,
                Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                    if buf.is_empty() {
                        return Err(OEM7Error::Timeout);
                    }
                    // Partial data received but no ack
                    let s = String::from_utf8_lossy(&buf);
                    return Err(OEM7Error::ParseError(format!(
                        "Timeout waiting for ack. Received so far: {}",
                        s.trim()
                    )));
                }
                Err(e) => return Err(OEM7Error::Io(e)),
            }
        }
    }

    /// Drain any trailing ASCII characters after an ack (port identifier, CRLF, etc.)
    fn drain_ascii_tail(&mut self) {
        let mut byte = [0u8; 1];
        // Read for a short time to consume trailing bytes
        let orig_timeout = self.port.timeout();
        let _ = self.port.set_timeout(Duration::from_millis(50));
        loop {
            match self.port.read(&mut byte) {
                Ok(1) => {
                    // Stop at newline — the tail is consumed
                    if byte[0] == b'\n' {
                        break;
                    }
                }
                _ => break,
            }
        }
        let _ = self.port.set_timeout(orig_timeout);
    }

    /// Flush any pending data in the serial input buffer.
    pub fn flush_input(&mut self) {
        let mut junk = [0u8; 256];
        let orig_timeout = self.port.timeout();
        let _ = self.port.set_timeout(Duration::from_millis(50));
        loop {
            match self.port.read(&mut junk) {
                Ok(n) if n > 0 => {
                    debug!("OEM7: Flushed {} bytes from input", n);
                }
                _ => break,
            }
        }
        let _ = self.port.set_timeout(orig_timeout);
    }

    // ──────── Binary Frame Reading ────────

    /// Scan the serial input for the binary sync pattern `AA 44 12`,
    /// then read the full frame: header (25 bytes after sync) + body + CRC.
    ///
    /// Returns the parsed header and the raw body bytes (CRC verified).
    fn read_binary_frame(&mut self) -> OEM7Result<(Header, Vec<u8>)> {
        // Step 1: Scan for sync pattern AA 44 12
        let mut sync_state = 0u8; // 0=looking for AA, 1=got AA, 2=got 44
        let mut byte = [0u8; 1];
        let mut scanned = 0usize;

        loop {
            match self.port.read(&mut byte) {
                Ok(1) => {
                    scanned += 1;
                    match (sync_state, byte[0]) {
                        (0, 0xAA) => sync_state = 1,
                        (1, 0x44) => sync_state = 2,
                        (2, 0x12) => break, // Sync found!
                        (_, 0xAA) => sync_state = 1,
                        _ => sync_state = 0,
                    }
                    if scanned > MAX_SYNC_SCAN {
                        return Err(OEM7Error::NoSync);
                    }
                }
                Ok(_) => continue,
                Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                    return Err(OEM7Error::Timeout);
                }
                Err(e) => return Err(OEM7Error::Io(e)),
            }
        }

        // Step 2: Read remaining header bytes (25 bytes after the 3 sync bytes)
        let mut hdr_buf = vec![0u8; 25];
        self.port.read_exact(&mut hdr_buf)?;
        let header = Header::parse(&hdr_buf)?;

        // Step 3: Read body (msg_len bytes)
        let mut body = vec![0u8; header.msg_len as usize];
        if header.msg_len > 0 {
            self.port.read_exact(&mut body)?;
        }

        // Step 4: Read CRC (4 bytes, little-endian)
        let mut crc_buf = [0u8; 4];
        self.port.read_exact(&mut crc_buf)?;
        let received_crc = u32::from_le_bytes(crc_buf);

        // Step 5: Verify CRC over sync + header + body
        let mut full_msg = Vec::with_capacity(3 + 25 + body.len());
        full_msg.extend_from_slice(&SYNC);
        full_msg.extend_from_slice(&hdr_buf);
        full_msg.extend_from_slice(&body);
        let computed_crc = calc_crc(&full_msg);

        if computed_crc != received_crc {
            return Err(OEM7Error::CrcMismatch {
                expected: received_crc,
                actual: computed_crc,
            });
        }

        debug!(
            "OEM7 RX: Binary frame msg_id={:?} body_len={}",
            header.msg_id, header.msg_len
        );

        Ok((header, body))
    }

    // ──────── High-Level Commands ────────

    /// Send an ASCII command and wait for acknowledgment.
    ///
    /// # Arguments
    /// * `cmd` - Command string (e.g., `"UNLOGALL TRUE"`)
    pub fn send_command(&mut self, cmd: &str) -> OEM7Result<()> {
        self.write_ascii(cmd)?;
        self.read_ack()
    }

    /// Request a one-shot binary log. Sends `LOG {name}B ONCE`,
    /// reads the ack, then reads the binary frame.
    ///
    /// # Arguments
    /// * `log_name` - Log name without format suffix (e.g., `"BESTXYZ"`)
    ///
    /// # Returns
    /// The parsed header and raw body bytes.
    pub fn request_log_once(&mut self, log_name: &str) -> OEM7Result<(Header, Vec<u8>)> {
        self.flush_input();
        let cmd = format!("LOG {}B ONCE", log_name);
        self.write_ascii(&cmd)?;
        self.read_ack()?;
        self.read_binary_frame()
    }

    /// Request and parse the VERSION log.
    pub fn request_version(&mut self) -> OEM7Result<(Header, VersionLog)> {
        let (header, body) = self.request_log_once("VERSION")?;
        let version = VersionLog::parse(&body)?;
        Ok((header, version))
    }

    /// Request and parse the BESTXYZ position/velocity log.
    pub fn request_bestxyz(&mut self) -> OEM7Result<(Header, BestXYZ)> {
        let (header, body) = self.request_log_once("BESTXYZ")?;
        let bestxyz = BestXYZ::parse(&body)?;
        Ok((header, bestxyz))
    }

    /// Request and parse the RXSTATUS log.
    pub fn request_rxstatus(&mut self) -> OEM7Result<(Header, RxStatusLog)> {
        let (header, body) = self.request_log_once("RXSTATUS")?;
        let rxstatus = RxStatusLog::parse(&body)?;
        Ok((header, rxstatus))
    }

    /// Send an UNLOG command for a specific log on the current port.
    ///
    /// # Arguments
    /// * `log_name` - Log name (e.g., `"BESTXYZ"`)
    pub fn unlog(&mut self, log_name: &str) -> OEM7Result<()> {
        self.send_command(&format!("UNLOG THISPORT {}", log_name))
    }

    /// Send an UNLOGALL command.
    ///
    /// # Arguments
    /// * `clear_holds` - If true, also clears logs with HOLD flag
    pub fn unlog_all(&mut self, clear_holds: bool) -> OEM7Result<()> {
        if clear_holds {
            self.send_command("UNLOGALL THISPORT TRUE")
        } else {
            self.send_command("UNLOGALL THISPORT")
        }
    }

    /// Send an arbitrary ASCII command and return the raw text response.
    ///
    /// This reads everything until the ack, then returns the accumulated text.
    /// For commands that produce binary output, use `request_log_once` instead.
    pub fn passthrough(&mut self, command: &str) -> OEM7Result<String> {
        self.flush_input();
        self.write_ascii(command)?;

        // Read bytes until <OK or <ERROR, accumulating everything
        let mut buf = Vec::with_capacity(1024);
        let mut byte = [0u8; 1];

        loop {
            match self.port.read(&mut byte) {
                Ok(1) => {
                    buf.push(byte[0]);

                    let s = String::from_utf8_lossy(&buf);
                    if s.contains("<OK") || s.contains("\nOK") {
                        self.drain_ascii_tail();
                        return Ok(s.trim().to_string());
                    }
                    if s.contains("<ERROR") || s.contains("\nERROR") {
                        self.drain_ascii_tail();
                        let msg = s.trim().to_string();
                        return Err(OEM7Error::CommandRejected(msg));
                    }

                    if buf.len() > 8192 {
                        return Err(OEM7Error::ParseError(
                            "Passthrough buffer overflow".to_string(),
                        ));
                    }
                }
                Ok(_) => continue,
                Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                    if buf.is_empty() {
                        return Err(OEM7Error::Timeout);
                    }
                    let s = String::from_utf8_lossy(&buf);
                    return Ok(s.trim().to_string());
                }
                Err(e) => return Err(OEM7Error::Io(e)),
            }
        }
    }
}
