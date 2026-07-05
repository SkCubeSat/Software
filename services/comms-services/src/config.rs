use std::time::Duration;

use kubos_comms::CommsConfig;
use kubos_service::Config;
use thiserror::Error;
use toml::Value;

pub const SERVICE_NAME: &str = "comms-services";

const DEFAULT_BACKLOG: usize = 10;
const DEFAULT_COMMAND_TIMEOUT_MS: u64 = 1_000;
const DEFAULT_MAX_FRAME_BYTES: usize = 260;
const DEFAULT_SFP_MAX_SPACE_PACKET_BYTES: usize = u16::MAX as usize + 6;
const DEFAULT_SFP_MTU: usize = 240;
const MAX_SFP_MTU_WITH_RDP: usize = 243;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("missing config table `{0}`")]
    MissingTable(String),
    #[error("missing config value `{0}`")]
    MissingValue(String),
    #[error("invalid config value `{key}`: {message}")]
    InvalidValue { key: String, message: String },
    #[error("communication service config error: {0}")]
    Comms(String),
}

#[derive(Debug, Clone)]
pub struct ServiceSettings {
    pub comms: CommsConfig,
    pub csp: CspSettings,
    pub radios: RadioSettings,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CspSettings {
    pub obc_node: u16,
    pub uplink_packet_csp_port: u8,
    pub uplink_sfp_csp_port: u8,
    pub ground_node: u16,
    pub ground_packet_csp_port: u8,
    pub ground_sfp_csp_port: u8,
    pub backlog: usize,
    pub max_frame_bytes: usize,
    pub sfp_mtu: usize,
    pub sfp_max_space_packet_bytes: usize,
    pub sfp_use_rdp: bool,
    pub uplink_crypto: UplinkCrypto,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UplinkCrypto {
    None,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RadioSettings {
    pub uplink: RadioConfig,
    pub downlink: RadioConfig,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RadioConfig {
    pub bus: String,
    pub csp_node: u16,
    pub i2c_addr: u8,
    pub slave_rx_device: Option<String>,
    pub command_timeout: Duration,
}

impl ServiceSettings {
    pub fn from_config(config: &Config) -> Result<Self, ConfigError> {
        let comms =
            CommsConfig::new(config.clone()).map_err(|err| ConfigError::Comms(err.to_string()))?;
        let csp = CspSettings::from_config(config)?;
        let radios = RadioSettings::from_config(config)?;

        Ok(Self { comms, csp, radios })
    }
}

impl CspSettings {
    fn from_config(config: &Config) -> Result<Self, ConfigError> {
        let table = config_table(config, "csp")?;
        let obc_node = required_u16(&table, "csp.obc_node")?;
        let uplink_packet_csp_port = required_port(&table, "csp.uplink_packet_csp_port")?;
        let uplink_sfp_csp_port = required_port(&table, "csp.uplink_sfp_csp_port")?;
        let ground_node = required_u16(&table, "csp.ground_node")?;
        let ground_packet_csp_port = required_port(&table, "csp.ground_packet_csp_port")?;
        let ground_sfp_csp_port = required_port(&table, "csp.ground_sfp_csp_port")?;
        require_distinct_ports(
            "csp.uplink_packet_csp_port",
            uplink_packet_csp_port,
            "csp.uplink_sfp_csp_port",
            uplink_sfp_csp_port,
        )?;
        require_distinct_ports(
            "csp.ground_packet_csp_port",
            ground_packet_csp_port,
            "csp.ground_sfp_csp_port",
            ground_sfp_csp_port,
        )?;
        let backlog = optional_usize(&table, "csp.backlog", DEFAULT_BACKLOG)?;
        let max_frame_bytes =
            optional_usize(&table, "csp.max_frame_bytes", DEFAULT_MAX_FRAME_BYTES)?;
        let sfp_mtu = optional_usize(&table, "csp.sfp_mtu", DEFAULT_SFP_MTU)?;
        if sfp_mtu == 0 || sfp_mtu > MAX_SFP_MTU_WITH_RDP {
            return Err(ConfigError::InvalidValue {
                key: "csp.sfp_mtu".to_string(),
                message: format!(
                    "expected 1..={MAX_SFP_MTU_WITH_RDP}; this reserves CSP buffer space for SFP and RDP headers"
                ),
            });
        }
        let sfp_max_space_packet_bytes = optional_usize(
            &table,
            "csp.sfp_max_space_packet_bytes",
            DEFAULT_SFP_MAX_SPACE_PACKET_BYTES,
        )?;
        if sfp_max_space_packet_bytes > DEFAULT_SFP_MAX_SPACE_PACKET_BYTES {
            return Err(ConfigError::InvalidValue {
                key: "csp.sfp_max_space_packet_bytes".to_string(),
                message: format!(
                    "SpacePacket length field allows at most {DEFAULT_SFP_MAX_SPACE_PACKET_BYTES} bytes"
                ),
            });
        }
        let sfp_use_rdp = optional_bool(&table, "csp.sfp_use_rdp", true)?;
        let uplink_crypto = match optional_str(&table, "csp.uplink_crypto", "none")? {
            "none" => UplinkCrypto::None,
            value => {
                return Err(ConfigError::InvalidValue {
                    key: "csp.uplink_crypto".to_string(),
                    message: format!(
                        "`{value}` is not implemented yet; use `none` for the first cleartext service"
                    ),
                });
            }
        };

        Ok(Self {
            obc_node,
            uplink_packet_csp_port,
            uplink_sfp_csp_port,
            ground_node,
            ground_packet_csp_port,
            ground_sfp_csp_port,
            backlog,
            max_frame_bytes,
            sfp_mtu,
            sfp_max_space_packet_bytes,
            sfp_use_rdp,
            uplink_crypto,
        })
    }
}

fn require_distinct_ports(
    first_key: &str,
    first: u8,
    second_key: &str,
    second: u8,
) -> Result<(), ConfigError> {
    if first != second {
        return Ok(());
    }

    Err(ConfigError::InvalidValue {
        key: second_key.to_string(),
        message: format!("must differ from `{first_key}`; both were {first}"),
    })
}

impl RadioSettings {
    fn from_config(config: &Config) -> Result<Self, ConfigError> {
        let radios = config_table(config, "radios")?;
        let uplink = radio_config(&radios, "uplink")?;
        let downlink = radio_config(&radios, "downlink")?;

        Ok(Self { uplink, downlink })
    }
}

fn radio_config(radios: &Value, role: &str) -> Result<RadioConfig, ConfigError> {
    let prefix = format!("radios.{role}");
    let table = value_table(radios.get(role), &prefix)?;
    let bus = required_str(table, &format!("{prefix}.bus"))?.to_string();
    let csp_node = required_u16(table, &format!("{prefix}.csp_node"))?;
    let i2c_addr = required_u8(table, &format!("{prefix}.i2c_addr"))?;
    let slave_rx_device = optional_string(table, &format!("{prefix}.slave_rx_device"))?;
    let command_timeout_ms = optional_u64(
        table,
        &format!("{prefix}.command_timeout_ms"),
        DEFAULT_COMMAND_TIMEOUT_MS,
    )?;

    Ok(RadioConfig {
        bus,
        csp_node,
        i2c_addr,
        slave_rx_device,
        command_timeout: Duration::from_millis(command_timeout_ms),
    })
}

fn config_table(config: &Config, key: &str) -> Result<Value, ConfigError> {
    let value = config
        .get(key)
        .ok_or_else(|| ConfigError::MissingTable(key.to_string()))?;

    if value.as_table().is_some() {
        Ok(value)
    } else {
        Err(ConfigError::InvalidValue {
            key: key.to_string(),
            message: "expected table".to_string(),
        })
    }
}

fn value_table<'a>(value: Option<&'a Value>, key: &str) -> Result<&'a Value, ConfigError> {
    let value = value.ok_or_else(|| ConfigError::MissingTable(key.to_string()))?;

    if value.as_table().is_some() {
        Ok(value)
    } else {
        Err(ConfigError::InvalidValue {
            key: key.to_string(),
            message: "expected table".to_string(),
        })
    }
}

fn required_str<'a>(table: &'a Value, key: &str) -> Result<&'a str, ConfigError> {
    table
        .get(key.rsplit('.').next().unwrap())
        .and_then(Value::as_str)
        .ok_or_else(|| ConfigError::MissingValue(key.to_string()))
}

