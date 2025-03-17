use crate::schema::Context;
use clyde_3g_eps_api::MotherboardTelemetry;
use juniper::FieldResult;

pub struct Root;

#[juniper::graphql_object(Context = Context)]
impl Root {
    /// Get service health
    fn ping() -> &'static str {
        "pong"
    }

    /// Get the current battery level (0-100%)
    async fn battery_level(context: &Context) -> FieldResult<f64> {
        // Access our simulator through the context
        let sim_eps = context.subsystem.sim_eps.lock().unwrap();

        // Use the motherboard telemetry API to get battery level
        Ok(sim_eps.get_motherboard_telemetry(MotherboardTelemetry::Type::OutputVoltageBattery)?)
    }

    /// Get the EPS firmware version information
    async fn version_info(context: &Context) -> FieldResult<String> {
        let sim_eps = context.subsystem.sim_eps.lock().unwrap();
        let version = sim_eps.get_version_info()?;

        Ok(format!(
            "Motherboard: rev {} firmware {}, Daughterboard: rev {} firmware {}",
            version.motherboard.revision,
            version.motherboard.firmware_number,
            version.daughterboard.as_ref().map_or(0, |v| v.revision),
            version
                .daughterboard
                .as_ref()
                .map_or(0, |v| v.firmware_number)
        ))
    }
}
