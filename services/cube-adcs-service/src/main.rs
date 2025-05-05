#![allow(warnings)] 

use ffi::csp_iface_t;
use kubos_service::{Config, Service, Context};
use juniper::{FieldResult, EmptyMutation};
use std::ffi::CString;
use std::thread;
use std::ptr;
use crate::schema::{MutationRoot, QueryRoot};
use libcsp::*;

// include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

mod csp {
    include!(concat!(env!("OUT_DIR"), "/csp_bindings.rs"));
}

use bindings::*;
use csp::*;

mod schema;






fn main() {
    unsafe {
        csp::csp_init();
    }

    
    

    let config = Config::new("cube-adcs-service").unwrap();
    // let subsystem = Subsystem;
    Service::new(config, (), QueryRoot, MutationRoot).start();
}