fn optional_str<'a>(table: &'a Value, key: &str, default: &'a str) -> Result<&'a str, ConfigError> {
    match table.get(key.rsplit('.').next().unwrap()) {
        Some(value) => value.as_str().ok_or_else(|| ConfigError::InvalidValue {
            key: key.to_string(),
            message: "expected string".to_string(),
        }),
        None => Ok(default),
    }
}

fn optional_string(table: &Value, key: &str) -> Result<Option<String>, ConfigError> {
    match table.get(key.rsplit('.').next().unwrap()) {
        Some(value) => value
            .as_str()
            .map(|value| Some(value.to_string()))
            .ok_or_else(|| ConfigError::InvalidValue {
                key: key.to_string(),
                message: "expected string".to_string(),
            }),
        None => Ok(None),
    }
}

fn required_u8(table: &Value, key: &str) -> Result<u8, ConfigError> {
    parse_integer(table, key).and_then(|value| {
        u8::try_from(value).map_err(|_| ConfigError::InvalidValue {
            key: key.to_string(),
            message: "expected integer in range 0..=255".to_string(),
        })
    })
}

fn required_u16(table: &Value, key: &str) -> Result<u16, ConfigError> {
    parse_integer(table, key).and_then(|value| {
        u16::try_from(value).map_err(|_| ConfigError::InvalidValue {
            key: key.to_string(),
            message: "expected integer in range 0..=65535".to_string(),
        })
    })
}

