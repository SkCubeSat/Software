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

    // fn send(message: String) -> FieldResult<String> {
    //     let dest = 2;
    //     let port = 10;
    //     let timeout = 1000;

    //     unsafe {
    //         let packet = bindings::csp_buffer_get(message.len() as usize);
    //         if packet.is_null() {
    //             return Ok("Failed to allocate CSP packet".to_string());
    //         }

    //         // Copy message into CSP packet
    //         std::ptr::copy_nonoverlapping(
    //             message.as_ptr(),
    //             (*packet).data.as_mut_ptr(),
    //             message.len(),
    //         );

    //         (*packet).length = message.len() as u16;

    //         let status = bindings::csp_send(
    //             dest,
    //             port,
    //             0, // priority
    //             timeout,
    //             packet,
    //         );

    //         if status != 1 {
    //             bindings::csp_buffer_free(packet);
    //             return Ok("Failed to send CSP packet".to_string());
    //         }

    //         Ok("Packet sent!".to_string())
    //     }
    // }
}

pub struct MutationRoot;

// Base GraphQL mutation model
#[graphql_object(context = Context)]
impl MutationRoot {
    fn noop(&self, _context: &Context) -> FieldResult<bool> {
        Ok(true)
    }
}