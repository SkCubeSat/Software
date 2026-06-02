use async_graphql::{Context, Object, Result};

use crate::model::{CommsHealth, RadioHealth, RadioRole, Subsystem, TelemetrySnapshot};
use crate::radio_control::{
    RadioIdent, RadioInterface, RadioInterfaceStats, RadioMutationResponse, RadioPayloadFormat,
    RadioPing, RadioStatus, RadioSystemStats, RadioUptime,
};

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

    async fn radio_ping(
        &self,
        ctx: &Context<'_>,
        role: RadioRole,
        payload_size: Option<i32>,
    ) -> Result<RadioPing> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .radio_ping(role, payload_size)
            .map_err(async_graphql::Error::new)
    }

    async fn radio_uptime(&self, ctx: &Context<'_>, role: RadioRole) -> Result<RadioUptime> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .radio_uptime(role)
            .map_err(async_graphql::Error::new)
    }

    async fn radio_status(&self, ctx: &Context<'_>, role: RadioRole) -> Result<RadioStatus> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .radio_status(role)
            .map_err(async_graphql::Error::new)
    }

    async fn radio_ident(&self, ctx: &Context<'_>, role: RadioRole) -> Result<RadioIdent> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .radio_ident(role)
            .map_err(async_graphql::Error::new)
    }

    async fn radio_interface_stats(
        &self,
        ctx: &Context<'_>,
        role: RadioRole,
        interface: RadioInterface,
    ) -> Result<RadioInterfaceStats> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .radio_interface_stats(role, interface)
            .map_err(async_graphql::Error::new)
    }

    async fn radio_system_stats(
        &self,
        ctx: &Context<'_>,
        role: RadioRole,
    ) -> Result<RadioSystemStats> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .radio_system_stats(role)
            .map_err(async_graphql::Error::new)
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

    async fn radio_reboot(
        &self,
        ctx: &Context<'_>,
        role: RadioRole,
    ) -> Result<RadioMutationResponse> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        Ok(context.subsystem().radio_reboot(role))
    }

    async fn radio_send_text_in_morse(
        &self,
        ctx: &Context<'_>,
        role: RadioRole,
        source_identification: String,
        text: String,
    ) -> Result<RadioMutationResponse> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        Ok(context
            .subsystem()
            .radio_send_text_in_morse(role, source_identification, text))
    }

    async fn radio_send_compressed_morse(
        &self,
        ctx: &Context<'_>,
        role: RadioRole,
        source_identification: String,
        num1: i32,
        num2: i32,
        num3: i32,
        num4: i32,
        num5: i32,
        num6: i32,
    ) -> Result<RadioMutationResponse> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        Ok(context.subsystem().radio_send_compressed_morse(
            role,
            source_identification,
            [num1, num2, num3, num4, num5, num6],
        ))
    }

    async fn radio_send_ax25_message(
        &self,
        ctx: &Context<'_>,
        role: RadioRole,
        data: String,
        format: Option<RadioPayloadFormat>,
    ) -> Result<RadioMutationResponse> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        Ok(context
            .subsystem()
            .radio_send_ax25_message(role, data, format))
    }
}
