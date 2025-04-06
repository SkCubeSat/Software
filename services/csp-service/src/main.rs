use kubos_service::{Config, Service, Context};
use juniper::{FieldResult, EmptyMutation};
use std::ffi::CString;
use crate::schema::{MutationRoot, QueryRoot};

mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

mod schema;

// #[derive(Clone)]
// struct Subsystem;

// #[juniper::graphql_object]
// impl Subsystem {
//     fn ping() -> FieldResult<String> {
//         let node_id: u16 = 1; // CSP node ID
//         unsafe {
//             bindings::csp_ping(node_id, 1000, 0, 1);
//         }
//         Ok("Ping sent!".to_string())
//     }
// }

fn main() {
    unsafe {
        bindings::csp_init();
    }

    let config = Config::new("csp-service").unwrap();
    // let subsystem = Subsystem;
    Service::new(config, (), QueryRoot, MutationRoot).start();
}
