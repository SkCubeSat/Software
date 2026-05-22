//! This crate provides FFI bindings for the [`libcsp` library](https://github.com/libcsp/libcsp).

//! Generally, you probably do not want to use this library directly and instead use the
//! [`libcsp`](https://crates.io/crates/libcsp) Rust crate which provides a safe and ergonomic Rust
//! API.
//!
//! You can find some more high-level information and examples in the
//! [main repository](https://egit.irs.uni-stuttgart.de/rust/libcsp-rust).
//!
//! ## Compile-time configuration
//!
//! This library requires some compile-time configuration file to be included to work
//! properly. You can see an example version of the file for the workspace
//! [here](https://egit.irs.uni-stuttgart.de/rust/libcsp-rust/src/branch/main/examples/autoconfig.rs).
//! The user has to provide the path to a directory containing this `autoconfig.rs` file using the
//! `CSP_CONFIG_DIR` environmental variable.
#![no_std]
#![cfg_attr(docs_rs, feature(doc_auto_cfg))]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod config;

// This file will be created by the build script by copying a user-provided file to the output
// directory. It contains important compile time constants. Compilation of the library is not
// possible without these constants.
include!(concat!(env!("OUT_DIR"), "/autoconfig.rs"));

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct csp_timestamp_t {
    pub tv_sec: u32,
    pub tv_nsec: u32,
}

#[doc = "CSP identifier/header."]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct csp_id_t {
    pub pri: u8,
    pub flags: u8,
    pub src: u16,
    pub dst: u16,
    pub dport: u8,
    pub sport: u8,
}

#[doc = " CSP Packet.\n\n This structure is constructed to fit with all interface and protocols to prevent the\n need to copy data (zero copy).\n\n .. note:: In most cases a CSP packet cannot be reused in case of send failure, because the\n \t\t\t lower layers may add additional data causing increased length (e.g. CRC32), convert\n \t\t\t the CSP id to different endian (e.g. I2C), etc.\n"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct csp_packet_s {
    pub packet_info: csp_packet_s_anon_union,
    pub length: u16,
    pub id: csp_id_t,
    pub next: *mut csp_packet_s,
    #[doc = " Additional header bytes, to prepend packed data before transmission\n This must be minimum 6 bytes to accomodate CSP 2.0. But some implementations\n require much more scratch working area for encryption for example.\n\n Ultimately after csp_id_pack() this area will be filled with the CSP header"]
    pub header: [u8; 8usize],
    pub packet_data_union: csp_packet_s_data_union,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union csp_packet_s_anon_union {
    pub rdp_only: csp_packet_s_anon_union_field_rdp_only,
    pub rx_tx_only: csp_packet_s_anon_union_field_rx_tx_only,
}

