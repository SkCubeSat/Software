//! GraphQL-facing NXTRX4 radio control types and command wrappers.
//!
//! The comms service owns the CSP router, I2C interfaces, and slave receive
//! backend, so these helpers expose the `nxtrx4-api` command surface through
//! the existing service rather than opening another hardware path.

use std::{fmt::Write, time::Duration};

use async_graphql::{Enum, SimpleObject};
use nxtrx4_api::{
    cmp::{CmpIdentResponse, CmpIfcResponse, NxtrxInterface},
    nx_services::{
        AX25_MESSAGE_MAX_BYTES, AdcData, SendCompressedMorseRequest, SriStatus, SystemInfo,
        SystemStats,
    },
    standard::PING_MAX_DATA_BYTES,
};

use crate::model::{RadioRole, Subsystem};

/// NXTRX4 internal interface name for CMP interface-stat queries.
#[derive(Enum, Copy, Clone, Debug, Eq, PartialEq)]
pub enum RadioInterface {
    /// RF modem interface counters.
    #[graphql(name = "RADIO")]
    Radio,
    /// CSP stack interface counters.
    #[graphql(name = "CSP")]
    Csp,
    /// First I2C interface counters.
    #[graphql(name = "I2C0")]
    I2c0,
    /// Second I2C interface counters.
    #[graphql(name = "I2C2")]
    I2c2,
    /// RS-485 interface counters.
    #[graphql(name = "RS485")]
    Rs485,
}

/// Input encoding for mutation payload strings.
#[derive(Enum, Copy, Clone, Debug, Eq, PartialEq)]
pub enum RadioPayloadFormat {
    /// Send the UTF-8 bytes of the GraphQL string.
    #[graphql(name = "TEXT")]
    Text,
    /// Decode ASCII hex digits; spaces, dashes, underscores, and colons are ignored.
    #[graphql(name = "HEX")]
    Hex,
}

/// Result of a CSP ping against one configured NXTRX4 radio.
#[derive(SimpleObject)]
pub struct RadioPing {
    /// Radio selected by the GraphQL `role` argument.
    pub role: RadioRole,
    /// Number of ping payload bytes requested.
    pub payload_size: i32,
    /// Measured round-trip time in milliseconds.
    pub round_trip_ms: f64,
}

/// Uptime returned by the radio's standard CSP uptime service.
#[derive(SimpleObject)]
pub struct RadioUptime {
    /// Radio selected by the GraphQL `role` argument.
    pub role: RadioRole,
    /// Seconds since the radio last powered up or reset.
    pub seconds: i32,
}

/// Free-space status of the radio transmit input buffer.
#[derive(SimpleObject)]
pub struct RadioStatus {
    /// Radio selected by the GraphQL `role` argument.
    pub role: RadioRole,
    /// Big-endian `buffree` value returned by the NXTRX4.
    pub buffer_free: i32,
}

/// Human-readable identity strings returned by the CSP Management Protocol.
#[derive(SimpleObject)]
pub struct RadioIdent {
    /// Radio selected by the GraphQL `role` argument.
    pub role: RadioRole,
    /// Configurable radio hostname.
    pub hostname: String,
    /// Needronix model string.
    pub model: String,
    /// Firmware or hardware revision string.
    pub revision: String,
    /// Build date string reported by the radio.
    pub build_date: String,
    /// Build time string reported by the radio.
    pub build_time: String,
}

/// Packet and error counters for one NXTRX4 internal interface.
#[derive(SimpleObject)]
pub struct RadioInterfaceStats {
    /// Radio selected by the GraphQL `role` argument.
    pub role: RadioRole,
    /// Interface requested through CMP.
    pub interface: RadioInterface,
    /// Interface name echoed by the radio.
    pub interface_name: String,
    /// Total transmitted packet count.
    pub tx_packet_count: i32,
    /// Total received packet count.
    pub rx_packet_count: i32,
    /// Packets locked in transmit.
    pub locked_in_tx_count: i32,
    /// Receive overrun counter.
    pub rx_overrun_count: i32,
    /// Transmit length error counter.
    pub tx_length_error: i32,
    /// Critical internal error counter.
    pub critical_internal_error: i32,
    /// Transmitted packet count since last reset.
    pub tx_packet_count_after_reset: i32,
    /// Received packet count since last reset.
    pub rx_packet_count_after_reset: i32,
}

/// Decoded response from the NXTRX4 Get System Statistics service.
#[derive(SimpleObject)]
pub struct RadioSystemStats {
    /// Radio selected by the GraphQL `role` argument.
    pub role: RadioRole,
    /// Protected-logbook system summary.
    pub system_info: RadioSystemInfo,
    /// Reset-cause and uptime counters.
    pub sri_status: RadioSriStatus,
    /// Real-time ADC and RF readings.
    pub adc_data: RadioAdcData,
}

