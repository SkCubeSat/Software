//! GraphQL-facing Needronix Management Protocol (NMP) types and wrappers.

use std::fmt::Write;

use async_graphql::{Enum, InputObject, SimpleObject};
use nxtrx4_api::{
    Nxtrx4,
    config::{
        AdcConfig, Ax25Config, Config1, CspConfig, DigipeaterConfig, InterfaceConfig, MorseConfig,
        RadioConfig, RadioLinkType,
    },
    nmp::{
        csp::RouteEntry,
        housekeeping::{FirmwareCrc, FsooStatus, ResetStatusBytes, RssiStatus, TelemetryPeriods},
    },
};

use crate::model::{RadioRole, Subsystem};

/// Encoding used for fixed-length NMP byte fields.
#[derive(Enum, Copy, Clone, Debug, Eq, PartialEq)]
pub enum NmpDataFormat {
    /// Use the UTF-8 bytes of the supplied GraphQL string.
    #[graphql(name = "TEXT")]
    Text,
    /// Decode hexadecimal bytes. Spaces, dashes, underscores, and colons are ignored.
    #[graphql(name = "HEX")]
    Hex,
}

/// NXTRX4 RF role stored in NMP configuration.
#[derive(Enum, Copy, Clone, Debug, Eq, PartialEq)]
pub enum NmpRadioLinkType {
    Uplink,
    Downlink,
}

/// One route-table entry supplied to `radioNmpSetRoutes`.
#[derive(InputObject)]
pub struct NmpRouteInput {
    pub csp_address: u8,
    pub destination_interface: u8,
    pub next_hop: u8,
}

/// One entry returned from the radio's 32-entry CSP route table.
#[derive(SimpleObject)]
pub struct NmpRouteEntry {
    pub csp_address: u8,
    pub destination_interface: u8,
    pub next_hop: u8,
}

/// A byte field represented both losslessly as hex and conveniently as text.
#[derive(SimpleObject)]
pub struct NmpByteValue {
    pub text: String,
    pub hex: String,
}

#[derive(SimpleObject)]
pub struct NmpFsooStatus {
    pub inhibit_enabled: bool,
    pub timer_enabled: bool,
    pub period_ms: u64,
    pub time_left_ms: u64,
    pub backup_timer_enabled: bool,
    pub backup_uptime_to_fire_s: u32,
}

#[derive(SimpleObject)]
pub struct NmpRssiStatus {
    pub rssi_rx_immediate: u16,
    pub rssi_rx_avg: f64,
    pub rssi_rx_max: f64,
    pub rssi_background_immediate: u16,
    pub rssi_background_avg: f64,
    pub rssi_background_max: f64,
}

#[derive(SimpleObject)]
pub struct NmpResetStatusBytes {
    pub persistent: u8,
    pub last: u8,
    pub persistent_hex: String,
    pub last_hex: String,
}

#[derive(SimpleObject)]
pub struct NmpTelemetryPeriods {
    pub ax25_system_status_period_ms: u32,
    pub morse_task_period_ms: u64,
}

#[derive(SimpleObject)]
pub struct NmpFirmwareCrc {
    pub firmware_crc32: u32,
    pub constants_crc32: u32,
    pub firmware_crc32_hex: String,
    pub constants_crc32_hex: String,
}

#[derive(SimpleObject)]
pub struct NmpConfig1 {
    pub interface: NmpInterfaceConfig,
    pub radio: NmpRadioConfig,
    pub ax25: NmpAx25Config,
    pub digipeater: NmpDigipeaterConfig,
    pub morse: NmpMorseConfig,
    pub adc: NmpAdcConfig,
}

#[derive(SimpleObject)]
pub struct NmpInterfaceConfig {
    pub csp_interface_tx_timeout: u16,
    #[graphql(name = "i2c0RxToTxBusRecoveryDelay")]
    pub i2c0_rx_to_tx_bus_recovery_delay: u16,
    #[graphql(name = "i2c2RxToTxBusRecoveryDelay")]
    pub i2c2_rx_to_tx_bus_recovery_delay: u16,
    #[graphql(name = "i2cTxTimeout")]
    pub i2c_tx_timeout: u16,
    pub rs485_rx_to_tx_bus_recovery_delay: u16,
    pub rs485_tx_timeout: u16,
    pub adf_digital_tx_timeout: u32,
    pub adf_morse_tx_timeout: u32,
    pub ground_segment_tx_rx_reconfig_delay: u16,
}

