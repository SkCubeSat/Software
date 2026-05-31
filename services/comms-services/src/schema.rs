use async_graphql::{Context, Object, Result};

use crate::model::{CommsHealth, RadioHealth, RadioRole, Subsystem, TelemetrySnapshot};

pub struct QueryRoot;
pub struct MutationRoot;

#[Object]
impl QueryRoot {
    async fn ping(&self) -> &str {
        "pong"
    }

    async fn telemetry(&self, ctx: &Context<'_>) -> Result<TelemetrySnapshot> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .telemetry()
            .map_err(async_graphql::Error::new)
    }

    async fn health(&self, ctx: &Context<'_>) -> Result<CommsHealth> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        Ok(context.subsystem().health())
    }

    async fn radio_health(&self, ctx: &Context<'_>, role: RadioRole) -> Result<RadioHealth> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        Ok(context.subsystem().radio_health(role))
    }

    async fn packets_up(&self, ctx: &Context<'_>) -> Result<i32> {
        Ok(self.telemetry(ctx).await?.packets_up)
    }

    async fn packets_down(&self, ctx: &Context<'_>) -> Result<i32> {
        Ok(self.telemetry(ctx).await?.packets_down)
    }

    async fn failed_packets_up(&self, ctx: &Context<'_>) -> Result<i32> {
        Ok(self.telemetry(ctx).await?.failed_packets_up)
    }

    async fn failed_packets_down(&self, ctx: &Context<'_>) -> Result<i32> {
        Ok(self.telemetry(ctx).await?.failed_packets_down)
    }

    async fn errors(&self, ctx: &Context<'_>) -> Result<Vec<String>> {
        Ok(self.telemetry(ctx).await?.errors)
    }
}

#[Object]
impl MutationRoot {
    async fn noop(&self) -> bool {
        true
    }
}
