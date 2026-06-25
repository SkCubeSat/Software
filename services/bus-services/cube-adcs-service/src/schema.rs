use async_graphql::{Context, MergedObject, Object, Result};
use cubespace_adcs_api::{
    COMMAND_SPECS, CommandInfo, CommandResponse, HealthInfo, MutationResponse, RawFrameResponse,
    TELEMETRY_SPECS, TelemetryInfo, command_spec, telemetry_spec,
};

use crate::schema_generated::{GeneratedMutationRoot, GeneratedQueryRoot};
use crate::subsystem::{CommandAck, RawFrame, Subsystem};

/// GraphQL query root.
#[derive(MergedObject, Default)]
pub struct QueryRoot(pub BaseQueryRoot, pub GeneratedQueryRoot);

/// GraphQL mutation root.
#[derive(MergedObject, Default)]
pub struct MutationRoot(pub BaseMutationRoot, pub GeneratedMutationRoot);

/// Hand-written GraphQL queries for service health, raw frames, and metadata.
#[derive(Default)]
pub struct BaseQueryRoot;

/// Hand-written GraphQL mutations for CAN interface and raw command support.
#[derive(Default)]
pub struct BaseMutationRoot;

#[Object]
impl BaseQueryRoot {
    /// Test query to verify the service is alive.
    async fn ping(&self) -> &str {
        "pong"
    }

    /// Returns service configuration.
    async fn health(&self, ctx: &Context<'_>) -> Result<HealthInfo> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        let config = context.subsystem().config();

        Ok(HealthInfo::new(
            config.interface.clone(),
            i64::from(config.bitrate),
            i32::from(config.source_address),
            i32::from(config.destination_address),
            config.timeout.as_millis() as i64,
        ))
    }

    /// Lists telecommands generated from the ADCS command matrix.
    async fn command_definitions(&self) -> Vec<CommandInfo> {
        COMMAND_SPECS.iter().map(CommandInfo::from_spec).collect()
    }

    /// Returns one telecommand definition by ID.
    async fn command_definition(&self, id: i32) -> Result<CommandInfo> {
        let id = checked_u8(id, "id")?;
        command_spec(id)
            .map(CommandInfo::from_spec)
            .ok_or_else(|| async_graphql::Error::new(format!("unknown telecommand ID: {id}")))
    }

    /// Lists telemetry requests generated from the ADCS telemetry matrix.
    async fn telemetry_definitions(&self) -> Vec<TelemetryInfo> {
        TELEMETRY_SPECS
            .iter()
            .map(TelemetryInfo::from_spec)
            .collect()
    }

    /// Returns one telemetry definition by ID.
    async fn telemetry_definition(&self, id: i32) -> Result<TelemetryInfo> {
        let id = checked_u8(id, "id")?;
        telemetry_spec(id)
            .map(TelemetryInfo::from_spec)
            .ok_or_else(|| async_graphql::Error::new(format!("unknown telemetry ID: {id}")))
    }

    /// Reads one raw CAN frame.
    async fn read_raw_frame(&self, ctx: &Context<'_>) -> Result<RawFrameResponse> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        match context.subsystem().read_raw_frame() {
            Ok(frame) => Ok(map_raw_frame(frame)),
            Err(err) => Ok(RawFrameResponse::failure(err)),
        }
    }
}

#[Object]
impl BaseMutationRoot {
    /// Sets the CAN interface up using the configured bitrate.
    async fn set_interface_up(&self, ctx: &Context<'_>) -> Result<MutationResponse> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        map_empty_result(context.subsystem().set_interface_up())
    }

    /// Sets the CAN interface down.
    async fn set_interface_down(&self, ctx: &Context<'_>) -> Result<MutationResponse> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        map_empty_result(context.subsystem().set_interface_down())
    }

    /// Resets the CAN interface down then up.
    async fn reset_interface(&self, ctx: &Context<'_>) -> Result<MutationResponse> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        map_empty_result(context.subsystem().reset_interface())
    }

    /// Sends a raw telecommand payload as hexadecimal and waits for ACK/NACK.
    async fn send_command_raw(
        &self,
        ctx: &Context<'_>,
        id: i32,
        payload_hex: String,
    ) -> Result<CommandResponse> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        let id = checked_u8(id, "id")?;
        let payload = decode_hex(&payload_hex)?;
        Ok(map_command_ack(
            id,
            context.subsystem().send_command_payload(id, &payload),
        ))
    }

    /// Sends a raw extended CAN frame.
    async fn send_raw_frame(
        &self,
        ctx: &Context<'_>,
        can_id: i64,
        payload_hex: String,
    ) -> Result<MutationResponse> {
        let context = ctx.data::<kubos_service::Context<Subsystem>>()?;
        let can_id = checked_u32(can_id, "canId")?;
        let payload = decode_hex(&payload_hex)?;
        map_empty_result(context.subsystem().send_raw_frame(can_id, &payload))
    }
}

fn map_empty_result(
    result: Result<(), crate::subsystem::CubeAdcsError>,
) -> Result<MutationResponse> {
    match result {
        Ok(()) => Ok(MutationResponse::success_response()),
        Err(err) => Ok(MutationResponse::failure(err)),
    }
}

pub(crate) fn map_command_ack(
    command_id: u8,
    result: Result<CommandAck, crate::subsystem::CubeAdcsError>,
) -> CommandResponse {
    match result {
        Ok(ack) => CommandResponse::from_ack(
            ack.command_id,
            ack.acknowledged,
            ack.error_code,
            encode_hex(&ack.payload),
        ),
        Err(err) => CommandResponse::failure(command_id, err),
    }
}

fn map_raw_frame(frame: RawFrame) -> RawFrameResponse {
    RawFrameResponse::success_response(frame.id, frame.extended, encode_hex(&frame.data))
}

fn checked_u8(value: i32, name: &str) -> Result<u8> {
    u8::try_from(value).map_err(|_| async_graphql::Error::new(format!("{name} must be 0..=255")))
}

fn checked_u32(value: i64, name: &str) -> Result<u32> {
    u32::try_from(value)
        .map_err(|_| async_graphql::Error::new(format!("{name} must be 0..=u32::MAX")))
}

fn encode_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn decode_hex(value: &str) -> Result<Vec<u8>> {
    let compact: String = value
        .chars()
        .filter(|character| !character.is_ascii_whitespace() && *character != '_')
        .collect();

    if compact.len() % 2 != 0 {
        return Err(async_graphql::Error::new(
            "hex payload must contain an even number of digits",
        ));
    }

    compact
        .as_bytes()
        .chunks(2)
        .map(|chunk| {
            std::str::from_utf8(chunk)
                .map_err(|err| async_graphql::Error::new(err.to_string()))
                .and_then(|hex| {
                    u8::from_str_radix(hex, 16)
                        .map_err(|err| async_graphql::Error::new(err.to_string()))
                })
        })
        .collect()
}