#[derive(SimpleObject)]
pub struct NmpRadioConfig {
    pub frequency_hz: u32,
    pub lowest_configurable_frequency_hz: u32,
    pub highest_configurable_frequency_hz: u32,
    pub link_type: NmpRadioLinkType,
    pub hdlc_preamble_size: u16,
    pub tx_enabled: bool,
    pub normal_output_power: bool,
    pub first_start_on_orbit_inhibit_period_ms: u64,
    pub communication_check_reset_period_hours: u16,
}

#[derive(SimpleObject)]
pub struct NmpAx25Config {
    pub callsign: NmpByteValue,
    pub ssid: u8,
    pub system_status_timer_period_ms: u32,
    pub external_beacon_inhibit_period_ms: u64,
}

#[derive(SimpleObject)]
pub struct NmpDigipeaterConfig {
    pub enabled: bool,
    pub message_retransmission_count: u16,
    pub message_retransmission_interval_ms: u64,
    pub needronix_enabled: bool,
    pub customer_enabled: bool,
}

#[derive(SimpleObject)]
pub struct NmpMorseConfig {
    pub task_period_after_startup_ms: u64,
    pub task_period_ms: u64,
    pub inhibit_period_after_startup_ms: u64,
    pub inhibit_period_ms: u64,
    pub task_tx_attempt_timeout_ms: u64,
    pub custom_ident: NmpByteValue,
    pub max_string_length: u16,
    pub custom_message: NmpByteValue,
}

#[derive(SimpleObject)]
pub struct NmpAdcConfig {
    pub rssi_new_sample_contribution_ratio: u8,
}

#[derive(SimpleObject)]
pub struct NmpConfig2 {
    pub csp_address: u8,
    pub route_table: Vec<NmpRouteEntry>,
    pub routing_from_rs485_enabled: bool,
    pub identification: NmpByteValue,
}

impl Subsystem {
    pub fn radio_nmp_get_csp_address(&self, role: RadioRole, key: u32) -> Result<u8, String> {
        self.nmp(role, "get CSP address", |radio| {
            radio.nmp_get_csp_address(key)
        })
    }

    pub fn radio_nmp_get_route_table(
        &self,
        role: RadioRole,
        key: u32,
    ) -> Result<Vec<NmpRouteEntry>, String> {
        self.nmp(role, "get route table", |radio| {
            radio.nmp_get_route_table(key)
        })
        .map(route_table)
    }

    pub fn radio_nmp_get_routing_from_rs485(
        &self,
        role: RadioRole,
        key: u32,
    ) -> Result<bool, String> {
        self.nmp(role, "get routing from RS-485", |radio| {
            radio.nmp_get_routing_from_rs485(key)
        })
    }

    pub fn radio_nmp_get_frequency(&self, role: RadioRole, key: u32) -> Result<u32, String> {
        self.nmp(role, "get frequency", |radio| radio.nmp_get_frequency(key))
    }

    pub fn radio_nmp_get_link_type(
        &self,
        role: RadioRole,
        key: u32,
    ) -> Result<NmpRadioLinkType, String> {
        self.nmp(role, "get link type", |radio| radio.nmp_get_link_type(key))
            .map(Into::into)
    }

    pub fn radio_nmp_get_tx_enable(&self, role: RadioRole, key: u32) -> Result<bool, String> {
        self.nmp(role, "get TX enable", |radio| radio.nmp_get_tx_enable(key))
    }

    pub fn radio_nmp_get_preamble_size(&self, role: RadioRole, key: u32) -> Result<u16, String> {
        self.nmp(role, "get preamble size", |radio| {
            radio.nmp_get_preamble_size(key)
        })
    }

