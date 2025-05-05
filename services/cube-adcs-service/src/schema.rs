use juniper::{graphql_object, FieldError, FieldResult};
type Context = kubos_service::Context<()>;
// use libcsp::*;
use std::ffi::CString;
use std::time::Duration;
use std::ptr;

use std::os::raw::c_void;

mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

mod csp {
    include!(concat!(env!("OUT_DIR"), "/csp_bindings.rs"));
}

use bindings::*;
use csp::*;

// Constants matching the C version
const CAN_DEVICE: &str = "can0";
const CSP_SRC_PORT: u8 = 10;
const DEFAULT_ROUTE: u16 = 0;
const CSP_NO_VIA_ADDRESS: u16 = 0;

// Error codes
const CSP_INIT_SUCCESS: i32 = 0;
const CSP_INIT_FAILURE: i32 = -1;

static mut iface: csp_iface_t = csp_iface_t {
    addr: 0,
    netmask: 0,
    name: ptr::null(),
    interface_data: ptr::null_mut(),
    driver_data: ptr::null_mut(),
    nexthop: None,
    is_default: 0,
    tx: 0,
    rx: 0,
    tx_error: 0,
    rx_error: 0,
    drop: 0,
    autherr: 0,
    frame: 0,
    txbytes: 0,
    rxbytes: 0,
    irq: 0,
    next: ptr::null_mut(),
};

static mut ifdata: csp_can_interface_data_t = csp_can_interface_data_t {
    cfp_packet_counter: 0,
    tx_func: None,
    pbufs: ptr::null_mut(),
};

static mut SOCKET: Option<csp_socket_t> = None;


pub struct QueryRoot;

#[graphql_object(context = Context)]
impl QueryRoot {
    fn ping() -> FieldResult<String> {
        let node_id: u16 = 1; // CSP node ID
        if let result = unsafe {csp_ping(node_id, 1000, 0, 1,)} == -1
        {
            println!("ping error: Could not ping specified node");
        }
        
        Ok("Ping sent!".to_string())
    }

    
}

pub struct MutationRoot;

// Base GraphQL mutation model
#[graphql_object(context = Context)]
impl MutationRoot {
    fn noop(&self, _context: &Context) -> FieldResult<bool> {
        Ok(true)
    }
}


pub fn initialize_csp(address: u8) -> i32 {

    unsafe {
        iface.name = CString::new(CAN_DEVICE).unwrap().into_raw();
        iface.interface_data = &raw mut ifdata as *mut _ as *mut c_void;

        // Add CAN interface
        if csp_can_add_interface(&raw mut iface) != CSP_ERR_NONE as i32 {
            println!("CSP IFC initialization failed");
            return CSP_INIT_FAILURE;
        }

        // Set routing table
        if csp_rtable_set(DEFAULT_ROUTE, 0, &raw mut iface, CSP_NO_VIA_ADDRESS) != CSP_ERR_NONE as i32 {
            println!("CSP routing table initialization failed");
            return CSP_INIT_FAILURE;
        }

        // Create socket
        let mut socket = csp_socket_t {
            rx_queue: ptr::null_mut(),
            rx_queue_static: ptr::null_mut(),
            rx_queue_static_data: [0; 128],
            opts: 0,
        };

        // NOTE: csp_socket function has been deprecated from libcsp
        // if csp_socket(&mut socket, CSP_SO_CONN_LESS as u32) != CSP_ERR_NONE as i32 {
        //     println!("CSP socket creation failed");
        //     return CSP_INIT_FAILURE;
        // }

        // Bind socket
        if csp_bind(&mut socket, CSP_SRC_PORT) != CSP_ERR_NONE as i32 {
            println!("CSP socket initialization failed");
            return CSP_INIT_FAILURE;
        }

        SOCKET = Some(socket);

        // Start route task
        if csp_route_start_task() != CSP_ERR_NONE as i32 {
            println!("CSP route task failed");
            return CSP_INIT_FAILURE;
        }
        
    }

    CSP_INIT_SUCCESS
}