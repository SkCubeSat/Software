use async_graphql::{Context, Object, Result};

use crate::model::{CommsHealth, RadioHealth, RadioRole, Subsystem, TelemetrySnapshot};
use crate::nmp_control::{
    NmpByteValue, NmpConfig1, NmpConfig2, NmpDataFormat, NmpFirmwareCrc, NmpFsooStatus,
    NmpRadioLinkType, NmpResetStatusBytes, NmpRouteEntry, NmpRouteInput, NmpRssiStatus,
    NmpTelemetryPeriods,
};
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

    async fn radio_nmp_get_csp_address(
        &self,
        ctx: &Context<'_>,
        role: RadioRole,
        key: u32,
    ) -> Result<u8> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .radio_nmp_get_csp_address(role, key)
            .map_err(async_graphql::Error::new)
    }

    async fn radio_nmp_get_route_table(
        &self,
        ctx: &Context<'_>,
        role: RadioRole,
        key: u32,
    ) -> Result<Vec<NmpRouteEntry>> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .radio_nmp_get_route_table(role, key)
            .map_err(async_graphql::Error::new)
    }

    async fn radio_nmp_get_routing_from_rs485(
        &self,
        ctx: &Context<'_>,
        role: RadioRole,
        key: u32,
    ) -> Result<bool> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .radio_nmp_get_routing_from_rs485(role, key)
            .map_err(async_graphql::Error::new)
    }

    async fn radio_nmp_get_frequency(
        &self,
        ctx: &Context<'_>,
        role: RadioRole,
        key: u32,
    ) -> Result<u32> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .radio_nmp_get_frequency(role, key)
            .map_err(async_graphql::Error::new)
    }

    async fn radio_nmp_get_link_type(
        &self,
        ctx: &Context<'_>,
        role: RadioRole,
        key: u32,
    ) -> Result<NmpRadioLinkType> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .radio_nmp_get_link_type(role, key)
            .map_err(async_graphql::Error::new)
    }

    async fn radio_nmp_get_tx_enable(
        &self,
        ctx: &Context<'_>,
        role: RadioRole,
        key: u32,
    ) -> Result<bool> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .radio_nmp_get_tx_enable(role, key)
            .map_err(async_graphql::Error::new)
    }

    async fn radio_nmp_get_preamble_size(
        &self,
        ctx: &Context<'_>,
        role: RadioRole,
        key: u32,
    ) -> Result<u16> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .radio_nmp_get_preamble_size(role, key)
            .map_err(async_graphql::Error::new)
    }

    async fn radio_nmp_get_normal_power(
        &self,
        ctx: &Context<'_>,
        role: RadioRole,
        key: u32,
    ) -> Result<bool> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .radio_nmp_get_normal_power(role, key)
            .map_err(async_graphql::Error::new)
    }

    async fn radio_nmp_get_gs_rx_tx_delay(
        &self,
        ctx: &Context<'_>,
        role: RadioRole,
        key: u32,
    ) -> Result<u16> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .radio_nmp_get_gs_rx_tx_delay(role, key)
            .map_err(async_graphql::Error::new)
    }

    async fn radio_nmp_get_digipeater_enable(
        &self,
        ctx: &Context<'_>,
        role: RadioRole,
        key: u32,
    ) -> Result<bool> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .radio_nmp_get_digipeater_enable(role, key)
            .map_err(async_graphql::Error::new)
    }

    async fn radio_nmp_get_callsign(
        &self,
        ctx: &Context<'_>,
        role: RadioRole,
        key: u32,
    ) -> Result<NmpByteValue> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .radio_nmp_get_callsign(role, key)
            .map_err(async_graphql::Error::new)
    }

    async fn radio_nmp_get_morse_custom_message(
        &self,
        ctx: &Context<'_>,
        role: RadioRole,
        key: u32,
    ) -> Result<NmpByteValue> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .radio_nmp_get_morse_custom_message(role, key)
            .map_err(async_graphql::Error::new)
    }

    async fn radio_nmp_get_morse_custom_ident(
        &self,
        ctx: &Context<'_>,
        role: RadioRole,
        key: u32,
    ) -> Result<NmpByteValue> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .radio_nmp_get_morse_custom_ident(role, key)
            .map_err(async_graphql::Error::new)
    }

    async fn radio_nmp_get_fsoo(
        &self,
        ctx: &Context<'_>,
        role: RadioRole,
        key: u32,
    ) -> Result<NmpFsooStatus> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .radio_nmp_get_fsoo(role, key)
            .map_err(async_graphql::Error::new)
    }

    async fn radio_nmp_get_hk_rssi(
        &self,
        ctx: &Context<'_>,
        role: RadioRole,
        key: u32,
    ) -> Result<NmpRssiStatus> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .radio_nmp_get_hk_rssi(role, key)
            .map_err(async_graphql::Error::new)
    }

    async fn radio_nmp_get_rssi_contribution_ratio(
        &self,
        ctx: &Context<'_>,
        role: RadioRole,
        key: u32,
    ) -> Result<u8> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .radio_nmp_get_rssi_contribution_ratio(role, key)
            .map_err(async_graphql::Error::new)
    }

    async fn radio_nmp_get_config1(
        &self,
        ctx: &Context<'_>,
        role: RadioRole,
        key: u32,
    ) -> Result<NmpConfig1> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .radio_nmp_get_config1(role, key)
            .map_err(async_graphql::Error::new)
    }

    async fn radio_nmp_get_config2(
        &self,
        ctx: &Context<'_>,
        role: RadioRole,
        key: u32,
    ) -> Result<NmpConfig2> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .radio_nmp_get_config2(role, key)
            .map_err(async_graphql::Error::new)
    }

    async fn radio_nmp_get_check_comm_reset_period(
        &self,
        ctx: &Context<'_>,
        role: RadioRole,
        key: u32,
    ) -> Result<u16> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .radio_nmp_get_check_comm_reset_period(role, key)
            .map_err(async_graphql::Error::new)
    }

    async fn radio_nmp_get_system_status_and_morse_period(
        &self,
        ctx: &Context<'_>,
        role: RadioRole,
        key: u32,
    ) -> Result<NmpTelemetryPeriods> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .radio_nmp_get_system_status_and_morse_period(role, key)
            .map_err(async_graphql::Error::new)
    }

    async fn radio_nmp_get_fw_crc(
        &self,
        ctx: &Context<'_>,
        role: RadioRole,
        key: u32,
    ) -> Result<NmpFirmwareCrc> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .radio_nmp_get_fw_crc(role, key)
            .map_err(async_graphql::Error::new)
    }

    async fn radio_nmp_get_hostname(
        &self,
        ctx: &Context<'_>,
        role: RadioRole,
        key: u32,
    ) -> Result<NmpByteValue> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .radio_nmp_get_hostname(role, key)
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

    async fn radio_nmp_unlock(&self, ctx: &Context<'_>, role: RadioRole, key: u32) -> Result<bool> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .radio_nmp_unlock(role, key)
            .map_err(async_graphql::Error::new)
    }

    async fn radio_nmp_set_csp_address(
        &self,
        ctx: &Context<'_>,
        role: RadioRole,
        key: u32,
        csp_address: u8,
    ) -> Result<u8> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .radio_nmp_set_csp_address(role, key, csp_address)
            .map_err(async_graphql::Error::new)
    }

    async fn radio_nmp_clear_route_table(
        &self,
        ctx: &Context<'_>,
        role: RadioRole,
        key: u32,
    ) -> Result<Vec<NmpRouteEntry>> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .radio_nmp_clear_route_table(role, key)
            .map_err(async_graphql::Error::new)
    }

    async fn radio_nmp_set_routes(
        &self,
        ctx: &Context<'_>,
        role: RadioRole,
        key: u32,
        routes: Vec<NmpRouteInput>,
    ) -> Result<Vec<NmpRouteEntry>> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .radio_nmp_set_routes(role, key, routes)
            .map_err(async_graphql::Error::new)
    }

    async fn radio_nmp_set_routing_from_rs485(
        &self,
        ctx: &Context<'_>,
        role: RadioRole,
        key: u32,
        enabled: bool,
    ) -> Result<bool> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .radio_nmp_set_routing_from_rs485(role, key, enabled)
            .map_err(async_graphql::Error::new)
    }

    async fn radio_nmp_set_frequency(
        &self,
        ctx: &Context<'_>,
        role: RadioRole,
        key: u32,
        frequency_hz: u32,
    ) -> Result<u32> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .radio_nmp_set_frequency(role, key, frequency_hz)
            .map_err(async_graphql::Error::new)
    }

    async fn radio_nmp_set_link_type(
        &self,
        ctx: &Context<'_>,
        role: RadioRole,
        key: u32,
        link_type: NmpRadioLinkType,
    ) -> Result<NmpRadioLinkType> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .radio_nmp_set_link_type(role, key, link_type)
            .map_err(async_graphql::Error::new)
    }

    async fn radio_nmp_set_tx_enable(
        &self,
        ctx: &Context<'_>,
        role: RadioRole,
        key: u32,
        enabled: bool,
    ) -> Result<bool> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .radio_nmp_set_tx_enable(role, key, enabled)
            .map_err(async_graphql::Error::new)
    }

    async fn radio_nmp_set_preamble_size(
        &self,
        ctx: &Context<'_>,
        role: RadioRole,
        key: u32,
        size: u16,
    ) -> Result<u16> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .radio_nmp_set_preamble_size(role, key, size)
            .map_err(async_graphql::Error::new)
    }

    async fn radio_nmp_set_normal_power(
        &self,
        ctx: &Context<'_>,
        role: RadioRole,
        key: u32,
        normal_power: bool,
    ) -> Result<bool> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .radio_nmp_set_normal_power(role, key, normal_power)
            .map_err(async_graphql::Error::new)
    }

    async fn radio_nmp_set_gs_rx_tx_delay(
        &self,
        ctx: &Context<'_>,
        role: RadioRole,
        key: u32,
        delay_ms: u16,
    ) -> Result<u16> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .radio_nmp_set_gs_rx_tx_delay(role, key, delay_ms)
            .map_err(async_graphql::Error::new)
    }

    async fn radio_nmp_set_digipeater_enable(
        &self,
        ctx: &Context<'_>,
        role: RadioRole,
        key: u32,
        enabled: bool,
    ) -> Result<bool> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .radio_nmp_set_digipeater_enable(role, key, enabled)
            .map_err(async_graphql::Error::new)
    }

    async fn radio_nmp_set_callsign(
        &self,
        ctx: &Context<'_>,
        role: RadioRole,
        key: u32,
        callsign: String,
        format: Option<NmpDataFormat>,
    ) -> Result<NmpByteValue> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .radio_nmp_set_callsign(role, key, callsign, format)
            .map_err(async_graphql::Error::new)
    }

    async fn radio_nmp_set_morse_custom_message(
        &self,
        ctx: &Context<'_>,
        role: RadioRole,
        key: u32,
        message: String,
    ) -> Result<NmpByteValue> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .radio_nmp_set_morse_custom_message(role, key, message)
            .map_err(async_graphql::Error::new)
    }

    async fn radio_nmp_set_morse_custom_ident(
        &self,
        ctx: &Context<'_>,
        role: RadioRole,
        key: u32,
        ident: String,
        format: Option<NmpDataFormat>,
    ) -> Result<NmpByteValue> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .radio_nmp_set_morse_custom_ident(role, key, ident, format)
            .map_err(async_graphql::Error::new)
    }

    async fn radio_nmp_set_user_key(
        &self,
        ctx: &Context<'_>,
        role: RadioRole,
        key: u32,
        new_user_key: u32,
    ) -> Result<bool> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .radio_nmp_set_user_key(role, key, new_user_key)
            .map_err(async_graphql::Error::new)
    }

    async fn radio_nmp_set_superuser_key(
        &self,
        ctx: &Context<'_>,
        role: RadioRole,
        key: u32,
        new_superuser_key: u32,
    ) -> Result<bool> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .radio_nmp_set_superuser_key(role, key, new_superuser_key)
            .map_err(async_graphql::Error::new)
    }

    async fn radio_nmp_set_itu_key(
        &self,
        ctx: &Context<'_>,
        role: RadioRole,
        key: u32,
        itu_key: String,
        format: Option<NmpDataFormat>,
    ) -> Result<bool> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .radio_nmp_set_itu_key(role, key, itu_key, format)
            .map_err(async_graphql::Error::new)
    }

    async fn radio_nmp_copy_factory_to_active(
        &self,
        ctx: &Context<'_>,
        role: RadioRole,
        key: u32,
    ) -> Result<bool> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .radio_nmp_copy_factory_to_active(role, key)
            .map_err(async_graphql::Error::new)
    }

    async fn radio_nmp_copy_active_to_factory(
        &self,
        ctx: &Context<'_>,
        role: RadioRole,
        key: u32,
    ) -> Result<bool> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .radio_nmp_copy_active_to_factory(role, key)
            .map_err(async_graphql::Error::new)
    }

    async fn radio_nmp_generate_reset_signal(
        &self,
        ctx: &Context<'_>,
        role: RadioRole,
        key: u32,
    ) -> Result<bool> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .radio_nmp_generate_reset_signal(role, key)
            .map_err(async_graphql::Error::new)
    }

    async fn radio_nmp_set_fsoo(
        &self,
        ctx: &Context<'_>,
        role: RadioRole,
        key: u32,
        inhibit: bool,
        period_ms: u64,
    ) -> Result<NmpFsooStatus> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .radio_nmp_set_fsoo(role, key, inhibit, period_ms)
            .map_err(async_graphql::Error::new)
    }

    async fn radio_nmp_reset_status_bytes(
        &self,
        ctx: &Context<'_>,
        role: RadioRole,
        key: u32,
    ) -> Result<NmpResetStatusBytes> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .radio_nmp_reset_status_bytes(role, key)
            .map_err(async_graphql::Error::new)
    }

    async fn radio_nmp_set_rssi_contribution_ratio(
        &self,
        ctx: &Context<'_>,
        role: RadioRole,
        key: u32,
        ratio: u8,
    ) -> Result<u8> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .radio_nmp_set_rssi_contribution_ratio(role, key, ratio)
            .map_err(async_graphql::Error::new)
    }

    async fn radio_nmp_set_check_comm_reset_period(
        &self,
        ctx: &Context<'_>,
        role: RadioRole,
        key: u32,
        hours: u16,
    ) -> Result<u16> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .radio_nmp_set_check_comm_reset_period(role, key, hours)
            .map_err(async_graphql::Error::new)
    }

    async fn radio_nmp_set_system_status_and_morse_period(
        &self,
        ctx: &Context<'_>,
        role: RadioRole,
        key: u32,
        ax25_system_status_period_ms: u32,
        morse_task_period_ms: u64,
    ) -> Result<NmpTelemetryPeriods> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .radio_nmp_set_system_status_and_morse_period(
                role,
                key,
                ax25_system_status_period_ms,
                morse_task_period_ms,
            )
            .map_err(async_graphql::Error::new)
    }

    async fn radio_nmp_set_hostname_user_part(
        &self,
        ctx: &Context<'_>,
        role: RadioRole,
        key: u32,
        user_part: String,
        format: Option<NmpDataFormat>,
    ) -> Result<NmpByteValue> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        context
            .subsystem()
            .radio_nmp_set_hostname_user_part(role, key, user_part, format)
            .map_err(async_graphql::Error::new)
    }
}

