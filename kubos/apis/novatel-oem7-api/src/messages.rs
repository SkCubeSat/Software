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

//! Binary message structures for the NovAtel OEM7 receiver.
//!
//! The OEM7 binary format is identical to OEM6:
//! - 3 sync bytes: `AA 44 12`
//! - 28-byte header (variable, check header_len field)
//! - Variable-length body
//! - 4-byte CRC32

use bitflags::bitflags;
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;

use crate::{OEM7Error, OEM7Result};

/// Binary sync pattern
pub const SYNC: [u8; 3] = [0xAA, 0x44, 0x12];
/// Standard header length
pub const HDR_LEN: u8 = 28;

// ──────────────────── Message ID ────────────────────

/// Supported binary message IDs
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MessageID {
    /// LOG command (1)
    Log,
    /// UNLOG command (36)
    Unlog,
    /// VERSION data log (37)
    Version,
    /// UNLOGALL command (38)
    UnlogAll,
    /// RXSTATUS data log (93)
    RxStatus,
    /// RXSTATUSEVENT data log (94)
    RxStatusEvent,
    /// BESTXYZ position/velocity data log (241)
    BestXYZ,
    /// Unknown message ID
    Unknown(u16),
}

impl From<u16> for MessageID {
    fn from(t: u16) -> MessageID {
        match t {
            1 => MessageID::Log,
            36 => MessageID::Unlog,
            37 => MessageID::Version,
            38 => MessageID::UnlogAll,
            93 => MessageID::RxStatus,
            94 => MessageID::RxStatusEvent,
            241 => MessageID::BestXYZ,
            other => MessageID::Unknown(other),
        }
    }
}

// ──────────────────── Binary Header ────────────────────

/// Common binary header structure (28 bytes)
#[derive(Clone, Debug)]
pub struct Header {
    /// Header length (typically 28)
    pub hdr_len: u8,
    /// Message ID
    pub msg_id: MessageID,
    /// Message type byte
    pub msg_type: u8,
    /// Port address
    pub port_addr: u8,
    /// Body length in bytes (not including header or CRC)
    pub msg_len: u16,
    /// Sequence number
    pub seq: u16,
    /// Idle time (0-200, divide by 2 for percentage)
    pub idle_time: u8,
    /// GPS reference time status
    pub time_status: u8,
    /// GPS reference week
    pub week: u16,
    /// Milliseconds from beginning of GPS reference week
    pub ms: i32,
    /// Receiver status flags
    pub recv_status: ReceiverStatusFlags,
    /// Receiver software version
    pub recv_ver: u16,
}

impl Header {
    /// Parse a binary header from a 28-byte (or longer) slice.
    /// The first 3 bytes (sync) should already be consumed.
    /// Pass the 25 bytes after sync.
    pub fn parse(raw: &[u8]) -> OEM7Result<Self> {
        if raw.len() < 25 {
            return Err(OEM7Error::ParseError(format!(
                "Header too short: {} bytes (need 25 after sync)",
                raw.len()
            )));
        }
        let mut c = Cursor::new(raw);
        let hdr_len = c.read_u8()?;
        let msg_id_raw = c.read_u16::<LittleEndian>()?;
        let msg_type = c.read_u8()?;
        let port_addr = c.read_u8()?;
        let msg_len = c.read_u16::<LittleEndian>()?;
        let seq = c.read_u16::<LittleEndian>()?;
        let idle_time = c.read_u8()?;
        let time_status = c.read_u8()?;
        let week = c.read_u16::<LittleEndian>()?;
        let ms = c.read_i32::<LittleEndian>()?;
        let recv_status_raw = c.read_u32::<LittleEndian>()?;
        let _reserved = c.read_u16::<LittleEndian>()?;
        let recv_ver = c.read_u16::<LittleEndian>()?;

        Ok(Header {
            hdr_len,
            msg_id: msg_id_raw.into(),
            msg_type,
            port_addr,
            msg_len,
            seq,
            idle_time,
            time_status,
            week,
            ms,
            recv_status: ReceiverStatusFlags::from_bits_truncate(recv_status_raw),
            recv_ver,
        })
    }
}

// ──────────────────── Receiver Status Flags (OEM7) ────────────────────

