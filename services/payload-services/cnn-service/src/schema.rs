use async_graphql::{Context, Object, Result, SimpleObject};
use base64::Engine;

use crate::subsystem::Subsystem;

pub struct QueryRoot;
pub struct MutationRoot;

// ---------------------------------------------------------------------------
// Response types
// ---------------------------------------------------------------------------

#[derive(SimpleObject)]
pub struct CommandResult {
    pub success: bool,
    pub error: Option<String>,
}

#[derive(SimpleObject)]
pub struct PipelineResponse {
    pub success: bool,
    pub coverage: Option<f64>,
    pub image_base64: Option<String>,
    pub image_size: Option<i64>,
    pub error: Option<String>,
}

#[derive(SimpleObject)]
pub struct SendImageResponse {
    pub success: bool,
    pub image_base64: Option<String>,
    pub image_size: Option<i64>,
    pub error: Option<String>,
}

#[derive(SimpleObject)]
pub struct CoverageResponse {
    pub success: bool,
    pub coverage: Option<f64>,
    pub error: Option<String>,
}

#[derive(SimpleObject)]
pub struct ListFilesResponse {
    pub success: bool,
    pub listing: Option<String>,
    pub error: Option<String>,
}

#[derive(SimpleObject)]
pub struct HandshakeResponse {
    pub success: bool,
    pub error: Option<String>,
}

#[derive(SimpleObject)]
pub struct RecvFileResponse {
    pub success: bool,
    pub filename: Option<String>,
    pub size: Option<i64>,
    pub error: Option<String>,
}

// ---------------------------------------------------------------------------
// Queries: send, sum, ls, wait
// ---------------------------------------------------------------------------

#[Object]
impl QueryRoot {
    async fn ping(&self) -> &str {
        "pong"
    }

    /// Retrieve the image currently in ESP32 PSRAM (`<<send>>`).
    /// Returns the image as base64-encoded data along with its size in bytes.
    async fn send_image(&self, ctx: &Context<'_>) -> Result<SendImageResponse> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        match context.subsystem().send_image() {
            Ok(data) => {
                let b64 = base64::engine::general_purpose::STANDARD.encode(&data.image);
                Ok(SendImageResponse {
                    success: true,
                    image_base64: Some(b64),
                    image_size: Some(data.size as i64),
                    error: None,
                })
            }
            Err(err) => Ok(SendImageResponse {
                success: false,
                image_base64: None,
                image_size: None,
                error: Some(err.to_string()),
            }),
        }
    }

    /// Calculate cloud coverage from the mask in ESP32 PSRAM (`<<sum>>`).
    /// Returns a float between 0.0 and 1.0.
    async fn get_coverage(&self, ctx: &Context<'_>) -> Result<CoverageResponse> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        match context.subsystem().get_coverage() {
            Ok(coverage) => Ok(CoverageResponse {
                success: true,
                coverage: Some(coverage),
                error: None,
            }),
            Err(err) => Ok(CoverageResponse {
                success: false,
                coverage: None,
                error: Some(err.to_string()),
            }),
        }
    }

    /// List files stored on ESP32 SPIFFS (`<<ls>>`).
    async fn list_files(&self, ctx: &Context<'_>) -> Result<ListFilesResponse> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        match context.subsystem().list_files() {
            Ok(listing) => Ok(ListFilesResponse {
                success: true,
                listing: Some(listing),
                error: None,
            }),
            Err(err) => Ok(ListFilesResponse {
                success: false,
                listing: None,
                error: Some(err.to_string()),
            }),
        }
    }

    /// Test the OBC↔ESP32 handshake mechanism (`<<wait>>`).
    async fn test_handshake(&self, ctx: &Context<'_>) -> Result<HandshakeResponse> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        match context.subsystem().test_handshake() {
            Ok(()) => Ok(HandshakeResponse {
                success: true,
                error: None,
            }),
            Err(err) => Ok(HandshakeResponse {
                success: false,
                error: Some(err.to_string()),
            }),
        }
    }
}

// ---------------------------------------------------------------------------
// Mutations: run, cap, cnn, recv
// ---------------------------------------------------------------------------

#[Object]
impl MutationRoot {
    /// Execute the full CNN pipeline (`<<run>>`): capture → CNN inference →
    /// cloud coverage → image transfer.
    async fn run_pipeline(&self, ctx: &Context<'_>) -> Result<PipelineResponse> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        match context.subsystem().run_pipeline() {
            Ok(result) => {
                let b64 = base64::engine::general_purpose::STANDARD.encode(&result.image);
                Ok(PipelineResponse {
                    success: true,
                    coverage: Some(result.coverage),
                    image_base64: Some(b64),
                    image_size: Some(result.image_size as i64),
                    error: None,
                })
            }
            Err(err) => Ok(PipelineResponse {
                success: false,
                coverage: None,
                image_base64: None,
                image_size: None,
                error: Some(err.to_string()),
            }),
        }
    }

    /// Capture a photo to ESP32 PSRAM (`<<cap>>`).
    async fn capture(&self, ctx: &Context<'_>) -> Result<CommandResult> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        match context.subsystem().capture() {
            Ok(()) => Ok(CommandResult {
                success: true,
                error: None,
            }),
            Err(err) => Ok(CommandResult {
                success: false,
                error: Some(err.to_string()),
            }),
        }
    }

    /// Run CNN inference on the image in ESP32 PSRAM (`<<cnn>>`).
    /// Prerequisite: `capture` must have been called first.
    async fn run_cnn(&self, ctx: &Context<'_>) -> Result<CommandResult> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        match context.subsystem().run_cnn() {
            Ok(()) => Ok(CommandResult {
                success: true,
                error: None,
            }),
            Err(err) => Ok(CommandResult {
                success: false,
                error: Some(err.to_string()),
            }),
        }
    }

    /// Send a file from the OBC filesystem to ESP32 SPIFFS (`<<recv>>`).
    /// The `file_path` is the absolute path on the OBC to the file to transfer.
    async fn recv_file(
        &self,
        ctx: &Context<'_>,
        file_path: String,
    ) -> Result<RecvFileResponse> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        match context.subsystem().recv_file(&file_path) {
            Ok(result) => Ok(RecvFileResponse {
                success: true,
                filename: Some(result.filename),
                size: Some(result.size as i64),
                error: None,
            }),
            Err(err) => Ok(RecvFileResponse {
                success: false,
                filename: None,
                size: None,
                error: Some(err.to_string()),
            }),
        }
    }
}