    pub fn radio_nmp_get_normal_power(&self, role: RadioRole, key: u32) -> Result<bool, String> {
        self.nmp(role, "get normal power", |radio| {
            radio.nmp_get_normal_power(key)
        })
    }

    pub fn radio_nmp_get_gs_rx_tx_delay(&self, role: RadioRole, key: u32) -> Result<u16, String> {
        self.nmp(role, "get ground-segment RX/TX delay", |radio| {
            radio.nmp_get_gs_rx_tx_delay(key)
        })
    }

    pub fn radio_nmp_get_digipeater_enable(
        &self,
        role: RadioRole,
        key: u32,
    ) -> Result<bool, String> {
        self.nmp(role, "get digipeater enable", |radio| {
            radio.nmp_get_digipeater_enable(key)
        })
    }

    pub fn radio_nmp_get_callsign(
        &self,
        role: RadioRole,
        key: u32,
    ) -> Result<NmpByteValue, String> {
        self.nmp(role, "get callsign", |radio| radio.nmp_get_callsign(key))
            .map(|value| NmpByteValue::new(&value))
    }

    pub fn radio_nmp_get_morse_custom_message(
        &self,
        role: RadioRole,
        key: u32,
    ) -> Result<NmpByteValue, String> {
        self.nmp(role, "get Morse custom message", |radio| {
            radio.nmp_get_morse_custom_message(key)
        })
        .map(|value| NmpByteValue::new(&value))
    }

    pub fn radio_nmp_get_morse_custom_ident(
        &self,
        role: RadioRole,
        key: u32,
    ) -> Result<NmpByteValue, String> {
        self.nmp(role, "get Morse custom identification", |radio| {
            radio.nmp_get_morse_custom_ident(key)
        })
        .map(|value| NmpByteValue::new(&value))
    }

    pub fn radio_nmp_get_fsoo(&self, role: RadioRole, key: u32) -> Result<NmpFsooStatus, String> {
        self.nmp(role, "get FSOO", |radio| radio.nmp_get_fsoo(key))
            .map(Into::into)
    }

    pub fn radio_nmp_get_hk_rssi(
        &self,
        role: RadioRole,
        key: u32,
    ) -> Result<NmpRssiStatus, String> {
        self.nmp(role, "get housekeeping RSSI", |radio| {
            radio.nmp_get_hk_rssi(key)
        })
        .map(Into::into)
    }

    pub fn radio_nmp_get_rssi_contribution_ratio(
        &self,
        role: RadioRole,
        key: u32,
    ) -> Result<u8, String> {
        self.nmp(role, "get RSSI contribution ratio", |radio| {
            radio.nmp_get_rssi_contribution_ratio(key)
        })
    }

    pub fn radio_nmp_get_config1(&self, role: RadioRole, key: u32) -> Result<NmpConfig1, String> {
        self.nmp(role, "get Config 1", |radio| radio.nmp_get_config1(key))
            .map(Into::into)
    }

    pub fn radio_nmp_get_config2(&self, role: RadioRole, key: u32) -> Result<NmpConfig2, String> {
        self.nmp(role, "get Config 2", |radio| radio.nmp_get_config2(key))
            .map(Into::into)
    }

    pub fn radio_nmp_get_check_comm_reset_period(
        &self,
        role: RadioRole,
        key: u32,
    ) -> Result<u16, String> {
        self.nmp(role, "get communication-check reset period", |radio| {
            radio.nmp_get_check_comm_reset_period(key)
        })
    }

    pub fn radio_nmp_get_system_status_and_morse_period(
        &self,
        role: RadioRole,
        key: u32,
    ) -> Result<NmpTelemetryPeriods, String> {
        self.nmp(role, "get system-status and Morse periods", |radio| {
            radio.nmp_get_system_status_and_morse_period(key)
        })
        .map(Into::into)
    }

    pub fn radio_nmp_get_fw_crc(
        &self,
        role: RadioRole,
        key: u32,
    ) -> Result<NmpFirmwareCrc, String> {
        self.nmp(role, "get firmware CRC", |radio| radio.nmp_get_fw_crc(key))
            .map(Into::into)
    }

