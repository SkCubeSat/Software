use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use kubos_comms::{CommsResult, CommsServiceError};
use nxtrx4_api::Nxtrx4;
use radsat_csp::{CspClient, CspListener};

use crate::config::{CspSettings, RadioSettings};

const LISTENER_POLL_TIMEOUT: Duration = Duration::from_millis(25);

#[derive(Clone)]
pub struct NxtrxComms {
    packet_listener: Arc<Mutex<CspListener>>,
    sfp_listener: Arc<Mutex<CspListener>>,
    packet_downlink_client: Arc<CspClient>,
    sfp_downlink_client: Arc<CspClient>,
    ground_node: u16,
    ground_packet_csp_port: u8,
    ground_sfp_csp_port: u8,
    sfp_mtu: usize,
    max_packet_space_packet_bytes: usize,
    max_sfp_space_packet_bytes: usize,
    pub uplink_radio: Arc<Nxtrx4>,
    pub downlink_radio: Arc<Nxtrx4>,
}

impl NxtrxComms {
    pub fn new(
        packet_listener: CspListener,
        sfp_listener: CspListener,
        csp: &CspSettings,
        radios: &RadioSettings,
    ) -> Self {
        let packet_downlink_client = CspClient::new().with_timeout(radios.downlink.command_timeout);
        let mut sfp_downlink_client =
            CspClient::new().with_timeout(radios.downlink.command_timeout);
        if csp.sfp_use_rdp {
            sfp_downlink_client = sfp_downlink_client.with_rdp();
        }

        Self {
            packet_listener: Arc::new(Mutex::new(packet_listener)),
            sfp_listener: Arc::new(Mutex::new(sfp_listener)),
            packet_downlink_client: Arc::new(packet_downlink_client),
            sfp_downlink_client: Arc::new(sfp_downlink_client),
            ground_node: csp.ground_node,
            ground_packet_csp_port: csp.ground_packet_csp_port,
            ground_sfp_csp_port: csp.ground_sfp_csp_port,
            sfp_mtu: csp.sfp_mtu,
            max_packet_space_packet_bytes: csp.max_frame_bytes.saturating_sub(4),
            max_sfp_space_packet_bytes: csp.sfp_max_space_packet_bytes,
            uplink_radio: Arc::new(
                Nxtrx4::new(radios.uplink.csp_node).with_timeout(radios.uplink.command_timeout),
            ),
            downlink_radio: Arc::new(
                Nxtrx4::new(radios.downlink.csp_node).with_timeout(radios.downlink.command_timeout),
            ),
        }
    }

    pub fn read(&self) -> CommsResult<Vec<u8>> {
        loop {
            if let Some(packet) = self.read_packet_port()? {
                return Ok(packet);
            }

            if let Some(packet) = self.read_sfp_port()? {
                return Ok(packet);
            }
        }
    }

    pub fn write(&self, data: &[u8]) -> CommsResult<()> {
        // `data` is already a serialized SpacePacket. Send it as the payload
        // addressed to the configured ground node/port. Small responses use
        // the normal packet port; larger responses use the explicit SFP port.
        if data.len() <= self.max_packet_space_packet_bytes {
            return self
                .packet_downlink_client
                .send(self.ground_node, self.ground_packet_csp_port, data)
                .map_err(|err| CommsServiceError::GenericError(err.to_string()));
        }

        if data.len() > self.max_sfp_space_packet_bytes {
            return Err(CommsServiceError::GenericError(format!(
                "downlink SpacePacket was {} bytes, maximum is {}",
                data.len(),
                self.max_sfp_space_packet_bytes
            )));
        }

        self.sfp_downlink_client
            .send_sfp(
                self.ground_node,
                self.ground_sfp_csp_port,
                data,
                self.sfp_mtu,
            )
            .map_err(|err| CommsServiceError::GenericError(err.to_string()))
    }

    fn read_packet_port(&self) -> CommsResult<Option<Vec<u8>>> {
        let mut listener = self
            .packet_listener
            .lock()
            .map_err(|_| CommsServiceError::MutexPoisoned)?;
        let packet = listener
            .receive_timeout(LISTENER_POLL_TIMEOUT)
            .map_err(|err| CommsServiceError::GenericError(err.to_string()))?;
        let Some(packet) = packet else {
            return Ok(None);
        };

        if packet.payload.len() > self.max_packet_space_packet_bytes {
            return Err(CommsServiceError::GenericError(format!(
                "packet-port uplink SpacePacket was {} bytes, maximum is {}",
                packet.payload.len(),
                self.max_packet_space_packet_bytes
            )));
        }

        log_uplink("packet", &packet);
        Ok(Some(packet.payload))
    }

    fn read_sfp_port(&self) -> CommsResult<Option<Vec<u8>>> {
        let mut listener = self
            .sfp_listener
            .lock()
            .map_err(|_| CommsServiceError::MutexPoisoned)?;
        let packet = listener
            .receive_sfp_timeout(LISTENER_POLL_TIMEOUT, self.max_sfp_space_packet_bytes)
            .map_err(|err| CommsServiceError::GenericError(err.to_string()))?;
        let Some(packet) = packet else {
            return Ok(None);
        };

        log_uplink("sfp", &packet);
        Ok(Some(packet.payload))
    }
}

fn log_uplink(mode: &str, packet: &radsat_csp::ReceivedPacket) {
    log::debug!(
        "received {mode} uplink CSP payload: src={} sport={} dst={} dport={} bytes={}",
        packet.source,
        packet.source_port,
        packet.destination,
        packet.destination_port,
        packet.payload.len()
    );
}

pub fn read(conn: &NxtrxComms) -> CommsResult<Vec<u8>> {
    conn.read()
}

pub fn write(conn: &NxtrxComms, data: &[u8]) -> CommsResult<()> {
    conn.write(data)
}
