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

    #[cfg(feature = "i2c")]
    #[error("invalid CSP I2C config: {0}")]
    InvalidI2cConfig(String),

    #[cfg(feature = "i2c")]
    #[error("CSP I2C interface registration failed with status {status}")]
    I2cInterfaceRegistration { status: i32 },

    #[cfg(feature = "i2c")]
    #[error("I2C driver error: {0}")]
    I2cDriver(String),

    #[cfg(feature = "i2c")]
    #[error("invalid Linux I2C address 0x{addr:02X}; expected a 7-bit address")]
    InvalidI2cAddress { addr: u8 },

    #[cfg(feature = "i2c")]
    #[error("I2C CSP frame length {len} exceeds maximum {max}")]
    I2cFrameTooLarge { len: usize, max: usize },

    #[cfg(feature = "i2c")]
    #[error("CSP I2C route registration failed with status {status}")]
    I2cRouteRegistration { status: i32 },
}

pub type Result<T> = std::result::Result<T, Error>;

/// Initializes the global libcsp stack. Safe to call multiple times; only the
/// first call takes effect. [`RouterWorker`] and [`ReservedServiceWorker`]
/// constructors call this automatically.
pub fn initialize() {
    CSP_INIT.call_once(|| {
        // SAFETY: csp_init initializes global libcsp state and must be called once per process.
        unsafe { csp_init() };
    });
}

/// Sends CSP packets to remote nodes. All instances share the same underlying
/// libcsp stack. Requires a running [`RouterWorker`] in the same process.
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

    /// Sends `request` and waits for a single reply packet into `response`.
    ///
    /// For I2C transports, a receive-polling thread must call
    /// [`LinuxI2cCspInterface::inject_received_frame`] before this times out,
    /// because I2C devices cannot push replies without being polled.
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

/// Routes incoming CSP packets to their bound sockets.
///
/// **Must be kept alive** for the lifetime of any CSP activity. When dropped,
/// the router thread stops and all packet delivery ceases.
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

/// Services packets arriving on a CSP port using `csp_service_handler`.
///
/// Required for this process to respond to CSP ping requests. Keep alive
/// alongside [`RouterWorker`].
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

#[cfg(feature = "i2c")]
pub mod i2c {
    use std::{
        ffi::c_void,
        ffi::CString,
        ptr,
        sync::{Arc, Mutex},
    };

    use embedded_hal::i2c::I2c;
    use libcsp::ffi;
    use linux_embedded_hal::I2cdev;

    use super::*;

    const CSP_ERR_NONE: i32 = 0;
    const CSP_ERR_INVAL: i32 = -2;
    const CSP_ERR_TX: i32 = -10;
    const CSP_NO_VIA_ADDRESS: u16 = 0xFFFF;
    const MAX_LINUX_I2C_ADDR: u8 = 0x7F;

    #[repr(C)]
    struct CspI2cInterfaceData {
        tx_func: Option<
            unsafe extern "C" fn(
                driver_data: *mut c_void,
                frame: *mut ffi::csp_packet_t,
            ) -> std::ffi::c_int,
        >,
    }

    unsafe extern "C" {
        fn csp_i2c_add_interface(iface: *mut ffi::csp_iface_t) -> std::ffi::c_int;
        fn csp_i2c_rx(
            iface: *mut ffi::csp_iface_t,
            frame: *mut ffi::csp_packet_t,
            px_task_woken: *mut c_void,
        );
        fn csp_id_setup_rx(packet: *mut ffi::csp_packet_t) -> std::ffi::c_int;
        fn csp_iflist_remove(ifc: *mut ffi::csp_iface_t);
        fn csp_rtable_set(
            dest_address: u16,
            netmask: std::ffi::c_int,
            ifc: *mut ffi::csp_iface_t,
            via: u16,
        ) -> std::ffi::c_int;
    }

