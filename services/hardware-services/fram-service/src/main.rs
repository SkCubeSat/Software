use kubos_service::{Config, Logger, Service};

use fram_service::schema::{MutationRoot, QueryRoot};
use fram_service::subsystem::Subsystem;

fn main() {
    if let Err(err) = Logger::init("fram-service") {
        eprintln!("failed to initialize logger: {err:?}");
        std::process::exit(1);
    }

    let config = match Config::new("fram-service") {
        Ok(config) => config,
        Err(err) => {
            log::error!("Failed to load service config: {:?}", err);
            eprintln!("Failed to load service config: {err}");
            std::process::exit(2);
        }
    };

    let subsystem = match Subsystem::from_config(&config) {
        Ok(subsystem) => subsystem,
        Err(err) => {
            log::error!("Failed to initialize FRAM subsystem: {}", err);
            eprintln!("Failed to initialize FRAM subsystem: {err}");
            std::process::exit(3);
        }
    };

    Service::new(config, subsystem, QueryRoot, MutationRoot).start();
}