/// Logbook-backed system summary from the radio.
#[derive(SimpleObject)]
pub struct RadioSystemInfo {
    /// CPU supply voltage in 10 mV units.
    pub cpu_voltage_10mv: i32,
    /// CPU temperature in Kelvin.
    pub cpu_temperature_kelvin: i32,
    /// Total reset counter.
    pub total_reset_count: i32,
}

/// Reset-cause and uptime counters from the radio SRI block.
#[derive(SimpleObject)]
pub struct RadioSriStatus {
    /// Watchdog reset counter.
    pub wdt_reset: i32,
    /// Brown-out reset counter.
    pub bor_reset: i32,
    /// Power-on reset counter.
    pub por_reset: i32,
    /// RST/NMI reset counter.
    pub rst_nmi_reset: i32,
    /// Low-power-mode exit counter.
    pub exit_from_lpm5: i32,
    /// Cumulative uptime counter.
    pub total_up_time: i32,
    /// Uptime since the last reset.
    pub up_time: i32,
    /// Raw two-byte status field encoded as uppercase hex.
    pub status_bytes_hex: String,
}

/// Real-time ADC and RF readings from the radio.
#[derive(SimpleObject)]
pub struct RadioAdcData {
    /// CPU temperature in Kelvin.
    pub cpu_temp_kelvin: i32,
    /// CPU supply voltage in 10 mV units.
    pub cpu_voltage_10mv: i32,
    /// External voltage reading in the radio's native units.
    pub ext_voltage: i32,
    /// Immediate receive RSSI reading.
    pub rssi_rx_immediate: i32,
    /// Average receive RSSI reading.
    pub rssi_rx_avg: f64,
    /// Maximum receive RSSI reading.
    pub rssi_rx_max: f64,
    /// Immediate background RSSI reading.
    pub rssi_background_immediate: i32,
    /// Average background RSSI reading.
    pub rssi_background_avg: f64,
    /// Maximum background RSSI reading.
    pub rssi_background_max: f64,
    /// Standing-wave-ratio reading.
    pub swr: i32,
    /// Raw NTC reading under the power amplifier.
    pub pa_ntc: i32,
}

/// Common response object for GraphQL mutations that send NXTRX4 commands.
#[derive(SimpleObject)]
pub struct RadioMutationResponse {
    /// Radio selected by the GraphQL `role` argument.
    pub role: RadioRole,
    /// Whether the command was accepted by the local API and radio service.
    pub success: bool,
    /// Human-readable command result.
    pub message: String,
    /// Two-byte verbal response decoded as text when the radio returns one.
    pub verbal_response_text: Option<String>,
    /// Two-byte verbal response encoded as uppercase hex when present.
    pub verbal_response_hex: Option<String>,
}

impl Subsystem {
    /// Pings the selected radio using the NXTRX4 standard CSP ping service.
    pub fn radio_ping(
        &self,
        role: RadioRole,
        payload_size: Option<i32>,
    ) -> Result<RadioPing, String> {
        let payload_size = payload_size.unwrap_or(0);
        if payload_size < 0 || payload_size as usize > PING_MAX_DATA_BYTES {
            return Err(format!(
                "payload_size must be in range 0..={PING_MAX_DATA_BYTES}"
            ));
        }

        self.with_radio(role, |radio, _| {
            let duration = radio
                .ping(payload_size as usize)
                .map_err(|err| format!("radio ping failed: {err}"))?;

            Ok(RadioPing {
                role,
                payload_size,
                round_trip_ms: duration_ms(duration),
            })
        })
    }

    /// Reads the selected radio's uptime in seconds.
    pub fn radio_uptime(&self, role: RadioRole) -> Result<RadioUptime, String> {
        self.with_radio(role, |radio, _| {
            let seconds = radio
                .get_uptime()
                .map_err(|err| format!("get_uptime failed: {err}"))?;

            Ok(RadioUptime {
                role,
                seconds: u32_to_i32(seconds),
            })
        })
    }

    /// Reads the selected radio's transmit-buffer free-space value.
    pub fn radio_status(&self, role: RadioRole) -> Result<RadioStatus, String> {
        self.with_radio(role, |radio, _| {
            let buffer_free = radio
                .get_radio_status()
                .map_err(|err| format!("get_radio_status failed: {err}"))?;

            Ok(RadioStatus { role, buffer_free })
        })
    }

