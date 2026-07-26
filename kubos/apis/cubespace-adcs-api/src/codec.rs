//! Bit-level codec helpers for generated CubeSpace ADCS types.

use crate::{AdcsError, AdcsResult};

/// Reads an unsigned little-endian bitfield.
pub fn read_unsigned(
    payload: &[u8],
    offset_bits: u32,
    length_bits: u32,
    name: &str,
) -> AdcsResult<u128> {
    if length_bits > 128 {
        return Err(invalid_field(
            name,
            "integer fields larger than 128 bits are not supported",
        ));
    }

    let mut value = 0_u128;
    for bit in 0..length_bits {
        if read_bit(payload, offset_bits + bit)? {
            value |= 1_u128 << bit;
        }
    }

    Ok(value)
}

/// Reads a signed little-endian bitfield.
pub fn read_signed(
    payload: &[u8],
    offset_bits: u32,
    length_bits: u32,
    name: &str,
) -> AdcsResult<i128> {
    let unsigned = read_unsigned(payload, offset_bits, length_bits, name)?;
    if length_bits == 0 {
        return Ok(0);
    }

    let sign_bit = 1_u128 << (length_bits - 1);
    if unsigned & sign_bit == 0 {
        Ok(unsigned as i128)
    } else {
        Ok((unsigned as i128) - (1_i128 << length_bits))
    }
}

/// Reads a 32-bit floating point value.
pub fn read_f32(payload: &[u8], offset_bits: u32, length_bits: u32, name: &str) -> AdcsResult<f32> {
    if length_bits != 32 {
        return Err(invalid_field(name, "FLOAT fields must be 32 bits"));
    }
    Ok(f32::from_bits(
        read_unsigned(payload, offset_bits, length_bits, name)? as u32,
    ))
}

/// Reads a 64-bit floating point value.
pub fn read_f64(payload: &[u8], offset_bits: u32, length_bits: u32, name: &str) -> AdcsResult<f64> {
    if length_bits != 64 {
        return Err(invalid_field(name, "DOUBLE fields must be 64 bits"));
    }
    Ok(f64::from_bits(
        read_unsigned(payload, offset_bits, length_bits, name)? as u64,
    ))
}

/// Reads a byte-aligned byte field.
pub fn read_bytes(
    payload: &[u8],
    offset_bits: u32,
    length_bits: u32,
    name: &str,
) -> AdcsResult<Vec<u8>> {
    if offset_bits % 8 != 0 || length_bits % 8 != 0 {
        return Err(invalid_field(
            name,
            "byte array fields must be byte-aligned",
        ));
    }

    let start = (offset_bits / 8) as usize;
    let len = (length_bits / 8) as usize;
    let end = start + len;
    if end > payload.len() {
        return Err(AdcsError::InvalidPayloadLength {
            expected: end,
            actual: payload.len(),
        });
    }

    Ok(payload[start..end].to_vec())
}

/// Writes an unsigned little-endian bitfield.
pub fn write_unsigned(
    payload: &mut [u8],
    offset_bits: u32,
    length_bits: u32,
    name: &str,
    value: u128,
) -> AdcsResult<()> {
    if length_bits > 128 {
        return Err(invalid_field(
            name,
            "integer fields larger than 128 bits are not supported",
        ));
    }
    if length_bits < 128 && value >= (1_u128 << length_bits) {
        return Err(AdcsError::InvalidValue {
            description: format!("value {} does not fit in field '{}'", value, name),
        });
    }

    for bit in 0..length_bits {
        write_bit(payload, offset_bits + bit, (value & (1_u128 << bit)) != 0)?;
    }

    Ok(())
}

/// Writes a signed little-endian bitfield.
pub fn write_signed(
    payload: &mut [u8],
    offset_bits: u32,
    length_bits: u32,
    name: &str,
    value: i128,
) -> AdcsResult<()> {
    if length_bits == 0 || length_bits > 127 {
        return Err(invalid_field(
            name,
            "signed integer fields must be between 1 and 127 bits",
        ));
    }

    let min = -(1_i128 << (length_bits - 1));
    let max = (1_i128 << (length_bits - 1)) - 1;
    if value < min || value > max {
        return Err(AdcsError::InvalidValue {
            description: format!("value {} does not fit in field '{}'", value, name),
        });
    }

    let encoded = if value < 0 {
        ((1_i128 << length_bits) + value) as u128
    } else {
        value as u128
    };
    write_unsigned(payload, offset_bits, length_bits, name, encoded)
}

/// Writes a byte-aligned byte field.
pub fn write_bytes(
    payload: &mut [u8],
    offset_bits: u32,
    length_bits: u32,
    name: &str,
    value: &[u8],
) -> AdcsResult<()> {
    if offset_bits % 8 != 0 || length_bits % 8 != 0 {
        return Err(invalid_field(
            name,
            "byte array fields must be byte-aligned",
        ));
    }

    let start = (offset_bits / 8) as usize;
    let len = (length_bits / 8) as usize;
    let end = start + len;
    if end > payload.len() {
        return Err(AdcsError::InvalidPayloadLength {
            expected: end,
            actual: payload.len(),
        });
    }
    if value.len() > len {
        return Err(AdcsError::InvalidValue {
            description: format!(
                "value for field '{}' is {} bytes, maximum is {}",
                name,
                value.len(),
                len
            ),
        });
    }

    payload[start..end].fill(0);
    payload[start..start + value.len()].copy_from_slice(value);
    Ok(())
}

fn read_bit(payload: &[u8], bit_offset: u32) -> AdcsResult<bool> {
    let byte_offset = (bit_offset / 8) as usize;
    if byte_offset >= payload.len() {
        return Err(AdcsError::InvalidPayloadLength {
            expected: byte_offset + 1,
            actual: payload.len(),
        });
    }

    Ok((payload[byte_offset] & (1 << (bit_offset % 8))) != 0)
}

fn write_bit(payload: &mut [u8], bit_offset: u32, value: bool) -> AdcsResult<()> {
    let byte_offset = (bit_offset / 8) as usize;
    if byte_offset >= payload.len() {
        return Err(AdcsError::InvalidPayloadLength {
            expected: byte_offset + 1,
            actual: payload.len(),
        });
    }

    let mask = 1 << (bit_offset % 8);
    if value {
        payload[byte_offset] |= mask;
    } else {
        payload[byte_offset] &= !mask;
    }

    Ok(())
}

fn invalid_field(name: &str, description: &str) -> AdcsError {
    AdcsError::InvalidValue {
        description: format!("{}: '{}'", description, name),
    }
}