bitflags! {
    /// OEM7 Receiver status flags (Table 198 in OEM7 Commands and Logs Reference)
    #[derive(Default)]
    pub struct ReceiverStatusFlags: u32 {
        /// Error flag — see receiver error word
        const ERROR_PRESENT               = 0x0000_0001;
        /// Temperature warning
        const TEMPERATURE_WARNING         = 0x0000_0002;
        /// Voltage supply warning
        const VOLTAGE_SUPPLY_WARNING      = 0x0000_0004;
        /// Primary antenna not powered
        const ANTENNA_NOT_POWERED         = 0x0000_0008;
        /// LNA failure
        const LNA_FAILURE                 = 0x0000_0010;
        /// Primary antenna open circuit
        const ANTENNA_OPEN                = 0x0000_0020;
        /// Primary antenna short circuit
        const ANTENNA_SHORT               = 0x0000_0040;
        /// CPU overload
        const CPU_OVERLOAD                = 0x0000_0080;
        /// COM port TX buffer overrun
        const COM_BUFFER_OVERRUN          = 0x0000_0100;
        /// Spoofing detected (OEM7-specific)
        const SPOOFING_DETECTED           = 0x0000_0200;
        /// Link overrun
        const LINK_OVERRUN                = 0x0000_0800;
        /// Input overrun (OEM7-specific)
        const INPUT_OVERRUN               = 0x0000_1000;
        /// Auxiliary transmit overrun
        const AUX_TRANSMIT_OVERRUN        = 0x0000_2000;
        /// Antenna gain out of range
        const ANTENNA_GAIN_OUT_OF_RANGE   = 0x0000_4000;
        /// Jammer detected (OEM7-specific)
        const JAMMER_DETECTED             = 0x0000_8000;
        /// INS has been reset
        const INS_RESET                   = 0x0001_0000;
        /// IMU communication failure
        const IMU_COMM_FAILURE            = 0x0002_0000;
        /// GPS almanac invalid / UTC unknown
        const GPS_ALMANAC_INVALID         = 0x0004_0000;
        /// Position solution invalid
        const POSITION_SOLUTION_INVALID   = 0x0008_0000;
        /// Position is fixed
        const POSITION_FIXED              = 0x0010_0000;
        /// Clock steering disabled
        const CLOCK_STEERING_DISABLED     = 0x0020_0000;
        /// Clock model invalid
        const CLOCK_MODEL_INVALID         = 0x0040_0000;
        /// External oscillator locked
        const EXTERNAL_OSCILLATOR_LOCKED  = 0x0080_0000;
        /// Software resource warning
        const SOFTWARE_RESOURCE_WARNING   = 0x0100_0000;
        /// HDR tracking mode (OEM7-specific)
        const HDR_TRACKING                = 0x0800_0000;
        /// Digital filtering enabled
        const DIGITAL_FILTERING_ENABLED   = 0x1000_0000;
        /// Auxiliary 3 status event
        const AUX3_STATUS_EVENT           = 0x2000_0000;
        /// Auxiliary 2 status event
        const AUX2_STATUS_EVENT           = 0x4000_0000;
        /// Auxiliary 1 status event
        const AUX1_STATUS_EVENT           = 0x8000_0000;
    }
}

impl ReceiverStatusFlags {
    /// Convert the flags into a vector of string representations.
    pub fn to_vec(self) -> Vec<String> {
        format!("{:?}", self)
            .split(" | ")
            .map(|x| x.to_string())
            .collect()
    }
}

// ──────────────────── BESTXYZ Log Body ────────────────────

/// Parsed BESTXYZ binary log body (Message ID 241)
#[derive(Clone, Debug, Default)]
pub struct BestXYZ {
    /// Position solution status
    pub pos_sol_status: u32,
    /// Position type
    pub pos_type: u32,
    /// Position X (m, ECEF)
    pub pos_x: f64,
    /// Position Y (m, ECEF)
    pub pos_y: f64,
    /// Position Z (m, ECEF)
    pub pos_z: f64,
    /// Std dev of position X (m)
    pub pos_x_sigma: f32,
    /// Std dev of position Y (m)
    pub pos_y_sigma: f32,
    /// Std dev of position Z (m)
    pub pos_z_sigma: f32,
    /// Velocity solution status
    pub vel_sol_status: u32,
    /// Velocity type
    pub vel_type: u32,
    /// Velocity X (m/s, ECEF)
    pub vel_x: f64,
    /// Velocity Y (m/s, ECEF)
    pub vel_y: f64,
    /// Velocity Z (m/s, ECEF)
    pub vel_z: f64,
    /// Std dev of velocity X (m/s)
    pub vel_x_sigma: f32,
    /// Std dev of velocity Y (m/s)
    pub vel_y_sigma: f32,
    /// Std dev of velocity Z (m/s)
    pub vel_z_sigma: f32,
    /// Station ID
    pub stn_id: [u8; 4],
    /// Velocity latency (s)
    pub vel_latency: f32,
    /// Differential age (s)
    pub diff_age: f32,
    /// Solution age (s)
    pub sol_age: f32,
    /// Number of satellites tracked
    pub num_svs: u8,
    /// Number of satellites used in solution
    pub num_soln_svs: u8,
}

