use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Once,
    },
    thread::{self, JoinHandle},
    time::Duration,
};

use libcsp::{
    csp_accept_guarded, csp_bind, csp_buffer_get, csp_connect_guarded, csp_init, csp_listen,
    csp_ping, csp_read_guarded, csp_route_work, csp_send, csp_service_handler, ConnectOpts,
    CspError, CspSocket, MsgPriority, ReservedPort, SocketFlags,
};
use thiserror::Error;

static CSP_INIT: Once = Once::new();

pub use libcsp::{CSP_ANY, CSP_LOOPBACK};

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to connect to CSP node {node} port {port}")]
    ConnectFailed { node: u16, port: u8 },

    #[error("CSP packet buffer unavailable")]
    NoPacketBuffer,

    #[error("payload length {len} exceeds CSP buffer size {max}")]
    PayloadTooLarge { len: usize, max: usize },

    #[error("response length {len} exceeds provided buffer size {max}")]
    ResponseTooLarge { len: usize, max: usize },

    #[error("CSP ping failed")]
    PingFailed,

    #[error("CSP transaction timed out")]
    TransactionTimedOut,
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn initialize() {
    CSP_INIT.call_once(|| {
        // SAFETY: csp_init initializes global libcsp state and must be called once per process.
        unsafe { csp_init() };
    });
}

pub struct CspClient {
    timeout: Duration,
    priority: MsgPriority,
    opts: ConnectOpts,
}

impl Default for CspClient {
    fn default() -> Self {
        Self {
            timeout: Duration::from_millis(1_000),
            priority: MsgPriority::Normal,
            opts: ConnectOpts::NONE,
        }
    }
}

impl CspClient {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn ping(&self, node: u16, size: usize) -> Result<Duration> {
        csp_ping(node, self.timeout, size, SocketFlags::NONE).map_err(|_| Error::PingFailed)
    }

    pub fn send(&self, node: u16, port: u8, payload: &[u8]) -> Result<()> {
        validate_payload_len(payload.len())?;

        let mut conn = csp_connect_guarded(self.priority, node, port, self.timeout, self.opts())
            .ok_or(Error::ConnectFailed { node, port })?;
        let mut packet = csp_buffer_get().ok_or(Error::NoPacketBuffer)?;
        packet.set_data(payload);

        csp_send(conn.as_mut(), packet);
        Ok(())
    }

    pub fn transaction(
        &self,
        node: u16,
        port: u8,
        request: &[u8],
        response: &mut [u8],
    ) -> Result<usize> {
        validate_payload_len(request.len())?;

        let mut conn = csp_connect_guarded(self.priority, node, port, self.timeout, self.opts())
            .ok_or(Error::ConnectFailed { node, port })?;
        let mut packet = csp_buffer_get().ok_or(Error::NoPacketBuffer)?;
        packet.set_data(request);
        csp_send(conn.as_mut(), packet);

        let packet =
            csp_read_guarded(conn.as_mut(), self.timeout).ok_or(Error::TransactionTimedOut)?;
        let data = packet.as_ref().packet_data();
        if data.len() > response.len() {
            return Err(Error::ResponseTooLarge {
                len: data.len(),
                max: response.len(),
            });
        }

        response[..data.len()].copy_from_slice(data);
        Ok(data.len())
    }

    fn opts(&self) -> ConnectOpts {
        ConnectOpts::from_bits_truncate(self.opts.bits())
    }
}

pub struct RouterWorker {
    stop: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
}

impl RouterWorker {
    pub fn start() -> Self {
        initialize();

        let stop = Arc::new(AtomicBool::new(false));
        let thread_stop = Arc::clone(&stop);
        let handle = thread::spawn(move || {
            while !thread_stop.load(Ordering::Relaxed) {
                if let Err(err) = csp_route_work() {
                    if err != CspError::TimedOut {
                        thread::sleep(Duration::from_millis(1));
                    }
                }
            }
        });

        Self {
            stop,
            handle: Some(handle),
        }
    }
}

