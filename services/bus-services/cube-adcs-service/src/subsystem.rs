use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use cubespace_adcs_api::{
    MSG_TYPE_TC, MSG_TYPE_TC_ACK, MSG_TYPE_TC_EXT, MSG_TYPE_TC_NACK, MSG_TYPE_TLM_NACK,
    MSG_TYPE_TLM_REQ, MSG_TYPE_TLM_RESP, MSG_TYPE_TLM_RESP_EXT, Telecommand, Telemetry,
    build_can_id, command_spec, decode_can_id, telemetry_spec,
};
use rust_can::{CanFrame, Connection};
use thiserror::Error;

const DEFAULT_INTERFACE: &str = "can0";
const DEFAULT_BITRATE: u32 = 1000000;
const DEFAULT_SRC_ADDRESS: u8 = 1;
const DEFAULT_DST_ADDRESS: u8 = 4;
const DEFAULT_TIMEOUT_MS: u64 = 5_000;

/// Service configuration values for CubeSpace ADCS CAN communication.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdcsServiceConfig {
    /// Linux SocketCAN interface name.
    pub interface: String,
    /// CAN bus bitrate.
    pub bitrate: u32,
    /// OBC/source CubeSpace address.
    pub source_address: u8,
    /// ADCS/destination CubeSpace address.
    pub destination_address: u8,
    /// Request/response timeout.
    pub timeout: Duration,
    /// Whether to run `ip link set <interface> up type can bitrate <bitrate>` at startup.
    pub bring_interface_up: bool,
}

impl AdcsServiceConfig {
    fn from_config(config: &kubos_service::Config) -> Self {
        Self {
            interface: config_string(config, "can_interface", DEFAULT_INTERFACE),
            bitrate: config_u32(config, "can_bitrate", DEFAULT_BITRATE),
            source_address: config_u8(config, "source_address", DEFAULT_SRC_ADDRESS),
            destination_address: config_u8(config, "destination_address", DEFAULT_DST_ADDRESS),
            timeout: Duration::from_millis(config_u64(config, "timeout_ms", DEFAULT_TIMEOUT_MS)),
            bring_interface_up: config_bool(config, "bring_interface_up", false),
        }
    }
}

/// Errors returned by the Cube ADCS service.
#[derive(Debug, Error)]
pub enum CubeAdcsError {
    /// CAN HAL error.
    #[error("CAN error: {0}")]
    Can(String),
    /// CubeSpace ADCS API error.
    #[error("ADCS API error: {0}")]
    Api(String),
    /// ADCS telecommand was rejected with an optional error code.
    #[error("telecommand {command_id} rejected by ADCS: {error_code:?}")]
    Nack {
        /// Rejected telecommand ID.
        command_id: u8,
        /// Optional ADCS error code from the NACK payload.
        error_code: Option<u8>,
    },
    /// A telemetry request was rejected with an optional error code.
    #[error("telemetry {telemetry_id} rejected by ADCS: {error_code:?}")]
    TelemetryNack {
        /// Rejected telemetry ID.
        telemetry_id: u8,
        /// Optional ADCS NACK error code.
        error_code: Option<u8>,
    },
    /// Unexpected CAN frame received while waiting for a response.
    #[error("unexpected response while waiting for ID {expected_id}: {details}")]
    UnexpectedResponse {
        /// Expected telecommand or telemetry ID.
        expected_id: u8,
        /// Details about the unexpected frame.
        details: String,
    },
    /// Internal lock was poisoned.
    #[error("subsystem lock poisoned")]
    LockPoisoned,
}

/// One raw CAN frame observed by the service.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RawFrame {
    /// CAN ID without Linux SocketCAN flags.
    pub id: u32,
    /// Whether the frame used an extended 29-bit CAN ID.
    pub extended: bool,
    /// CAN frame payload.
    pub data: Vec<u8>,
}

