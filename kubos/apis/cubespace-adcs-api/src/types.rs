//! Typed CubeSpace ADCS command and telemetry contracts.

use async_graphql::SimpleObject;

use crate::AdcsResult;

/// Data type used by a command or telemetry field in the CubeSpace matrix.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DataType {
    /// Unsigned integer field.
    Uint,
    /// Signed integer field.
    Int,
    /// IEEE-754 32-bit floating point field.
    Float,
    /// IEEE-754 64-bit floating point field.
    Double,
    /// Enumerated integer field.
    Enum,
    /// Boolean field.
    Bool,
    /// Byte array field.
    Array,
    /// UTF-8 string field.
    String,
    /// Padding field.
    Padding,
    /// Unknown field type from the source workbook.
    Unknown,
}

/// Static metadata for one command or telemetry field.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FieldSpec {
    /// Bit offset from the start of the payload.
    pub offset_bits: u32,
    /// Field length in bits.
    pub length_bits: u32,
    /// Field name from the CubeSpace matrix.
    pub name: &'static str,
    /// Field data type.
    pub data_type: DataType,
    /// Field description from the CubeSpace matrix.
    pub description: &'static str,
    /// Engineering-value multiplier for decoded telemetry, if known.
    pub scale: Option<f64>,
    /// Engineering unit, if known.
    pub unit: Option<&'static str>,
    /// Referenced enum table key, if known.
    pub enum_table: Option<&'static str>,
}

/// Static metadata for one telecommand.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CommandSpec {
    /// Telecommand ID.
    pub id: u8,
    /// Telecommand name.
    pub name: &'static str,
    /// Telecommand purpose from the CubeSpace matrix.
    pub purpose: &'static str,
    /// Payload length in bytes.
    pub length_bytes: usize,
    /// Field metadata.
    pub fields: &'static [FieldSpec],
}

/// Static metadata for one telemetry request.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TelemetrySpec {
    /// Telemetry ID.
    pub id: u8,
    /// Telemetry name.
    pub name: &'static str,
    /// Telemetry purpose from the CubeSpace matrix.
    pub purpose: &'static str,
    /// Response payload length in bytes.
    pub length_bytes: usize,
    /// Field metadata.
    pub fields: &'static [FieldSpec],
}

/// A typed CubeSpace ADCS telecommand.
pub trait Telecommand {
    /// Telecommand ID.
    const ID: u8;
    /// Telecommand name from the CubeSpace matrix.
    const NAME: &'static str;
    /// Payload length in bytes.
    const LENGTH_BYTES: usize;

    /// Encodes this command into the CubeSpace ADCS payload format.
    fn encode(&self) -> AdcsResult<Vec<u8>>;
}

/// A typed CubeSpace ADCS telemetry response.
pub trait Telemetry: Sized {
    /// Telemetry ID.
    const ID: u8;
    /// Telemetry name from the CubeSpace matrix.
    const NAME: &'static str;
    /// Payload length in bytes.
    const LENGTH_BYTES: usize;

    /// Decodes this telemetry response from a CubeSpace ADCS payload.
    fn decode(payload: &[u8]) -> AdcsResult<Self>;
}

/// Service health/configuration information.
#[derive(Clone, Debug, PartialEq, SimpleObject)]
pub struct HealthInfo {
    /// Linux SocketCAN interface name.
    pub interface: String,
    /// CAN bitrate.
    pub bitrate: i64,
    /// OBC/source CubeSpace address.
    pub source_address: i32,
    /// ADCS/destination CubeSpace address.
    pub destination_address: i32,
    /// Request timeout in milliseconds.
    pub timeout_ms: i64,
}

impl HealthInfo {
    /// Builds service health/configuration information.
    pub fn new(
        interface: String,
        bitrate: i64,
        source_address: i32,
        destination_address: i32,
        timeout_ms: i64,
    ) -> Self {
        Self {
            interface,
            bitrate,
            source_address,
            destination_address,
            timeout_ms,
        }
    }
}

/// One ADCS command or telemetry field definition.
#[derive(Clone, Debug, PartialEq, SimpleObject)]
pub struct FieldInfo {
    /// Field name.
    pub name: String,
    /// Bit offset from the start of the payload.
    pub offset_bits: i64,
    /// Field length in bits.
    pub length_bits: i64,
    /// Field data type.
    pub data_type: String,
    /// Field description.
    pub description: String,
    /// Engineering-value multiplier, if available.
    pub scale: Option<f64>,
    /// Engineering unit, if available.
    pub unit: Option<String>,
    /// Referenced enum table, if available.
    pub enum_table: Option<String>,
}

impl FieldInfo {
    /// Builds GraphQL field metadata from a generated ADCS field spec.
    pub fn from_spec(field: &FieldSpec) -> Self {
        Self {
            name: field.name.to_string(),
            offset_bits: i64::from(field.offset_bits),
            length_bits: i64::from(field.length_bits),
            data_type: data_type_name(field.data_type).to_string(),
            description: field.description.to_string(),
            scale: field.scale,
            unit: field.unit.map(ToOwned::to_owned),
            enum_table: field.enum_table.map(ToOwned::to_owned),
        }
    }
}