    /// Reads CMP identity strings from the selected radio.
    pub fn radio_ident(&self, role: RadioRole) -> Result<RadioIdent, String> {
        self.with_radio(role, |radio, _| {
            let ident = radio
                .get_cmp_ident()
                .map_err(|err| format!("get_cmp_ident failed: {err}"))?;

            Ok(RadioIdent::from_response(role, ident))
        })
    }

    /// Reads CMP counters for one internal interface on the selected radio.
    pub fn radio_interface_stats(
        &self,
        role: RadioRole,
        interface: RadioInterface,
    ) -> Result<RadioInterfaceStats, String> {
        self.with_radio(role, |radio, _| {
            let stats = radio
                .get_cmp_ifc(interface.into())
                .map_err(|err| format!("get_cmp_ifc failed: {err}"))?;

            Ok(RadioInterfaceStats::from_response(role, interface, stats))
        })
    }

    /// Reads and decodes NXTRX4 system statistics from the selected radio.
    pub fn radio_system_stats(&self, role: RadioRole) -> Result<RadioSystemStats, String> {
        self.with_radio(role, |radio, _| {
            let stats = radio
                .get_system_stats()
                .map_err(|err| format!("get_system_stats failed: {err}"))?;

            Ok(RadioSystemStats::from_response(role, stats))
        })
    }

    /// Sends the selected radio's no-response reboot command.
    pub fn radio_reboot(&self, role: RadioRole) -> RadioMutationResponse {
        self.with_radio(role, |radio, _| {
            radio
                .reboot()
                .map_err(|err| format!("radio reboot failed: {err}"))?;

            Ok(RadioMutationResponse::ok(
                role,
                "reboot command sent; the radio does not return a response",
            ))
        })
        .unwrap_or_else(|err| RadioMutationResponse::failure(role, err))
    }

    /// Requests transmission of text as Morse code.
    ///
    /// `source_identification` must be exactly four ASCII bytes. The text is
    /// validated by `nxtrx4-api` for the radio's wire-format limits.
    pub fn radio_send_text_in_morse(
        &self,
        role: RadioRole,
        source_identification: String,
        text: String,
    ) -> RadioMutationResponse {
        let source_identification = match source_identification_bytes(&source_identification) {
            Ok(value) => value,
            Err(err) => return RadioMutationResponse::failure(role, err),
        };

        self.with_radio(role, |radio, _| {
            let response = radio
                .send_text_in_morse(source_identification, &text)
                .map_err(|err| format!("send_text_in_morse failed: {err}"))?;

            Ok(RadioMutationResponse::with_verbal_response(
                role,
                "text Morse command accepted",
                response.verbal_response,
            ))
        })
        .unwrap_or_else(|err| RadioMutationResponse::failure(role, err))
    }

    /// Requests transmission of compressed Morse code words.
    ///
    /// `source_identification` must be exactly four ASCII bytes, and each
    /// integer word must be non-negative so it can be packed as a `u32`.
    pub fn radio_send_compressed_morse(
        &self,
        role: RadioRole,
        source_identification: String,
        nums: [i32; 6],
    ) -> RadioMutationResponse {
        let source_identification = match source_identification_bytes(&source_identification) {
            Ok(value) => value,
            Err(err) => return RadioMutationResponse::failure(role, err),
        };
        let nums = match u32_array(nums) {
            Ok(value) => value,
            Err(err) => return RadioMutationResponse::failure(role, err),
        };
        let request = SendCompressedMorseRequest {
            morse_source_identification: source_identification,
            num1: nums[0],
            num2: nums[1],
            num3: nums[2],
            num4: nums[3],
            num5: nums[4],
            num6: nums[5],
        };

        self.with_radio(role, |radio, _| {
            let response = radio
                .send_compressed_morse(request)
                .map_err(|err| format!("send_compressed_morse failed: {err}"))?;

            Ok(RadioMutationResponse::with_verbal_response(
                role,
                "compressed Morse command accepted",
                response.verbal_response,
            ))
        })
        .unwrap_or_else(|err| RadioMutationResponse::failure(role, err))
    }