    /// Configuration for opening a Linux I2C CSP interface.
    #[derive(Clone, Debug)]
    pub struct I2cInterfaceConfig {
        pub bus: String,
        pub local_addr: u16,
        pub name: String,
        pub is_default: bool,
        pub netmask: u16,
    }

    impl I2cInterfaceConfig {
        /// `local_addr` is **this node's own CSP address** (the OBC), not the
        /// remote device address.
        pub fn new(bus: impl Into<String>, local_addr: u16) -> Self {
            Self {
                bus: bus.into(),
                local_addr,
                name: "I2C".to_string(),
                is_default: true,
                netmask: 0,
            }
        }

        pub fn with_name(mut self, name: impl Into<String>) -> Self {
            self.name = name.into();
            self
        }

        pub fn with_default_route(mut self, is_default: bool) -> Self {
            self.is_default = is_default;
            self
        }

        pub fn with_netmask(mut self, netmask: u16) -> Self {
            self.netmask = netmask;
            self
        }
    }

    struct LinuxI2cDriverData {
        bus: Mutex<I2cdev>,
    }

    impl LinuxI2cDriverData {
        fn write_frame(&self, addr: u8, frame: &[u8]) -> Result<()> {
            let mut bus = self
                .bus
                .lock()
                .map_err(|err| Error::I2cDriver(format!("I2C bus lock poisoned: {err}")))?;
            bus.write(addr, frame)
                .map_err(|err| Error::I2cDriver(err.to_string()))
        }

        fn read_frame(&self, addr: u8, frame: &mut [u8]) -> Result<()> {
            let mut bus = self
                .bus
                .lock()
                .map_err(|err| Error::I2cDriver(format!("I2C bus lock poisoned: {err}")))?;
            bus.read(addr, frame)
                .map_err(|err| Error::I2cDriver(err.to_string()))
        }
    }

    /// A Linux I2C bus registered as a libcsp network interface.
    ///
    /// **Must be kept alive** while CSP packets are being exchanged over I2C.
    /// Dropping this deregisters the interface from the libcsp router.
    pub struct LinuxI2cCspInterface {
        iface: Box<ffi::csp_iface_t>,
        _ifdata: Box<CspI2cInterfaceData>,
        driver_data: Arc<LinuxI2cDriverData>,
        _name: CString,
    }

    // SAFETY: The registered libcsp pointers reference heap allocations owned by this
    // struct, and the Linux I2C device is protected by a mutex.
    unsafe impl Send for LinuxI2cCspInterface {}

    impl LinuxI2cCspInterface {
        pub fn open(config: I2cInterfaceConfig) -> Result<Self> {
            initialize();

            let name = CString::new(config.name.clone()).map_err(|_| {
                Error::InvalidI2cConfig("I2C interface name cannot contain NUL bytes".to_string())
            })?;
            let i2c = I2cdev::new(&config.bus).map_err(|err| Error::I2cDriver(err.to_string()))?;
            let driver_data = Arc::new(LinuxI2cDriverData {
                bus: Mutex::new(i2c),
            });
            let mut ifdata = Box::new(CspI2cInterfaceData {
                tx_func: Some(linux_i2c_tx),
            });
            let mut iface = Box::new(ffi::csp_iface_t::default());

            iface.addr = config.local_addr;
            iface.netmask = config.netmask;
            iface.name = name.as_ptr();
            iface.interface_data = (&mut *ifdata as *mut CspI2cInterfaceData).cast::<c_void>();
            iface.driver_data = Arc::as_ptr(&driver_data).cast_mut().cast::<c_void>();
            iface.is_default = u8::from(config.is_default);

            let status = unsafe { csp_i2c_add_interface(&mut *iface) };
            if status != CSP_ERR_NONE {
                return Err(Error::I2cInterfaceRegistration { status });
            }

            Ok(Self {
                iface,
                _ifdata: ifdata,
                driver_data,
                _name: name,
            })
        }