impl Drop for RouterWorker {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

pub struct ReservedServiceWorker {
    stop: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
}

impl ReservedServiceWorker {
    pub fn start_ping_service() -> Self {
        Self::start(ReservedPort::Ping as u8)
    }

    pub fn start(port: u8) -> Self {
        initialize();

        let stop = Arc::new(AtomicBool::new(false));
        let thread_stop = Arc::clone(&stop);
        let handle = thread::spawn(move || {
            let mut socket = CspSocket::default();
            csp_bind(&mut socket, port);
            csp_listen(&mut socket, 10);

            while !thread_stop.load(Ordering::Relaxed) {
                let Some(mut conn) = csp_accept_guarded(&mut socket, Duration::from_millis(100))
                else {
                    continue;
                };

                while !thread_stop.load(Ordering::Relaxed) {
                    let Some(packet) = csp_read_guarded(conn.as_mut(), Duration::from_millis(100))
                    else {
                        break;
                    };
                    csp_service_handler(packet.take());
                }
            }
        });

        Self {
            stop,
            handle: Some(handle),
        }
    }
}

impl Drop for ReservedServiceWorker {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

fn validate_payload_len(len: usize) -> Result<()> {
    let max = libcsp::ffi::CSP_BUFFER_SIZE;
    if len > max {
        return Err(Error::PayloadTooLarge { len, max });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use libcsp::csp_conn_dport;

    const ECHO_PORT: u8 = 10;

    struct EchoServer {
        stop: Arc<AtomicBool>,
        handle: Option<JoinHandle<()>>,
    }

    impl EchoServer {
        fn start() -> Self {
            let stop = Arc::new(AtomicBool::new(false));
            let thread_stop = Arc::clone(&stop);
            let handle = thread::spawn(move || {
                let mut socket = CspSocket::default();
                csp_bind(&mut socket, ECHO_PORT);
                csp_listen(&mut socket, 10);

                while !thread_stop.load(Ordering::Relaxed) {
                    let Some(mut conn) =
                        csp_accept_guarded(&mut socket, Duration::from_millis(100))
                    else {
                        continue;
                    };

                    while !thread_stop.load(Ordering::Relaxed) {
                        let Some(packet) =
                            csp_read_guarded(conn.as_mut(), Duration::from_millis(100))
                        else {
                            break;
                        };

                        if csp_conn_dport(conn.as_ref()) == ECHO_PORT as i32 {
                            csp_send(conn.as_mut(), packet.take());
                        }
                    }
                }
            });

            Self {
                stop,
                handle: Some(handle),
            }
        }
    }

    impl Drop for EchoServer {
        fn drop(&mut self) {
            self.stop.store(true, Ordering::Relaxed);
            if let Some(handle) = self.handle.take() {
                let _ = handle.join();
            }
        }
    }

    #[test]
    fn loopback_ping_and_transaction_work() {
        initialize();
        let _router = RouterWorker::start();
        let _ping_service = ReservedServiceWorker::start_ping_service();
        let _echo_server = EchoServer::start();
        let client = CspClient::new().with_timeout(Duration::from_millis(1_000));

        client.ping(CSP_LOOPBACK, 16).expect("loopback ping failed");

        let request = b"hello-csp";
        let mut response = [0_u8; 32];
        let len = client
            .transaction(CSP_LOOPBACK, ECHO_PORT, request, &mut response)
            .expect("loopback transaction failed");

        assert_eq!(&response[..len], request);
    }

    #[test]
    fn send_rejects_payloads_larger_than_csp_buffer() {
        initialize();
        let client = CspClient::new();
        let payload = vec![0_u8; libcsp::ffi::CSP_BUFFER_SIZE + 1];

        assert!(matches!(
            client.send(CSP_LOOPBACK, CSP_ANY, &payload),
            Err(Error::PayloadTooLarge { .. })
        ));
    }
}
