use async_graphql::{Context, Object, Result, SimpleObject};

use crate::model::{MissionFlagKey, MissionState, ReconcileResponse};
use crate::subsystem::Subsystem;

pub struct QueryRoot;
pub struct MutationRoot;

#[derive(SimpleObject)]
pub struct HealthInfo {
    pub backend: String,
    pub capacity_bytes: i64,
    pub fram_reachable: bool,
    pub last_error: Option<String>,
}

#[derive(SimpleObject)]
pub struct MutationResponse {
    pub success: bool,
    pub errors: String,
}

#[derive(SimpleObject)]
pub struct StateMutationResponse {
    pub success: bool,
    pub errors: String,
    pub state: Option<MissionState>,
}

#[Object]
impl QueryRoot {
    async fn ping(&self) -> &str {
        "pong"
    }

    async fn health(&self, ctx: &Context<'_>) -> Result<HealthInfo> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        let health = context.subsystem().health();
        Ok(HealthInfo {
            backend: health.backend,
            capacity_bytes: health.capacity_bytes as i64,
            fram_reachable: health.fram_reachable,
            last_error: health.last_error,
        })
    }

    async fn mission_state(
        &self,
        ctx: &Context<'_>,
        reconcile: Option<bool>,
    ) -> Result<MissionState> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .mission_state(reconcile.unwrap_or(false))
            .map_err(async_graphql::Error::new)
    }
}

#[Object]
impl MutationRoot {
    async fn reconcile_mission_state(
        &self,
        ctx: &Context<'_>,
        dry_run: Option<bool>,
    ) -> Result<ReconcileResponse> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .reconcile(dry_run.unwrap_or(false))
            .map_err(async_graphql::Error::new)
    }

    async fn set_mission_flag(
        &self,
        ctx: &Context<'_>,
        key: MissionFlagKey,
        value: bool,
        mirror_to_env: Option<bool>,
    ) -> Result<StateMutationResponse> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        match context
            .subsystem()
            .set_flag(key, value, mirror_to_env.unwrap_or(true))
        {
            Ok(state) => Ok(StateMutationResponse {
                success: true,
                errors: String::new(),
                state: Some(state),
            }),
            Err(err) => Ok(StateMutationResponse {
                success: false,
                errors: err,
                state: None,
            }),
        }
    }

    async fn set_deploy_start(
        &self,
        ctx: &Context<'_>,
        timestamp: Option<i64>,
        mirror_to_env: Option<bool>,
    ) -> Result<StateMutationResponse> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        let timestamp = match timestamp {
            Some(value) if value < 0 => {
                return Ok(StateMutationResponse {
                    success: false,
                    errors: "timestamp must be >= 0".to_string(),
                    state: None,
                });
            }
            Some(value) => Some(value as u64),
            None => None,
        };

        match context
            .subsystem()
            .set_deploy_start(timestamp, mirror_to_env.unwrap_or(true))
        {
            Ok(state) => Ok(StateMutationResponse {
                success: true,
                errors: String::new(),
                state: Some(state),
            }),
            Err(err) => Ok(StateMutationResponse {
                success: false,
                errors: err,
                state: None,
            }),
        }
    }

    async fn initialize_flight_state(
        &self,
        ctx: &Context<'_>,
        confirm: bool,
    ) -> Result<MutationResponse> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        match context.subsystem().initialize_flight_state(confirm) {
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
