use std::{
    sync::{
        Arc, Mutex,
        mpsc::{self, Receiver, Sender},
    },
    thread,
};

use aes_gcm::{
    Aes128Gcm, Nonce,
    aead::{Aead, KeyInit},
};
use kubos_comms::{CommsResult, CommsServiceError};
use nxtrx4_api::Nxtrx4;
use radsat_csp::{CspClient, CspListener};

use crate::config::{CspSettings, RadioSettings, UplinkCrypto};

const CSP_HEADER_BYTES: usize = 4;
const AES_128_GCM_NONCE_BYTES: usize = 12;
const AES_128_GCM_TAG_BYTES: usize = 16;
const AES_128_GCM_OVERHEAD_BYTES: usize = AES_128_GCM_NONCE_BYTES + AES_128_GCM_TAG_BYTES;

#[derive(Clone)]
pub struct NxtrxComms {
    uplink_rx: Arc<Mutex<Receiver<CommsResult<Vec<u8>>>>>,
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

        let (uplink_tx, uplink_rx) = mpsc::channel();
        let crypto_overhead = crypto_overhead_bytes(&csp.uplink_crypto);
        let max_packet_space_packet_bytes = csp.max_frame_bytes.saturating_sub(CSP_HEADER_BYTES);
        let max_packet_uplink_space_packet_bytes =
            max_packet_space_packet_bytes.saturating_sub(crypto_overhead);
        let max_sfp_uplink_payload_bytes = csp
            .sfp_max_space_packet_bytes
            .saturating_add(crypto_overhead);

        spawn_packet_poller(
            packet_listener,
            uplink_tx.clone(),
            csp.uplink_crypto.clone(),
            max_packet_space_packet_bytes,
            max_packet_uplink_space_packet_bytes,
        );
        spawn_sfp_poller(
            sfp_listener,
            uplink_tx,
            csp.uplink_crypto.clone(),
            csp.sfp_max_space_packet_bytes,
            max_sfp_uplink_payload_bytes,
        );

