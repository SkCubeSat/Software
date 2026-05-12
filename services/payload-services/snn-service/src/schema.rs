use async_graphql::{Context, Enum, Object, Result, SimpleObject};
use base64::Engine;

use crate::driver::{DriverPhase, JobPhase};
use crate::subsystem::Subsystem;

pub struct QueryRoot;
pub struct MutationRoot;

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug)]
pub enum DriverPhaseGql {
    Initializing,
    Idle,
    Busy,
    ShuttingDown,
    Faulted,
}

impl From<DriverPhase> for DriverPhaseGql {
    fn from(value: DriverPhase) -> Self {
        match value {
            DriverPhase::Initializing => DriverPhaseGql::Initializing,
            DriverPhase::Idle => DriverPhaseGql::Idle,
            DriverPhase::Busy => DriverPhaseGql::Busy,
            DriverPhase::ShuttingDown => DriverPhaseGql::ShuttingDown,
            DriverPhase::Faulted => DriverPhaseGql::Faulted,
        }
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug)]
pub enum JobPhaseGql {
    Queued,
    SendingImage,
    Processing,
    ResultReady,
    Delivered,
    Failed,
    Cancelled,
}

impl From<JobPhase> for JobPhaseGql {
    fn from(value: JobPhase) -> Self {
        match value {
            JobPhase::Queued => JobPhaseGql::Queued,
            JobPhase::SendingImage => JobPhaseGql::SendingImage,
            JobPhase::Processing => JobPhaseGql::Processing,
            JobPhase::ResultReady => JobPhaseGql::ResultReady,
            JobPhase::Delivered => JobPhaseGql::Delivered,
            JobPhase::Failed => JobPhaseGql::Failed,
            JobPhase::Cancelled => JobPhaseGql::Cancelled,
        }
    }
}

#[derive(SimpleObject)]
pub struct SnnState {
    pub phase: DriverPhaseGql,
    pub current_image_id: Option<i64>,
    pub queue_depth: i64,
    pub queue_capacity: i64,
    pub queued_image_ids: Vec<i64>,
    pub last_error: Option<String>,
}

#[derive(SimpleObject)]
pub struct JobStatus {
    pub image_id: i64,
    pub phase: JobPhaseGql,
    pub queue_position: Option<i64>,
    pub error: Option<String>,
}

#[derive(SimpleObject)]
pub struct SubmitResponse {
    pub success: bool,
    pub accepted: bool,
    pub image_id: Option<i64>,
    pub queue_position: Option<i64>,
    pub queue_depth: Option<i64>,
    pub error: Option<String>,
}

#[derive(SimpleObject)]
pub struct ResultPayload {
    pub image_id: i64,
    pub size_bytes: i64,
    pub crc32: String,
    pub bitmap_base64: String,
    pub phase: JobPhaseGql,
}

#[derive(SimpleObject)]
pub struct InferenceResult {
    pub success: bool,
    pub image_id: Option<i64>,
    pub size_bytes: Option<i64>,
    pub crc32: Option<String>,
    pub bitmap_base64: Option<String>,
    pub error: Option<String>,
}

#[derive(SimpleObject)]
pub struct CancelResponse {
    pub success: bool,
    pub cancelled: bool,
    pub error: Option<String>,
}

#[derive(SimpleObject)]
pub struct HealthInfo {
    pub uart_bus: String,
    pub uart_baud: i64,
    pub phase: DriverPhaseGql,
    pub queue_depth: i64,
    pub queue_capacity: i64,
    pub jobs_completed: i64,
    pub jobs_failed: i64,
    pub last_error: Option<String>,
}

#[Object]
impl QueryRoot {
    async fn ping(&self) -> &str {
        "pong"
    }

    async fn health(&self, ctx: &Context<'_>) -> Result<HealthInfo> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        let h = context.subsystem().health();
        Ok(HealthInfo {
            uart_bus: h.uart_bus,
            uart_baud: h.uart_baud as i64,
            phase: h.phase.into(),
            queue_depth: h.queue_depth as i64,
            queue_capacity: h.queue_capacity as i64,
            jobs_completed: h.jobs_completed as i64,
            jobs_failed: h.jobs_failed as i64,
            last_error: h.last_error,
        })
    }

    async fn state(&self, ctx: &Context<'_>) -> Result<SnnState> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        let s = context.subsystem().state();
        Ok(SnnState {
            phase: s.phase.into(),
            current_image_id: s.current_image_id.map(|v| v as i64),
            queue_depth: s.queue_depth as i64,
            queue_capacity: s.queue_capacity as i64,
            queued_image_ids: s.queued_image_ids.into_iter().map(|v| v as i64).collect(),
            last_error: s.last_error,
        })
    }

