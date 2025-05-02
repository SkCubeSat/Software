use juniper::{graphql_object, FieldError, FieldResult};
type Context = kubos_service::Context<()>;
use libcsp::*;
use std::time::Duration;

// mod bindings {
//     include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
// }


pub struct QueryRoot;

#[graphql_object(context = Context)]
impl QueryRoot {
    fn ping() -> FieldResult<String> {
        let node_id: u16 = 1; // CSP node ID
        if let Err(e) = csp_ping(node_id, Duration::from_millis(1000), 0, SocketFlags::NONE,)
        {
            println!("ping error: {:?}", e);
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