    pub fn radio_nmp_get_hostname(
        &self,
        role: RadioRole,
        key: u32,
    ) -> Result<NmpByteValue, String> {
        self.nmp(role, "get hostname", |radio| radio.nmp_get_hostname(key))
            .map(|value| NmpByteValue::new(&value))
    }

    pub fn radio_nmp_unlock(&self, role: RadioRole, key: u32) -> Result<bool, String> {
        self.nmp(role, "unlock", |radio| radio.nmp_unlock(key))?;
        Ok(true)
    }

    pub fn radio_nmp_set_csp_address(
        &self,
        role: RadioRole,
        key: u32,
        csp_address: u8,
    ) -> Result<u8, String> {
        self.nmp(role, "set CSP address", |radio| {
            radio.nmp_set_csp_address(key, csp_address)
        })
    }

    pub fn radio_nmp_clear_route_table(
        &self,
        role: RadioRole,
        key: u32,
    ) -> Result<Vec<NmpRouteEntry>, String> {
        self.nmp(role, "clear route table", |radio| {
            radio.nmp_clear_route_table(key)
        })
        .map(route_table)
    }

    pub fn radio_nmp_set_routes(
        &self,
        role: RadioRole,
        key: u32,
        routes: Vec<NmpRouteInput>,
    ) -> Result<Vec<NmpRouteEntry>, String> {
        let routes: Vec<RouteEntry> = routes.into_iter().map(Into::into).collect();
        self.nmp(role, "set routes", |radio| {
            radio.nmp_set_routes(key, &routes)
        })
        .map(route_table)
    }

    pub fn radio_nmp_set_routing_from_rs485(
        &self,
        role: RadioRole,
        key: u32,
        enabled: bool,
    ) -> Result<bool, String> {
        self.nmp(role, "set routing from RS-485", |radio| {
            radio.nmp_set_routing_from_rs485(key, enabled)
        })
    }

    pub fn radio_nmp_set_frequency(
        &self,
        role: RadioRole,
        key: u32,
        frequency_hz: u32,
    ) -> Result<u32, String> {
        self.nmp(role, "set frequency", |radio| {
            radio.nmp_set_frequency(key, frequency_hz)
        })
    }

    pub fn radio_nmp_set_link_type(
        &self,
        role: RadioRole,
        key: u32,
        link_type: NmpRadioLinkType,
    ) -> Result<NmpRadioLinkType, String> {
        self.nmp(role, "set link type", |radio| {
            radio.nmp_set_link_type(key, link_type.into())
        })
        .map(Into::into)
    }

    pub fn radio_nmp_set_tx_enable(
        &self,
        role: RadioRole,
        key: u32,
        enabled: bool,
    ) -> Result<bool, String> {
        self.nmp(role, "set TX enable", |radio| {
            radio.nmp_set_tx_enable(key, enabled)
        })
    }

    pub fn radio_nmp_set_preamble_size(
        &self,
        role: RadioRole,
        key: u32,
        size: u16,
    ) -> Result<u16, String> {
        self.nmp(role, "set preamble size", |radio| {
            radio.nmp_set_preamble_size(key, size)
        })
    }

    pub fn radio_nmp_set_normal_power(
        &self,
        role: RadioRole,
        key: u32,
        normal_power: bool,
    ) -> Result<bool, String> {
        self.nmp(role, "set normal power", |radio| {
            radio.nmp_set_normal_power(key, normal_power)
        })
    }

    pub fn radio_nmp_set_gs_rx_tx_delay(
        &self,
        role: RadioRole,
        key: u32,
        delay_ms: u16,
    ) -> Result<u16, String> {
        self.nmp(role, "set ground-segment RX/TX delay", |radio| {
            radio.nmp_set_gs_rx_tx_delay(key, delay_ms)
        })
    }

    pub fn radio_nmp_set_digipeater_enable(
        &self,
        role: RadioRole,
        key: u32,
        enabled: bool,
    ) -> Result<bool, String> {
        self.nmp(role, "set digipeater enable", |radio| {
            radio.nmp_set_digipeater_enable(key, enabled)
        })
    }

