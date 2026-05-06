use fram_service::backend::{
    ByteStorage, FileImageBackend, memory_address_bytes, validate_fm24cl64b_i2c_addr,
};
use fram_service::layout::FramStorage;
use fram_service::model::{MissionKey, MissionValue};
use tempfile::TempDir;

fn storage() -> (TempDir, FramStorage) {
    let tmp = TempDir::new().expect("tempdir");
    let path = tmp.path().join("fram.img");
    let backend = FileImageBackend::new(path.to_str().unwrap(), 8192).expect("backend");
    let storage = FramStorage::mount(Box::new(backend)).expect("mount");
    (tmp, storage)
}

#[test]
fn file_backend_enforces_8192_byte_capacity() {
    let tmp = TempDir::new().expect("tempdir");
    let path = tmp.path().join("fram.img");
    let mut backend = FileImageBackend::new(path.to_str().unwrap(), 8192).expect("backend");

    assert_eq!(backend.capacity(), 8192);
    backend.write(8191, &[0xAA]).expect("last byte write");
    let err = backend.write(8192, &[0xBB]).unwrap_err().to_string();
    assert!(err.contains("out of bounds"));
}

#[test]
fn fm24cl64b_uses_linux_7_bit_i2c_address_range() {
    validate_fm24cl64b_i2c_addr(0x50).expect("A2/A1/A0 all low");
    validate_fm24cl64b_i2c_addr(0x57).expect("A2/A1/A0 all high");

    let err = validate_fm24cl64b_i2c_addr(0xA0).unwrap_err().to_string();
    assert!(err.contains("7-bit"));
    assert!(err.contains("0xA0"));
}

#[test]
fn fm24cl64b_memory_address_is_two_bytes_msb_first() {
    assert_eq!(memory_address_bytes(0x0000, 2).unwrap(), [0x00, 0x00]);
    assert_eq!(memory_address_bytes(0x0100, 2).unwrap(), [0x01, 0x00]);
    assert_eq!(memory_address_bytes(0x1FFF, 2).unwrap(), [0x1F, 0xFF]);
}

#[test]
fn fixed_slots_read_empty_as_missing() {
    let (_tmp, mut storage) = storage();
    assert_eq!(storage.read_key(MissionKey::Deployed).unwrap(), None);
}

#[test]
fn redundant_slots_keep_latest_sequence() {
    let (_tmp, mut storage) = storage();

    storage
        .write_key(MissionKey::Deployed, &MissionValue::Bool(true))
        .expect("write true");
    storage
        .write_key(MissionKey::Deployed, &MissionValue::Bool(false))
        .expect("write false");

    assert_eq!(
        storage.read_key(MissionKey::Deployed).unwrap(),
        Some(MissionValue::Bool(false))
    );
}

#[test]
fn deploy_start_can_be_unset_or_timestamp() {
    let (_tmp, mut storage) = storage();

    storage
        .write_key(MissionKey::DeployStart, &MissionValue::Timestamp(None))
        .expect("write unset");
    assert_eq!(
        storage.read_key(MissionKey::DeployStart).unwrap(),
        Some(MissionValue::Timestamp(None))
    );

    storage
        .write_key(
            MissionKey::DeployStart,
            &MissionValue::Timestamp(Some(1_770_000_000)),
        )
        .expect("write timestamp");
    assert_eq!(
        storage.read_key(MissionKey::DeployStart).unwrap(),
        Some(MissionValue::Timestamp(Some(1_770_000_000)))
    );
}