fn required_port(table: &Value, key: &str) -> Result<u8, ConfigError> {
    let port = required_u8(table, key)?;
    if port <= 63 {
        Ok(port)
    } else {
        Err(ConfigError::InvalidValue {
            key: key.to_string(),
            message: "CSP v1 ports must be in range 0..=63".to_string(),
        })
    }
}

fn optional_u64(table: &Value, key: &str, default: u64) -> Result<u64, ConfigError> {
    match table.get(key.rsplit('.').next().unwrap()) {
        Some(_) => parse_integer(table, key),
        None => Ok(default),
    }
}

fn optional_usize(table: &Value, key: &str, default: usize) -> Result<usize, ConfigError> {
    match table.get(key.rsplit('.').next().unwrap()) {
        Some(_) => {
            let value = parse_integer(table, key)?;
            usize::try_from(value).map_err(|_| ConfigError::InvalidValue {
                key: key.to_string(),
                message: "expected non-negative integer".to_string(),
            })
        }
        None => Ok(default),
    }
}

fn optional_bool(table: &Value, key: &str, default: bool) -> Result<bool, ConfigError> {
    match table.get(key.rsplit('.').next().unwrap()) {
        Some(value) => value.as_bool().ok_or_else(|| ConfigError::InvalidValue {
            key: key.to_string(),
            message: "expected boolean".to_string(),
        }),
        None => Ok(default),
    }
}

