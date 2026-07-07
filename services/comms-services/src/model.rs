use std::sync::{Arc, Mutex};

use async_graphql::{Enum, SimpleObject};
use kubos_comms::CommsTelemetry;
use nxtrx4_api::Nxtrx4;
use nxtrx4_api::cmp::NxtrxInterface;

use crate::{
    config::{NmpKeys, RadioConfig, ServiceSettings},
    nxtrx_comms::NxtrxComms,
};

#[derive(Clone)]
pub struct Subsystem {
    telemetry: Arc<Mutex<CommsTelemetry>>,
    comms: NxtrxComms,
    settings: ServiceSettings,
    radio_commands: Arc<Mutex<()>>,
}

/// Selects which configured NXTRX4 transceiver a GraphQL radio command targets.
///
/// `Uplink` maps to `[comms-services.radios.uplink]` and `Downlink` maps to
/// `[comms-services.radios.downlink]` in the service config. The selected
/// config determines the CSP node, Linux I2C bus, I2C address, and timeout used
/// for the command.
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum RadioRole {
    /// The transceiver configured as the RF receive/uplink radio.
    Uplink,
    /// The transceiver configured as the RF transmit/downlink radio.
    Downlink,
}

#[derive(Copy, Clone)]
pub(crate) enum NmpKeyAccess {
    Read,
    Superuser,
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
            radio_commands: Arc::new(Mutex::new(())),
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
            uplink_crypto: self.settings.csp.uplink_crypto.mode().to_string(),
        }
    }

    pub fn radio_health(&self, role: RadioRole) -> RadioHealth {
        let csp_node = self.radio_csp_node(role);

        match self.with_radio(role, |radio, config| {
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

            Ok(RadioHealth {
                role,
                csp_node: i32::from(config.csp_node),
                uptime_seconds,
                radio_status,
                radio_tx_packets: interface.map(|value| value.tx_packet_count as i32),
                radio_rx_packets: interface.map(|value| value.rx_packet_count as i32),
                radio_rx_overruns: interface.map(|value| value.rx_overrun_count as i32),
                errors,
            })
        }) {
            Ok(health) => health,
            Err(err) => RadioHealth {
                role,
                csp_node,
                uptime_seconds: None,
                radio_status: None,
                radio_tx_packets: None,
                radio_rx_packets: None,
                radio_rx_overruns: None,
                errors: vec![err],
            },
        }
    }

    pub(crate) fn with_radio<T>(
        &self,
        role: RadioRole,
        action: impl FnOnce(&Nxtrx4, &RadioConfig) -> Result<T, String>,
    ) -> Result<T, String> {
        let _guard = self
            .radio_commands
            .lock()
            .map_err(|_| "failed to lock NXTRX4 command path".to_string())?;
        let (radio, config) = match role {
            RadioRole::Uplink => (&self.comms.uplink_radio, &self.settings.radios.uplink),
            RadioRole::Downlink => (&self.comms.downlink_radio, &self.settings.radios.downlink),
        };

        action(radio.as_ref(), config)
    }

    pub(crate) fn nmp_key(
        &self,
        role: RadioRole,
        explicit: Option<u32>,
        access: NmpKeyAccess,
    ) -> Result<u32, String> {
        if let Some(key) = explicit {
            return Ok(key);
        }

        let (name, keys) = match role {
            RadioRole::Uplink => ("uplink", &self.settings.radios.uplink.nmp_keys),
            RadioRole::Downlink => ("downlink", &self.settings.radios.downlink.nmp_keys),
        };
        let key = select_nmp_key(keys, access);

        key.ok_or_else(|| match access {
            NmpKeyAccess::Read => format!(
                "no NMP key configured for {name} radio; set nmp_user_key or nmp_superuser_key, or provide key explicitly"
            ),
            NmpKeyAccess::Superuser => format!(
                "no NMP superuser key configured for {name} radio; set nmp_superuser_key or provide key explicitly"
            ),
        })
    }

    fn radio_csp_node(&self, role: RadioRole) -> i32 {
        match role {
            RadioRole::Uplink => i32::from(self.settings.radios.uplink.csp_node),
            RadioRole::Downlink => i32::from(self.settings.radios.downlink.csp_node),
        }
    }
}

fn select_nmp_key(keys: &NmpKeys, access: NmpKeyAccess) -> Option<u32> {
    match access {
        NmpKeyAccess::Read => keys.user.or(keys.superuser),
        NmpKeyAccess::Superuser => keys.superuser,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selects_least_privileged_configured_nmp_key() {
        let keys = NmpKeys {
            user: Some(1),
            superuser: Some(2),
        };
        assert_eq!(select_nmp_key(&keys, NmpKeyAccess::Read), Some(1));
        assert_eq!(select_nmp_key(&keys, NmpKeyAccess::Superuser), Some(2));

        let superuser_only = NmpKeys {
            user: None,
            superuser: Some(2),
        };
        assert_eq!(select_nmp_key(&superuser_only, NmpKeyAccess::Read), Some(2));
    }
}