        Self {
            uplink_rx: Arc::new(Mutex::new(uplink_rx)),
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
        let receiver = self
            .uplink_rx
            .lock()
            .map_err(|_| CommsServiceError::MutexPoisoned)?;

        receiver.recv().map_err(|_| {
            CommsServiceError::GenericError("all uplink CSP listener pollers stopped".to_string())
        })?
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
}

fn spawn_packet_poller(
    mut listener: CspListener,
    tx: Sender<CommsResult<Vec<u8>>>,
    crypto: UplinkCrypto,
    max_wire_payload_bytes: usize,
    max_space_packet_bytes: usize,
) {
    thread::spawn(move || {
        loop {
            let result = receive_packet_uplink(
                &mut listener,
                &crypto,
                max_wire_payload_bytes,
                max_space_packet_bytes,
            );
            if tx.send(result).is_err() {
                break;
            }
        }
    });
}

fn spawn_sfp_poller(
    mut listener: CspListener,
    tx: Sender<CommsResult<Vec<u8>>>,
    crypto: UplinkCrypto,
    max_space_packet_bytes: usize,
    max_wire_payload_bytes: usize,
) {
    thread::spawn(move || {
        loop {
            let result = receive_sfp_uplink(
                &mut listener,
                &crypto,
                max_space_packet_bytes,
                max_wire_payload_bytes,
            );
            if tx.send(result).is_err() {
                break;
            }
        }
    });
}

fn receive_packet_uplink(
    listener: &mut CspListener,
    crypto: &UplinkCrypto,
    max_wire_payload_bytes: usize,
    max_space_packet_bytes: usize,
) -> CommsResult<Vec<u8>> {
    let packet = listener
        .receive()
        .map_err(|err| CommsServiceError::GenericError(err.to_string()))?;
    if packet.payload.len() > max_wire_payload_bytes {
        return Err(CommsServiceError::GenericError(format!(
            "packet-port uplink CSP payload was {} bytes, maximum is {}",
            packet.payload.len(),
            max_wire_payload_bytes
        )));
    }

    log_uplink("packet", &packet);
    let payload = decrypt_uplink_payload(crypto, &packet.payload)?;
    if payload.len() > max_space_packet_bytes {
        return Err(CommsServiceError::GenericError(format!(
            "packet-port uplink SpacePacket was {} bytes, maximum is {}",
            payload.len(),
            max_space_packet_bytes
        )));
    }

    Ok(payload)
}

fn receive_sfp_uplink(
    listener: &mut CspListener,
    crypto: &UplinkCrypto,
    max_space_packet_bytes: usize,
    max_wire_payload_bytes: usize,
) -> CommsResult<Vec<u8>> {
    let packet = listener
        .receive_sfp(max_wire_payload_bytes)
        .map_err(|err| CommsServiceError::GenericError(err.to_string()))?;

    log_uplink("sfp", &packet);
    let payload = decrypt_uplink_payload(crypto, &packet.payload)?;
    if payload.len() > max_space_packet_bytes {
        return Err(CommsServiceError::GenericError(format!(
            "SFP uplink SpacePacket was {} bytes, maximum is {}",
            payload.len(),
            max_space_packet_bytes
        )));
    }

    Ok(payload)
}

pub(crate) fn decrypt_uplink_payload(
    crypto: &UplinkCrypto,
    payload: &[u8],
) -> CommsResult<Vec<u8>> {
    match crypto {
        UplinkCrypto::None => Ok(payload.to_vec()),
        UplinkCrypto::Aes128 { key } => {
            if payload.len() < AES_128_GCM_OVERHEAD_BYTES {
                return Err(CommsServiceError::GenericError(format!(
                    "encrypted uplink payload was {} bytes, minimum is {}",
                    payload.len(),
                    AES_128_GCM_OVERHEAD_BYTES
                )));
            }

            let (nonce, ciphertext) = payload.split_at(AES_128_GCM_NONCE_BYTES);
            let cipher = Aes128Gcm::new_from_slice(key)
                .map_err(|err| CommsServiceError::GenericError(err.to_string()))?;
            cipher
                .decrypt(Nonce::from_slice(nonce), ciphertext)
                .map_err(|_| {
                    CommsServiceError::GenericError(
                        "failed to decrypt or authenticate AES-128-GCM uplink payload".to_string(),
                    )
                })
        }
    }
}

fn crypto_overhead_bytes(crypto: &UplinkCrypto) -> usize {
    match crypto {
        UplinkCrypto::None => 0,
        UplinkCrypto::Aes128 { .. } => AES_128_GCM_OVERHEAD_BYTES,
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

#[cfg(test)]
mod tests {
    use super::*;
    use aes_gcm::aead::Aead;
    use kubos_comms::{LinkPacket, PayloadType, SpacePacket};

    fn encrypt_for_test(key: [u8; 16], plaintext: &[u8]) -> Vec<u8> {
        let nonce = [0xA5; AES_128_GCM_NONCE_BYTES];
        let cipher = Aes128Gcm::new_from_slice(&key).unwrap();
        let mut encrypted = nonce.to_vec();
        encrypted.extend(
            cipher
                .encrypt(Nonce::from_slice(&nonce), plaintext)
                .unwrap(),
        );
        encrypted
    }

    #[test]
    fn decrypts_aes_128_gcm_uplink_payload() {
        let key = [0x11; 16];
        let packet = SpacePacket::build(42, PayloadType::GraphQL, 15001, b"{\"query\":\"{ping}\"}")
            .unwrap()
            .to_bytes()
            .unwrap();
        let encrypted = encrypt_for_test(key, &packet);

        let decrypted = decrypt_uplink_payload(&UplinkCrypto::Aes128 { key }, &encrypted).unwrap();

        assert_eq!(decrypted, packet);
    }

    #[test]
    fn rejects_tampered_aes_128_gcm_uplink_payload() {
        let key = [0x22; 16];
        let packet = SpacePacket::build(43, PayloadType::GraphQL, 15001, b"{\"query\":\"{ping}\"}")
            .unwrap()
            .to_bytes()
            .unwrap();
        let mut encrypted = encrypt_for_test(key, &packet);
        let last = encrypted.last_mut().unwrap();
        *last ^= 0x01;

        assert!(decrypt_uplink_payload(&UplinkCrypto::Aes128 { key }, &encrypted).is_err());
    }
}
