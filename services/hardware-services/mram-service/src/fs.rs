use std::cmp::min;
use std::convert::TryFrom;
use std::time::{SystemTime, UNIX_EPOCH};

use littlefs2::driver::Storage;
use littlefs2::fs::Filesystem;
use littlefs2::io::{Error as LfsError, Result as LfsResult};
use littlefs2::path;
use thiserror::Error;

use crate::backend::{BackendError, ByteStorage};

const FILES_DIR: &str = "/files";
const ATTR_META_ID: u8 = 1;
const ATTR_META_VERSION: u8 = 1;
const META_MAX_MIME_LEN: usize = 32;

const BLOCK_SIZE: usize = 256;
const BLOCK_COUNT: usize = rust_mram::CAPACITY_BYTES as usize / BLOCK_SIZE;

#[derive(Debug, Error)]
pub enum FsError {
    #[error(transparent)]
    Backend(#[from] BackendError),
    #[error("littlefs error: {err:?} (code={code})")]
    LittleFs { err: LfsError, code: i32 },
    #[error("invalid file name length: {0} (max 64)")]
    InvalidName(usize),
    #[error("invalid mime type length: {0} (max 32)")]
    InvalidMime(usize),
    #[error("file not found: {0}")]
    FileNotFound(String),
    #[error("invalid read range")]
    InvalidRange,
    #[error("storage capacity mismatch: got {actual}, expected {expected}")]
    CapacityMismatch { actual: u32, expected: u32 },
    #[error("filesystem initialization failed: {0}")]
    Initialization(String),
}

impl From<LfsError> for FsError {
    fn from(value: LfsError) -> Self {
        Self::LittleFs {
            code: value.code(),
            err: value,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FileRecord {
    pub name: String,
    pub mime_type: String,
    pub compressed: bool,
    pub offset: u32,
    pub size: u32,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone)]
pub struct StorageStats {
    pub capacity_bytes: u32,
    pub allocated_bytes: u32,
    pub live_bytes: u32,
    pub reclaimable_bytes: u32,
    pub free_bytes: u32,
    pub file_count: u32,
}

#[derive(Debug, Clone)]
pub struct FilePayload {
    pub record: FileRecord,
    pub range_offset: u32,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
struct FileMeta {
    mime_type: String,
    compressed: bool,
    created_at: i64,
    updated_at: i64,
}

impl Default for FileMeta {
    fn default() -> Self {
        Self {
            mime_type: "application/octet-stream".to_string(),
            compressed: false,
            created_at: 0,
            updated_at: 0,
        }
    }
}

impl FileMeta {
    fn encode(&self) -> Vec<u8> {
        let mime = self.mime_type.as_bytes();
        let mime_len = min(mime.len(), META_MAX_MIME_LEN);

        let mut out = Vec::with_capacity(1 + 1 + 8 + 8 + 1 + mime_len);
        out.push(ATTR_META_VERSION);
        out.push(if self.compressed { 1 } else { 0 });
        out.extend_from_slice(&self.created_at.to_le_bytes());
        out.extend_from_slice(&self.updated_at.to_le_bytes());
        out.push(mime_len as u8);
        out.extend_from_slice(&mime[..mime_len]);
        out
    }

    fn decode(raw: &[u8]) -> Option<Self> {
        if raw.len() < 19 {
            return None;
        }
        if raw[0] != ATTR_META_VERSION {
            return None;
        }

        let compressed = raw[1] != 0;
        let created_at = i64::from_le_bytes([
            raw[2], raw[3], raw[4], raw[5], raw[6], raw[7], raw[8], raw[9],
        ]);
        let updated_at = i64::from_le_bytes([
            raw[10], raw[11], raw[12], raw[13], raw[14], raw[15], raw[16], raw[17],
        ]);
        let mime_len = min(raw[18] as usize, META_MAX_MIME_LEN);
        if raw.len() < 19 + mime_len {
            return None;
        }

        let mime_type = String::from_utf8_lossy(&raw[19..(19 + mime_len)]).to_string();

        Some(Self {
            mime_type,
            compressed,
            created_at,
            updated_at,
        })
    }
}

struct LittleFsStorage {
    backend: Box<dyn ByteStorage>,
}

impl Storage for LittleFsStorage {
    const READ_SIZE: usize = 1;
    const WRITE_SIZE: usize = 1;
    const BLOCK_SIZE: usize = BLOCK_SIZE;
    const BLOCK_COUNT: usize = BLOCK_COUNT;
    type CACHE_SIZE = littlefs2::consts::U256;
    type LOOKAHEAD_SIZE = littlefs2::consts::U32;

    fn read(&mut self, off: usize, buf: &mut [u8]) -> LfsResult<usize> {
        self.backend
            .read(off as u32, buf)
            .map_err(map_backend_to_lfs)?;
        Ok(buf.len())
    }

    fn write(&mut self, off: usize, data: &[u8]) -> LfsResult<usize> {
        self.backend
            .write(off as u32, data)
            .map_err(map_backend_to_lfs)?;
        Ok(data.len())
    }

    fn erase(&mut self, off: usize, len: usize) -> LfsResult<usize> {
        let mut remaining = len;
        let mut cursor = off as u32;
        let fill = [0xFFu8; BLOCK_SIZE];

        while remaining > 0 {
            let chunk = min(remaining, fill.len());
            self.backend
                .write(cursor, &fill[..chunk])
                .map_err(map_backend_to_lfs)?;
            cursor = cursor.saturating_add(chunk as u32);
            remaining -= chunk;
        }

        Ok(len)
    }
}

pub struct TinyMramFs {
    storage: LittleFsStorage,
}

impl TinyMramFs {
    pub fn mount(backend: Box<dyn ByteStorage>) -> Result<Self, FsError> {
        if backend.capacity() != rust_mram::CAPACITY_BYTES {
            return Err(FsError::CapacityMismatch {
                actual: backend.capacity(),
                expected: rust_mram::CAPACITY_BYTES,
            });
        }

        let mut this = Self {
            storage: LittleFsStorage { backend },
        };

        this.ensure_filesystem()?;
        Ok(this)
    }

    pub fn list_files(&mut self) -> Result<Vec<FileRecord>, FsError> {
        let mut files = self.with_fs(|fs| {
            let mut out = Vec::new();
            fs.read_dir_and_then(path!("/files"), |read_dir| {
                for entry in read_dir.skip(2) {
                    let entry = entry?;
                    if !entry.file_type().is_file() {
                        continue;
                    }

                    let name = entry.file_name().as_str().to_string();
                    let metadata = entry.metadata();
                    let meta = read_meta(fs, entry.path())?;

                    out.push(FileRecord {
                        name,
                        mime_type: meta.mime_type,
                        compressed: meta.compressed,
                        offset: 0,
                        size: metadata.len() as u32,
                        created_at: meta.created_at,
                        updated_at: meta.updated_at,
                    });
                }
                Ok(())
            })?;
            Ok(out)
        })?;

        files.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(files)
    }

    pub fn file(&mut self, name: &str) -> Result<Option<FileRecord>, FsError> {
        let path = file_path(name)?;
        self.with_fs(|fs| {
            if !fs.exists(&path) {
                return Ok(None);
            }

            let md = fs.metadata(&path)?;
            let meta = read_meta(fs, &path)?;

            Ok(Some(FileRecord {
                name: name.to_string(),
                mime_type: meta.mime_type,
                compressed: meta.compressed,
                offset: 0,
                size: md.len() as u32,
                created_at: meta.created_at,
                updated_at: meta.updated_at,
            }))
        })
    }

    pub fn write_file(
        &mut self,
        name: &str,
        mime_type: Option<&str>,
        compressed: bool,
        data: &[u8],
    ) -> Result<FileRecord, FsError> {
        if name.is_empty() || name.len() > 64 || name.contains('/') {
            return Err(FsError::InvalidName(name.len()));
        }

        let mime = mime_type.unwrap_or("application/octet-stream");
        if mime.len() > META_MAX_MIME_LEN {
            return Err(FsError::InvalidMime(mime.len()));
        }

        let path = file_path(name)?;
        let now = unix_now();

        self.with_fs(|fs| {
            let created_at = if fs.exists(&path) {
                read_meta(fs, &path)?.created_at
            } else {
                now
            };

            fs.write(&path, data)?;

            let meta = FileMeta {
                mime_type: mime.to_string(),
                compressed,
                created_at,
                updated_at: now,
            };

            fs.set_attribute(&path, ATTR_META_ID, &meta.encode())?;
            let md = fs.metadata(&path)?;

            Ok(FileRecord {
                name: name.to_string(),
                mime_type: meta.mime_type,
                compressed,
                offset: 0,
                size: md.len() as u32,
                created_at: meta.created_at,
                updated_at: meta.updated_at,
            })
        })
    }

    pub fn read_file(
        &mut self,
        name: &str,
        offset: u32,
        length: Option<u32>,
    ) -> Result<FilePayload, FsError> {
        let path = file_path(name)?;

        let result = self.with_fs(|fs| {
            if !fs.exists(&path) {
                return Err(LfsError::NO_SUCH_ENTRY);
            }

            let md = fs.metadata(&path)?;
            let total = md.len() as u32;
            if offset > total {
                return Err(LfsError::INVALID);
            }

            let mut content = vec![0u8; total as usize];
            fs.open_file_and_then(&path, |file| {
                let mut read = 0usize;
                while read < content.len() {
                    let n = file.read(&mut content[read..])?;
                    if n == 0 {
                        break;
                    }
                    read += n;
                }
                content.truncate(read);
                Ok(())
            })?;

            let available = total.saturating_sub(offset);
            let requested = length.unwrap_or(available);
            let read_len = min(requested, available) as usize;

            let start = offset as usize;
            let end = start + read_len;
            if end > content.len() {
                return Err(LfsError::INVALID);
            }

            let slice = content[start..end].to_vec();
            let meta = read_meta(fs, &path)?;

            Ok(FilePayload {
                record: FileRecord {
                    name: name.to_string(),
                    mime_type: meta.mime_type,
                    compressed: meta.compressed,
                    offset: 0,
                    size: total,
                    created_at: meta.created_at,
                    updated_at: meta.updated_at,
                },
                range_offset: offset,
                data: slice,
            })
        });

        match result {
            Err(FsError::LittleFs { err, .. }) if err == LfsError::NO_SUCH_ENTRY => {
                Err(FsError::FileNotFound(name.to_string()))
            }
            Err(FsError::LittleFs { err, .. }) if err == LfsError::INVALID => {
                Err(FsError::InvalidRange)
            }
            other => other,
        }
    }

    pub fn delete_file(&mut self, name: &str) -> Result<bool, FsError> {
        let path = file_path(name)?;
        self.with_fs(|fs| {
            if !fs.exists(&path) {
                return Ok(false);
            }
            fs.remove(&path)?;
            Ok(true)
        })
    }

    pub fn format(&mut self) -> Result<(), FsError> {
        Filesystem::<LittleFsStorage>::format(&mut self.storage)?;
        self.with_fs(|fs| {
            fs.create_dir_all(path!("/files"))?;
            Ok(())
        })
    }

    pub fn stats(&mut self) -> Result<StorageStats, FsError> {
        self.with_fs(|fs| {
            let capacity = fs.total_space() as u32;
            let free = fs.available_space()? as u32;

            let mut live_bytes = 0u32;
            let mut file_count = 0u32;
            fs.read_dir_and_then(path!("/files"), |read_dir| {
                for entry in read_dir.skip(2) {
                    let entry = entry?;
                    if entry.file_type().is_file() {
                        live_bytes = live_bytes.saturating_add(entry.metadata().len() as u32);
                        file_count = file_count.saturating_add(1);
                    }
                }
                Ok(())
            })?;

            let allocated = capacity.saturating_sub(free);

            Ok(StorageStats {
                capacity_bytes: capacity,
                allocated_bytes: allocated,
                live_bytes,
                reclaimable_bytes: 0,
                free_bytes: free,
                file_count,
            })
        })
    }

    fn ensure_filesystem(&mut self) -> Result<(), FsError> {
        // Try mounting first. If mount/setup fails (fresh chip, stale/corrupt fs),
        // format and retry once.
        let mount_result = self.with_fs(|fs| {
            fs.create_dir_all(path!("/files"))?;
            Ok(())
        });

        if mount_result.is_ok() {
            return Ok(());
        }

        let mount_err = mount_result
            .err()
            .map(|e| e.to_string())
            .unwrap_or_else(|| "unknown error".to_string());

        if let Err(err) = Filesystem::<LittleFsStorage>::format(&mut self.storage) {
            return Err(FsError::Initialization(format!(
                "initial mount failed: {mount_err}; format failed: {err:?}"
            )));
        }

        if let Err(err) = self.with_fs(|fs| {
            fs.create_dir_all(path!("/files"))?;
            Ok(())
        }) {
            return Err(FsError::Initialization(format!(
                "initial mount failed: {mount_err}; remount after format failed: {err}"
            )));
        }

        Ok(())
    }

    fn with_fs<R>(
        &mut self,
        f: impl FnOnce(&Filesystem<'_, LittleFsStorage>) -> LfsResult<R>,
    ) -> Result<R, FsError> {
        let mut alloc = Filesystem::<LittleFsStorage>::allocate();
        let fs = Filesystem::<LittleFsStorage>::mount(&mut alloc, &mut self.storage)?;
        f(&fs).map_err(FsError::from)
    }
}

fn file_path(name: &str) -> Result<littlefs2::path::PathBuf, FsError> {
    if name.is_empty() || name.len() > 64 || name.contains('/') {
        return Err(FsError::InvalidName(name.len()));
    }

    let full = format!("{}/{}", FILES_DIR, name);
    littlefs2::path::PathBuf::try_from(full.as_str()).map_err(|_| FsError::InvalidName(name.len()))
}

fn map_backend_to_lfs(err: BackendError) -> LfsError {
    eprintln!("mram-service backend I/O error: {err}");
    match err {
        BackendError::OutOfBounds { .. } => LfsError::NO_SPACE,
        BackendError::Io(_) | BackendError::InvalidConfig(_) => LfsError::IO,
        #[cfg(feature = "spidev")]
        BackendError::Driver(_) => LfsError::IO,
    }
}

fn read_meta(
    fs: &Filesystem<'_, LittleFsStorage>,
    path: &littlefs2::path::Path,
) -> LfsResult<FileMeta> {
    let mut buffer = [0u8; 64];
    match fs.attribute(path, ATTR_META_ID, &mut buffer)? {
        Some(attribute) => Ok(FileMeta::decode(attribute.data()).unwrap_or_default()),
        None => Ok(FileMeta::default()),
    }
}

fn unix_now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct InMemoryBackend {
        data: Vec<u8>,
    }

    impl InMemoryBackend {
        fn new(capacity: u32) -> Self {
            Self {
                data: vec![0xFFu8; capacity as usize],
            }
        }
    }

    impl ByteStorage for InMemoryBackend {
        fn capacity(&self) -> u32 {
            self.data.len() as u32
        }

        fn read(&mut self, offset: u32, out: &mut [u8]) -> Result<(), BackendError> {
            let start = offset as usize;
            let end = start + out.len();
            out.copy_from_slice(&self.data[start..end]);
            Ok(())
        }

        fn write(&mut self, offset: u32, data: &[u8]) -> Result<(), BackendError> {
            let start = offset as usize;
            let end = start + data.len();
            self.data[start..end].copy_from_slice(data);
            Ok(())
        }
    }

    #[test]
    fn write_read_list_delete_cycle() {
        let backend = Box::new(InMemoryBackend::new(rust_mram::CAPACITY_BYTES));
        let mut fs = TinyMramFs::mount(backend).unwrap();

        let rec = fs
            .write_file(
                "image.bin",
                Some("application/octet-stream"),
                true,
                &[1, 2, 3, 4],
            )
            .unwrap();
        assert_eq!(rec.size, 4);
        assert!(rec.compressed);

        let files = fs.list_files().unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].name, "image.bin");

        let payload = fs.read_file("image.bin", 1, Some(2)).unwrap();
        assert_eq!(payload.data, vec![2, 3]);

        assert!(fs.delete_file("image.bin").unwrap());
        assert!(fs.list_files().unwrap().is_empty());
    }

    #[test]
    fn format_clears_entries() {
        let backend = Box::new(InMemoryBackend::new(rust_mram::CAPACITY_BYTES));
        let mut fs = TinyMramFs::mount(backend).unwrap();

        fs.write_file("a", None, false, &[9, 9, 9]).unwrap();
        assert_eq!(fs.list_files().unwrap().len(), 1);

        fs.format().unwrap();
        assert!(fs.list_files().unwrap().is_empty());

        let stats = fs.stats().unwrap();
        assert_eq!(stats.file_count, 0);
    }
}