fn parse_integer(table: &Value, key: &str) -> Result<u64, ConfigError> {
    let short_key = key.rsplit('.').next().unwrap();
    let value = table
        .get(short_key)
        .and_then(Value::as_integer)
        .ok_or_else(|| ConfigError::MissingValue(key.to_string()))?;

    u64::try_from(value).map_err(|_| ConfigError::InvalidValue {
        key: key.to_string(),
        message: "expected non-negative integer".to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(contents: &str) -> ServiceSettings {
        let config = Config::new_from_str(SERVICE_NAME, contents).unwrap();
        ServiceSettings::from_config(&config).unwrap()
    }

    #[test]
    fn parses_separate_bus_layout() {
        let settings = parse(
            r#"
            [comms-services.addr]
            ip = "127.0.0.1"
            port = 8150

            [comms-services.comms]
            ip = "127.0.0.1"
            downlink_ports = [14011]

            [comms-services.csp]
            obc_node = 1
            uplink_packet_csp_port = 10
            uplink_sfp_csp_port = 12
            ground_node = 2
            ground_packet_csp_port = 11
            ground_sfp_csp_port = 13
            sfp_mtu = 240
            sfp_use_rdp = true

            [comms-services.radios.uplink]
            bus = "/dev/i2c-1"
            csp_node = 8
            i2c_addr = 8
            slave_rx_device = "/dev/i2c-slave-frameq-1-01"

            [comms-services.radios.downlink]
            bus = "/dev/i2c-2"
            csp_node = 9
            i2c_addr = 9
            "#,
        );

        assert_eq!(settings.csp.obc_node, 1);
        assert_eq!(settings.csp.uplink_packet_csp_port, 10);
        assert_eq!(settings.csp.uplink_sfp_csp_port, 12);
        assert_eq!(settings.csp.ground_packet_csp_port, 11);
        assert_eq!(settings.csp.ground_sfp_csp_port, 13);
        assert_eq!(settings.csp.sfp_mtu, 240);
        assert!(settings.csp.sfp_use_rdp);
        assert_eq!(settings.csp.uplink_crypto, UplinkCrypto::None);
        assert_eq!(settings.radios.uplink.bus, "/dev/i2c-1");
        assert_eq!(
            settings.radios.uplink.slave_rx_device.as_deref(),
            Some("/dev/i2c-slave-frameq-1-01")
        );
        assert_eq!(settings.radios.downlink.bus, "/dev/i2c-2");
    }

    #[test]
    fn rejects_unimplemented_crypto() {
        let config = Config::new_from_str(
            SERVICE_NAME,
            r#"
            [comms-services.addr]
            ip = "127.0.0.1"
            port = 8150

            [comms-services.comms]
            ip = "127.0.0.1"

            [comms-services.csp]
            obc_node = 1
            uplink_packet_csp_port = 10
            uplink_sfp_csp_port = 12
            ground_node = 2
            ground_packet_csp_port = 11
            ground_sfp_csp_port = 13
            uplink_crypto = "aes-128"

            [comms-services.radios.uplink]
            bus = "/dev/i2c-1"
            csp_node = 8
            i2c_addr = 8

            [comms-services.radios.downlink]
            bus = "/dev/i2c-1"
            csp_node = 9
            i2c_addr = 9
            "#,
        )
        .unwrap();

        assert!(matches!(
            ServiceSettings::from_config(&config),
            Err(ConfigError::InvalidValue { key, .. }) if key == "csp.uplink_crypto"
        ));
    }

    #[test]
    fn rejects_duplicate_uplink_ports() {
        let config = Config::new_from_str(
            SERVICE_NAME,
            r#"
            [comms-services.addr]
            ip = "127.0.0.1"
            port = 8150

            [comms-services.comms]
            ip = "127.0.0.1"

            [comms-services.csp]
            obc_node = 1
            uplink_packet_csp_port = 10
            uplink_sfp_csp_port = 10
            ground_node = 2
            ground_packet_csp_port = 11
            ground_sfp_csp_port = 13

            [comms-services.radios.uplink]
            bus = "/dev/i2c-1"
            csp_node = 8
            i2c_addr = 8

            [comms-services.radios.downlink]
            bus = "/dev/i2c-1"
            csp_node = 9
            i2c_addr = 9
            "#,
        )
        .unwrap();

        assert!(matches!(
            ServiceSettings::from_config(&config),
            Err(ConfigError::InvalidValue { key, .. }) if key == "csp.uplink_sfp_csp_port"
        ));
    }

    #[test]
    fn rejects_sfp_mtu_too_large_for_rdp() {
        let config = Config::new_from_str(
            SERVICE_NAME,
            r#"
            [comms-services.addr]
            ip = "127.0.0.1"
            port = 8150

            [comms-services.comms]
            ip = "127.0.0.1"

            [comms-services.csp]
            obc_node = 1
            uplink_packet_csp_port = 10
            uplink_sfp_csp_port = 12
            ground_node = 2
            ground_packet_csp_port = 11
            ground_sfp_csp_port = 13
            sfp_mtu = 248

            [comms-services.radios.uplink]
            bus = "/dev/i2c-1"
            csp_node = 8
            i2c_addr = 8

            [comms-services.radios.downlink]
            bus = "/dev/i2c-1"
            csp_node = 9
            i2c_addr = 9
            "#,
        )
        .unwrap();

        assert!(matches!(
            ServiceSettings::from_config(&config),
            Err(ConfigError::InvalidValue { key, .. }) if key == "csp.sfp_mtu"
        ));
    }
}
