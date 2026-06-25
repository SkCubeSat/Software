//! Packet helpers for CubeSpace ADCS telecommands and telemetry.

/// CubeSpace CAN message type for telecommands.
pub const MSG_TYPE_TC: u8 = 1;
/// CubeSpace CAN message type for acknowledged telecommands.
pub const MSG_TYPE_TC_ACK: u8 = 2;
/// CubeSpace CAN message type for rejected telecommands.
pub const MSG_TYPE_TC_NACK: u8 = 3;
/// CubeSpace CAN message type for telemetry requests.
pub const MSG_TYPE_TLM_REQ: u8 = 4;
/// CubeSpace CAN message type for extended telemetry responses.
pub const MSG_TYPE_TLM_RESP_EXT: u8 = 8;

/// Decoded CubeSpace CAN identifier fields.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanIdFields {
    /// CubeSpace message type.
    pub msg_type: u8,
    /// Telecommand or telemetry ID.
    pub tctlm_id: u8,
    /// Source node address.
    pub src_addr: u8,
    /// Destination node address.
    pub dst_addr: u8,
}

/// Builds a CubeSpace extended CAN ID without the Linux SocketCAN EFF flag.
pub fn build_can_id(msg_type: u8, tctlm_id: u8, src_addr: u8, dst_addr: u8) -> u32 {
    ((u32::from(msg_type) & 0x1F) << 24)
        | (u32::from(tctlm_id) << 16)
        | (u32::from(src_addr) << 8)
        | u32::from(dst_addr)
}

/// Decodes a CubeSpace extended CAN ID without the Linux SocketCAN EFF flag.
pub fn decode_can_id(can_id: u32) -> CanIdFields {
    CanIdFields {
        msg_type: ((can_id >> 24) & 0x1F) as u8,
        tctlm_id: ((can_id >> 16) & 0xFF) as u8,
        src_addr: ((can_id >> 8) & 0xFF) as u8,
        dst_addr: (can_id & 0xFF) as u8,
    }
}

/// Builds an empty telemetry request payload.
pub fn telemetry_request_payload() -> Vec<u8> {
    Vec::new()
}