    async fn inference_status(
        &self,
        ctx: &Context<'_>,
        image_id: i64,
    ) -> Result<Option<JobStatus>> {
        if image_id < 0 {
            return Err(async_graphql::Error::new("imageId must be >= 0"));
        }
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        Ok(context
            .subsystem()
            .inference_status(image_id as u32)
            .map(|s| JobStatus {
                image_id: s.image_id as i64,
                phase: s.phase.into(),
                queue_position: s.queue_position.map(|p| p as i64),
                error: s.error,
            }))
    }

    async fn get_result(&self, ctx: &Context<'_>, image_id: i64) -> Result<ResultPayload> {
        if image_id < 0 {
            return Err(async_graphql::Error::new("imageId must be >= 0"));
        }
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        let entry = context
            .subsystem()
            .get_result(image_id as u32)
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        let bitmap_base64 = base64::engine::general_purpose::STANDARD.encode(&entry.bitmap);
        Ok(ResultPayload {
            image_id: entry.image_id as i64,
            size_bytes: entry.size as i64,
            crc32: format!("{:08X}", entry.crc32),
            bitmap_base64,
            phase: entry.phase.into(),
        })
    }
}

#[Object]
impl MutationRoot {
    async fn submit_image(
        &self,
        ctx: &Context<'_>,
        image_base64: String,
    ) -> Result<SubmitResponse> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        let bytes = match base64::engine::general_purpose::STANDARD.decode(image_base64.as_bytes())
        {
            Ok(b) => b,
            Err(err) => {
                return Ok(SubmitResponse {
                    success: false,
                    accepted: false,
                    image_id: None,
                    queue_position: None,
                    queue_depth: None,
                    error: Some(format!("invalid base64: {err}")),
                });
            }
        };
        match context.subsystem().submit(bytes) {
            Ok(outcome) => Ok(SubmitResponse {
                success: true,
                accepted: true,
                image_id: Some(outcome.image_id as i64),
                queue_position: Some(outcome.queue_position as i64),
                queue_depth: Some(outcome.queue_depth as i64),
                error: None,
            }),
            Err(err) => Ok(SubmitResponse {
                success: false,
                accepted: false,
                image_id: None,
                queue_position: None,
                queue_depth: None,
                error: Some(err.to_string()),
            }),
        }
    }

    /// Submit + poll-to-completion convenience wrapper. Goes through the same queue and
    /// driver as `submitImage`; the only difference is who is holding the HTTP request open.
    async fn infer(&self, ctx: &Context<'_>, image_base64: String) -> Result<InferenceResult> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        let bytes = match base64::engine::general_purpose::STANDARD.decode(image_base64.as_bytes())
        {
            Ok(b) => b,
            Err(err) => {
                return Ok(InferenceResult {
                    success: false,
                    image_id: None,
                    size_bytes: None,
                    crc32: None,
                    bitmap_base64: None,
                    error: Some(format!("invalid base64: {err}")),
                });
            }
        };
        match context.subsystem().infer(bytes).await {
            Ok(entry) => {
                let bitmap_base64 =
                    base64::engine::general_purpose::STANDARD.encode(&entry.bitmap);
                Ok(InferenceResult {
                    success: true,
                    image_id: Some(entry.image_id as i64),
                    size_bytes: Some(entry.size as i64),
                    crc32: Some(format!("{:08X}", entry.crc32)),
                    bitmap_base64: Some(bitmap_base64),
                    error: None,
                })
            }
            Err(err) => Ok(InferenceResult {
                success: false,
                image_id: None,
                size_bytes: None,
                crc32: None,
                bitmap_base64: None,
                error: Some(err.to_string()),
            }),
        }
    }

    async fn cancel(&self, ctx: &Context<'_>, image_id: i64) -> Result<CancelResponse> {
        if image_id < 0 {
            return Ok(CancelResponse {
                success: false,
                cancelled: false,
                error: Some("imageId must be >= 0".to_string()),
            });
        }
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        match context.subsystem().cancel(image_id as u32) {
            Ok(cancelled) => Ok(CancelResponse {
                success: true,
                cancelled,
                error: None,
            }),
            Err(err) => Ok(CancelResponse {
                success: false,
                cancelled: false,
                error: Some(err.to_string()),
            }),
        }
    }
}
