use crc32fast::Hasher;
use thiserror::Error;

use crate::backend::{BackendError, ByteStorage};
use crate::model::{MissionKey, MissionValue, ValueType};

pub const RECORD_SIZE: usize = 32;
pub const SLOTS_PER_KEY: u32 = 2;
pub const RESERVED_KEYS: u32 = 32;

const MAGIC: u16 = 0x4652; // "FR"
const VERSION: u8 = 1;
const PAYLOAD_LEN: usize = 16;
const CRC_OFFSET: usize = 28;

#[derive(Debug, Error)]
pub enum LayoutError {
    #[error("backend error: {0}")]
    Backend(#[from] BackendError),
    #[error("FRAM capacity {capacity} is too small for reserved layout {required}")]
    CapacityTooSmall { capacity: u32, required: u32 },
    #[error("invalid record value for {0}")]
    InvalidValue(String),
}

pub struct FramStorage {
    storage: Box<dyn ByteStorage>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct FramRecord {
    key: MissionKey,
    value: MissionValue,
    sequence: u32,
}

impl FramStorage {
    pub fn mount(storage: Box<dyn ByteStorage>) -> Result<Self, LayoutError> {
        let required = RESERVED_KEYS * SLOTS_PER_KEY * RECORD_SIZE as u32;
        let capacity = storage.capacity();
        if capacity < required {
            return Err(LayoutError::CapacityTooSmall { capacity, required });
        }

        Ok(Self { storage })
    }

    pub fn capacity(&self) -> u32 {
        self.storage.capacity()
    }

    pub fn read_key(&mut self, key: MissionKey) -> Result<Option<MissionValue>, LayoutError> {
        Ok(self.current_record(key)?.map(|record| record.value))
    }

    pub fn write_key(&mut self, key: MissionKey, value: &MissionValue) -> Result<(), LayoutError> {
        validate_key_value(key, value)?;

        let a = self.read_slot(key, 0)?;
        let b = self.read_slot(key, 1)?;
        let current = current_record(a.as_ref(), b.as_ref());
        let sequence = current
            .map(|record| record.sequence.wrapping_add(1))
            .unwrap_or(1);

        let target_slot = match (&a, &b) {
            (None, _) => 0,
            (_, None) => 1,
            (Some(a), Some(b)) if a.sequence <= b.sequence => 0,
            (Some(_), Some(_)) => 1,
        };

        let record = FramRecord {
            key,
            value: value.clone(),
            sequence,
        };
        let encoded = encode_record(&record)?;
        self.storage
            .write(slot_offset(key, target_slot), &encoded)
            .map_err(LayoutError::Backend)?;

        let readback = self.read_slot(key, target_slot)?;
        if readback.as_ref() != Some(&record) {
            return Err(LayoutError::InvalidValue(format!(
                "{} readback verification failed",
                key.env_name()
            )));
        }

        Ok(())
    }

    fn current_record(&mut self, key: MissionKey) -> Result<Option<FramRecord>, LayoutError> {
        let a = self.read_slot(key, 0)?;
        let b = self.read_slot(key, 1)?;
        Ok(current_record(a.as_ref(), b.as_ref()).cloned())
    }

    fn read_slot(&mut self, key: MissionKey, slot: u32) -> Result<Option<FramRecord>, LayoutError> {
        let mut raw = [0u8; RECORD_SIZE];
        self.storage
            .read(slot_offset(key, slot), &mut raw)
            .map_err(LayoutError::Backend)?;
        decode_record(key, &raw)
    }
}

fn current_record<'a>(
    a: Option<&'a FramRecord>,
    b: Option<&'a FramRecord>,
) -> Option<&'a FramRecord> {
    match (a, b) {
        (Some(a), Some(b)) if a.sequence >= b.sequence => Some(a),
        (Some(_), Some(b)) => Some(b),
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    }
}

fn slot_offset(key: MissionKey, slot: u32) -> u32 {
    let key_index = u32::from(key.id() - 1);
    ((key_index * SLOTS_PER_KEY) + slot) * RECORD_SIZE as u32
}