/// ACK/NACK response from an ADCS telecommand.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommandAck {
    /// Whether the ADCS acknowledged the command.
    pub acknowledged: bool,
    /// Telecommand ID.
    pub command_id: u8,
    /// Optional NACK error code.
    pub error_code: Option<u8>,
    /// Raw ACK/NACK frame payload.
    pub payload: Vec<u8>,
}

/// CubeSpace ADCS subsystem handle used by GraphQL resolvers.
#[derive(Clone)]
pub struct Subsystem {
    config: AdcsServiceConfig,
    connection: Arc<Mutex<Connection>>,
}

impl Subsystem {
    /// Creates a subsystem from KubOS service configuration.
    pub fn from_config(config: &kubos_service::Config) -> Result<Self, CubeAdcsError> {
        let config = AdcsServiceConfig::from_config(config);

        if config.bring_interface_up {
            Connection::set_interface_up(&config.interface, config.bitrate)
                .map_err(|err| CubeAdcsError::Can(err.to_string()))?;
        }

        let connection = Connection::from_interface(&config.interface)
            .map_err(|err| CubeAdcsError::Can(err.to_string()))?;

        Ok(Self {
            config,
            connection: Arc::new(Mutex::new(connection)),
        })
    }

    #[cfg(test)]
    pub(crate) fn with_connection(config: AdcsServiceConfig, connection: Connection) -> Self {
        Self {
            config,
            connection: Arc::new(Mutex::new(connection)),
        }
    }

    /// Returns service configuration.
    pub fn config(&self) -> &AdcsServiceConfig {
        &self.config
    }

    /// Sets the CAN interface up.
    pub fn set_interface_up(&self) -> Result<(), CubeAdcsError> {
        Connection::set_interface_up(&self.config.interface, self.config.bitrate)
            .map_err(|err| CubeAdcsError::Can(err.to_string()))
    }

    /// Sets the CAN interface down.
    pub fn set_interface_down(&self) -> Result<(), CubeAdcsError> {
        Connection::set_interface_down(&self.config.interface)
            .map_err(|err| CubeAdcsError::Can(err.to_string()))
    }

    /// Resets the CAN interface down then up.
    pub fn reset_interface(&self) -> Result<(), CubeAdcsError> {
        Connection::reset_interface(&self.config.interface, self.config.bitrate)
            .map_err(|err| CubeAdcsError::Can(err.to_string()))
    }

    /// Sends a telecommand payload and waits for ACK/NACK.
    pub fn send_command_payload(
        &self,
        command_id: u8,
        payload: &[u8],
    ) -> Result<CommandAck, CubeAdcsError> {
        let command = command_spec(command_id)
            .ok_or_else(|| CubeAdcsError::Api(format!("unknown telecommand ID: {command_id}")))?;
        if payload.len() != command.length_bytes {
            return Err(CubeAdcsError::Api(format!(
                "telecommand {command_id} expects {} payload bytes, received {}",
                command.length_bytes,
                payload.len()
            )));
        }

        let message_type = if payload.len() <= 8 {
            MSG_TYPE_TC
        } else {
            MSG_TYPE_TC_EXT
        };
        let can_id = build_can_id(
            message_type,
            command_id,
            self.config.source_address,
            self.config.destination_address,
        );
        let connection = self
            .connection
            .lock()
            .map_err(|_| CubeAdcsError::LockPoisoned)?;

        if message_type == MSG_TYPE_TC_EXT {
            write_extended_payload(&connection, can_id, payload)?;
        } else {
            connection
                .write(CanFrame::extended(can_id, payload))
                .map_err(|err| CubeAdcsError::Can(err.to_string()))?;
        }

        loop {
            let response = connection
                .read(self.config.timeout)
                .map_err(|err| CubeAdcsError::Can(err.to_string()))?;
            let fields = decode_can_id(response.id);

            if fields.tctlm_id != command_id
                || fields.src_addr != self.config.destination_address
                || fields.dst_addr != self.config.source_address
            {
                continue;
            }

            match fields.msg_type {
                MSG_TYPE_TC_ACK => {
                    return Ok(CommandAck {
                        acknowledged: true,
                        command_id,
                        error_code: None,
                        payload: response.data,
                    });
                }
                MSG_TYPE_TC_NACK => {
                    let error_code = response.data.first().copied();
                    return Ok(CommandAck {
                        acknowledged: false,
                        command_id,
                        error_code,
                        payload: response.data,
                    });
                }
                _ => {
                    return Err(CubeAdcsError::UnexpectedResponse {
                        expected_id: command_id,
                        details: format!("unexpected message type {}", fields.msg_type),
                    });
                }
            }
        }
    }