    /// Requests RF transmission of an AX.25 message through the selected radio.
    ///
    /// `TEXT` sends the string bytes directly. `HEX` decodes ASCII hex digits
    /// and ignores common separators before applying the NXTRX4 MTU limit.
    pub fn radio_send_ax25_message(
        &self,
        role: RadioRole,
        data: String,
        format: Option<RadioPayloadFormat>,
    ) -> RadioMutationResponse {
        let payload = match decode_payload(&data, format.unwrap_or(RadioPayloadFormat::Text)) {
            Ok(value) => value,
            Err(err) => return RadioMutationResponse::failure(role, err),
        };
        if payload.len() > AX25_MESSAGE_MAX_BYTES {
            return RadioMutationResponse::failure(
                role,
                format!(
                    "AX.25 payload was {} bytes, expected at most {AX25_MESSAGE_MAX_BYTES}",
                    payload.len()
                ),
            );
        }

        self.with_radio(role, |radio, _| {
            let response = radio
                .send_ax25_message(&payload)
                .map_err(|err| format!("send_ax25_message failed: {err}"))?;

            Ok(RadioMutationResponse::with_verbal_response(
                role,
                "AX.25 transmit request accepted",
                response.verbal_response,
            ))
        })
        .unwrap_or_else(|err| RadioMutationResponse::failure(role, err))
    }
}

impl From<RadioInterface> for NxtrxInterface {
    fn from(value: RadioInterface) -> Self {
        match value {
            RadioInterface::Radio => NxtrxInterface::Radio,
            RadioInterface::Csp => NxtrxInterface::Csp,
            RadioInterface::I2c0 => NxtrxInterface::I2c0,
            RadioInterface::I2c2 => NxtrxInterface::I2c2,
            RadioInterface::Rs485 => NxtrxInterface::Rs485,
        }
    }
}

impl RadioIdent {
    fn from_response(role: RadioRole, response: CmpIdentResponse) -> Self {
        Self {
            role,
            hostname: bytes_to_text(&response.hostname),
            model: bytes_to_text(&response.model),
            revision: bytes_to_text(&response.revision),
            build_date: bytes_to_text(&response.date),
            build_time: bytes_to_text(&response.time),
        }
    }
}

impl RadioInterfaceStats {
    fn from_response(role: RadioRole, interface: RadioInterface, response: CmpIfcResponse) -> Self {
        Self {
            role,
            interface,
            interface_name: bytes_to_text(&response.interface_name),
            tx_packet_count: u32_to_i32(response.tx_packet_count),
            rx_packet_count: u32_to_i32(response.rx_packet_count),
            locked_in_tx_count: u32_to_i32(response.locked_in_tx_count),
            rx_overrun_count: u32_to_i32(response.rx_overrun_count),
            tx_length_error: u32_to_i32(response.tx_length_error),
            critical_internal_error: u32_to_i32(response.critical_internal_error),
            tx_packet_count_after_reset: u32_to_i32(response.tx_packet_count_after_reset),
            rx_packet_count_after_reset: u32_to_i32(response.rx_packet_count_after_reset),
        }
    }
}

impl RadioSystemStats {
    fn from_response(role: RadioRole, response: SystemStats) -> Self {
        Self {
            role,
            system_info: RadioSystemInfo::from_response(response.system_info),
            sri_status: RadioSriStatus::from_response(response.sri_status),
            adc_data: RadioAdcData::from_response(response.adc_data),
        }
    }
}

impl RadioSystemInfo {
    fn from_response(response: SystemInfo) -> Self {
        Self {
            cpu_voltage_10mv: i32::from(response.cpu_voltage),
            cpu_temperature_kelvin: i32::from(response.cpu_temperature),
            total_reset_count: i32::from(response.total_reset_count),
        }
    }
}

impl RadioSriStatus {
    fn from_response(response: SriStatus) -> Self {
        Self {
            wdt_reset: u32_to_i32(response.wdt_reset),
            bor_reset: u32_to_i32(response.bor_reset),
            por_reset: u32_to_i32(response.por_reset),
            rst_nmi_reset: u32_to_i32(response.rst_nmi_reset),
            exit_from_lpm5: u32_to_i32(response.exit_from_lpm5),
            total_up_time: u32_to_i32(response.total_up_time),
            up_time: u32_to_i32(response.up_time),
            status_bytes_hex: bytes_to_hex(&response.status_byte),
        }
    }
}

impl RadioAdcData {
    fn from_response(response: AdcData) -> Self {
        Self {
            cpu_temp_kelvin: i32::from(response.cpu_temp),
            cpu_voltage_10mv: i32::from(response.cpu_voltage),
            ext_voltage: i32::from(response.ext_voltage),
            rssi_rx_immediate: i32::from(response.rssi_rx_immediate),
            rssi_rx_avg: f64::from(response.rssi_rx_avg),
            rssi_rx_max: f64::from(response.rssi_rx_max),
            rssi_background_immediate: i32::from(response.rssi_background_immediate),
            rssi_background_avg: f64::from(response.rssi_background_avg),
            rssi_background_max: f64::from(response.rssi_background_max),
            swr: i32::from(response.swr),
            pa_ntc: i32::from(response.pa_ntc),
        }
    }
}

