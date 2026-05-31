//
// Copyright (C) 2019 Kubos Corporation
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

//! Packet Definition for SpacePacket

use crate::packet::{LinkPacket, PayloadType};
use crate::{CommsResult, CommsServiceError};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::convert::TryFrom;
use std::io::Cursor;

const PRIMARY_HEADER_LEN: usize = 6;
const SECONDARY_HEADER_LEN: usize = 10;
const MIN_PACKET_LEN: usize = PRIMARY_HEADER_LEN + SECONDARY_HEADER_LEN;
const SEQUENCE_FLAGS_UNSEGMENTED: u8 = 0b11;

#[derive(Eq, Debug, PartialEq)]
struct PrimaryHeader {
    /// Packet Version Number - 3 bits
    version: u8,
    /// Packet Type - 1 bit
    packet_type: u8,
    /// Secondary Header Flag - 1 bit
    sec_header_flag: u8,
    /// Application Process ID - 11 bits
    app_proc_id: u16,
    /// Sequence Flags - 2 bits
    sequence_flags: u8,
    /// Packet Sequence Count or Packet Name - 14 bits
    sequence_count: u16,
    /// Packet Data Length - 2 bytes
    data_length: u16,
}

#[derive(Eq, Debug, PartialEq)]
struct SecondaryHeader {
    /// Command ID from MT - 64 bits
    command_id: u64,
    /// Destination service port - 16 bits
    destination_port: u16,
}

/// Structure used to implement SpacePacket version of LinkPacket
#[derive(Eq, Debug, PartialEq)]
pub struct SpacePacket {
    primary_header: PrimaryHeader,
    secondary_header: SecondaryHeader,
    payload: Vec<u8>,
}

impl LinkPacket for SpacePacket {
    fn build(
        command_id: u64,
        payload_type: PayloadType,
        destination_port: u16,
        payload: &[u8],
    ) -> CommsResult<Box<Self>> {
        // The SpacePacket is the inner comms envelope. Its payload is the
        // GraphQL body or UDP bytes; its destination is a local service port.
        //
        // Fragmentation is intentionally not represented here. Larger payloads
        // should be split/reassembled by the CSP SFP layer before this packet
        // reaches kubos-comms.
        let packet_data_len = SECONDARY_HEADER_LEN + payload.len();
        let data_length =
            u16::try_from(packet_data_len).map_err(|_| {
                CommsServiceError::ParsingError(format!(
                    "SpacePacket payload is too large: packet data length {packet_data_len} exceeds u16"
                ))
            })?;

        Ok(Box::new(SpacePacket {
            primary_header: PrimaryHeader {
                version: 0,
                packet_type: 0,
                sec_header_flag: 0,
                app_proc_id: u16::from(payload_type),
                sequence_flags: SEQUENCE_FLAGS_UNSEGMENTED,
                sequence_count: 0,
                data_length,
            },
            secondary_header: SecondaryHeader {
                command_id,
                destination_port,
            },
            payload: payload.to_vec(),
        }))
    }

    fn parse(raw: &[u8]) -> CommsResult<Box<Self>> {
        // Parse a single complete SpacePacket from the CSP payload. Fragment
        // reassembly belongs to CSP SFP, so segmented SpacePackets are rejected.
        if raw.len() < MIN_PACKET_LEN {
            return Err(CommsServiceError::ParsingError(format!(
                "SpacePacket is too short: {} bytes, minimum is {MIN_PACKET_LEN}",
                raw.len()
            )));
        }

        let mut reader = Cursor::new(raw.to_vec());

        let header_0 = reader.read_u16::<BigEndian>()?;
        let version = ((header_0 & 0xE000) >> 13) as u8;
        let packet_type = ((header_0 & 0x1000) >> 12) as u8;
        let sec_header_flag = ((header_0 & 0x800) >> 11) as u8;
        let app_proc_id = header_0 & 0x7FF;

        let header_1 = reader.read_u16::<BigEndian>()?;
        let sequence_flags = ((header_1 & 0xC000) >> 14) as u8;
        let sequence_count = header_1 & 0x3FFF;

        let data_length = reader.read_u16::<BigEndian>()?;
        let expected_len = PRIMARY_HEADER_LEN + usize::from(data_length);
        if raw.len() != expected_len {
            return Err(CommsServiceError::ParsingError(format!(
                "SpacePacket length mismatch: header declares {expected_len} bytes, received {}",
                raw.len()
            )));
        }

        if sequence_flags != SEQUENCE_FLAGS_UNSEGMENTED {
            return Err(CommsServiceError::ParsingError(format!(
                "segmented SpacePacket received with sequence flags {sequence_flags:#04b}; use CSP SFP for fragmentation"
            )));
        }

        let command_id = reader.read_u64::<BigEndian>()?;
        let destination_port = reader.read_u16::<BigEndian>()?;
        let pos = reader.position() as usize;
        let payload = raw[pos..].to_vec();
        Ok(Box::new(SpacePacket {
            primary_header: PrimaryHeader {
                version,
                packet_type,
                sec_header_flag,
                app_proc_id,
                sequence_flags,
                sequence_count,
                data_length,
            },
            secondary_header: SecondaryHeader {
                command_id,
                destination_port,
            },
            payload,
        }))
    }