    pub fn radio_nmp_set_callsign(
        &self,
        role: RadioRole,
        key: u32,
        callsign: String,
        format: Option<NmpDataFormat>,
    ) -> Result<NmpByteValue, String> {
        let callsign = fixed_bytes::<6>("callsign", &callsign, format)?;
        self.nmp(role, "set callsign", |radio| {
            radio.nmp_set_callsign(key, callsign)
        })
        .map(|value| NmpByteValue::new(&value))
    }

    pub fn radio_nmp_set_morse_custom_message(
        &self,
        role: RadioRole,
        key: u32,
        message: String,
    ) -> Result<NmpByteValue, String> {
        self.nmp(role, "set Morse custom message", |radio| {
            radio.nmp_set_morse_custom_message(key, &message)
        })
        .map(|value| NmpByteValue::new(&value))
    }

    pub fn radio_nmp_set_morse_custom_ident(
        &self,
        role: RadioRole,
        key: u32,
        ident: String,
        format: Option<NmpDataFormat>,
    ) -> Result<NmpByteValue, String> {
        let ident = fixed_bytes::<4>("Morse custom identification", &ident, format)?;
        self.nmp(role, "set Morse custom identification", |radio| {
            radio.nmp_set_morse_custom_ident(key, ident)
        })
        .map(|value| NmpByteValue::new(&value))
    }

    pub fn radio_nmp_set_user_key(
        &self,
        role: RadioRole,
        key: u32,
        new_user_key: u32,
    ) -> Result<bool, String> {
        self.nmp(role, "set user key", |radio| {
            radio.nmp_set_user_key(key, new_user_key)
        })?;
        Ok(true)
    }

    pub fn radio_nmp_set_superuser_key(
        &self,
        role: RadioRole,
        key: u32,
        new_superuser_key: u32,
    ) -> Result<bool, String> {
        self.nmp(role, "set superuser key", |radio| {
            radio.nmp_set_superuser_key(key, new_superuser_key)
        })?;
        Ok(true)
    }

    pub fn radio_nmp_set_itu_key(
        &self,
        role: RadioRole,
        key: u32,
        itu_key: String,
        format: Option<NmpDataFormat>,
    ) -> Result<bool, String> {
        let itu_key = fixed_bytes::<5>("ITU key", &itu_key, format)?;
        self.nmp(role, "set ITU key", |radio| {
            radio.nmp_set_itu_key(key, itu_key)
        })?;
        Ok(true)
    }

    pub fn radio_nmp_copy_factory_to_active(
        &self,
        role: RadioRole,
        key: u32,
    ) -> Result<bool, String> {
        self.nmp(role, "copy factory profile to active", |radio| {
            radio.nmp_copy_factory_to_active(key)
        })?;
        Ok(true)
    }

    pub fn radio_nmp_copy_active_to_factory(
        &self,
        role: RadioRole,
        key: u32,
    ) -> Result<bool, String> {
        self.nmp(role, "copy active profile to factory", |radio| {
            radio.nmp_copy_active_to_factory(key)
        })?;
        Ok(true)
    }

    pub fn radio_nmp_generate_reset_signal(
        &self,
        role: RadioRole,
        key: u32,
    ) -> Result<bool, String> {
        self.nmp(role, "generate reset signal", |radio| {
            radio.nmp_generate_reset_signal(key)
        })?;
        Ok(true)
    }

    pub fn radio_nmp_set_fsoo(
        &self,
        role: RadioRole,
        key: u32,
        inhibit: bool,
        period_ms: u64,
    ) -> Result<NmpFsooStatus, String> {
        self.nmp(role, "set FSOO", |radio| {
            radio.nmp_set_fsoo(key, inhibit, period_ms)
        })
        .map(Into::into)
    }

    pub fn radio_nmp_reset_status_bytes(
        &self,
        role: RadioRole,
        key: u32,
    ) -> Result<NmpResetStatusBytes, String> {
        self.nmp(role, "reset status bytes", |radio| {
            radio.nmp_reset_status_bytes(key)
        })
        .map(Into::into)
    }

