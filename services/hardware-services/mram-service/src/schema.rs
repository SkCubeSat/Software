use async_graphql::{Context, InputObject, Object, Result, SimpleObject};
use base64::Engine;

use crate::subsystem::Subsystem;

pub struct QueryRoot;
pub struct MutationRoot;

#[derive(SimpleObject, Clone)]
pub struct FileInfo {
    pub name: String,
    pub mime_type: String,
    pub compressed: bool,
    pub offset: i64,
    pub size: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(SimpleObject)]
pub struct StorageInfo {
    pub backend: String,
    pub capacity_bytes: i64,
    pub allocated_bytes: i64,
    pub live_bytes: i64,
    pub reclaimable_bytes: i64,
    pub free_bytes: i64,
    pub file_count: i64,
}

#[derive(SimpleObject)]
pub struct FileData {
    pub name: String,
    pub mime_type: String,
    pub compressed: bool,
    pub total_size: i64,
    pub offset: i64,
    pub length: i64,
    pub data_base64: String,
}

#[derive(SimpleObject)]
pub struct MutationResponse {
    pub success: bool,
    pub errors: String,
}

#[derive(SimpleObject)]
pub struct WriteFileResponse {
    pub success: bool,
    pub errors: String,
    pub file: Option<FileInfo>,
}

#[derive(SimpleObject)]
pub struct DeleteFileResponse {
    pub success: bool,
    pub errors: String,
    pub deleted: bool,
}

#[derive(InputObject)]
pub struct WriteFileInput {
    pub name: String,
    pub data_base64: String,
    pub mime_type: Option<String>,
    pub compressed: Option<bool>,
}

#[Object]
impl QueryRoot {
    async fn ping(&self) -> &str {
        "pong"
    }

    async fn storage(&self, ctx: &Context<'_>) -> Result<StorageInfo> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        let stats = context
            .subsystem()
            .stats()
            .map_err(async_graphql::Error::new)?;

        Ok(StorageInfo {
            backend: context.subsystem().backend_name().to_string(),
            capacity_bytes: stats.capacity_bytes as i64,
            allocated_bytes: stats.allocated_bytes as i64,
            live_bytes: stats.live_bytes as i64,
            reclaimable_bytes: stats.reclaimable_bytes as i64,
            free_bytes: stats.free_bytes as i64,
            file_count: stats.file_count as i64,
        })
    }

    async fn files(&self, ctx: &Context<'_>) -> Result<Vec<FileInfo>> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        let files = context
            .subsystem()
            .list_files()
            .map_err(async_graphql::Error::new)?;

        Ok(files.into_iter().map(map_record).collect())
    }

    async fn file(&self, ctx: &Context<'_>, name: String) -> Result<Option<FileInfo>> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        let file = context
            .subsystem()
            .file(&name)
            .map_err(async_graphql::Error::new)?;

        Ok(file.map(map_record))
    }

    async fn read_file(
        &self,
        ctx: &Context<'_>,
        name: String,
        offset: Option<i64>,
        length: Option<i64>,
    ) -> Result<FileData> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        let offset = offset.unwrap_or(0);
        if offset < 0 {
            return Err(async_graphql::Error::new("offset must be >= 0"));
        }

        let length_u32 = match length {
            Some(val) if val < 0 => {
                return Err(async_graphql::Error::new("length must be >= 0"));
            }
            Some(val) => Some(val as u32),
            None => None,
        };

        let payload = context
            .subsystem()
            .read_file(&name, offset as u32, length_u32)
            .map_err(async_graphql::Error::new)?;

        let byte_len = payload.data.len();
        let encoded = base64::engine::general_purpose::STANDARD.encode(payload.data);

        Ok(FileData {
            name: payload.record.name,
            mime_type: payload.record.mime_type,
            compressed: payload.record.compressed,
            total_size: payload.record.size as i64,
            offset: payload.range_offset as i64,
            length: byte_len as i64,
            data_base64: encoded,
        })
    }
}

#[Object]
impl MutationRoot {
    async fn write_file(
        &self,
        ctx: &Context<'_>,
        input: WriteFileInput,
    ) -> Result<WriteFileResponse> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;

        let data = base64::engine::general_purpose::STANDARD
            .decode(input.data_base64.as_bytes())
            .map_err(|err| async_graphql::Error::new(err.to_string()))?;

        match context.subsystem().write_file(
            &input.name,
            input.mime_type.as_deref(),
            input.compressed.unwrap_or(false),
            &data,
        ) {
            Ok(file) => Ok(WriteFileResponse {
                success: true,
                errors: String::new(),
                file: Some(map_record(file)),
            }),
            Err(err) => Ok(WriteFileResponse {
                success: false,
                errors: err,
                file: None,
            }),
        }
    }

    async fn delete_file(&self, ctx: &Context<'_>, name: String) -> Result<DeleteFileResponse> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;

        match context.subsystem().delete_file(&name) {
            Ok(deleted) => Ok(DeleteFileResponse {
                success: true,
                errors: String::new(),
                deleted,
            }),
            Err(err) => Ok(DeleteFileResponse {
                success: false,
                errors: err,
                deleted: false,
            }),
        }
    }

    async fn format(&self, ctx: &Context<'_>, confirm: bool) -> Result<MutationResponse> {
        if !confirm {
            return Ok(MutationResponse {
                success: false,
                errors: "format is destructive: pass confirm=true".to_string(),
            });
        }

        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        match context.subsystem().format() {
            Ok(()) => Ok(MutationResponse {
                success: true,
                errors: String::new(),
            }),
            Err(err) => Ok(MutationResponse {
                success: false,
                errors: err,
            }),
        }
    }
}

fn map_record(record: crate::fs::FileRecord) -> FileInfo {
    FileInfo {
        name: record.name,
        mime_type: record.mime_type,
        compressed: record.compressed,
        offset: record.offset as i64,
        size: record.size as i64,
        created_at: record.created_at,
        updated_at: record.updated_at,
    }
}
