use std::ffi::CString;
use std::ptr;
use libc::c_void;
use crate::bindings::*;

pub fn init_csp(node_id: u8) {
    unsafe {
        csp_init();

        // Set up a loopback interface (or use UART, CAN, etc. for real hw)
        let mut interface: csp_iface_t = std::mem::zeroed();
        let name = CString::new("LOOP").unwrap();
        csp_if_lo_init(&mut interface, name.as_ptr());

        // Add to CSP
        csp_add_interface(&mut interface, name.as_ptr());

        // Set default route (all packets go to the interface)
        csp_route_set(CSP_DEFAULT_ROUTE, &mut interface, CSP_NODE_MAC);
        csp_set_address(node_id as csp_id_t);
    }
}