        pub fn open_default(bus: impl Into<String>, local_addr: u16) -> Result<Self> {
            Self::open(I2cInterfaceConfig::new(bus, local_addr))
        }

        /// Routes `csp_node` through this interface via `i2c_addr`.
        ///
        /// Only needed when the device's CSP node id differs from its 7-bit I2C
        /// address. When they are equal, the default route handles delivery.
        pub fn route_node_via_i2c_addr(&mut self, csp_node: u16, i2c_addr: u8) -> Result<()> {
            self.set_route(csp_node, -1, Some(i2c_addr))
        }

        /// Low-level route registration. Prefer [`route_node_via_i2c_addr`] for
        /// single-node routes.
        pub fn set_route(
            &mut self,
            destination: u16,
            netmask: i32,
            via_i2c_addr: Option<u8>,
        ) -> Result<()> {
            let via = match via_i2c_addr {
                Some(addr) => {
                    validate_linux_i2c_addr(addr)?;
                    u16::from(addr)
                }
                None => CSP_NO_VIA_ADDRESS,
            };

            let status = unsafe { csp_rtable_set(destination, netmask, &mut *self.iface, via) };
            if status != CSP_ERR_NONE {
                return Err(Error::I2cRouteRegistration { status });
            }

            Ok(())
        }

        /// Feeds a raw CSP frame (header + payload bytes) into the libcsp router.
        ///
        /// Call this from a device-specific polling loop after reading the reply
        /// bytes from the I2C slave. The frame must include the CSP header bytes.
        pub fn inject_received_frame(&mut self, frame: &[u8]) -> Result<()> {
            let packet = unsafe { ffi::csp_buffer_get(0) };
            if packet.is_null() {
                return Err(Error::NoPacketBuffer);
            }

            let header_len = unsafe { csp_id_setup_rx(packet) };
            let max = ffi::CSP_BUFFER_SIZE + header_len as usize;
            if frame.len() > max {
                unsafe {
                    ffi::csp_buffer_free(packet.cast::<c_void>());
                }
                return Err(Error::I2cFrameTooLarge {
                    len: frame.len(),
                    max,
                });
            }

            unsafe {
                let rx_info = &mut (*packet).packet_info.rx_tx_only;
                ptr::copy_nonoverlapping(frame.as_ptr(), rx_info.frame_begin, frame.len());
                rx_info.frame_length = frame.len() as u16;
                csp_i2c_rx(&mut *self.iface, packet, ptr::null_mut());
            }

            Ok(())
        }

        /// Reads `frame_len` bytes from `i2c_addr` over I2C and passes them to
        /// [`inject_received_frame`]. Use when the reply frame length is fixed.
        ///
        /// For variable-length replies, read the bytes externally and call
        /// [`inject_received_frame`] directly.
        pub fn read_frame_from(&mut self, i2c_addr: u8, frame_len: usize) -> Result<()> {
            validate_linux_i2c_addr(i2c_addr)?;

            let max = ffi::CSP_BUFFER_SIZE + 8;
            if frame_len > max {
                return Err(Error::I2cFrameTooLarge {
                    len: frame_len,
                    max,
                });
            }

            let mut frame = vec![0_u8; frame_len];
            self.driver_data.read_frame(i2c_addr, &mut frame)?;
            self.inject_received_frame(&frame)
        }
    }

    impl Drop for LinuxI2cCspInterface {
        fn drop(&mut self) {
            unsafe {
                csp_iflist_remove(&mut *self.iface);
            }
        }
    }

    fn validate_linux_i2c_addr(addr: u8) -> Result<()> {
        if addr <= MAX_LINUX_I2C_ADDR {
            Ok(())
        } else {
            Err(Error::InvalidI2cAddress { addr })
        }
    }