impl BestXYZ {
    /// Parse the BESTXYZ log body from binary data (112 bytes).
    pub fn parse(body: &[u8]) -> OEM7Result<Self> {
        if body.len() < 112 {
            return Err(OEM7Error::ParseError(format!(
                "BESTXYZ body too short: {} bytes (need 112)",
                body.len()
            )));
        }
        let mut c = Cursor::new(body);
        Ok(BestXYZ {
            pos_sol_status: c.read_u32::<LittleEndian>()?,
            pos_type: c.read_u32::<LittleEndian>()?,
            pos_x: c.read_f64::<LittleEndian>()?,
            pos_y: c.read_f64::<LittleEndian>()?,
            pos_z: c.read_f64::<LittleEndian>()?,
            pos_x_sigma: c.read_f32::<LittleEndian>()?,
            pos_y_sigma: c.read_f32::<LittleEndian>()?,
            pos_z_sigma: c.read_f32::<LittleEndian>()?,
            vel_sol_status: c.read_u32::<LittleEndian>()?,
            vel_type: c.read_u32::<LittleEndian>()?,
            vel_x: c.read_f64::<LittleEndian>()?,
            vel_y: c.read_f64::<LittleEndian>()?,
            vel_z: c.read_f64::<LittleEndian>()?,
            vel_x_sigma: c.read_f32::<LittleEndian>()?,
            vel_y_sigma: c.read_f32::<LittleEndian>()?,
            vel_z_sigma: c.read_f32::<LittleEndian>()?,
            stn_id: {
                let mut buf = [0u8; 4];
                std::io::Read::read_exact(&mut c, &mut buf)?;
                buf
            },
            vel_latency: c.read_f32::<LittleEndian>()?,
            diff_age: c.read_f32::<LittleEndian>()?,
            sol_age: c.read_f32::<LittleEndian>()?,
            num_svs: c.read_u8()?,
            num_soln_svs: c.read_u8()?,
        })
    }
}

// ──────────────────── VERSION Log Body ────────────────────

/// A single hardware/firmware component from the VERSION log
#[derive(Clone, Debug, Default)]
pub struct Component {
    /// Component type (0=Unknown, 1=GPSCARD, 3=ENCLOSURE, 7=IMUCARD, etc.)
    pub comp_type: u32,
    /// Model identifier
    pub model: String,
    /// Product serial number
    pub serial_num: String,
    /// Hardware version
    pub hw_version: String,
    /// Firmware/software version
    pub sw_version: String,
    /// Boot code version
    pub boot_version: String,
    /// Compile date (YYYY/Mmm/DD)
    pub compile_date: String,
    /// Compile time (HH:MM:SS)
    pub compile_time: String,
}

/// Parsed VERSION binary log body (Message ID 37)
#[derive(Clone, Debug, Default)]
pub struct VersionLog {
    /// Number of components
    pub num_components: u32,
    /// Component data
    pub components: Vec<Component>,
}

/// Read a fixed-size string from a byte buffer, trimming null bytes.
fn read_fixed_string(c: &mut Cursor<&[u8]>, len: usize) -> OEM7Result<String> {
    let mut buf = vec![0u8; len];
    std::io::Read::read_exact(c, &mut buf)?;
    Ok(String::from_utf8_lossy(&buf)
        .trim_end_matches('\0')
        .trim()
        .to_string())
}