    fn to_bytes(&self) -> CommsResult<Vec<u8>> {
        let mut bytes = vec![];

        let header_0: u16 = (self.primary_header.app_proc_id)
            | (u16::from(self.primary_header.sec_header_flag) << 11)
            | (u16::from(self.primary_header.packet_type) << 12)
            | (u16::from(self.primary_header.version) << 13);

        let header_1 = (self.primary_header.sequence_count)
            | (u16::from(self.primary_header.sequence_flags) << 14);

        let header_2 = self.primary_header.data_length;

        bytes.write_u16::<BigEndian>(header_0)?;
        bytes.write_u16::<BigEndian>(header_1)?;
        bytes.write_u16::<BigEndian>(header_2)?;
        bytes.write_u64::<BigEndian>(self.secondary_header.command_id)?;
        bytes.write_u16::<BigEndian>(self.secondary_header.destination_port)?;
        bytes.append(&mut self.payload.clone());

        Ok(bytes)
    }

    fn command_id(&self) -> u64 {
        self.secondary_header.command_id
    }

    fn payload(&self) -> Vec<u8> {
        self.payload.clone()
    }

    fn payload_type(&self) -> PayloadType {
        PayloadType::from(self.primary_header.app_proc_id)
    }

    fn destination(&self) -> u16 {
        self.secondary_header.destination_port
    }

    fn validate(&self) -> bool {
        self.primary_header.sequence_flags == SEQUENCE_FLAGS_UNSEGMENTED
            && usize::from(self.primary_header.data_length)
                == SECONDARY_HEADER_LEN + self.payload.len()
    }
}

#[cfg(test)]
mod tests {
    use super::SECONDARY_HEADER_LEN;
    use crate::*;

    #[test]
    fn do_build_parse() {
        let packet =
            SpacePacket::build(1294, PayloadType::GraphQL, 15001, &[5, 4, 3, 2, 1]).unwrap();
        println!("packet {:?}", packet);

        let raw = packet.to_bytes();
        println!("bytes {:?}", raw);

        let parsed = SpacePacket::parse(&raw.unwrap());
        println!("parsed {:?}", parsed);

        assert_eq!(packet, parsed.unwrap());
    }

    #[test]
    fn parse_python_spacepacket() {
        let raw = b"\x00\x01\xc0\x00\x00\x0f\x00\x00\x00\x00\x00\x00\x00o\x05\xdcquery";
        let parsed = SpacePacket::parse(raw).unwrap();
        dbg!(parsed);
    }

    #[test]
    fn build_marks_packet_unsegmented() {
        let packet = SpacePacket::build(1, PayloadType::GraphQL, 15001, b"query").unwrap();
        let raw = packet.to_bytes().unwrap();
        let sequence_header = u16::from_be_bytes([raw[2], raw[3]]);

        assert_eq!((sequence_header & 0xC000) >> 14, 0b11);
    }

    #[test]
    fn parse_rejects_segmented_packet() {
        let packet = SpacePacket::build(1, PayloadType::GraphQL, 15001, b"query").unwrap();
        let mut raw = packet.to_bytes().unwrap();
        raw[2] = 0x00;
        raw[3] = 0x00;

        assert!(SpacePacket::parse(&raw).is_err());
    }

    #[test]
    fn parse_rejects_truncated_packet() {
        let packet = SpacePacket::build(1, PayloadType::GraphQL, 15001, b"query").unwrap();
        let mut raw = packet.to_bytes().unwrap();
        raw.pop();

        assert!(SpacePacket::parse(&raw).is_err());
    }

    #[test]
    fn parse_rejects_length_mismatch() {
        let packet = SpacePacket::build(1, PayloadType::GraphQL, 15001, b"query").unwrap();
        let mut raw = packet.to_bytes().unwrap();
        raw[5] += 1;

        assert!(SpacePacket::parse(&raw).is_err());
    }

    #[test]
    fn build_rejects_payload_too_large_for_length_field() {
        let payload = vec![0; usize::from(u16::MAX) - SECONDARY_HEADER_LEN + 1];

        assert!(SpacePacket::build(1, PayloadType::GraphQL, 15001, &payload).is_err());
    }
}
