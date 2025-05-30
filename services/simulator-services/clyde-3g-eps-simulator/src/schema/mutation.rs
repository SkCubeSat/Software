use crate::schema::Context;
use juniper::{
    graphql_object, graphql_value, EmptyMutation, EmptySubscription, FieldResult, GraphQLEnum,
    Variables,
};

pub struct Root;

#[juniper::graphql_object(Context = Context)]
impl Root {
    /// Ping the service to verify it's running
    fn ping() -> String {
        "pong".to_string()
    }

    /// Manual reset - simulate a manual reset of the EPS
    async fn manual_reset(context: &Context) -> FieldResult<String> {
        let sim_eps = context.subsystem.sim_eps.lock().unwrap();
        sim_eps.manual_reset()?;

        Ok("Reset command sent to EPS simulator".to_string())
    }

    /// Reset the communications watchdog
    async fn reset_watchdog(context: &Context) -> FieldResult<String> {
        let sim_eps = context.subsystem.sim_eps.lock().unwrap();
        sim_eps.reset_comms_watchdog()?;

        Ok("Watchdog reset command sent to EPS simulator".to_string())
    }
}