    pub fn radio_nmp_set_rssi_contribution_ratio(
        &self,
        role: RadioRole,
        key: u32,
        ratio: u8,
    ) -> Result<u8, String> {
        self.nmp(role, "set RSSI contribution ratio", |radio| {
            radio.nmp_set_rssi_contribution_ratio(key, ratio)
        })
    }

    pub fn radio_nmp_set_check_comm_reset_period(
        &self,
        role: RadioRole,
        key: u32,
        hours: u16,
    ) -> Result<u16, String> {
        self.nmp(role, "set communication-check reset period", |radio| {
            radio.nmp_set_check_comm_reset_period(key, hours)
        })
    }

    pub fn radio_nmp_set_system_status_and_morse_period(
        &self,
        role: RadioRole,
        key: u32,
        ax25_system_status_period_ms: u32,
        morse_task_period_ms: u64,
    ) -> Result<NmpTelemetryPeriods, String> {
        self.nmp(role, "set system-status and Morse periods", |radio| {
            radio.nmp_set_system_status_and_morse_period(
                key,
                ax25_system_status_period_ms,
                morse_task_period_ms,
            )
        })
        .map(Into::into)
    }

    pub fn radio_nmp_set_hostname_user_part(
        &self,
        role: RadioRole,
        key: u32,
        user_part: String,
        format: Option<NmpDataFormat>,
    ) -> Result<NmpByteValue, String> {
        let user_part = fixed_bytes::<6>("hostname user part", &user_part, format)?;
        self.nmp(role, "set hostname user part", |radio| {
            radio.nmp_set_hostname_user_part(key, user_part)
        })
        .map(|value| NmpByteValue::new(&value))
    }

    fn nmp<T>(
        &self,
        role: RadioRole,
        command: &str,
        action: impl FnOnce(&Nxtrx4) -> nxtrx4_api::NxResult<T>,
    ) -> Result<T, String> {
        self.with_radio(role, |radio, _| {
            action(radio).map_err(|err| format!("NMP {command} failed: {err}"))
        })
    }
}

impl From<NmpRouteInput> for RouteEntry {
    fn from(value: NmpRouteInput) -> Self {
        Self {
            csp_adr: value.csp_address,
            dst_intf: value.destination_interface,
            next_hop: value.next_hop,
        }
    }
}

impl From<RouteEntry> for NmpRouteEntry {
    fn from(value: RouteEntry) -> Self {
        Self {
            csp_address: value.csp_adr,
            destination_interface: value.dst_intf,
            next_hop: value.next_hop,
        }
    }
}

impl From<RadioLinkType> for NmpRadioLinkType {
    fn from(value: RadioLinkType) -> Self {
        match value {
            RadioLinkType::Uplink => Self::Uplink,
            RadioLinkType::Downlink => Self::Downlink,
        }
    }
}

impl From<NmpRadioLinkType> for RadioLinkType {
    fn from(value: NmpRadioLinkType) -> Self {
        match value {
            NmpRadioLinkType::Uplink => Self::Uplink,
            NmpRadioLinkType::Downlink => Self::Downlink,
        }
    }
}

impl From<FsooStatus> for NmpFsooStatus {
    fn from(value: FsooStatus) -> Self {
        Self {
            inhibit_enabled: value.inhibit_enabled,
            timer_enabled: value.timer_enabled,
            period_ms: value.period_ms,
            time_left_ms: value.time_left_ms,
            backup_timer_enabled: value.backup_timer_enabled,
            backup_uptime_to_fire_s: value.backup_uptime_to_fire_s,
        }
    }
}

impl From<RssiStatus> for NmpRssiStatus {
    fn from(value: RssiStatus) -> Self {
        Self {
            rssi_rx_immediate: value.rssi_rx_immediate,
            rssi_rx_avg: f64::from(value.rssi_rx_avg),
            rssi_rx_max: f64::from(value.rssi_rx_max),
            rssi_background_immediate: value.rssi_background_immediate,
            rssi_background_avg: f64::from(value.rssi_background_avg),
            rssi_background_max: f64::from(value.rssi_background_max),
        }
    }
}