fn encode_record(record: &FramRecord) -> Result<[u8; RECORD_SIZE], LayoutError> {
    validate_key_value(record.key, &record.value)?;

    let mut raw = [0u8; RECORD_SIZE];
    raw[0..2].copy_from_slice(&MAGIC.to_le_bytes());
    raw[2] = VERSION;
    raw[3] = record.key.id();
    raw[4] = record.value.value_type().id();
    raw[6..10].copy_from_slice(&record.sequence.to_le_bytes());

    match &record.value {
        MissionValue::Bool(value) => {
            raw[5] = 1;
            raw[10] = u8::from(*value);
        }
        MissionValue::Timestamp(Some(value)) => {
            raw[5] = 8;
            raw[10..18].copy_from_slice(&value.to_le_bytes());
        }
        MissionValue::Timestamp(None) => {
            raw[5] = 0;
        }
    }

    let crc = crc32(&raw[..CRC_OFFSET]);
    raw[CRC_OFFSET..CRC_OFFSET + 4].copy_from_slice(&crc.to_le_bytes());

    Ok(raw)
}

fn decode_record(
    key: MissionKey,
    raw: &[u8; RECORD_SIZE],
) -> Result<Option<FramRecord>, LayoutError> {
    let magic = u16::from_le_bytes([raw[0], raw[1]]);
    if magic != MAGIC {
        return Ok(None);
    }

    if raw[2] != VERSION || raw[3] != key.id() {
        return Ok(None);
    }

    let expected_crc = u32::from_le_bytes([
        raw[CRC_OFFSET],
        raw[CRC_OFFSET + 1],
        raw[CRC_OFFSET + 2],
        raw[CRC_OFFSET + 3],
    ]);
    if expected_crc != crc32(&raw[..CRC_OFFSET]) {
        return Ok(None);
    }

    let value_type = match ValueType::from_id(raw[4]) {
        Some(value_type) => value_type,
        None => return Ok(None),
    };
    let payload_len = raw[5] as usize;
    if payload_len > PAYLOAD_LEN {
        return Ok(None);
    }

    let value = match value_type {
        ValueType::Bool if payload_len == 1 => MissionValue::Bool(raw[10] != 0),
        ValueType::Timestamp if payload_len == 0 => MissionValue::Timestamp(None),
        ValueType::Timestamp if payload_len == 8 => {
            let mut buf = [0u8; 8];
            buf.copy_from_slice(&raw[10..18]);
            MissionValue::Timestamp(Some(u64::from_le_bytes(buf)))
        }
        _ => return Ok(None),
    };

    if validate_key_value(key, &value).is_err() {
        return Ok(None);
    }

    let sequence = u32::from_le_bytes([raw[6], raw[7], raw[8], raw[9]]);
    Ok(Some(FramRecord {
        key,
        value,
        sequence,
    }))
}

fn validate_key_value(key: MissionKey, value: &MissionValue) -> Result<(), LayoutError> {
    match (key, value) {
        (MissionKey::DeployStart, MissionValue::Timestamp(_)) => Ok(()),
        (MissionKey::DeployStart, _) => Err(LayoutError::InvalidValue(key.env_name().to_string())),
        (_, MissionValue::Bool(_)) => Ok(()),
        (_, _) => Err(LayoutError::InvalidValue(key.env_name().to_string())),
    }
}

fn crc32(data: &[u8]) -> u32 {
    let mut hasher = Hasher::new();
    hasher.update(data);
    hasher.finalize()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::FileImageBackend;
    use tempfile::TempDir;

    fn storage() -> (TempDir, FramStorage) {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("fram.img");
        let backend = FileImageBackend::new(path.to_str().unwrap(), 8192).unwrap();
        (
            tmp,
            FramStorage::mount(Box::new(backend)).expect("mount failed"),
        )
    }

    #[test]
    fn missing_key_reads_none() {
        let (_tmp, mut storage) = storage();
        assert_eq!(storage.read_key(MissionKey::Deployed).unwrap(), None);
    }

    #[test]
    fn write_and_read_bool() {
        let (_tmp, mut storage) = storage();
        storage
            .write_key(MissionKey::Deployed, &MissionValue::Bool(true))
            .unwrap();
        assert_eq!(
            storage.read_key(MissionKey::Deployed).unwrap(),
            Some(MissionValue::Bool(true))
        );
    }

    #[test]
    fn write_and_read_unset_timestamp() {
        let (_tmp, mut storage) = storage();
        storage
            .write_key(MissionKey::DeployStart, &MissionValue::Timestamp(None))
            .unwrap();
        assert_eq!(
            storage.read_key(MissionKey::DeployStart).unwrap(),
            Some(MissionValue::Timestamp(None))
        );
    }
}