impl Default for csp_packet_s_anon_union {
    fn default() -> Self {
        Self {
            rdp_only: Default::default(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct csp_packet_s_anon_union_field_rdp_only {
    pub rdp_quarantine: u32,
    pub timestamp_tx: u32,
    pub timestamp_rx: u32,
    pub conn: *mut csp_conn_s,
}

impl Default for csp_packet_s_anon_union_field_rdp_only {
    fn default() -> Self {
        Self {
            rdp_quarantine: Default::default(),
            timestamp_tx: Default::default(),
            timestamp_rx: Default::default(),
            conn: core::ptr::null_mut(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct csp_packet_s_anon_union_field_rx_tx_only {
    pub rx_count: u16,
    pub remain: u16,
    pub cfpid: u32,
    pub last_used: u32,
    pub frame_begin: *mut u8,
    pub frame_length: u16,
}

impl Default for csp_packet_s_anon_union_field_rx_tx_only {
    fn default() -> Self {
        Self {
            rx_count: Default::default(),
            remain: Default::default(),
            cfpid: Default::default(),
            last_used: Default::default(),
            frame_begin: core::ptr::null_mut(),
            frame_length: Default::default(),
        }
    }
}

#[doc = " Data part of packet:"]
#[repr(C)]
#[derive(Copy, Clone)]
pub union csp_packet_s_data_union {
    pub data: [u8; CSP_BUFFER_SIZE],
    pub data16: [u16; CSP_BUFFER_SIZE / 2usize],
    pub data32: [u32; CSP_BUFFER_SIZE / 4usize],
}

impl Default for csp_packet_s_data_union {
    fn default() -> Self {
        Self {
            data: [0; CSP_BUFFER_SIZE],
        }
    }
}

#[doc = " CSP Packet.\n\n This structure is constructed to fit with all interface and protocols to prevent the\n need to copy data (zero copy).\n\n .. note:: In most cases a CSP packet cannot be reused in case of send failure, because the\n \t\t\t lower layers may add additional data causing increased length (e.g. CRC32), convert\n \t\t\t the CSP id to different endian (e.g. I2C), etc.\n"]
pub type csp_packet_t = csp_packet_s;

pub type csp_queue_handle_t = *mut core::ffi::c_void;
pub type csp_static_queue_t = *mut core::ffi::c_void;

#[doc = " @brief Connection struct"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct csp_socket_s {
    pub rx_queue: csp_queue_handle_t,
    pub rx_queue_static: csp_static_queue_t,
    pub rx_queue_static_data:
        [core::ffi::c_char; CSP_CONN_RXQUEUE_LEN * core::mem::size_of::<*const csp_packet_s>()],
    pub opts: u32,
}

impl Default for csp_socket_s {
    fn default() -> Self {
        Self {
            rx_queue: core::ptr::null_mut(),
            rx_queue_static: core::ptr::null_mut(),
            rx_queue_static_data: [0; CSP_CONN_RXQUEUE_LEN
                * core::mem::size_of::<*const csp_packet_s>()],
            opts: Default::default(),
        }
    }
}

#[doc = " Forward declaration of socket structure"]
pub type csp_socket_t = csp_socket_s;

#[doc = " Forward declaration of connection structure"]
pub type csp_conn_t = csp_conn_s;

pub type atomic_int = u32;

#[doc = " Connection states"]
pub type csp_conn_state_t = ::core::ffi::c_uint;

#[doc = " Connection types"]
pub type csp_conn_type_t = ::core::ffi::c_uint;

#[doc = " RDP Connection states"]
pub type csp_rdp_state_t = ::core::ffi::c_uint;

#[cfg(unix)]
pub type csp_bin_sem_t = libc::sem_t;

#[doc = " RDP Connection"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct csp_rdp_t {
    #[doc = "< Connection state"]
    pub state: csp_rdp_state_t,
    #[doc = "< Tracks 'who' have closed the RDP connection"]
    pub closed_by: u8,
    #[doc = "< The sequence number of the next segment that is to be sent"]
    pub snd_nxt: u16,
    #[doc = "< The sequence number of the oldest unacknowledged segment"]
    pub snd_una: u16,
    #[doc = "< The initial send sequence number"]
    pub snd_iss: u16,
    #[doc = "< The sequence number of the last segment received correctly and in sequence"]
    pub rcv_cur: u16,
    #[doc = "< The initial receive sequence number"]
    pub rcv_irs: u16,
    #[doc = "< The last sequence number acknowledged by the receiver"]
    pub rcv_lsa: u16,
    pub window_size: u32,
    pub conn_timeout: u32,
    pub packet_timeout: u32,
    pub delayed_acks: u32,
    pub ack_timeout: u32,
    pub ack_delay_count: u32,
    pub ack_timestamp: u32,
    pub tx_wait: csp_bin_sem_t,
}

#[doc = " @brief Connection struct"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct csp_conn_s {
    pub type_: atomic_int,
    pub state: atomic_int,
    pub idin: csp_id_t,
    pub idout: csp_id_t,
    pub sport_outgoing: u8,
    pub rx_queue: csp_queue_handle_t,
    pub rx_queue_static: csp_static_queue_t,
    pub rx_queue_static_data:
        [core::ffi::c_char; CSP_CONN_RXQUEUE_LEN * core::mem::size_of::<*const csp_packet_s>()],
    pub callback: ::core::option::Option<unsafe extern "C" fn(packet: *mut csp_packet_t)>,
    pub dest_socket: *mut csp_socket_t,
    pub timestamp: u32,
    pub opts: u32,
    pub rdp: csp_rdp_t,
}

#[doc = " Interface Tx function.\n\n @return #CSP_ERR_NONE on success, otherwise an error code."]
pub type nexthop_t = ::core::option::Option<
    unsafe extern "C" fn(
        iface: *mut csp_iface_t,
        via: u16,
        packet: *mut csp_packet_t,
        from_me: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int,
>;

#[doc = " This struct is referenced in documentation.\n Update doc when you change this."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct csp_iface_s {
    #[doc = "< Host address on this subnet"]
    pub addr: u16,
    #[doc = "< Subnet mask"]
    pub netmask: u16,
    #[doc = "< Name, max compare length is #CSP_IFLIST_NAME_MAX"]
    pub name: *const ::core::ffi::c_char,
    #[doc = "< Interface data, only known/used by the interface layer, e.g. state information."]
    pub interface_data: *mut ::core::ffi::c_void,
    #[doc = "< Driver data, only known/used by the driver layer, e.g. device/channel references."]
    pub driver_data: *mut ::core::ffi::c_void,
    #[doc = "< Next hop (Tx) function"]
    pub nexthop: nexthop_t,
    #[doc = "< Set default IF flag (CSP supports multiple defaults)"]
    pub is_default: u8,
    #[doc = "< Successfully transmitted packets"]
    pub tx: u32,
    #[doc = "< Successfully received packets"]
    pub rx: u32,
    #[doc = "< Transmit errors (packets)"]
    pub tx_error: u32,
    #[doc = "< Receive errors, e.g. too large message"]
    pub rx_error: u32,
    #[doc = "< Dropped packets"]
    pub drop: u32,
    #[doc = "< Authentication errors (packets)"]
    pub autherr: u32,
    #[doc = "< Frame format errors (packets)"]
    pub frame: u32,
    #[doc = "< Transmitted bytes"]
    pub txbytes: u32,
    #[doc = "< Received bytes"]
    pub rxbytes: u32,
    #[doc = "< Interrupts"]
    pub irq: u32,
    pub next: *mut csp_iface_s,
}

impl Default for csp_iface_s {
    fn default() -> Self {
        Self {
            addr: Default::default(),
            netmask: Default::default(),
            name: core::ptr::null(),
            interface_data: core::ptr::null_mut(),
            driver_data: core::ptr::null_mut(),
            nexthop: None,
            is_default: Default::default(),
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
        }
    }
}

#[doc = " Forward declaration of CSP interface, see #csp_iface_s for details."]
pub type csp_iface_t = csp_iface_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct csp_if_udp_conf_t {
    pub host: *mut ::core::ffi::c_char,
    pub lport: ::core::ffi::c_int,
    pub rport: ::core::ffi::c_int,
    pub server_handle: libc::pthread_t,
    pub peer_addr: libc::sockaddr_in,
    pub sockfd: ::core::ffi::c_int,
}

impl Default for csp_if_udp_conf_t {
    fn default() -> Self {
        Self {
            host: core::ptr::null_mut(),
            lport: Default::default(),
            rport: Default::default(),
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
        }
    }
}

extern "C" {
    #[doc = " Error counters"]
    pub static mut csp_dbg_buffer_out: u8;
    pub static mut csp_dbg_conn_out: u8;
    pub static mut csp_dbg_conn_ovf: u8;
    pub static mut csp_dbg_conn_noroute: u8;
    pub static mut csp_dbg_inval_reply: u8;
    pub static mut csp_dbg_errno: u8;
    pub static mut csp_dbg_can_errno: u8;
    pub static mut csp_dbg_eth_errno: u8;
    pub static mut csp_dbg_rdp_print: u8;
    pub static mut csp_dbg_packet_print: u8;

    #[doc = " Initialize CSP.\n This will configure basic structures."]
    pub fn csp_init();

    pub fn csp_print_func(fmt: *const core::ffi::c_char, ...);

    #[doc = " Bind port to socket.\n\n @param[in] socket socket to bind port to\n @param[in] port port number to bind, use #CSP_ANY for all ports. Bindnig to a specific will take precedence over #CSP_ANY.\n @return #CSP_ERR_NONE on success, otherwise an error code."]
    pub fn csp_bind(socket: *mut csp_socket_t, port: u8) -> core::ffi::c_int;

    #[doc = " Set socket to listen for incoming connections.\n\n @param[in] socket socket\n @param[in] backlog max length of backlog queue. The backlog queue holds incoming connections, waiting to be returned by call to csp_accept().\n @return #CSP_ERR_NONE on success, otherwise an error code."]
    pub fn csp_listen(socket: *mut csp_socket_t, backlog: usize) -> ::core::ffi::c_int;

    #[doc = " Route packet from the incoming router queue and check RDP timeouts.\n In order for incoming packets to routed and RDP timeouts to be checked, this function must be called reguarly.\n @return #CSP_ERR_NONE on success, otherwise an error code."]
    pub fn csp_route_work() -> ::core::ffi::c_int;

    #[doc = " Wait/accept a new connection.\n\n @param[in] socket socket to accept connections on, created by calling csp_socket().\n @param[in] timeout  timeout in mS to wait for a connection, use CSP_MAX_TIMEOUT for infinite timeout.\n @return New connection on success, NULL on failure or timeout."]
    pub fn csp_accept(socket: *mut csp_socket_t, timeout: u32) -> *mut csp_conn_t;

    #[doc = " Read packet from a connection.\n This fuction will wait on the connection's RX queue for the specified timeout.\n\n @param[in] conn  connection\n @param[in] timeout timeout in mS to wait for a packet, use CSP_MAX_TIMEOUT for infinite timeout.\n @return Packet or NULL in case of failure or timeout."]
    pub fn csp_read(conn: *mut csp_conn_t, timeout: u32) -> *mut csp_packet_t;

    #[doc = " Send packet on a connection.\n The packet buffer is automatically freed, and cannot be used after the call to csp_send()\n\n @param[in] conn connection\n @param[in] packet packet to send"]
    pub fn csp_send(conn: *mut csp_conn_t, packet: *mut csp_packet_t);

    #[doc = " Change the default priority of the connection and send a packet.\n\n .. note:: The priority of the connection will be changed.\n           If you need to change it back, call csp_send_prio() again.\n\n @param[in] prio priority to set on the connection\n @param[in] conn connection\n @param[in] packet packet to send"]
    pub fn csp_send_prio(prio: u8, conn: *mut csp_conn_t, packet: *mut csp_packet_t);

    #[doc = " Send a packet as a reply to a request (without a connection).\n Calls csp_sendto() with the source address and port from the request.\n\n @param[in] request incoming request\n @param[out] reply reply packet\n @param[in] opts connection options, see @ref CSP_CONNECTION_OPTIONS."]
    pub fn csp_sendto_reply(request: *const csp_packet_t, reply: *mut csp_packet_t, opts: u32);

    #[doc = " Read data from a connection-less server socket.\n\n @param[in] socket connection-less socket.\n @param[in] timeout timeout in mS to wait for a packet, use #CSP_MAX_TIMEOUT for infinite timeout.\n @return Packet on success, or NULL on failure or timeout."]
    pub fn csp_recvfrom(socket: *mut csp_socket_t, timeout: u32) -> *mut csp_packet_t;

    #[doc = " Perform an entire request & reply transaction.\n Creates a connection, send \\a outbuf, wait for reply, copy reply to \\a inbuf and close the connection.\n\n @param[in] prio priority, see #csp_prio_t\n @param[in] dst destination address\n @param[in] dst_port destination port\n @param[in] timeout timeout in mS to wait for a reply\n @param[in] outbuf outgoing data (request)\n @param[in] outlen length of data in \\a outbuf (request)\n @param[out] inbuf user provided buffer for receiving data (reply)\n @param[in] inlen length of expected reply, -1 for unknown size (inbuf MUST be large enough), 0 for no reply.\n @param[in] opts connection options, see @ref CSP_CONNECTION_OPTIONS.\n\n Returns:\n   int: 1 or reply size on success, 0 on failure (error, incoming length does not match, timeout)"]
    pub fn csp_transaction_w_opts(
        prio: u8,
        dst: u16,
        dst_port: u8,
        timeout: u32,
        outbuf: *const ::core::ffi::c_void,
        outlen: ::core::ffi::c_int,
        inbuf: *mut ::core::ffi::c_void,
        inlen: ::core::ffi::c_int,
        opts: u32,
    ) -> ::core::ffi::c_int;

    #[doc = " Handle CSP service request.\n If the given packet is a service-request (the destination port matches one of CSP service ports #csp_service_port_t),\n the packet will be processed by the specific CSP service handler.\n The packet will either process it or free it, so this function is typically called in the last \"default\" clause of\n a switch/case statement in a CSP listener task.\n In order to listen to csp service ports, bind your listener to the specific services ports #csp_service_port_t or\n use #CSP_ANY to all ports.\n\n @param[in] packet first packet, obtained by using csp_read()"]
    pub fn csp_service_handler(packet: *mut csp_packet_t);

    #[doc = " Close an open connection.\n Any packets in the RX queue will be freed.\n\n @param[in] conn connection. Closing a NULL connection is acceptable.\n @return #CSP_ERR_NONE on success, otherwise an error code."]
    pub fn csp_close(conn: *mut csp_conn_t) -> ::core::ffi::c_int;

    #[doc = " Send a single ping/echo packet.\n\n @param[in] node address of subsystem.\n @param[in] timeout timeout in ms to wait for reply.\n @param[in] size payload size in bytes.\n @param[in] opts connection options, see @ref CSP_CONNECTION_OPTIONS.\n @return >=0 echo time in mS on success, otherwise -1 for error."]
    pub fn csp_ping(
        node: u16,
        timeout: u32,
        size: ::core::ffi::c_uint,
        opts: u8,
    ) -> ::core::ffi::c_int;

    #[doc = " Send a single ping/echo packet without waiting for reply.\n Payload is 1 byte.\n\n @param[in] node address of subsystem."]
    pub fn csp_ping_noreply(node: u16);

    #[doc = " Request process list.\n\n .. note:: This is currently only supported on FreeRTOS systems.\n\n @param[in] node address of subsystem.\n @param[in] timeout timeout in mS to wait for replies. The function will not return until the timeout occurrs."]
    pub fn csp_ps(node: u16, timeout: u32);

    #[doc = " Request free memory.\n\n @param[in] node address of subsystem.\n @param[in] timeout timeout in mS to wait for reply.\n @param[out] size free memory on subsystem.\n @return #CSP_ERR_NONE on success, otherwise an error code."]
    pub fn csp_get_memfree(node: u16, timeout: u32, size: *mut u32) -> ::core::ffi::c_int;

    #[doc = " Request free memory and print to stdout.\n\n @param[in] node address of subsystem.\n @param[in] timeout timeout in mS to wait for reply."]
    pub fn csp_memfree(node: u16, timeout: u32);

    #[doc = " Request free buffers.\n\n @param[in] node address of subsystem.\n @param[in] timeout timeout in mS to wait for reply.\n @param[out] size free buffers.\n @return #CSP_ERR_NONE on success, otherwise an error code."]
    pub fn csp_get_buf_free(node: u16, timeout: u32, size: *mut u32) -> ::core::ffi::c_int;

    #[doc = " Request free buffers and print to stdout.\n\n @param[in] node address of subsystem.\n @param[in] timeout timeout in mS to wait for reply."]
    pub fn csp_buf_free(node: u16, timeout: u32);

    #[doc = " Reboot subsystem.\n If handled by the standard CSP service handler, the reboot handler set by csp_sys_set_reboot() on the subsystem, will be invoked.\n\n @param[in] node address of subsystem.\n"]
    pub fn csp_reboot(node: u16);

    #[doc = " Shutdown subsystem.\n If handled by the standard CSP service handler, the shutdown handler set by csp_sys_set_shutdown() on the subsystem, will be invoked.\n\n @param[in] node address of subsystem.\n"]
    pub fn csp_shutdown(node: u16);

    #[doc = " Request uptime and print to stdout.\n\n @param[in] node address of subsystem.\n @param[in] timeout timeout in mS to wait for reply.\n"]
    pub fn csp_uptime(node: u16, timeout: u32);

    #[doc = " Request uptime\n\n @param[in] node address of subsystem.\n @param[in] timeout timeout in mS to wait for reply.\n @param[out] uptime uptime in seconds.\n @return #CSP_ERR_NONE on success, otherwise an error code."]
    pub fn csp_get_uptime(node: u16, timeout: u32, uptime: *mut u32) -> ::core::ffi::c_int;

    #[doc = " Perform an entire request & reply transaction on an existing connection.\n Send \\a outbuf, wait for reply and copy reply to \\a inbuf.\n\n @param[in] conn connection\n @param[in] timeout timeout in mS to wait for a reply\n @param[in] outbuf outgoing data (request)\n @param[in] outlen length of data in \\a outbuf (request)\n @param[out] inbuf user provided buffer for receiving data (reply)\n @param[in] inlen length of expected reply, -1 for unknown size (inbuf MUST be large enough), 0 for no reply.\n @return 1 or reply size on success, 0 on failure (error, incoming length does not match, timeout)"]
    pub fn csp_transaction_persistent(
        conn: *mut csp_conn_t,
        timeout: u32,
        outbuf: *const ::core::ffi::c_void,
        outlen: ::core::ffi::c_int,
        inbuf: *mut ::core::ffi::c_void,
        inlen: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;

    #[doc = " Establish outgoing connection.\n The call will return immediately, unless it is a RDP connection (#CSP_O_RDP) in which case it will wait until the other\n end acknowleges the connection (timeout is determined by the current connection timeout set by csp_rdp_set_opt()).\n\n @param[in] prio priority, see #csp_prio_t\n @param[in] dst Destination address\n @param[in] dst_port Destination port\n @param[in] timeout unused.\n @param[in] opts connection options, see @ref CSP_CONNECTION_OPTIONS.\n @return Established connection or NULL on failure (no free connections, timeout)."]
    pub fn csp_connect(
        prio: u8,
        dst: u16,
        dst_port: u8,
        timeout: u32,
        opts: u32,
    ) -> *mut csp_conn_t;

    #[doc = " Close a socket, freeing it's RX queue and unbinding it from the associated\n port.\n\n @param[in] sock Socket\n @return #CSP_ERR_NONE on success, otherwise an error code."]
    pub fn csp_socket_close(sock: *mut csp_socket_t) -> ::core::ffi::c_int;

    #[doc = " Return destination port of connection.\n\n @param[in] conn connection\n @return destination port of an incoming connection"]
    pub fn csp_conn_dport(conn: *const csp_conn_t) -> ::core::ffi::c_int;
    #[doc = " Return source port of connection.\n\n @param[in] conn connection\n @return source port of an incoming connection"]
    pub fn csp_conn_sport(conn: *const csp_conn_t) -> ::core::ffi::c_int;
    #[doc = " Return destination address of connection.\n\n @param[in] conn connection\n @return destination address of an incoming connection"]
    pub fn csp_conn_dst(conn: *const csp_conn_t) -> ::core::ffi::c_int;
    #[doc = " Return source address of connection.\n\n @param[in] conn connection\n @return source address of an incoming connection"]
    pub fn csp_conn_src(conn: *const csp_conn_t) -> ::core::ffi::c_int;
    #[doc = " Return flags of connection.\n\n @param[in] conn connection\n @return flags of an incoming connection, see @ref CSP_HEADER_FLAGS"]
    pub fn csp_conn_flags(conn: *const csp_conn_t) -> ::core::ffi::c_int;

    #[doc = " Get free buffer from task context.\n\n @param[in] unused OBSOLETE ignored field, csp packets have a fixed size now\n @return Buffer pointer to #csp_packet_t or NULL if no buffers available"]
    pub fn csp_buffer_get(unused: usize) -> *mut csp_packet_t;

    #[doc = " Free buffer (from task context).\n\n @param[in] buffer buffer to free. NULL is handled gracefully."]
    pub fn csp_buffer_free(buffer: *const ::core::ffi::c_void);

    #[doc = " Print connection table to stdout."]
    pub fn csp_conn_print_table();
}

pub mod iflist {
    use super::*;

    extern "C" {
        #[doc = " Add interface to the list.\n\n @param[in] iface The interface must remain valid as long as the application is running.\n @return #CSP_ERR_NONE on success, otherwise an error code."]
        pub fn csp_iflist_add(iface: *mut csp_iface_t) -> ::core::ffi::c_int;

        pub fn csp_iflist_print();

    }
}

pub mod udp {
    use super::*;

    extern "C" {

        #[doc = " Setup UDP peer\n\n RX task:\n   A server task will attempt at binding to ip 0.0.0.0 port 9600\n   If this fails, it is because another udp server is already running.\n   The server task will continue attemting the bind and will not exit before the application is closed.\n\n TX peer:\n   Outgoing CSP packets will be transferred to the peer specified by the host argument"]
        pub fn csp_if_udp_init(iface: *mut csp_iface_t, ifconf: *mut csp_if_udp_conf_t);

    }
}

/// Hook module for CSP.
///
/// You can override these methods by providing them with an implementation block in your
/// application for library with the function signature specified in this module.
pub mod hooks {
    use super::*;

    extern "C" {
        pub fn csp_output_hook(
            idout: *mut csp_id_t,
            packet: *mut csp_packet_t,
            iface: *mut csp_iface_t,
            via: u16,
            from_me: ::core::ffi::c_int,
        );
        pub fn csp_input_hook(iface: *mut csp_iface_t, packet: *mut csp_packet_t);
        pub fn csp_reboot_hook();
        pub fn csp_shutdown_hook();
        pub fn csp_memfree_hook() -> u32;
        pub fn csp_ps_hook(packet: *mut csp_packet_t) -> ::core::ffi::c_uint;
        #[doc = " Implement these, if you use csp_if_tun"]
        pub fn csp_crypto_decrypt(
            ciphertext_in: *mut u8,
            ciphertext_len: u8,
            msg_out: *mut u8,
        ) -> ::core::ffi::c_int;
        pub fn csp_crypto_encrypt(
            msg_begin: *mut u8,
            msg_len: u8,
            ciphertext_out: *mut u8,
        ) -> ::core::ffi::c_int;
        pub fn csp_clock_get_time(time: *mut csp_timestamp_t);
        pub fn csp_clock_set_time(time: *const csp_timestamp_t) -> ::core::ffi::c_int;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, size_of, MaybeUninit};

    #[test]
    fn bindgen_test_layout_csp_timestamp_t() {
        const UNINIT: MaybeUninit<csp_timestamp_t> = MaybeUninit::uninit();
        let ptr = UNINIT.as_ptr();
        assert_eq!(
            ::core::mem::size_of::<csp_timestamp_t>(),
            8usize,
            concat!("Size of: ", stringify!(csp_timestamp_t))
        );
        assert_eq!(
            core::mem::align_of::<csp_timestamp_t>(),
            4usize,
            concat!("Alignment of ", stringify!(csp_timestamp_t))
        );
        assert_eq!(
            unsafe { ::core::ptr::addr_of!((*ptr).tv_sec) as usize - ptr as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_timestamp_t),
                "::",
                stringify!(tv_sec)
            )
        );
        assert_eq!(
            unsafe { ::core::ptr::addr_of!((*ptr).tv_nsec) as usize - ptr as usize },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_timestamp_t),
                "::",
                stringify!(tv_nsec)
            )
        );
    }

    #[test]
    fn bindgen_test_layout_csp_id() {
        const UNINIT: MaybeUninit<csp_id_t> = MaybeUninit::uninit();
        let ptr = UNINIT.as_ptr();
        assert_eq!(
            size_of::<csp_id_t>(),
            8usize,
            concat!("Size of: ", stringify!(__packed))
        );
        assert_eq!(
            align_of::<csp_id_t>(),
            2usize,
            concat!("Alignment of ", stringify!(__packed))
        );
        assert_eq!(
            unsafe { ::core::ptr::addr_of!((*ptr).pri) as usize - ptr as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(__packed),
                "::",
                stringify!(pri)
            )
        );
        assert_eq!(
            unsafe { core::ptr::addr_of!((*ptr).flags) as usize - ptr as usize },
            1usize,
            concat!(
                "Offset of field: ",
                stringify!(__packed),
                "::",
                stringify!(flags)
            )
        );
        assert_eq!(
            unsafe { core::ptr::addr_of!((*ptr).src) as usize - ptr as usize },
            2usize,
            concat!(
                "Offset of field: ",
                stringify!(__packed),
                "::",
                stringify!(src)
            )
        );
        assert_eq!(
            unsafe { core::ptr::addr_of!((*ptr).dst) as usize - ptr as usize },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(__packed),
                "::",
                stringify!(dst)
            )
        );
        assert_eq!(
            unsafe { core::ptr::addr_of!((*ptr).dport) as usize - ptr as usize },
            6usize,
            concat!(
                "Offset of field: ",
                stringify!(__packed),
                "::",
                stringify!(dport)
            )
        );
        assert_eq!(
            unsafe { core::ptr::addr_of!((*ptr).sport) as usize - ptr as usize },
            7usize,
            concat!(
                "Offset of field: ",
                stringify!(__packed),
                "::",
                stringify!(sport)
            )
        );
    }

    #[test]
    fn bindgen_test_layout_csp_packet_s__bindgen_ty_1__bindgen_ty_1() {
        const UNINIT: MaybeUninit<csp_packet_s_anon_union_field_rdp_only> = MaybeUninit::uninit();
        let ptr = UNINIT.as_ptr();
        assert_eq!(
            size_of::<csp_packet_s_anon_union_field_rdp_only>(),
            24usize,
            concat!(
                "Size of: ",
                stringify!(csp_packet_s__bindgen_ty_1__bindgen_ty_1)
            )
        );
        assert_eq!(
            align_of::<csp_packet_s_anon_union_field_rdp_only>(),
            8usize,
            concat!(
                "Alignment of ",
                stringify!(csp_packet_s__bindgen_ty_1__bindgen_ty_1)
            )
        );
        assert_eq!(
            unsafe { core::ptr::addr_of!((*ptr).rdp_quarantine) as usize - ptr as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_packet_s__bindgen_ty_1__bindgen_ty_1),
                "::",
                stringify!(rdp_quarantine)
            )
        );
        assert_eq!(
            unsafe { core::ptr::addr_of!((*ptr).timestamp_tx) as usize - ptr as usize },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_packet_s__bindgen_ty_1__bindgen_ty_1),
                "::",
                stringify!(timestamp_tx)
            )
        );
        assert_eq!(
            unsafe { core::ptr::addr_of!((*ptr).timestamp_rx) as usize - ptr as usize },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_packet_s__bindgen_ty_1__bindgen_ty_1),
                "::",
                stringify!(timestamp_rx)
            )
        );
        assert_eq!(
            unsafe { core::ptr::addr_of!((*ptr).conn) as usize - ptr as usize },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_packet_s__bindgen_ty_1__bindgen_ty_1),
                "::",
                stringify!(conn)
            )
        );
    }

    #[test]
    fn bindgen_test_layout_csp_packet_s__bindgen_ty_1__bindgen_ty_2() {
        const UNINIT: MaybeUninit<csp_packet_s_anon_union_field_rx_tx_only> = MaybeUninit::uninit();
        let ptr = UNINIT.as_ptr();
        assert_eq!(
            size_of::<csp_packet_s_anon_union_field_rx_tx_only>(),
            32usize,
            concat!(
                "Size of: ",
                stringify!(csp_packet_s__bindgen_ty_1__bindgen_ty_2)
            )
        );
        assert_eq!(
            align_of::<csp_packet_s_anon_union_field_rx_tx_only>(),
            8usize,
            concat!(
                "Alignment of ",
                stringify!(csp_packet_s__bindgen_ty_1__bindgen_ty_2)
            )
        );
        assert_eq!(
            unsafe { core::ptr::addr_of!((*ptr).rx_count) as usize - ptr as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_packet_s__bindgen_ty_1__bindgen_ty_2),
                "::",
                stringify!(rx_count)
            )
        );
        assert_eq!(
            unsafe { core::ptr::addr_of!((*ptr).remain) as usize - ptr as usize },
            2usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_packet_s__bindgen_ty_1__bindgen_ty_2),
                "::",
                stringify!(remain)
            )
        );
        assert_eq!(
            unsafe { core::ptr::addr_of!((*ptr).cfpid) as usize - ptr as usize },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_packet_s__bindgen_ty_1__bindgen_ty_2),
                "::",
                stringify!(cfpid)
            )
        );
        assert_eq!(
            unsafe { core::ptr::addr_of!((*ptr).last_used) as usize - ptr as usize },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_packet_s__bindgen_ty_1__bindgen_ty_2),
                "::",
                stringify!(last_used)
            )
        );
        assert_eq!(
            unsafe { core::ptr::addr_of!((*ptr).frame_begin) as usize - ptr as usize },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_packet_s__bindgen_ty_1__bindgen_ty_2),
                "::",
                stringify!(frame_begin)
            )
        );
        assert_eq!(
            unsafe { core::ptr::addr_of!((*ptr).frame_length) as usize - ptr as usize },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_packet_s__bindgen_ty_1__bindgen_ty_2),
                "::",
                stringify!(frame_length)
            )
        );
    }
    #[test]
    fn bindgen_test_layout_csp_packet_s__bindgen_ty_1() {
        assert_eq!(
            size_of::<csp_packet_s_anon_union>(),
            32usize,
            concat!("Size of: ", stringify!(csp_packet_s_anon_union))
        );
        assert_eq!(
            align_of::<csp_packet_s_anon_union>(),
            8usize,
            concat!("Alignment of ", stringify!(csp_packet_s_anon_union))
        );
    }

    #[test]
    fn bindgen_test_layout_csp_packet_s__bindgen_ty_2() {
        const UNINIT: MaybeUninit<csp_packet_s_data_union> = MaybeUninit::uninit();
        let ptr = UNINIT.as_ptr();
        assert_eq!(
            size_of::<csp_packet_s_data_union>(),
            256usize,
            concat!("Size of: ", stringify!(csp_packet_s__bindgen_ty_2))
        );
        assert_eq!(
            align_of::<csp_packet_s_data_union>(),
            4usize,
            concat!("Alignment of ", stringify!(csp_packet_s__bindgen_ty_2))
        );
        assert_eq!(
            unsafe { core::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_packet_s__bindgen_ty_2),
                "::",
                stringify!(data)
            )
        );
        assert_eq!(
            unsafe { core::ptr::addr_of!((*ptr).data16) as usize - ptr as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_packet_s__bindgen_ty_2),
                "::",
                stringify!(data16)
            )
        );
        assert_eq!(
            unsafe { core::ptr::addr_of!((*ptr).data32) as usize - ptr as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_packet_s__bindgen_ty_2),
                "::",
                stringify!(data32)
            )
        );
    }
    #[test]
    fn bindgen_test_layout_csp_packet_s() {
        const UNINIT: MaybeUninit<csp_packet_s> = MaybeUninit::uninit();
        let ptr = UNINIT.as_ptr();
        assert_eq!(
            size_of::<csp_packet_s>(),
            320usize,
            concat!("Size of: ", stringify!(csp_packet_s))
        );
        assert_eq!(
            align_of::<csp_packet_s>(),
            8usize,
            concat!("Alignment of ", stringify!(csp_packet_s))
        );
        assert_eq!(
            unsafe { core::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
            32usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_packet_s),
                "::",
                stringify!(length)
            )
        );
        assert_eq!(
            unsafe { core::ptr::addr_of!((*ptr).id) as usize - ptr as usize },
            34usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_packet_s),
                "::",
                stringify!(id)
            )
        );
        assert_eq!(
            unsafe { core::ptr::addr_of!((*ptr).next) as usize - ptr as usize },
            48usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_packet_s),
                "::",
                stringify!(next)
            )
        );
        assert_eq!(
            unsafe { core::ptr::addr_of!((*ptr).header) as usize - ptr as usize },
            56usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_packet_s),
                "::",
                stringify!(header)
            )
        );
    }

    #[test]
    fn bindgen_test_layout_csp_socket_s() {
        const UNINIT: core::mem::MaybeUninit<csp_socket_s> = core::mem::MaybeUninit::uninit();
        let ptr = UNINIT.as_ptr();
        assert_eq!(
            core::mem::size_of::<csp_socket_s>(),
            152usize,
            concat!("Size of: ", stringify!(csp_socket_s))
        );
        assert_eq!(
            core::mem::align_of::<csp_socket_s>(),
            8usize,
            concat!("Alignment of ", stringify!(csp_socket_s))
        );
        assert_eq!(
            unsafe { core::ptr::addr_of!((*ptr).rx_queue) as usize - ptr as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_socket_s),
                "::",
                stringify!(rx_queue)
            )
        );
        assert_eq!(
            unsafe { core::ptr::addr_of!((*ptr).rx_queue_static) as usize - ptr as usize },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_socket_s),
                "::",
                stringify!(rx_queue_static)
            )
        );
        assert_eq!(
            unsafe { core::ptr::addr_of!((*ptr).rx_queue_static_data) as usize - ptr as usize },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_socket_s),
                "::",
                stringify!(rx_queue_static_data)
            )
        );
        assert_eq!(
            unsafe { core::ptr::addr_of!((*ptr).opts) as usize - ptr as usize },
            144usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_socket_s),
                "::",
                stringify!(opts)
            )
        );
    }

    #[test]
    fn bindgen_test_layout_csp_if_udp_conf_t() {
        const UNINIT: ::core::mem::MaybeUninit<csp_if_udp_conf_t> =
            ::core::mem::MaybeUninit::uninit();
        let ptr = UNINIT.as_ptr();
        assert_eq!(
            ::core::mem::size_of::<csp_if_udp_conf_t>(),
            48usize,
            concat!("Size of: ", stringify!(csp_if_udp_conf_t))
        );
        assert_eq!(
            ::core::mem::align_of::<csp_if_udp_conf_t>(),
            8usize,
            concat!("Alignment of ", stringify!(csp_if_udp_conf_t))
        );
        assert_eq!(
            unsafe { ::core::ptr::addr_of!((*ptr).host) as usize - ptr as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_if_udp_conf_t),
                "::",
                stringify!(host)
            )
        );
        assert_eq!(
            unsafe { ::core::ptr::addr_of!((*ptr).lport) as usize - ptr as usize },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_if_udp_conf_t),
                "::",
                stringify!(lport)
            )
        );
        assert_eq!(
            unsafe { ::core::ptr::addr_of!((*ptr).rport) as usize - ptr as usize },
            12usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_if_udp_conf_t),
                "::",
                stringify!(rport)
            )
        );
        assert_eq!(
            unsafe { ::core::ptr::addr_of!((*ptr).server_handle) as usize - ptr as usize },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_if_udp_conf_t),
                "::",
                stringify!(server_handle)
            )
        );
        assert_eq!(
            unsafe { ::core::ptr::addr_of!((*ptr).peer_addr) as usize - ptr as usize },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_if_udp_conf_t),
                "::",
                stringify!(peer_addr)
            )
        );
        assert_eq!(
            unsafe { ::core::ptr::addr_of!((*ptr).sockfd) as usize - ptr as usize },
            40usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_if_udp_conf_t),
                "::",
                stringify!(sockfd)
            )
        );
    }

    #[test]
    fn bindgen_test_layout_csp_conn_s() {
        const UNINIT: ::core::mem::MaybeUninit<csp_conn_s> = ::core::mem::MaybeUninit::uninit();
        let ptr = UNINIT.as_ptr();
        assert_eq!(
            ::core::mem::size_of::<csp_conn_s>(),
            280usize,
            concat!("Size of: ", stringify!(csp_conn_s))
        );
        assert_eq!(
            ::core::mem::align_of::<csp_conn_s>(),
            8usize,
            concat!("Alignment of ", stringify!(csp_conn_s))
        );
        assert_eq!(
            unsafe { ::core::ptr::addr_of!((*ptr).type_) as usize - ptr as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_conn_s),
                "::",
                stringify!(type_)
            )
        );
        assert_eq!(
            unsafe { ::core::ptr::addr_of!((*ptr).state) as usize - ptr as usize },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_conn_s),
                "::",
                stringify!(state)
            )
        );
        assert_eq!(
            unsafe { ::core::ptr::addr_of!((*ptr).idin) as usize - ptr as usize },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_conn_s),
                "::",
                stringify!(idin)
            )
        );
        assert_eq!(
            unsafe { ::core::ptr::addr_of!((*ptr).idout) as usize - ptr as usize },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_conn_s),
                "::",
                stringify!(idout)
            )
        );
        assert_eq!(
            unsafe { ::core::ptr::addr_of!((*ptr).sport_outgoing) as usize - ptr as usize },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_conn_s),
                "::",
                stringify!(sport_outgoing)
            )
        );
        assert_eq!(
            unsafe { ::core::ptr::addr_of!((*ptr).rx_queue) as usize - ptr as usize },
            32usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_conn_s),
                "::",
                stringify!(rx_queue)
            )
        );
        assert_eq!(
            unsafe { ::core::ptr::addr_of!((*ptr).rx_queue_static) as usize - ptr as usize },
            40usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_conn_s),
                "::",
                stringify!(rx_queue_static)
            )
        );
        assert_eq!(
            unsafe { ::core::ptr::addr_of!((*ptr).rx_queue_static_data) as usize - ptr as usize },
            48usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_conn_s),
                "::",
                stringify!(rx_queue_static_data)
            )
        );
        assert_eq!(
            unsafe { ::core::ptr::addr_of!((*ptr).callback) as usize - ptr as usize },
            176usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_conn_s),
                "::",
                stringify!(callback)
            )
        );
        assert_eq!(
            unsafe { ::core::ptr::addr_of!((*ptr).dest_socket) as usize - ptr as usize },
            184usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_conn_s),
                "::",
                stringify!(dest_socket)
            )
        );
        assert_eq!(
            unsafe { ::core::ptr::addr_of!((*ptr).timestamp) as usize - ptr as usize },
            192usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_conn_s),
                "::",
                stringify!(timestamp)
            )
        );
        assert_eq!(
            unsafe { ::core::ptr::addr_of!((*ptr).opts) as usize - ptr as usize },
            196usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_conn_s),
                "::",
                stringify!(opts)
            )
        );
        assert_eq!(
            unsafe { ::core::ptr::addr_of!((*ptr).rdp) as usize - ptr as usize },
            200usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_conn_s),
                "::",
                stringify!(rdp)
            )
        );
    }

    #[test]
    fn bindgen_test_layout_csp_iface_s() {
        const UNINIT: ::core::mem::MaybeUninit<csp_iface_s> = ::core::mem::MaybeUninit::uninit();
        let ptr = UNINIT.as_ptr();
        assert_eq!(
            ::core::mem::size_of::<csp_iface_s>(),
            96usize,
            concat!("Size of: ", stringify!(csp_iface_s))
        );
        assert_eq!(
            ::core::mem::align_of::<csp_iface_s>(),
            8usize,
            concat!("Alignment of ", stringify!(csp_iface_s))
        );
        assert_eq!(
            unsafe { ::core::ptr::addr_of!((*ptr).addr) as usize - ptr as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_iface_s),
                "::",
                stringify!(addr)
            )
        );
        assert_eq!(
            unsafe { ::core::ptr::addr_of!((*ptr).netmask) as usize - ptr as usize },
            2usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_iface_s),
                "::",
                stringify!(netmask)
            )
        );
        assert_eq!(
            unsafe { ::core::ptr::addr_of!((*ptr).name) as usize - ptr as usize },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_iface_s),
                "::",
                stringify!(name)
            )
        );
        assert_eq!(
            unsafe { ::core::ptr::addr_of!((*ptr).interface_data) as usize - ptr as usize },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_iface_s),
                "::",
                stringify!(interface_data)
            )
        );
        assert_eq!(
            unsafe { ::core::ptr::addr_of!((*ptr).driver_data) as usize - ptr as usize },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_iface_s),
                "::",
                stringify!(driver_data)
            )
        );
        assert_eq!(
            unsafe { ::core::ptr::addr_of!((*ptr).nexthop) as usize - ptr as usize },
            32usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_iface_s),
                "::",
                stringify!(nexthop)
            )
        );
        assert_eq!(
            unsafe { ::core::ptr::addr_of!((*ptr).is_default) as usize - ptr as usize },
            40usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_iface_s),
                "::",
                stringify!(is_default)
            )
        );
        assert_eq!(
            unsafe { ::core::ptr::addr_of!((*ptr).tx) as usize - ptr as usize },
            44usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_iface_s),
                "::",
                stringify!(tx)
            )
        );
        assert_eq!(
            unsafe { ::core::ptr::addr_of!((*ptr).rx) as usize - ptr as usize },
            48usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_iface_s),
                "::",
                stringify!(rx)
            )
        );
        assert_eq!(
            unsafe { ::core::ptr::addr_of!((*ptr).tx_error) as usize - ptr as usize },
            52usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_iface_s),
                "::",
                stringify!(tx_error)
            )
        );
        assert_eq!(
            unsafe { ::core::ptr::addr_of!((*ptr).rx_error) as usize - ptr as usize },
            56usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_iface_s),
                "::",
                stringify!(rx_error)
            )
        );
        assert_eq!(
            unsafe { ::core::ptr::addr_of!((*ptr).drop) as usize - ptr as usize },
            60usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_iface_s),
                "::",
                stringify!(drop)
            )
        );
        assert_eq!(
            unsafe { ::core::ptr::addr_of!((*ptr).autherr) as usize - ptr as usize },
            64usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_iface_s),
                "::",
                stringify!(autherr)
            )
        );
        assert_eq!(
            unsafe { ::core::ptr::addr_of!((*ptr).frame) as usize - ptr as usize },
            68usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_iface_s),
                "::",
                stringify!(frame)
            )
        );
        assert_eq!(
            unsafe { ::core::ptr::addr_of!((*ptr).txbytes) as usize - ptr as usize },
            72usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_iface_s),
                "::",
                stringify!(txbytes)
            )
        );
        assert_eq!(
            unsafe { ::core::ptr::addr_of!((*ptr).rxbytes) as usize - ptr as usize },
            76usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_iface_s),
                "::",
                stringify!(rxbytes)
            )
        );
        assert_eq!(
            unsafe { ::core::ptr::addr_of!((*ptr).irq) as usize - ptr as usize },
            80usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_iface_s),
                "::",
                stringify!(irq)
            )
        );
        assert_eq!(
            unsafe { ::core::ptr::addr_of!((*ptr).next) as usize - ptr as usize },
            88usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_iface_s),
                "::",
                stringify!(next)
            )
        );
    }
}
