use kubos_service::{Config, Logger, Service};

use mram_service::schema::{MutationRoot, QueryRoot};
use mram_service::subsystem::Subsystem;

fn main() {
    Logger::init("mram-service").unwrap();

    let config = Config::new("mram-service")
        .map_err(|err| {
            log::error!("Failed to load service config: {:?}", err);
            err
        })
        .unwrap();

    let subsystem = Subsystem::from_config(&config)
        .map_err(|err| {
            log::error!("Failed to initialize MRAM subsystem: {}", err);
            err
        })
        .unwrap();

    Service::new(config, subsystem, QueryRoot, MutationRoot).start();
}