impl VersionLog {
    /// Parse the VERSION log body from binary data.
    /// Body is 4 + (num_components × 108) bytes.
    pub fn parse(body: &[u8]) -> OEM7Result<Self> {
        if body.len() < 4 {
            return Err(OEM7Error::ParseError(
                "VERSION body too short".to_string(),
            ));
        }
        let mut c = Cursor::new(body);
        let num_components = c.read_u32::<LittleEndian>()?;

        let expected = 4 + (num_components as usize) * 108;
        if body.len() < expected {
            return Err(OEM7Error::ParseError(format!(
                "VERSION body too short: {} bytes (need {} for {} components)",
                body.len(),
                expected,
                num_components
            )));
        }

        let mut components = Vec::with_capacity(num_components as usize);
        for _ in 0..num_components {
            let comp_type = c.read_u32::<LittleEndian>()?;
            let model = read_fixed_string(&mut c, 16)?;
            let serial_num = read_fixed_string(&mut c, 16)?;
            let hw_version = read_fixed_string(&mut c, 16)?;
            let sw_version = read_fixed_string(&mut c, 16)?;
            let boot_version = read_fixed_string(&mut c, 16)?;
            let compile_date = read_fixed_string(&mut c, 12)?;
            let compile_time = read_fixed_string(&mut c, 12)?;

            components.push(Component {
                comp_type,
                model,
                serial_num,
                hw_version,
                sw_version,
                boot_version,
                compile_date,
                compile_time,
            });
        }

        Ok(VersionLog {
            num_components,
            components,
        })
    }
}

// ──────────────────── RXSTATUS Log Body ────────────────────

/// Parsed RXSTATUS binary log body (Message ID 93)
#[derive(Clone, Debug, Default)]
pub struct RxStatusLog {
    /// Receiver error word
    pub error: u32,
    /// Number of status codes
    pub num_stats: u32,
    /// Receiver status word
    pub rx_stat: u32,
    /// Receiver status priority mask
    pub rx_stat_pri: u32,
    /// Receiver status event set mask
    pub rx_stat_set: u32,
    /// Receiver status event clear mask
    pub rx_stat_clear: u32,
    /// Auxiliary 1 status word
    pub aux1_stat: u32,
    /// Auxiliary 2 status word
    pub aux2_stat: u32,
    /// Auxiliary 3 status word
    pub aux3_stat: u32,
    /// Auxiliary 4 status word
    pub aux4_stat: u32,
}

impl RxStatusLog {
    /// Parse the RXSTATUS log body from binary data (at least 88 bytes).
    pub fn parse(body: &[u8]) -> OEM7Result<Self> {
        if body.len() < 88 {
            return Err(OEM7Error::ParseError(format!(
                "RXSTATUS body too short: {} bytes (need 88)",
                body.len()
            )));
        }
        let mut c = Cursor::new(body);
        let error = c.read_u32::<LittleEndian>()?;
        let num_stats = c.read_u32::<LittleEndian>()?;

        // Receiver status: status, pri, set, clear
        let rx_stat = c.read_u32::<LittleEndian>()?;
        let rx_stat_pri = c.read_u32::<LittleEndian>()?;
        let rx_stat_set = c.read_u32::<LittleEndian>()?;
        let rx_stat_clear = c.read_u32::<LittleEndian>()?;

        // Aux1: status, pri, set, clear
        let aux1_stat = c.read_u32::<LittleEndian>()?;
        let _aux1_pri = c.read_u32::<LittleEndian>()?;
        let _aux1_set = c.read_u32::<LittleEndian>()?;
        let _aux1_clear = c.read_u32::<LittleEndian>()?;

        // Aux2
        let aux2_stat = c.read_u32::<LittleEndian>()?;
        let _aux2_pri = c.read_u32::<LittleEndian>()?;
        let _aux2_set = c.read_u32::<LittleEndian>()?;
        let _aux2_clear = c.read_u32::<LittleEndian>()?;

        // Aux3
        let aux3_stat = c.read_u32::<LittleEndian>()?;
        let _aux3_pri = c.read_u32::<LittleEndian>()?;
        let _aux3_set = c.read_u32::<LittleEndian>()?;
        let _aux3_clear = c.read_u32::<LittleEndian>()?;

        // Aux4
        let aux4_stat = c.read_u32::<LittleEndian>()?;
        // remaining aux4 fields: pri, set, clear
        let _aux4_pri = c.read_u32::<LittleEndian>()?;
        let _aux4_set = c.read_u32::<LittleEndian>()?;
        let _aux4_clear = c.read_u32::<LittleEndian>()?;

        Ok(RxStatusLog {
            error,
            num_stats,
            rx_stat,
            rx_stat_pri,
            rx_stat_set,
            rx_stat_clear,
            aux1_stat,
            aux2_stat,
            aux3_stat,
            aux4_stat,
        })
    }
}
