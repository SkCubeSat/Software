use std::{
    ffi::CStr,
    sync::{
        atomic::{AtomicBool, AtomicU32},
        Arc,
    },
    thread,
    time::Duration,
};

use libcsp::{
    csp_accept_guarded, csp_bind, csp_buffer_get, csp_conn_dport, csp_conn_print_table,
    csp_connect_guarded, csp_init, csp_listen, csp_ping, csp_read_guarded, csp_reboot,
    csp_route_work, csp_send, csp_service_handler, iflist::csp_iflist_print, ConnectOpts, CspError,
    CspSocket, MsgPriority, SocketFlags, CSP_ANY, CSP_LOOPBACK,
};

const MY_SERVER_PORT: i32 = 10;
const TEST_MODE: bool = false;
const RUN_DURATION_IN_SECS: u32 = 3;

fn main() -> Result<(), u32> {
    println!("CSP client/server example");

    // SAFETY: We only call this once.
    unsafe { csp_init() };

    let stop_signal = Arc::new(AtomicBool::new(false));
    let stop_signal_server = stop_signal.clone();
    let stop_signal_client = stop_signal.clone();
    let stop_signal_router = stop_signal.clone();
    let server_received = Arc::new(AtomicU32::new(0));
    let server_recv_copy = server_received.clone();

    let csp_router_jh = thread::spawn(move || loop {
        if stop_signal_router.load(std::sync::atomic::Ordering::Relaxed) {
            break;
        }
        if let Err(e) = csp_route_work() {
            match e {
                CspError::TimedOut => continue,
                e => {
                    println!("CSP router error: {:?}", e);
                    break;
                }
            }
        }
    });

    let csp_server_jh = thread::spawn(move || {
        server(server_received, stop_signal_server);
    });

    let csp_client_jh = thread::spawn(move || {
        client(stop_signal_client);
    });

    println!("CSP connection table");
    csp_conn_print_table();

    println!("CSP interfaces");
    csp_iflist_print();
    let mut app_result = Ok(());
    // Wait for execution to end (ctrl+c)
    loop {
        std::thread::sleep(Duration::from_secs(RUN_DURATION_IN_SECS as u64));

        if TEST_MODE {
            // Test mode is intended for checking that host & client can exchange packets over loopback
            let received_count = server_recv_copy.load(std::sync::atomic::Ordering::Relaxed);
            println!("CSP: Server received {} packets", received_count);
            if received_count < 5 {
                app_result = Err(1);
            }
            stop_signal.store(true, std::sync::atomic::Ordering::Relaxed);
            break;
        }
    }

    csp_router_jh.join().unwrap();
    csp_server_jh.join().unwrap();
    csp_client_jh.join().unwrap();
    app_result
}

fn server(server_received: Arc<AtomicU32>, stop_signal: Arc<AtomicBool>) {
    println!("server task started");

    // Create socket with no specific socket options, e.g. accepts CRC32, HMAC, etc. if enabled
    // during compilation
    let mut csp_socket = CspSocket::default();

    // Bind socket to all ports, e.g. all incoming connections will be handled here
    csp_bind(&mut csp_socket, CSP_ANY);

    // Create a backlog of 10 connections, i.e. up to 10 new connections can be queued
    csp_listen(&mut csp_socket, 10);

    // Wait for connections and then process packets on the connection
    loop {
        if stop_signal.load(std::sync::atomic::Ordering::Relaxed) {
            break;
        }

        // Wait for a new connection, 10000 mS timeout
        let conn = csp_accept_guarded(&mut csp_socket, Duration::from_millis(10000));
        if conn.is_none() {
            continue;
        }
        let mut conn = conn.unwrap();

        // Read packets on connection, timout is 100 mS
        loop {
            if stop_signal.load(std::sync::atomic::Ordering::Relaxed) {
                break;
            }

            // Guarded packet is cleaned up automatically.
            let packet = csp_read_guarded(&mut conn.0, Duration::from_millis(100));
            if packet.is_none() {
                break;
            }
            let packet = packet.unwrap();
            match csp_conn_dport(&conn.0) {
                MY_SERVER_PORT => {
                    server_received.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    let cstr = CStr::from_bytes_with_nul(packet.as_ref().packet_data())
                        .expect("invalid packet data format, is not C string");
                    // Process packet here.
                    println!("packet received on MY_SERVER_PORT: {:?}", cstr);
                }
                _ => {
                    csp_service_handler(packet.take());
                }
            };
        }
        // No need to close, we accepted the connection with a guard.
    }
}

fn client(stop_signal: Arc<AtomicBool>) {
    println!("client task started");
    let mut current_letter = 'A';

    loop {
        if stop_signal.load(std::sync::atomic::Ordering::Relaxed) {
            break;
        }
        if TEST_MODE {
            thread::sleep(Duration::from_millis(20));
        } else {
            thread::sleep(Duration::from_millis(100));
        }

        // Send ping to server, timeout 1000 mS, ping size 20 bytes
        if let Err(e) = csp_ping(
            CSP_LOOPBACK,
            Duration::from_millis(1000),
            20,
            SocketFlags::NONE,
        ) {
            println!("ping error: {:?}", e);
        }

        // Send reboot request to server, the server has no actual implementation of
        // csp_sys_reboot() and fails to reboot.
        csp_reboot(CSP_LOOPBACK);

        // Send data packet (string) to server

        // 1. Connect to host on 'server_address', port MY_SERVER_PORT with regular UDP-like
        // protocol and 1000 ms timeout.
        let conn = csp_connect_guarded(
            MsgPriority::Normal,
            CSP_LOOPBACK,
            MY_SERVER_PORT as u8,
            Duration::from_millis(1000),
            ConnectOpts::NONE,
        );
        if conn.is_none() {
            println!("CSP client: connection failed");
            return;
        }
        let mut conn = conn.unwrap();

        // 2. Get packet buffer for message/data.
        let packet_ref = csp_buffer_get();
        if packet_ref.is_none() {
            println!("CSP client: failed to get CSP buffer");
            return;
        }
        let mut packet_mut = packet_ref.unwrap();

        // 3. Copy data to packet.
        let mut string_to_set = String::from("Hello world");
        string_to_set.push(' ');
        string_to_set.push(current_letter);
        current_letter = (current_letter as u8 + 1) as char;
        string_to_set.push('\0');
        packet_mut.set_data(string_to_set.as_bytes());

        // 4. Send data.
        csp_send(&mut conn.0, packet_mut);
    }
}