impl From<ResetStatusBytes> for NmpResetStatusBytes {
    fn from(value: ResetStatusBytes) -> Self {
        Self {
            persistent: value.persistent,
            last: value.last,
            persistent_hex: format!("{:02X}", value.persistent),
            last_hex: format!("{:02X}", value.last),
        }
    }
}

impl From<TelemetryPeriods> for NmpTelemetryPeriods {
    fn from(value: TelemetryPeriods) -> Self {
        Self {
            ax25_system_status_period_ms: value.ax25_system_status_period_ms,
            morse_task_period_ms: value.morse_task_period_ms,
        }
    }
}

impl From<FirmwareCrc> for NmpFirmwareCrc {
    fn from(value: FirmwareCrc) -> Self {
        Self {
            firmware_crc32: value.fw_crc32,
            constants_crc32: value.const_crc32,
            firmware_crc32_hex: format!("{:08X}", value.fw_crc32),
            constants_crc32_hex: format!("{:08X}", value.const_crc32),
        }
    }
}

impl From<Config1> for NmpConfig1 {
    fn from(value: Config1) -> Self {
        Self {
            interface: value.interface.into(),
            radio: value.radio.into(),
            ax25: value.ax25.into(),
            digipeater: value.digipeater.into(),
            morse: value.morse.into(),
            adc: value.adc.into(),
        }
    }
}

impl From<InterfaceConfig> for NmpInterfaceConfig {
    fn from(value: InterfaceConfig) -> Self {
        Self {
            csp_interface_tx_timeout: value.csp_intf_tx_timeout,
            i2c0_rx_to_tx_bus_recovery_delay: value.i2c0_rx_to_tx_bus_recovery_delay,
            i2c2_rx_to_tx_bus_recovery_delay: value.i2c2_rx_to_tx_bus_recovery_delay,
            i2c_tx_timeout: value.i2c_tx_timeout,
            rs485_rx_to_tx_bus_recovery_delay: value.rs485_rx_to_tx_bus_recovery_delay,
            rs485_tx_timeout: value.rs485_tx_timeout,
            adf_digital_tx_timeout: value.adf_digital_tx_timeout,
            adf_morse_tx_timeout: value.adf_morse_tx_timeout,
            ground_segment_tx_rx_reconfig_delay: value.ground_segment_tx_rx_reconfig_delay,
        }
    }
}

impl From<RadioConfig> for NmpRadioConfig {
    fn from(value: RadioConfig) -> Self {
        Self {
            frequency_hz: value.frequency,
            lowest_configurable_frequency_hz: value.lowest_configurable_freq,
            highest_configurable_frequency_hz: value.highest_configurable_freq,
            link_type: value.link_type.into(),
            hdlc_preamble_size: value.hdlc_preamble_size,
            tx_enabled: value.tx_enabled != 0,
            normal_output_power: value.normal_output_power_bool != 0,
            first_start_on_orbit_inhibit_period_ms: value.first_start_on_orbit_inhibit_period,
            communication_check_reset_period_hours: value.comm_check_reset_period,
        }
    }
}

impl From<Ax25Config> for NmpAx25Config {
    fn from(value: Ax25Config) -> Self {
        Self {
            callsign: NmpByteValue::new(&value.callsign),
            ssid: value.ssid,
            system_status_timer_period_ms: value.system_status_timer_period,
            external_beacon_inhibit_period_ms: value.external_beacon_inhibit_period,
        }
    }
}

impl From<DigipeaterConfig> for NmpDigipeaterConfig {
    fn from(value: DigipeaterConfig) -> Self {
        Self {
            enabled: value.enabled != 0,
            message_retransmission_count: value.message_retransmission_count,
            message_retransmission_interval_ms: value.message_retransmission_interval,
            needronix_enabled: value.dnxd_needronix_enabled != 0,
            customer_enabled: value.dnxd_customer_enabled != 0,
        }
    }
}

