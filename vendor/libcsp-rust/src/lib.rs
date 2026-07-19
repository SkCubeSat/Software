//! This crate provides a (mostly) safe and ergonomic Rust API for the
//! [`libcsp` C library](https://github.com/libcsp/libcsp) on top of the
//! [`libcsp-sys`](https://crates.io/crates/libcsp-sys) crate.
//!
//! You can find some more high-level information and examples in the
//! [main repository](https://egit.irs.uni-stuttgart.de/rust/libcsp-rust).
#![no_std]
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(any(feature = "std", test))]
extern crate std;

use core::time::Duration;

use num_enum::{IntoPrimitive, TryFromPrimitive};

use bitflags::bitflags;
use ffi::{csp_conn_s, csp_packet_s, csp_socket_s};
pub use libcsp_sys as ffi;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ReservedPort {
    Cmp = 0,
    Ping = 1,
    Ps = 2,
    Memfree = 3,
    Reboot = 4,
    BufFree = 5,
    Uptime = 6,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum CspError {
    None = 0,
    NoMem = -1,
    Inval = -2,
    TimedOut = -3,
    Used = -4,
    NotSup = -5,
    Busy = -6,
    Already = -7,
    Reset = -8,
    NoBufs = -9,
    Tx = -10,
    Driver = -11,
    Again = -12,
    NoSys = -38,
    Hmac = -100,
    Crc32 = -102,
    Sfp = -103,
}

/// Listen on all ports, primarily used with [csp_bind]
pub const CSP_ANY: u8 = 255;
pub const CSP_LOOPBACK: u16 = 0;

bitflags! {
    pub struct SocketFlags: u32 {
        const NONE = 0x0000;
        /// RDP required.
        const RDPREQ = 0x0001;
        /// RDP prohibited.
        const RDPPROHIB = 0x0002;
        /// HMAC required
        const HMACREQ = 0x0004;
        /// HMAC prohibited.
        const HMACPROHIB = 0x0008;
        /// CRC32 required.
        const CRC32REQ = 0x0040;
        const CRC32PROHIB = 0x0080;
        const CONN_LESS = 0x0100;
        /// Copy opts from incoming packets. Only applies to [csp_sendto_reply]
        const SAME = 0x8000;

        // The source may set any bits
        const _ = !0;
    }
}

bitflags! {
    pub struct ConnectOpts: u32 {
        const NONE = SocketFlags::NONE.bits();

        const RDP = SocketFlags::RDPREQ.bits();
        const NORDP = SocketFlags::RDPPROHIB.bits();
        const HMAC = SocketFlags::HMACREQ.bits();
        const NOHMAC = SocketFlags::HMACPROHIB.bits();
        const CRC32 = SocketFlags::CRC32REQ.bits();
        const NOCRC32 = SocketFlags::CRC32PROHIB.bits();
        const SAME = SocketFlags::SAME.bits();

        // The source may set any bits
        const _ = !0;
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum ConnState {
    Closed = 0,
    Open = 1,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum ConnType {
    Client = 0,
    Server = 1,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum RdpState {
    Closed = 0,
    SynSent = 1,
    SynRcvd = 2,
    Open = 3,
    CloseWait = 4,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum MsgPriority {
    Critical = 0,
    High = 1,
    Normal = 2,
    Low = 3,
}

pub struct CspPacket(pub csp_packet_s);

pub struct CspPacketRef(*mut csp_packet_s);

pub struct CspPacketMut(*mut csp_packet_s);

impl From<CspPacketMut> for CspPacketRef {
    fn from(value: CspPacketMut) -> Self {
        Self(value.0)
    }
}

impl CspPacketRef {
    pub fn packet_data(&self) -> &[u8] {
        unsafe { &(&(*self.0).packet_data_union.data)[..self.packet_length()] }
    }

    pub fn whole_data(&self) -> &[u8; ffi::CSP_BUFFER_SIZE] {
        unsafe { &(*self.0).packet_data_union.data }
    }

    pub fn packet_length(&self) -> usize {
        unsafe { (*self.0).length.into() }
    }

    pub fn inner(&self) -> *const csp_packet_s {
        self.0
    }
}

pub struct CspPacketRefGuard(Option<CspPacketRef>);

impl Drop for CspPacketRefGuard {
    fn drop(&mut self) {
        if let Some(packet) = self.0.take() {
            csp_buffer_free(packet)
        }
    }
}

impl CspPacketRefGuard {
    /// Take the packet out of the guard, preventing it from being freed.
    pub fn take(mut self) -> CspPacketRef {
        self.0.take().unwrap()
    }
}

impl AsRef<CspPacketRef> for CspPacketRefGuard {
    fn as_ref(&self) -> &CspPacketRef {
        self.0.as_ref().unwrap()
    }
}

impl CspPacketMut {
    pub fn packet_data(&self) -> &[u8] {
        unsafe { &(&(*self.0).packet_data_union.data)[..self.packet_length()] }
    }

    pub fn whole_data(&self) -> &[u8; ffi::CSP_BUFFER_SIZE] {
        unsafe { &(*self.0).packet_data_union.data }
    }

    pub fn packet_length(&self) -> usize {
        unsafe { (*self.0).length.into() }
    }

    pub fn inner(&self) -> *const csp_packet_s {
        self.0
    }

    pub fn whole_data_mut(&mut self) -> &mut [u8; ffi::CSP_BUFFER_SIZE] {
        unsafe { &mut (*self.0).packet_data_union.data }
    }

    pub fn set_data(&mut self, data: &[u8]) -> bool {
        if data.len() > self.whole_data().len() {
            return false;
        }
        self.whole_data_mut()[0..data.len()].copy_from_slice(data);
        unsafe {
            (*self.0).length = data.len() as u16;
        }
        true
    }

    pub fn inner_mut(&self) -> *mut csp_packet_s {
        self.0
    }
}

impl CspPacket {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for CspPacket {
    fn default() -> Self {
        Self(csp_packet_s {
            timestamp_tx: Default::default(),
            conn: core::ptr::null_mut(),
            rx_count: Default::default(),
            remain: Default::default(),
            cfpid: Default::default(),
            last_used: Default::default(),
            frame_begin: core::ptr::null_mut(),
            frame_length: Default::default(),
            length: Default::default(),
            id: Default::default(),
            next: core::ptr::null_mut(),
            header: Default::default(),
            packet_data_union: Default::default(),
        })
    }
}

#[derive(Default)]
pub struct CspSocket(pub csp_socket_s);

impl CspSocket {
    pub fn inner_as_mut_ptr(&mut self) -> *mut csp_socket_s {
        &mut self.0
    }
}

/// Rust wrapper for [ffi::csp_init]. Initialize the CSP stack.
///
/// # Safety
///
/// - You must call this function only once.
pub unsafe fn csp_init() {
    // SAFETY: FFI call
    unsafe {
        ffi::csp_init();
    }
}

/// Rust wrapper for [ffi::csp_bind].
pub fn csp_bind(socket: &mut CspSocket, port: u8) {
    // SAFETY: FFI call
    unsafe {
        ffi::csp_bind(socket.inner_as_mut_ptr(), port);
    }
}

/// Rust wrapper for [ffi::csp_listen].
pub fn csp_listen(socket: &mut CspSocket, backlog: usize) {
    // SAFETY: FFI call
    unsafe {
        ffi::csp_listen(socket.inner_as_mut_ptr(), backlog);
    }
}

/// Rust wrapper for [ffi::csp_route_work].
pub fn csp_route_work_raw() -> i32 {
    unsafe { ffi::csp_route_work() }
}

/// Rust wrapper for [ffi::csp_route_work] which also converts errors to the [CspError] type.
/// This function will panic if the returned error type is not among the known values of
/// [CspError].
///
/// [csp_route_work_raw] can be used if this is not acceptable.
pub fn csp_route_work() -> Result<(), CspError> {
    let result = unsafe { ffi::csp_route_work() };
    if result == CspError::None as i32 {
        return Ok(());
    }
    Err(CspError::try_from(result)
        .unwrap_or_else(|_| panic!("unexpected error value {} from csp_route_work", result)))
}

#[derive(Debug, Copy, Clone)]
pub struct CspConnRef(*mut csp_conn_s);

impl CspConnRef {
    pub fn inner(&mut self) -> Option<&csp_conn_s> {
        // SAFETY: Raw pointer access, we return [None] if the pointers is NULL.
        unsafe { self.0.as_ref() }
    }

    pub fn inner_mut(&mut self) -> Option<&mut csp_conn_s> {
        // SAFETY: Raw pointer access, we return [None] if the pointers is NULL.
        unsafe { self.0.as_mut() }
    }
}

pub struct CspConnGuard(pub CspConnRef);

impl Drop for CspConnGuard {
    fn drop(&mut self) {
        csp_close(self.0);
    }
}

impl AsRef<CspConnRef> for CspConnGuard {
    fn as_ref(&self) -> &CspConnRef {
        &self.0
    }
}

impl AsMut<CspConnRef> for CspConnGuard {
    fn as_mut(&mut self) -> &mut CspConnRef {
        &mut self.0
    }
}

#[derive(Default)]
pub struct CspInterface(pub ffi::csp_iface_t);

impl CspInterface {
    pub fn new(host_addr: u16, is_default: bool) -> Self {
        Self(ffi::csp_iface_t {
            addr: host_addr,
            netmask: Default::default(),
            name: core::ptr::null(),
            interface_data: core::ptr::null_mut(),
            driver_data: core::ptr::null_mut(),
            nexthop: None,
            is_default: is_default as u8,
            tx: Default::default(),
            rx: Default::default(),
            tx_error: Default::default(),
            rx_error: Default::default(),
            drop: Default::default(),
            autherr: Default::default(),
            frame: Default::default(),
            txbytes: Default::default(),
            rxbytes: Default::default(),
            irq: Default::default(),
            next: core::ptr::null_mut(),
        })
    }
}

#[derive(Default)]
pub struct CspUdpConf(pub ffi::csp_if_udp_conf_t);

impl CspUdpConf {
    pub fn new(addr: &'static str, lport: u16, rport: u16) -> Self {
        Self(ffi::csp_if_udp_conf_t {
            host: addr.as_ptr() as *mut core::ffi::c_char,
            lport: lport.into(),
            rport: rport.into(),
            server_handle: Default::default(),
            peer_addr: libc::sockaddr_in {
                sin_family: Default::default(),
                sin_port: Default::default(),
                sin_addr: libc::in_addr {
                    s_addr: Default::default(),
                },
                sin_zero: Default::default(),
            },
            sockfd: Default::default(),
        })
    }
}

pub fn csp_accept_guarded(socket: &mut CspSocket, timeout: Duration) -> Option<CspConnGuard> {
    Some(CspConnGuard(csp_accept(socket, timeout)?))
}

/// Rust wrapper for [ffi::csp_accept].
pub fn csp_accept(socket: &mut CspSocket, timeout: Duration) -> Option<CspConnRef> {
    let timeout_millis = timeout.as_millis();
    if timeout_millis > u32::MAX as u128 {
        return None;
    }
    Some(CspConnRef(unsafe {
        let addr = ffi::csp_accept(socket.inner_as_mut_ptr(), timeout_millis as u32);
        if addr.is_null() {
            return None;
        }
        addr
    }))
}

/// Rust wrapper for [ffi::csp_socket_close] which returns the result code directly.
pub fn csp_socket_close_raw(sock: &mut CspSocket) -> i32 {
    unsafe { ffi::csp_socket_close(&mut sock.0) }
}

/// Rust wrapper for [ffi::csp_socket_close].
///
/// This function will panic if the error code returned from [ffi::csp_socket_close] is not one of
/// [CspError]. [csp_socket_close_raw] can be used if this is not acceptable.
pub fn csp_socket_close(sock: &mut CspSocket) -> Result<(), CspError> {
    let result = unsafe { ffi::csp_socket_close(&mut sock.0) };
    Err(CspError::try_from(result)
        .unwrap_or_else(|_| panic!("unexpected error value {} from csp_socket_close", result)))
}

/// Rust wrapper for [ffi::csp_read].
pub fn csp_read(conn: &mut CspConnRef, timeout: Duration) -> Option<CspPacketRef> {
    let timeout_millis = timeout.as_millis();
    if timeout_millis > u32::MAX as u128 {
        return None;
    }
    let opt_packet = unsafe { ffi::csp_read(conn.0, timeout_millis as u32) };
    if opt_packet.is_null() {
        return None;
    }
    // SAFETY: FFI pointer.
    Some(CspPacketRef(unsafe { &mut *opt_packet }))
}

/// Rust wrapper for [ffi::csp_read] which returns a guarded packet reference. This packet
/// will cleaned up automatically with [csp_buffer_free] on drop.
pub fn csp_read_guarded(conn: &mut CspConnRef, timeout: Duration) -> Option<CspPacketRefGuard> {
    Some(CspPacketRefGuard(Some(csp_read(conn, timeout)?)))
}

/// Rust wrapper for [ffi::csp_recvfrom].
pub fn csp_recvfrom(socket: &mut CspSocket, timeout: u32) -> Option<CspPacketRef> {
    let opt_packet = unsafe { ffi::csp_recvfrom(&mut socket.0, timeout) };
    if opt_packet.is_null() {
        return None;
    }
    Some(CspPacketRef(unsafe { &mut *opt_packet }))
}

/// Rust wrapper for [ffi::csp_recvfrom] which returns a guarded packet reference. This packet
/// will cleaned up automatically with [csp_buffer_free] on drop.
pub fn csp_recvfrom_guarded(socket: &mut CspSocket, timeout: u32) -> Option<CspPacketRefGuard> {
    Some(CspPacketRefGuard(Some(csp_recvfrom(socket, timeout)?)))
}

/// Rust wrapper for [ffi::csp_conn_dport].
pub fn csp_conn_dport(conn: &CspConnRef) -> i32 {
    // SAFETY: FFI call.
    unsafe { ffi::csp_conn_dport(conn.0) }
}

/// Rust wrapper for [ffi::csp_conn_sport].
pub fn csp_conn_sport(conn: &CspConnRef) -> i32 {
    // SAFETY: FFI call.
    unsafe { ffi::csp_conn_sport(conn.0) }
}

/// Rust wrapper for [ffi::csp_conn_dst].
pub fn csp_conn_dst(conn: &CspConnRef) -> i32 {
    // SAFETY: FFI call.
    unsafe { ffi::csp_conn_dst(conn.0) }
}

/// Rust wrapper for [ffi::csp_conn_src].
pub fn csp_conn_src(conn: &CspConnRef) -> i32 {
    // SAFETY: FFI call.
    unsafe { ffi::csp_conn_src(conn.0) }
}

/// Rust wrapper for [ffi::csp_conn_src].
pub fn csp_conn_flags(conn: &CspConnRef) -> i32 {
    // SAFETY: FFI call.
    unsafe { ffi::csp_conn_flags(conn.0) }
}

/// Rust wrapper for [ffi::csp_conn_src] which also tries to convert the options to
/// a [ConnectOpts] bitfield.
pub fn csp_conn_flags_typed(conn: &CspConnRef) -> Option<ConnectOpts> {
    let flags_raw = csp_conn_flags(conn);
    if flags_raw < 0 {
        return None;
    }
    // SAFETY: FFI call.
    ConnectOpts::from_bits(flags_raw as u32)
}

pub fn csp_service_handler(packet: CspPacketRef) {
    // SAFETY: FFI call.
    unsafe { ffi::csp_service_handler(&mut *packet.0) }
}

/// Rust wrapper for [ffi::csp_close].
pub fn csp_close(conn: CspConnRef) -> i32 {
    // SAFETY: FFI call.
    unsafe { ffi::csp_close(conn.0) }
}

/// Rust wrapper for [ffi::csp_ping], returns the result code directly.
pub fn csp_ping_raw(node: u16, timeout: Duration, size: usize, opts: SocketFlags) -> i32 {
    // SAFETY: FFI call.
    unsafe {
        ffi::csp_ping(
            node,
            timeout.as_millis() as u32,
            size as u32,
            opts.bits() as u8,
        )
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct PingError;

/// Rust wrapper for [ffi::csp_ping].
pub fn csp_ping(
    node: u16,
    timeout: Duration,
    size: usize,
    opts: SocketFlags,
) -> Result<Duration, PingError> {
    let result = csp_ping_raw(node, timeout, size, opts);
    if result < 0 {
        return Err(PingError);
    }
    Ok(Duration::from_millis(result as u64))
}

/// Rust wrapper for [ffi::csp_reboot].
pub fn csp_reboot(node: u16) {
    // SAFETY: FFI call.
    unsafe { ffi::csp_reboot(node) }
}

/// Rust wrapper for [ffi::csp_connect].
pub fn csp_connect(
    prio: MsgPriority,
    dst: u16,
    dst_port: u8,
    timeout: Duration,
    opts: ConnectOpts,
) -> Option<CspConnRef> {
    // SAFETY: FFI call.
    let conn = unsafe {
        ffi::csp_connect(
            prio as u8,
            dst,
            dst_port,
            timeout.as_millis() as u32,
            opts.bits(),
        )
    };
    if conn.is_null() {
        return None;
    }
    // SAFETY: We checked that the pointer is valid.
    Some(CspConnRef(conn))
}

/// Rust wrapper for [ffi::csp_connect] which returns a guard structure. The connection will be
/// be closed automatically when the guard structure is dropped.
pub fn csp_connect_guarded(
    prio: MsgPriority,
    dst: u16,
    dst_port: u8,
    timeout: Duration,
    opts: ConnectOpts,
) -> Option<CspConnGuard> {
    Some(CspConnGuard(csp_connect(
        prio, dst, dst_port, timeout, opts,
    )?))
}

/// Rust wrapper for [ffi::csp_buffer_get].
pub fn csp_buffer_get() -> Option<CspPacketMut> {
    let packet_ref = unsafe {
        // The size argument is unused
        ffi::csp_buffer_get(0)
    };
    if packet_ref.is_null() {
        return None;
    }
    // SAFETY: We checked that the pointer is valid.
    Some(CspPacketMut(unsafe { &mut *packet_ref }))
}

/// Rust wrapper for [ffi::csp_send].
pub fn csp_send(conn: &mut CspConnRef, packet: impl Into<CspPacketRef>) {
    // SAFETY: FFI call.
    unsafe { ffi::csp_send(conn.0, packet.into().0) }
}

/// Rust wrapper for [ffi::csp_conn_print_table].
pub fn csp_conn_print_table() {
    // SAFETY: FFI call.
    unsafe { ffi::csp_conn_print_table() }
}

/// Rust wrapper for [ffi::csp_buffer_free].
pub fn csp_buffer_free(packet: impl Into<CspPacketRef>) {
    // SAFETY: FFI call and the Rust type system actually ensure the correct type
    // is free'd here, while also taking the packet by value.
    unsafe { ffi::csp_buffer_free(packet.into().0 as *const libc::c_void) }
}

/// Rust wrapper for [ffi::csp_transaction_persistent].
///
/// # Parameters
///
/// * `in_len`: Use [None] if the length is unknown, and the expected reply length otherwise.
///
/// # Returns
///
/// 1 or reply size on success, 0 otherwise.
pub fn csp_transaction_persistent(
    conn: &mut CspConnRef,
    timeout: Duration,
    out_data: &[u8],
    in_data: &mut [u8],
    in_len: Option<usize>,
) -> i32 {
    unsafe {
        ffi::csp_transaction_persistent(
            conn.0,
            timeout.as_millis() as u32,
            out_data.as_ptr() as *const core::ffi::c_void,
            out_data.len() as i32,
            in_data.as_ptr() as *mut core::ffi::c_void,
            in_len.map(|v| v as i32).unwrap_or(-1),
        )
    }
}

/// Rust wrapper for [ffi::csp_transaction_w_opts].
///
/// # Parameters
///
/// * `in_len`: Use [None] if the length is unknown, and the expected reply length otherwise.
///
/// # Returns
///
/// 1 or reply size on success, 0 otherwise.
#[allow(clippy::too_many_arguments)]
pub fn csp_transaction_w_opts(
    prio: MsgPriority,
    dst: u16,
    dst_port: u8,
    timeout: Duration,
    out_data: &[u8],
    in_data: &mut [u8],
    in_len: Option<usize>,
    opts: ConnectOpts,
) -> i32 {
    unsafe {
        ffi::csp_transaction_w_opts(
            prio as u8,
            dst,
            dst_port,
            timeout.as_millis() as u32,
            out_data.as_ptr() as *const core::ffi::c_void,
            out_data.len() as i32,
            in_data.as_ptr() as *mut core::ffi::c_void,
            in_len.map(|v| v as i32).unwrap_or(-1),
            opts.bits(),
        )
    }
}

/// Calls [csp_transaction_w_opts] with [ConnectOpts::NONE].
pub fn csp_transaction(
    prio: MsgPriority,
    dst: u16,
    dst_port: u8,
    timeout: Duration,
    out_data: &[u8],
    in_data: &mut [u8],
    in_len: Option<usize>,
) -> i32 {
    csp_transaction_w_opts(
        prio,
        dst,
        dst_port,
        timeout,
        out_data,
        in_data,
        in_len,
        ConnectOpts::NONE,
    )
}

pub mod udp {
    use super::*;

    /// Rust wrapper for [ffi::udp::csp_if_udp_init].
    pub fn csp_if_udp_init(iface: &mut CspInterface, ifconf: &mut CspUdpConf) {
        unsafe { ffi::udp::csp_if_udp_init(&mut iface.0, &mut ifconf.0) }
    }
}

pub mod iflist {
    use super::*;

    /// Rust wrapper for [ffi::iflist::csp_iflist_print].
    pub fn csp_iflist_print() {
        // SAFETY: FFI call.
        unsafe { ffi::iflist::csp_iflist_print() }
    }

    /// Rust wrapper for [ffi::iflist::csp_iflist_add].
    pub fn csp_iflist_add(iface: &mut CspInterface) -> i32 {
        unsafe { ffi::iflist::csp_iflist_add(&mut iface.0) }
    }
}