    unsafe extern "C" fn linux_i2c_tx(
        driver_data: *mut c_void,
        frame: *mut ffi::csp_packet_t,
    ) -> std::ffi::c_int {
        if driver_data.is_null() || frame.is_null() {
            return CSP_ERR_INVAL;
        }

        let driver_data = unsafe { &*(driver_data.cast::<LinuxI2cDriverData>()) };
        let tx_info = unsafe { (*frame).packet_info.rx_tx_only };
        if tx_info.frame_begin.is_null() {
            return CSP_ERR_INVAL;
        }

        let i2c_addr = (tx_info.cfpid & 0x7F) as u8;
        let bytes = unsafe {
            std::slice::from_raw_parts(tx_info.frame_begin, tx_info.frame_length as usize)
        };

        match driver_data.write_frame(i2c_addr, bytes) {
            Ok(()) => {
                unsafe {
                    ffi::csp_buffer_free(frame.cast::<c_void>());
                }
                CSP_ERR_NONE
            }
            Err(_) => CSP_ERR_TX,
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn route_table_symbol_is_linked() {
            initialize();

            let status = unsafe { csp_rtable_set(1, -1, ptr::null_mut(), CSP_NO_VIA_ADDRESS) };

            assert_eq!(status, CSP_ERR_INVAL);
        }

        #[test]
        fn rejects_non_7_bit_linux_i2c_addresses() {
            assert!(matches!(
                validate_linux_i2c_addr(0x80),
                Err(Error::InvalidI2cAddress { addr: 0x80 })
            ));
        }
    }
}

#[cfg(feature = "i2c")]
pub use i2c::{I2cInterfaceConfig, LinuxI2cCspInterface};

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

    #[test]
    fn transaction_rejects_payload_too_large() {
        initialize();
        let client = CspClient::new();
        let payload = vec![0_u8; libcsp::ffi::CSP_BUFFER_SIZE + 1];
        let mut response = [0_u8; 32];

        assert!(matches!(
            client.transaction(CSP_LOOPBACK, ECHO_PORT, &payload, &mut response),
            Err(Error::PayloadTooLarge { .. })
        ));
    }

    #[test]
    fn transaction_returns_error_when_response_buffer_is_too_small() {
        const LARGE_REPLY_PORT: u8 = 11;
        const REPLY_SIZE: usize = 16;

        struct LargeReplyServer {
            stop: Arc<AtomicBool>,
            handle: Option<JoinHandle<()>>,
        }

        impl LargeReplyServer {
            fn start() -> Self {
                let stop = Arc::new(AtomicBool::new(false));
                let thread_stop = Arc::clone(&stop);
                let handle = thread::spawn(move || {
                    let mut socket = CspSocket::default();
                    csp_bind(&mut socket, LARGE_REPLY_PORT);
                    csp_listen(&mut socket, 10);

                    while !thread_stop.load(Ordering::Relaxed) {
                        let Some(mut conn) =
                            csp_accept_guarded(&mut socket, Duration::from_millis(100))
                        else {
                            continue;
                        };

                        while !thread_stop.load(Ordering::Relaxed) {
                            let Some(_request) =
                                csp_read_guarded(conn.as_mut(), Duration::from_millis(100))
                            else {
                                break;
                            };
                            if let Some(mut reply) = csp_buffer_get() {
                                reply.set_data(&[0xAB_u8; REPLY_SIZE]);
                                csp_send(conn.as_mut(), reply);
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

        impl Drop for LargeReplyServer {
            fn drop(&mut self) {
                self.stop.store(true, Ordering::Relaxed);
                if let Some(handle) = self.handle.take() {
                    let _ = handle.join();
                }
            }
        }

        initialize();
        let _router = RouterWorker::start();
        let _server = LargeReplyServer::start();
        let client = CspClient::new().with_timeout(Duration::from_millis(1_000));

        let mut response = [0_u8; 4]; // too small for REPLY_SIZE (16) bytes
        assert!(matches!(
            client.transaction(CSP_LOOPBACK, LARGE_REPLY_PORT, b"hi", &mut response),
            Err(Error::ResponseTooLarge { len: 16, max: 4 })
        ));
    }
}