    /// Encodes and sends a typed telecommand.
    pub fn send_typed_command<C: Telecommand>(
        &self,
        command: &C,
    ) -> Result<CommandAck, CubeAdcsError> {
        let payload = command
            .encode()
            .map_err(|err| CubeAdcsError::Api(err.to_string()))?;
        self.send_command_payload(C::ID, &payload)
    }

    /// Sends a telemetry request and returns the raw payload.
    pub fn request_telemetry_payload(
        &self,
        telemetry_id: u8,
        length_bytes: usize,
    ) -> Result<Vec<u8>, CubeAdcsError> {
        let telemetry = telemetry_spec(telemetry_id)
            .ok_or_else(|| CubeAdcsError::Api(format!("unknown telemetry ID: {telemetry_id}")))?;
        if length_bytes != telemetry.length_bytes {
            return Err(CubeAdcsError::Api(format!(
                "telemetry {telemetry_id} expects {} payload bytes, requested {}",
                telemetry.length_bytes, length_bytes
            )));
        }
        let request_id = build_can_id(
            MSG_TYPE_TLM_REQ,
            telemetry_id,
            self.config.source_address,
            self.config.destination_address,
        );
        let connection = self
            .connection
            .lock()
            .map_err(|_| CubeAdcsError::LockPoisoned)?;

        connection
            .write(CanFrame::extended(request_id, &[]))
            .map_err(|err| CubeAdcsError::Can(err.to_string()))?;

        let start = Instant::now();
        let mut payload = Vec::with_capacity(length_bytes);
        let extended_frame_count = length_bytes.div_ceil(7);
        let initial_extended_counter = u8::try_from(extended_frame_count - 1).map_err(|_| {
            CubeAdcsError::Api(format!(
                "extended telemetry requires too many CAN frames: {extended_frame_count}"
            ))
        })?;
        let mut expected_extended_counter = Some(initial_extended_counter);
        while payload.len() < length_bytes {
            let remaining = self
                .config
                .timeout
                .checked_sub(start.elapsed())
                .ok_or_else(|| CubeAdcsError::Can("CAN read timed out".to_string()))?;
            let response = connection
                .read(remaining)
                .map_err(|err| CubeAdcsError::Can(err.to_string()))?;
            let fields = decode_can_id(response.id);

            if !response.extended
                || fields.tctlm_id != telemetry_id
                || fields.src_addr != self.config.destination_address
                || fields.dst_addr != self.config.source_address
            {
                continue;
            }

            match fields.msg_type {
                MSG_TYPE_TLM_NACK => {
                    return Err(CubeAdcsError::TelemetryNack {
                        telemetry_id,
                        error_code: response.data.first().copied(),
                    });
                }
                MSG_TYPE_TLM_RESP => {
                    payload.extend_from_slice(&response.data);
                }
                MSG_TYPE_TLM_RESP_EXT => {
                    let (&counter, data) = response.data.split_last().ok_or_else(|| {
                        CubeAdcsError::Can(
                            "extended telemetry frame did not contain a frame counter".to_string(),
                        )
                    })?;
                    let expected = expected_extended_counter.ok_or_else(|| {
                        CubeAdcsError::Can(
                            "received an extra extended telemetry frame after counter zero"
                                .to_string(),
                        )
                    })?;
                    if counter != expected {
                        return Err(CubeAdcsError::Can(format!(
                            "extended telemetry frame counter out of sequence: expected {expected}, received {counter}"
                        )));
                    }
                    payload.extend_from_slice(data);
                    expected_extended_counter = counter.checked_sub(1);
                }
                _ => continue,
            }
        }

        payload.truncate(length_bytes);
        Ok(payload)
    }

