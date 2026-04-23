//! MR2xH40 protocol constants and helpers.

/// Total memory capacity in bits.
pub const CAPACITY_BITS: u32 = 4_194_304;
/// Total memory capacity in bytes.
pub const CAPACITY_BYTES: u32 = 524_288;
/// Addressable word width in bits.
pub const WORD_SIZE_BITS: u8 = 8;

/// Number of address bytes in READ/WRITE instructions.
pub const ADDRESS_BYTES: usize = 3;

pub(crate) const CMD_WREN: u8 = 0x06;
pub(crate) const CMD_WRDI: u8 = 0x04;
pub(crate) const CMD_RDSR: u8 = 0x05;
pub(crate) const CMD_WRSR: u8 = 0x01;
pub(crate) const CMD_READ: u8 = 0x03;
pub(crate) const CMD_WRITE: u8 = 0x02;
pub(crate) const CMD_SLEEP: u8 = 0xB9;
pub(crate) const CMD_WAKE: u8 = 0xAB;

pub(crate) const STATUS_WEL: u8 = 0b0000_0010;

pub(crate) fn command_address_header(command: u8, offset: u32) -> [u8; 1 + ADDRESS_BYTES] {
    [
        command,
        ((offset >> 16) & 0xFF) as u8,
        ((offset >> 8) & 0xFF) as u8,
        (offset & 0xFF) as u8,
    ]
}
