use std::sync::{Arc, Mutex};

use crate::backend::{ByteStorage, FileImageBackend};
use crate::fs::{FilePayload, FileRecord, StorageStats, TinyMramFs};

#[derive(Clone)]
pub struct Subsystem {
    fs: Arc<Mutex<TinyMramFs>>,
    backend_name: String,
}

impl Subsystem {
    pub fn from_config(config: &kubos_service::Config) -> Result<Self, String> {
        let backend = config
            .get("backend")
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .unwrap_or_else(|| "file".to_string());

        let storage: Box<dyn ByteStorage> = match backend.as_str() {
            "file" => {
                let path = config
                    .get("image_path")
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_else(|| "/tmp/mram-service.img".to_string());

                let capacity = config
                    .get("image_capacity_bytes")
                    .and_then(|v| v.as_integer())
                    .map(|v| v as u32)
                    .unwrap_or(rust_mram::CAPACITY_BYTES);

                Box::new(FileImageBackend::new(&path, capacity).map_err(|e| e.to_string())?)
            }
            "spidev" => {
                #[cfg(feature = "spidev")]
                {
                    let device = config
                        .get("spi_device")
                        .and_then(|v| v.as_str().map(|s| s.to_string()))
                        .unwrap_or_else(|| "/dev/spidev1.0".to_string());

                    let speed_hz = config
                        .get("spi_speed_hz")
                        .and_then(|v| v.as_integer())
                        .map(|v| v as u32)
                        .unwrap_or(10_000_000);

                    let mode = config
                        .get("spi_mode")
                        .and_then(|v| v.as_integer())
                        .map(|v| v as u8)
                        .unwrap_or(0);

                    Box::new(
                        crate::backend::SpidevBackend::new(&device, speed_hz, mode)
                            .map_err(|e| e.to_string())?,
                    )
                }

                #[cfg(not(feature = "spidev"))]
                {
                    return Err(
                        "backend='spidev' requested but service was built without 'spidev' feature"
                            .to_string(),
                    );
                }
            }
            other => {
                return Err(format!(
                    "unsupported backend '{other}'. expected 'file' or 'spidev'"
                ));
            }
        };

        let fs = TinyMramFs::mount(storage).map_err(|e| e.to_string())?;

        Ok(Self {
            fs: Arc::new(Mutex::new(fs)),
            backend_name: backend,
        })
    }

    pub fn backend_name(&self) -> &str {
        &self.backend_name
    }

    pub fn list_files(&self) -> Result<Vec<FileRecord>, String> {
        let mut fs = self.fs.lock().map_err(lock_error)?;
        fs.list_files().map_err(|e| e.to_string())
    }

    pub fn file(&self, name: &str) -> Result<Option<FileRecord>, String> {
        let mut fs = self.fs.lock().map_err(lock_error)?;
        fs.file(name).map_err(|e| e.to_string())
    }

    pub fn read_file(
        &self,
        name: &str,
        offset: u32,
        length: Option<u32>,
    ) -> Result<FilePayload, String> {
        let mut fs = self.fs.lock().map_err(lock_error)?;
        fs.read_file(name, offset, length)
            .map_err(|e| e.to_string())
    }

    pub fn write_file(
        &self,
        name: &str,
        mime_type: Option<&str>,
        compressed: bool,
        data: &[u8],
    ) -> Result<FileRecord, String> {
        let mut fs = self.fs.lock().map_err(lock_error)?;
        fs.write_file(name, mime_type, compressed, data)
            .map_err(|e| e.to_string())
    }

    pub fn delete_file(&self, name: &str) -> Result<bool, String> {
        let mut fs = self.fs.lock().map_err(lock_error)?;
        fs.delete_file(name).map_err(|e| e.to_string())
    }

    pub fn format(&self) -> Result<(), String> {
        let mut fs = self.fs.lock().map_err(lock_error)?;
        fs.format().map_err(|e| e.to_string())
    }

    pub fn stats(&self) -> Result<StorageStats, String> {
        let mut fs = self.fs.lock().map_err(lock_error)?;
        fs.stats().map_err(|e| e.to_string())
    }
}

fn lock_error<E>(_: E) -> String {
    "subsystem lock poisoned".to_string()
}