impl RadioMutationResponse {
    fn ok(role: RadioRole, message: impl Into<String>) -> Self {
        Self {
            role,
            success: true,
            message: message.into(),
            verbal_response_text: None,
            verbal_response_hex: None,
        }
    }

    fn with_verbal_response(
        role: RadioRole,
        message: impl Into<String>,
        verbal_response: [u8; 2],
    ) -> Self {
        Self {
            role,
            success: true,
            message: message.into(),
            verbal_response_text: Some(bytes_to_text(&verbal_response)),
            verbal_response_hex: Some(bytes_to_hex(&verbal_response)),
        }
    }

    fn failure(role: RadioRole, message: impl Into<String>) -> Self {
        Self {
            role,
            success: false,
            message: message.into(),
            verbal_response_text: None,
            verbal_response_hex: None,
        }
    }
}

fn duration_ms(duration: Duration) -> f64 {
    duration.as_secs_f64() * 1_000.0
}

fn u32_to_i32(value: u32) -> i32 {
    i32::try_from(value).unwrap_or(i32::MAX)
}

fn u32_array(values: [i32; 6]) -> Result<[u32; 6], String> {
    Ok([
        non_negative_u32("num1", values[0])?,
        non_negative_u32("num2", values[1])?,
        non_negative_u32("num3", values[2])?,
        non_negative_u32("num4", values[3])?,
        non_negative_u32("num5", values[4])?,
        non_negative_u32("num6", values[5])?,
    ])
}

fn non_negative_u32(name: &str, value: i32) -> Result<u32, String> {
    u32::try_from(value).map_err(|_| format!("{name} must be >= 0"))
}

fn source_identification_bytes(value: &str) -> Result<[u8; 4], String> {
    if !value.is_ascii() {
        return Err("sourceIdentification must contain only ASCII characters".to_string());
    }
    let bytes = value.as_bytes();
    if bytes.len() != 4 {
        return Err(format!(
            "sourceIdentification must be exactly 4 bytes, got {}",
            bytes.len()
        ));
    }

    Ok(bytes.try_into().unwrap())
}

fn decode_payload(data: &str, format: RadioPayloadFormat) -> Result<Vec<u8>, String> {
    match format {
        RadioPayloadFormat::Text => Ok(data.as_bytes().to_vec()),
        RadioPayloadFormat::Hex => decode_hex(data),
    }
}

fn decode_hex(data: &str) -> Result<Vec<u8>, String> {
    let mut nibbles = Vec::new();
    for byte in data.bytes() {
        match byte {
            b' ' | b'\n' | b'\r' | b'\t' | b'_' | b'-' | b':' => {}
            byte => match hex_value(byte) {
                Some(value) => nibbles.push(value),
                None => {
                    return Err(format!("invalid hex character `{}`", byte as char));
                }
            },
        }
    }

    if nibbles.len() % 2 != 0 {
        return Err("hex payload must contain an even number of digits".to_string());
    }

    Ok(nibbles
        .chunks_exact(2)
        .map(|pair| (pair[0] << 4) | pair[1])
        .collect())
}

fn hex_value(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

fn bytes_to_text(bytes: &[u8]) -> String {
    let end = bytes
        .iter()
        .position(|byte| *byte == 0)
        .unwrap_or(bytes.len());
    String::from_utf8_lossy(&bytes[..end]).trim().to_string()
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
    fn parses_source_identification() {
        assert_eq!(source_identification_bytes("SAT1").unwrap(), *b"SAT1");
    }

    #[test]
    fn rejects_invalid_source_identification() {
        assert!(source_identification_bytes("SAT").is_err());
        assert!(source_identification_bytes("SAT12").is_err());
        assert!(source_identification_bytes("SÅT1").is_err());
    }

    #[test]
    fn decodes_hex_payload_with_common_separators() {
        assert_eq!(decode_hex("01 02-0A:ff").unwrap(), vec![1, 2, 10, 255]);
    }

    #[test]
    fn rejects_invalid_hex_payload() {
        assert!(decode_hex("0").is_err());
        assert!(decode_hex("00GG").is_err());
    }

    #[test]
    fn bytes_to_text_stops_at_first_nul_and_trims_padding() {
        assert_eq!(bytes_to_text(b"RADIO\0\0\0"), "RADIO");
        assert_eq!(bytes_to_text(b" OK "), "OK");
    }

    #[test]
    fn u32_to_i32_saturates() {
        assert_eq!(u32_to_i32(42), 42);
        assert_eq!(u32_to_i32(u32::MAX), i32::MAX);
    }
}