#[cfg(test)]
mod tests {
    use async_graphql::{EmptySubscription, Schema};

    use super::*;

    #[test]
    fn schema_exposes_every_implemented_nmp_command() {
        let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription).finish();
        let sdl = schema.sdl();
        let fields = [
            "radioNmpGetCspAddress",
            "radioNmpGetRouteTable",
            "radioNmpGetRoutingFromRs485",
            "radioNmpGetFrequency",
            "radioNmpGetLinkType",
            "radioNmpGetTxEnable",
            "radioNmpGetPreambleSize",
            "radioNmpGetNormalPower",
            "radioNmpGetGsRxTxDelay",
            "radioNmpGetDigipeaterEnable",
            "radioNmpGetCallsign",
            "radioNmpGetMorseCustomMessage",
            "radioNmpGetMorseCustomIdent",
            "radioNmpGetFsoo",
            "radioNmpGetHkRssi",
            "radioNmpGetRssiContributionRatio",
            "radioNmpGetConfig1",
            "radioNmpGetConfig2",
            "radioNmpGetCheckCommResetPeriod",
            "radioNmpGetSystemStatusAndMorsePeriod",
            "radioNmpGetFwCrc",
            "radioNmpGetHostname",
            "radioNmpUnlock",
            "radioNmpSetCspAddress",
            "radioNmpClearRouteTable",
            "radioNmpSetRoutes",
            "radioNmpSetRoutingFromRs485",
            "radioNmpSetFrequency",
            "radioNmpSetLinkType",
            "radioNmpSetTxEnable",
            "radioNmpSetPreambleSize",
            "radioNmpSetNormalPower",
            "radioNmpSetGsRxTxDelay",
            "radioNmpSetDigipeaterEnable",
            "radioNmpSetCallsign",
            "radioNmpSetMorseCustomMessage",
            "radioNmpSetMorseCustomIdent",
            "radioNmpSetUserKey",
            "radioNmpSetSuperuserKey",
            "radioNmpSetItuKey",
            "radioNmpCopyFactoryToActive",
            "radioNmpCopyActiveToFactory",
            "radioNmpGenerateResetSignal",
            "radioNmpSetFsoo",
            "radioNmpResetStatusBytes",
            "radioNmpSetRssiContributionRatio",
            "radioNmpSetCheckCommResetPeriod",
            "radioNmpSetSystemStatusAndMorsePeriod",
            "radioNmpSetHostnameUserPart",
        ];

        assert_eq!(fields.len(), 49);
        for field in fields {
            assert!(sdl.contains(field), "missing {field}");
        }
    }
}
