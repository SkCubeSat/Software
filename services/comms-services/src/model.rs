use std::sync::{Arc, Mutex};

use async_graphql::{Enum, SimpleObject};
use kubos_comms::CommsTelemetry;
use nxtrx4_api::cmp::NxtrxInterface;

use crate::{config::ServiceSettings, nxtrx_comms::NxtrxComms};

#[derive(Clone)]
pub struct Subsystem {
    telemetry: Arc<Mutex<CommsTelemetry>>,
    comms: NxtrxComms,
    settings: ServiceSettings,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum RadioRole {
    Uplink,
    Downlink,
}

#[derive(SimpleObject)]
pub struct TelemetrySnapshot {
    pub packets_up: i32,
    pub packets_down: i32,
    pub failed_packets_up: i32,
    pub failed_packets_down: i32,
    pub errors: Vec<String>,
}

#[derive(SimpleObject)]
pub struct CommsHealth {
    pub uplink_node: i32,
    pub downlink_node: i32,
    pub ground_node: i32,
    pub uplink_packet_csp_port: i32,
    pub uplink_sfp_csp_port: i32,
    pub ground_packet_csp_port: i32,
    pub ground_sfp_csp_port: i32,
    pub max_packet_space_packet_bytes: i32,
    pub max_sfp_space_packet_bytes: i32,
    pub sfp_mtu: i32,
    pub sfp_use_rdp: bool,
    pub uplink_crypto: String,
}

#[derive(SimpleObject)]
pub struct RadioHealth {
    pub role: RadioRole,
    pub csp_node: i32,
    pub uptime_seconds: Option<i32>,
    pub radio_status: Option<i32>,
    pub radio_tx_packets: Option<i32>,
    pub radio_rx_packets: Option<i32>,
    pub radio_rx_overruns: Option<i32>,
    pub errors: Vec<String>,
}

impl Subsystem {
    pub fn new(
        telemetry: Arc<Mutex<CommsTelemetry>>,
        comms: NxtrxComms,
        settings: ServiceSettings,
    ) -> Self {
        Self {
            telemetry,
            comms,
            settings,
        }
    }

    pub fn telemetry(&self) -> Result<TelemetrySnapshot, String> {
        let telemetry = self
            .telemetry
            .lock()
            .map_err(|_| "failed to lock communications telemetry".to_string())?;

        Ok(TelemetrySnapshot {
            packets_up: telemetry.packets_up,
            packets_down: telemetry.packets_down,
            failed_packets_up: telemetry.failed_packets_up,
            failed_packets_down: telemetry.failed_packets_down,
            errors: telemetry.errors.clone(),
        })
    }

    pub fn health(&self) -> CommsHealth {
        CommsHealth {
            uplink_node: i32::from(self.settings.radios.uplink.csp_node),
            downlink_node: i32::from(self.settings.radios.downlink.csp_node),
            ground_node: i32::from(self.settings.csp.ground_node),
            uplink_packet_csp_port: i32::from(self.settings.csp.uplink_packet_csp_port),
            uplink_sfp_csp_port: i32::from(self.settings.csp.uplink_sfp_csp_port),
            ground_packet_csp_port: i32::from(self.settings.csp.ground_packet_csp_port),
            ground_sfp_csp_port: i32::from(self.settings.csp.ground_sfp_csp_port),
            max_packet_space_packet_bytes: self.settings.csp.max_frame_bytes.saturating_sub(4)
                as i32,
            max_sfp_space_packet_bytes: self.settings.csp.sfp_max_space_packet_bytes as i32,
            sfp_mtu: self.settings.csp.sfp_mtu as i32,
            sfp_use_rdp: self.settings.csp.sfp_use_rdp,
            uplink_crypto: "none".to_string(),
        }
    }

    pub fn radio_health(&self, role: RadioRole) -> RadioHealth {
        let (radio, config) = match role {
            RadioRole::Uplink => (&self.comms.uplink_radio, &self.settings.radios.uplink),
            RadioRole::Downlink => (&self.comms.downlink_radio, &self.settings.radios.downlink),
        };

        let mut errors = Vec::new();
        let uptime_seconds = match radio.get_uptime() {
            Ok(value) => Some(value as i32),
            Err(err) => {
                errors.push(format!("get_uptime: {err}"));
                None
            }
        };
        let radio_status = match radio.get_radio_status() {
            Ok(value) => Some(value),
            Err(err) => {
                errors.push(format!("get_radio_status: {err}"));
                None
            }
        };
        let interface = match radio.get_cmp_ifc(NxtrxInterface::Radio) {
            Ok(value) => Some(value),
            Err(err) => {
                errors.push(format!("get_cmp_ifc(RADIO): {err}"));
                None
            }
        };

        RadioHealth {
            role,
            csp_node: i32::from(config.csp_node),
            uptime_seconds,
            radio_status,
            radio_tx_packets: interface.map(|value| value.tx_packet_count as i32),
            radio_rx_packets: interface.map(|value| value.rx_packet_count as i32),
            radio_rx_overruns: interface.map(|value| value.rx_overrun_count as i32),
            errors,
        }
    }
}
