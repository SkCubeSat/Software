use juniper::{graphql_object, FieldError, FieldResult};
type Context = kubos_service::Context<()>;

mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub struct QueryRoot;

#[graphql_object(context = Context)]
impl QueryRoot {
    fn ping() -> FieldResult<String> {
        let node_id: u16 = 1; // CSP node ID
        unsafe {
            bindings::csp_ping(node_id, 1000, 0, 1);
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