    /// Sends a telemetry request and decodes the response into a typed telemetry struct.
    pub fn request_typed_telemetry<T: Telemetry>(&self) -> Result<T, CubeAdcsError> {
        let payload = self.request_telemetry_payload(T::ID, T::LENGTH_BYTES)?;
        T::decode(&payload).map_err(|err| CubeAdcsError::Api(err.to_string()))
    }

    /// Sends a raw extended CAN frame.
    pub fn send_raw_frame(&self, can_id: u32, payload: &[u8]) -> Result<(), CubeAdcsError> {
        let connection = self
            .connection
            .lock()
            .map_err(|_| CubeAdcsError::LockPoisoned)?;
        connection
            .write(CanFrame::extended(can_id, payload))
            .map_err(|err| CubeAdcsError::Can(err.to_string()))
    }

    /// Reads a single raw CAN frame.
    pub fn read_raw_frame(&self) -> Result<RawFrame, CubeAdcsError> {
        let connection = self
            .connection
            .lock()
            .map_err(|_| CubeAdcsError::LockPoisoned)?;
        let frame = connection
            .read(self.config.timeout)
            .map_err(|err| CubeAdcsError::Can(err.to_string()))?;

        Ok(RawFrame {
            id: frame.id,
            extended: frame.extended,
            data: frame.data,
        })
    }
}

fn write_extended_payload(
    connection: &Connection,
    can_id: u32,
    payload: &[u8],
) -> Result<(), CubeAdcsError> {
    let frame_count = payload.len().div_ceil(7);
    let initial_counter = u8::try_from(frame_count - 1).map_err(|_| {
        CubeAdcsError::Api(format!(
            "extended telecommand requires too many CAN frames: {frame_count}"
        ))
    })?;

    for (index, chunk) in payload.chunks(7).enumerate() {
        let index = u8::try_from(index).map_err(|_| {
            CubeAdcsError::Api("extended telecommand frame index overflow".to_string())
        })?;
        let mut frame_payload = Vec::with_capacity(chunk.len() + 1);
        frame_payload.extend_from_slice(chunk);
        frame_payload.push(initial_counter - index);
        connection
            .write(CanFrame::extended(can_id, &frame_payload))
            .map_err(|err| CubeAdcsError::Can(err.to_string()))?;
    }

    Ok(())
}

fn config_string(config: &kubos_service::Config, key: &str, default: &str) -> String {
    config
        .get(key)
        .and_then(|value| value.as_str().map(ToOwned::to_owned))
        .unwrap_or_else(|| default.to_string())
}

fn config_u64(config: &kubos_service::Config, key: &str, default: u64) -> u64 {
    config
        .get(key)
        .and_then(|value| value.as_integer())
        .and_then(|value| u64::try_from(value).ok())
        .unwrap_or(default)
}

fn config_u32(config: &kubos_service::Config, key: &str, default: u32) -> u32 {
    config
        .get(key)
        .and_then(|value| value.as_integer())
        .and_then(|value| u32::try_from(value).ok())
        .unwrap_or(default)
}

fn config_u8(config: &kubos_service::Config, key: &str, default: u8) -> u8 {
    config
        .get(key)
        .and_then(|value| value.as_integer())
        .and_then(|value| u8::try_from(value).ok())
        .unwrap_or(default)
}

fn config_bool(config: &kubos_service::Config, key: &str, default: bool) -> bool {
    config
        .get(key)
        .and_then(|value| value.as_bool())
        .unwrap_or(default)
}
