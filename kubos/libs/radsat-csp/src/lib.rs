use std::{
    ffi::c_void,
    os::raw::c_char,
    ptr,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Once,
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

use libcsp::{
    csp_accept_guarded, csp_bind, csp_buffer_get, csp_conn_dport, csp_conn_dst, csp_conn_sport,
    csp_conn_src, csp_connect_guarded, csp_init, csp_listen, csp_ping, csp_read_guarded,
    csp_route_work, csp_send, csp_service_handler, ConnectOpts, CspError, CspSocket, MsgPriority,
    ReservedPort, SocketFlags,
};
use thiserror::Error;

static CSP_INIT: Once = Once::new();

pub use libcsp::{CSP_ANY, CSP_LOOPBACK};

#[repr(C)]
struct CspConf {
    version: u8,
    hostname: *const c_char,
    model: *const c_char,
    revision: *const c_char,
    conn_dfl_so: u32,
    dedup: u8,
}

extern "C" {
    static mut csp_conf: CspConf;
    fn csp_sfp_send_own_memcpy(
        conn: *mut libcsp::ffi::csp_conn_t,
        data: *const c_void,
        datasize: u32,
        mtu: u32,
        timeout: u32,
        memcpyfcn: CspMemcpyFn,
    ) -> i32;
}

type CspMemPtr = *mut c_void;
type CspConstMemPtr = *const c_void;
type CspMemcpyFn = Option<unsafe extern "C" fn(CspMemPtr, CspConstMemPtr, usize) -> CspMemPtr>;

const CSP_ERR_NONE: i32 = 0;
const SFP_HEADER_LEN: usize = 8;
const RDP_HEADER_LEN: usize = 5;
const CSP_FFRAG: u8 = 0x10;
const CSP_ERR_TIMEDOUT: i32 = -3;

#[cfg(test)]
extern "C" {
    fn csp_get_conf() -> *const CspConf;
    fn radsat_csp_capture_iface_reset();
    fn radsat_csp_capture_iface_add(addr: u16);
    fn radsat_csp_capture_iface_remove();
    fn radsat_csp_capture_frame_ptr() -> *const u8;
    fn radsat_csp_capture_frame_len() -> u16;
    fn radsat_csp_capture_i2c_addr() -> u8;
    fn radsat_csp_capture_from_me() -> std::os::raw::c_int;
    fn radsat_csp_capture_id() -> libcsp::ffi::csp_id_t;
}

#[cfg(test)]
static TEST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[cfg(test)]
fn csp_test_guard() -> std::sync::MutexGuard<'static, ()> {
    TEST_LOCK.lock().unwrap_or_else(|err| err.into_inner())
}

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

    #[error("CSP listener timed out")]
    ListenerTimedOut,

    #[error("CSP SFP transfer failed with status {status}")]
    Sfp { status: i32 },

    #[error("CSP SFP MTU {mtu} is invalid; expected 1..={max}")]
    InvalidSfpMtu { mtu: usize, max: usize },

    #[error("CSP SFP payload length {len} exceeds maximum {max}")]
    SfpPayloadTooLarge { len: usize, max: usize },

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
        // NXTRX4 uses CSP v1 framing: a 4-byte big-endian CSP header.
        unsafe {
            csp_conf.version = 1;
        }
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

    pub fn with_rdp(mut self) -> Self {
        self.opts |= ConnectOpts::RDP;
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

    /// Sends one logical payload using libcsp's Small Fragmentation Protocol.
    ///
    /// The receiver must use [`CspListener::receive_sfp`] on the same CSP
    /// connection. SFP splits the payload into multiple CSP packets, so this
    /// is the path to use when the SpacePacket cannot fit in one CSP packet.
    pub fn send_sfp(&self, node: u16, port: u8, payload: &[u8], mtu: usize) -> Result<()> {
        validate_sfp_mtu(mtu)?;
        if payload.len() > u32::MAX as usize {
            return Err(Error::SfpPayloadTooLarge {
                len: payload.len(),
                max: u32::MAX as usize,
            });
        }

        let mut conn = csp_connect_guarded(self.priority, node, port, self.timeout, self.opts())
            .ok_or(Error::ConnectFailed { node, port })?;
        let conn_ptr = conn
            .as_mut()
            .inner_mut()
            .map(|conn| conn as *mut libcsp::ffi::csp_conn_t)
            .ok_or(Error::ConnectFailed { node, port })?;

        let status = unsafe {
            csp_sfp_send_own_memcpy(
                conn_ptr,
                payload.as_ptr().cast::<c_void>(),
                payload.len() as u32,
                mtu as u32,
                self.timeout.as_millis() as u32,
                Some(sfp_memcpy),
            )
        };
        if status != CSP_ERR_NONE {
            return Err(Error::Sfp { status });
        }

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

/// Payload and connection metadata received by a [`CspListener`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReceivedPacket {
    pub source: u16,
    pub source_port: u8,
    pub destination: u16,
    pub destination_port: u8,
    pub payload: Vec<u8>,
}

/// A blocking CSP server-side listener bound to one local CSP port.
///
/// This is intended for services that need to receive payloads routed into the
/// process by [`RouterWorker`]. Keep the listener alive for as long as packets
/// should be accepted on the bound port.
pub struct CspListener {
    socket: CspSocket,
    accept_timeout: Duration,
    read_timeout: Duration,
}

// SAFETY: The socket is only accessed through `&mut self`, and service users
// wrap the listener in synchronization before moving it across threads.
unsafe impl Send for CspListener {}

impl CspListener {
    pub fn bind(port: u8, backlog: usize) -> Self {
        initialize();

        let mut socket = CspSocket::default();
        csp_bind(&mut socket, port);
        csp_listen(&mut socket, backlog);

        Self {
            socket,
            accept_timeout: Duration::from_millis(100),
            read_timeout: Duration::from_millis(100),
        }
    }

    pub fn with_accept_timeout(mut self, timeout: Duration) -> Self {
        self.accept_timeout = timeout;
        self
    }

    pub fn with_read_timeout(mut self, timeout: Duration) -> Self {
        self.read_timeout = timeout;
        self
    }

    pub fn receive(&mut self) -> Result<ReceivedPacket> {
        loop {
            if let Some(packet) = self.receive_once(self.accept_timeout, self.read_timeout) {
                return Ok(packet);
            }
        }
    }

    pub fn receive_sfp(&mut self, max_payload_len: usize) -> Result<ReceivedPacket> {
        loop {
            if let Some(packet) =
                self.receive_sfp_once(self.accept_timeout, self.read_timeout, max_payload_len)
            {
                return packet;
            }
        }
    }

    pub fn receive_sfp_timeout(
        &mut self,
        timeout: Duration,
        max_payload_len: usize,
    ) -> Result<Option<ReceivedPacket>> {
        let start = Instant::now();

        loop {
            let elapsed = start.elapsed();
            if elapsed >= timeout {
                return Ok(None);
            }

            let remaining = timeout - elapsed;
            let accept_timeout = remaining.min(self.accept_timeout);
            if let Some(packet) =
                self.receive_sfp_once(accept_timeout, self.read_timeout, max_payload_len)
            {
                return packet.map(Some);
            }
        }
    }

    pub fn receive_timeout(&mut self, timeout: Duration) -> Result<Option<ReceivedPacket>> {
        let start = Instant::now();

        loop {
            let elapsed = start.elapsed();
            if elapsed >= timeout {
                return Ok(None);
            }

            let remaining = timeout - elapsed;
            let accept_timeout = remaining.min(self.accept_timeout);
            let read_timeout = remaining.min(self.read_timeout);

            if let Some(packet) = self.receive_once(accept_timeout, read_timeout) {
                return Ok(Some(packet));
            }
        }
    }

    fn receive_once(
        &mut self,
        accept_timeout: Duration,
        read_timeout: Duration,
    ) -> Option<ReceivedPacket> {
        let mut conn = csp_accept_guarded(&mut self.socket, accept_timeout)?;
        let source = csp_conn_src(conn.as_ref()) as u16;
        let source_port = csp_conn_sport(conn.as_ref()) as u8;
        let destination = csp_conn_dst(conn.as_ref()) as u16;
        let destination_port = csp_conn_dport(conn.as_ref()) as u8;
        let packet = csp_read_guarded(conn.as_mut(), read_timeout)?;
        let payload = packet.as_ref().packet_data().to_vec();

        Some(ReceivedPacket {
            source,
            source_port,
            destination,
            destination_port,
            payload,
        })
    }

    fn receive_sfp_once(
        &mut self,
        accept_timeout: Duration,
        read_timeout: Duration,
        max_payload_len: usize,
    ) -> Option<Result<ReceivedPacket>> {
        let mut conn = csp_accept_guarded(&mut self.socket, accept_timeout)?;
        let source = csp_conn_src(conn.as_ref()) as u16;
        let source_port = csp_conn_sport(conn.as_ref()) as u8;
        let destination = csp_conn_dst(conn.as_ref()) as u16;
        let destination_port = csp_conn_dport(conn.as_ref()) as u8;

        let mut payload = Vec::new();
        let mut expected_total = None;

        loop {
            let Some(packet) = csp_read_guarded(conn.as_mut(), read_timeout) else {
                return Some(Err(Error::Sfp {
                    status: CSP_ERR_TIMEDOUT,
                }));
            };
            let flags = unsafe { (*packet.as_ref().inner()).id.flags };
            if flags & CSP_FFRAG == 0 {
                return Some(Err(Error::Sfp { status: -103 }));
            }

            let data = packet.as_ref().packet_data();
            if data.len() < SFP_HEADER_LEN {
                return Some(Err(Error::Sfp { status: -103 }));
            }

            let chunk_len = data.len() - SFP_HEADER_LEN;
            let offset = u32::from_be_bytes([
                data[chunk_len],
                data[chunk_len + 1],
                data[chunk_len + 2],
                data[chunk_len + 3],
            ]) as usize;
            let total = u32::from_be_bytes([
                data[chunk_len + 4],
                data[chunk_len + 5],
                data[chunk_len + 6],
                data[chunk_len + 7],
            ]) as usize;

            if total > max_payload_len {
                return Some(Err(Error::SfpPayloadTooLarge {
                    len: total,
                    max: max_payload_len,
                }));
            }

            match expected_total {
                Some(expected) if expected != total => {
                    return Some(Err(Error::Sfp { status: -103 }));
                }
                None => {
                    expected_total = Some(total);
                    payload.reserve(total);
                }
                _ => {}
            }

            if offset != payload.len() || payload.len() + chunk_len > total {
                return Some(Err(Error::Sfp { status: -103 }));
            }

            payload.extend_from_slice(&data[..chunk_len]);
            if payload.len() == total {
                break;
            }
        }

        Some(Ok(ReceivedPacket {
            source,
            source_port,
            destination,
            destination_port,
            payload,
        }))
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

fn validate_sfp_mtu(mtu: usize) -> Result<()> {
    let max = libcsp::ffi::CSP_BUFFER_SIZE.saturating_sub(SFP_HEADER_LEN + RDP_HEADER_LEN);
    if mtu == 0 || mtu > max {
        return Err(Error::InvalidSfpMtu { mtu, max });
    }
    Ok(())
}

unsafe extern "C" fn sfp_memcpy(dst: CspMemPtr, src: CspConstMemPtr, len: usize) -> CspMemPtr {
    unsafe {
        ptr::copy_nonoverlapping(src.cast::<u8>(), dst.cast::<u8>(), len);
    }
    dst
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
                ptr::copy_nonoverlapping(frame.as_ptr(), (*packet).frame_begin, frame.len());
                (*packet).frame_length = frame.len() as u16;
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
        if unsafe { (*frame).frame_begin.is_null() } {
            return CSP_ERR_INVAL;
        }

        let i2c_addr = unsafe { ((*frame).cfpid & 0x7F) as u8 };
        let bytes = unsafe {
            std::slice::from_raw_parts((*frame).frame_begin, (*frame).frame_length as usize)
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
            let _guard = csp_test_guard();
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
    const CAPTURE_IFACE_ADDR: u16 = 19;
    const CAPTURE_DEST_ADDR: u16 = 8;
    const CAPTURE_DEST_PORT: u8 = 0;

    #[test]
    fn initialize_selects_csp_v1() {
        let _guard = csp_test_guard();
        initialize();

        let conf = unsafe { &*csp_get_conf() };
        assert_eq!(conf.version, 1);
    }

    struct CaptureInterface {
        _private: (),
    }

    impl CaptureInterface {
        fn start() -> Self {
            unsafe {
                radsat_csp_capture_iface_reset();
                radsat_csp_capture_iface_add(CAPTURE_IFACE_ADDR);
            }

            Self { _private: () }
        }
    }

    impl Drop for CaptureInterface {
        fn drop(&mut self) {
            unsafe {
                radsat_csp_capture_iface_remove();
            }
        }
    }

    #[test]
    fn transaction_request_frame_uses_csp_v1_header_before_i2c_write() {
        let _guard = csp_test_guard();
        initialize();

        let _capture_iface = CaptureInterface::start();
        let client = CspClient::new().with_timeout(Duration::from_millis(1));
        let request = [0x00, 0x01]; // CMP ident request payload
        let mut response = [0u8; 1];

        let err = client
            .transaction(
                CAPTURE_DEST_ADDR,
                CAPTURE_DEST_PORT,
                &request,
                &mut response,
            )
            .expect_err("capture interface does not send a reply");
        assert!(matches!(err, Error::TransactionTimedOut));

        let frame_len = unsafe { radsat_csp_capture_frame_len() } as usize;
        assert_eq!(frame_len, 4 + request.len());
        let frame = unsafe {
            std::slice::from_raw_parts(radsat_csp_capture_frame_ptr(), frame_len).to_vec()
        };
        let id = unsafe { radsat_csp_capture_id() };

        assert_eq!(
            unsafe { radsat_csp_capture_i2c_addr() },
            CAPTURE_DEST_ADDR as u8
        );
        assert_eq!(unsafe { radsat_csp_capture_from_me() }, 1);
        assert_eq!(id.pri, MsgPriority::Normal as u8);
        assert_eq!(id.src, CAPTURE_IFACE_ADDR);
        assert_eq!(id.dst, CAPTURE_DEST_ADDR);
        assert_eq!(id.dport, CAPTURE_DEST_PORT);
        assert_eq!(id.flags, 0);
        assert_eq!(&frame[4..], request);

        let header = u32::from_be_bytes(frame[0..4].try_into().unwrap());
        assert_eq!((header >> 30) & 0x03, MsgPriority::Normal as u32);
        assert_eq!((header >> 25) & 0x1F, CAPTURE_IFACE_ADDR as u32);
        assert_eq!((header >> 20) & 0x1F, CAPTURE_DEST_ADDR as u32);
        assert_eq!((header >> 14) & 0x3F, CAPTURE_DEST_PORT as u32);
        assert_eq!((header >> 8) & 0x3F, id.sport as u32);
        assert_eq!(header & 0xFF, 0);

        let expected_header = ((MsgPriority::Normal as u32) << 30
            | (CAPTURE_IFACE_ADDR as u32) << 25
            | (CAPTURE_DEST_ADDR as u32) << 20
            | (CAPTURE_DEST_PORT as u32) << 14
            | (id.sport as u32) << 8)
            .to_be_bytes();
        assert_eq!(&frame[..4], expected_header);
    }

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
        let _guard = csp_test_guard();
        initialize();
        let _router = RouterWorker::start();
        let _ping_service = ReservedServiceWorker::start_ping_service();
        let _echo_server = EchoServer::start();
        let client = CspClient::new().with_timeout(Duration::from_millis(5_000));

        let mut ping_ok = false;
        for _ in 0..20 {
            if client.ping(CSP_LOOPBACK, 16).is_ok() {
                ping_ok = true;
                break;
            }
            thread::sleep(Duration::from_millis(25));
        }
        assert!(ping_ok, "loopback ping failed");

        let request = b"hello-csp";
        let mut response = [0_u8; 32];
        let len = client
            .transaction(CSP_LOOPBACK, ECHO_PORT, request, &mut response)
            .expect("loopback transaction failed");

        assert_eq!(&response[..len], request);
    }

    #[test]
    fn send_rejects_payloads_larger_than_csp_buffer() {
        let _guard = csp_test_guard();
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
        let _guard = csp_test_guard();
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

        let _guard = csp_test_guard();
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