impl From<MorseConfig> for NmpMorseConfig {
    fn from(value: MorseConfig) -> Self {
        Self {
            task_period_after_startup_ms: value.my_task_period_after_startup,
            task_period_ms: value.my_task_period,
            inhibit_period_after_startup_ms: value.inhibit_period_after_startup,
            inhibit_period_ms: value.inhibit_period,
            task_tx_attempt_timeout_ms: value.task_tx_attempt_timeout,
            custom_ident: NmpByteValue::new(&value.custom_ident),
            max_string_length: value.max_string_length,
            custom_message: NmpByteValue::new(&value.morse_custom_msg),
        }
    }
}

impl From<AdcConfig> for NmpAdcConfig {
    fn from(value: AdcConfig) -> Self {
        Self {
            rssi_new_sample_contribution_ratio: value.rssi_new_sample_contribution_ratio,
        }
    }
}

impl From<CspConfig> for NmpConfig2 {
    fn from(value: CspConfig) -> Self {
        Self {
            csp_address: value.adr,
            route_table: route_table(value.route_table),
            routing_from_rs485_enabled: value.routing_from_rs485_enabled != 0,
            identification: NmpByteValue::new(&value.identification),
        }
    }
}

impl NmpByteValue {
    fn new(bytes: &[u8]) -> Self {
        let text_end = bytes
            .iter()
            .position(|byte| *byte == 0)
            .unwrap_or(bytes.len());
        Self {
            text: String::from_utf8_lossy(&bytes[..text_end])
                .trim_end()
                .to_string(),
            hex: bytes_to_hex(bytes),
        }
    }
}

fn route_table<const N: usize>(routes: [RouteEntry; N]) -> Vec<NmpRouteEntry> {
    routes.into_iter().map(Into::into).collect()
}

fn fixed_bytes<const N: usize>(
    name: &str,
    data: &str,
    format: Option<NmpDataFormat>,
) -> Result<[u8; N], String> {
    let bytes = match format.unwrap_or(NmpDataFormat::Text) {
        NmpDataFormat::Text => data.as_bytes().to_vec(),
        NmpDataFormat::Hex => decode_hex(data)?,
    };

    bytes
        .try_into()
        .map_err(|value: Vec<u8>| format!("{name} must be exactly {N} bytes, got {}", value.len()))
}

fn decode_hex(data: &str) -> Result<Vec<u8>, String> {
    let mut nibbles = Vec::new();
    for byte in data.bytes() {
        match byte {
            b' ' | b'\n' | b'\r' | b'\t' | b'_' | b'-' | b':' => {}
            b'0'..=b'9' => nibbles.push(byte - b'0'),
            b'a'..=b'f' => nibbles.push(byte - b'a' + 10),
            b'A'..=b'F' => nibbles.push(byte - b'A' + 10),
            _ => return Err(format!("invalid hex character `{}`", byte as char)),
        }
    }
    if nibbles.len() % 2 != 0 {
        return Err("hex data must contain an even number of digits".to_string());
    }

    Ok(nibbles
        .chunks_exact(2)
        .map(|pair| (pair[0] << 4) | pair[1])
        .collect())
}

fn bytes_to_hex(bytes: &[u8]) -> String {
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        let _ = write!(output, "{byte:02X}");
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_text_and_hex_fields_are_lossless() {
        assert_eq!(fixed_bytes::<4>("ident", "OBC1", None).unwrap(), *b"OBC1");
        assert_eq!(
            fixed_bytes::<4>("ident", "4f 42-43:31", Some(NmpDataFormat::Hex)).unwrap(),
            *b"OBC1"
        );
    }

    #[test]
    fn fixed_fields_reject_wrong_lengths() {
        assert!(fixed_bytes::<4>("ident", "OBC", None).is_err());
        assert!(fixed_bytes::<4>("ident", "ABC", Some(NmpDataFormat::Hex)).is_err());
    }

    #[test]
    fn byte_values_preserve_hex_and_trim_text_padding() {
        let value = NmpByteValue::new(b"SAT1  \0");
        assert_eq!(value.text, "SAT1");
        assert_eq!(value.hex, "53415431202000");
    }
}