/// One ADCS telecommand definition.
#[derive(Clone, Debug, PartialEq, SimpleObject)]
pub struct CommandInfo {
    /// Telecommand ID.
    pub id: i32,
    /// Telecommand name.
    pub name: String,
    /// Purpose from the command matrix.
    pub purpose: String,
    /// Payload length in bytes.
    pub length_bytes: i64,
    /// Parameter definitions.
    pub fields: Vec<FieldInfo>,
}

impl CommandInfo {
    /// Builds GraphQL command metadata from a generated ADCS command spec.
    pub fn from_spec(command: &CommandSpec) -> Self {
        Self {
            id: i32::from(command.id),
            name: command.name.to_string(),
            purpose: command.purpose.to_string(),
            length_bytes: command.length_bytes as i64,
            fields: command.fields.iter().map(FieldInfo::from_spec).collect(),
        }
    }
}

/// One ADCS telemetry definition.
#[derive(Clone, Debug, PartialEq, SimpleObject)]
pub struct TelemetryInfo {
    /// Telemetry ID.
    pub id: i32,
    /// Telemetry name.
    pub name: String,
    /// Purpose from the telemetry matrix.
    pub purpose: String,
    /// Response length in bytes.
    pub length_bytes: i64,
    /// Channel definitions.
    pub fields: Vec<FieldInfo>,
}

impl TelemetryInfo {
    /// Builds GraphQL telemetry metadata from a generated ADCS telemetry spec.
    pub fn from_spec(telemetry: &TelemetrySpec) -> Self {
        Self {
            id: i32::from(telemetry.id),
            name: telemetry.name.to_string(),
            purpose: telemetry.purpose.to_string(),
            length_bytes: telemetry.length_bytes as i64,
            fields: telemetry.fields.iter().map(FieldInfo::from_spec).collect(),
        }
    }
}

/// Generic mutation response.
#[derive(Clone, Debug, PartialEq, Eq, SimpleObject)]
pub struct MutationResponse {
    /// Whether the mutation succeeded.
    pub success: bool,
    /// Error text if the mutation failed.
    pub errors: String,
}

impl MutationResponse {
    /// Builds a successful mutation response.
    pub fn success_response() -> Self {
        Self {
            success: true,
            errors: String::new(),
        }
    }

    /// Builds a failed mutation response.
    pub fn failure(error: impl ToString) -> Self {
        Self {
            success: false,
            errors: error.to_string(),
        }
    }
}

/// Telecommand ACK/NACK response.
#[derive(Clone, Debug, PartialEq, Eq, SimpleObject)]
pub struct CommandResponse {
    /// Whether the command was acknowledged by the ADCS.
    pub success: bool,
    /// Error text if the command failed or was NACKed.
    pub errors: String,
    /// Telecommand ID.
    pub command_id: i32,
    /// Whether the ADCS returned ACK.
    pub acknowledged: bool,
    /// Optional ADCS NACK error code.
    pub error_code: Option<i32>,
    /// Raw ACK/NACK payload as hexadecimal.
    pub payload_hex: String,
}

impl CommandResponse {
    /// Builds a telecommand response from ADCS ACK/NACK data.
    pub fn from_ack(
        command_id: u8,
        acknowledged: bool,
        error_code: Option<u8>,
        payload_hex: String,
    ) -> Self {
        Self {
            success: acknowledged,
            errors: if acknowledged {
                String::new()
            } else {
                format!("ADCS returned NACK: {:?}", error_code)
            },
            command_id: i32::from(command_id),
            acknowledged,
            error_code: error_code.map(i32::from),
            payload_hex,
        }
    }

    /// Builds a failed telecommand response.
    pub fn failure(command_id: u8, error: impl ToString) -> Self {
        Self {
            success: false,
            errors: error.to_string(),
            command_id: i32::from(command_id),
            acknowledged: false,
            error_code: None,
            payload_hex: String::new(),
        }
    }
}

/// Raw CAN frame response.
#[derive(Clone, Debug, PartialEq, Eq, SimpleObject)]
pub struct RawFrameResponse {
    /// Whether the operation succeeded.
    pub success: bool,
    /// Error text if the operation failed.
    pub errors: String,
    /// CAN ID without Linux SocketCAN flags.
    pub id: i64,
    /// Whether the frame used an extended 29-bit CAN ID.
    pub extended: bool,
    /// Payload as hexadecimal.
    pub data_hex: String,
}

impl RawFrameResponse {
    /// Builds a successful raw CAN frame response.
    pub fn success_response(id: u32, extended: bool, data_hex: String) -> Self {
        Self {
            success: true,
            errors: String::new(),
            id: i64::from(id),
            extended,
            data_hex,
        }
    }

    /// Builds a failed raw CAN frame response.
    pub fn failure(error: impl ToString) -> Self {
        Self {
            success: false,
            errors: error.to_string(),
            id: 0,
            extended: false,
            data_hex: String::new(),
        }
    }
}

fn data_type_name(data_type: DataType) -> &'static str {
    match data_type {
        DataType::Uint => "uint",
        DataType::Int => "int",
        DataType::Float => "float",
        DataType::Double => "double",
        DataType::Enum => "enum",
        DataType::Bool => "bool",
        DataType::Array => "array",
        DataType::String => "string",
        DataType::Padding => "padding",
        DataType::Unknown => "unknown",
    }
}
