use kubos_service::{Config, Logger, Service};

use mram_service::schema::{MutationRoot, QueryRoot};
use mram_service::subsystem::Subsystem;

fn main() {
    if let Err(err) = Logger::init("mram-service") {
        eprintln!("failed to initialize logger: {err:?}");
        std::process::exit(1);
    }

    let config = match Config::new("mram-service") {
        Ok(config) => config,
        Err(err) => {
            log::error!("Failed to load service config: {:?}", err);
            std::process::exit(2);
        }
    };

    let subsystem = match Subsystem::from_config(&config) {
        Ok(subsystem) => subsystem,
        Err(err) => {
            log::error!("Failed to initialize MRAM subsystem: {}", err);
            std::process::exit(3);
        }
    };

    Service::new(config, subsystem, QueryRoot, MutationRoot).start();
}
