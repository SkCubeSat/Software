use kubos_service::{Config, Logger, Service};

use snn_service::schema::{MutationRoot, QueryRoot};
use snn_service::subsystem::Subsystem;

#[tokio::main]
async fn main() {
    if let Err(err) = Logger::init("snn-service") {
        eprintln!("failed to initialize logger: {err:?}");
        std::process::exit(1);
    }

    let config = match Config::new("snn-service") {
        Ok(config) => config,
        Err(err) => {
            log::error!("failed to load service config: {err:?}");
            eprintln!("failed to load service config: {err}");
            std::process::exit(2);
        }
    };

    let subsystem = match Subsystem::from_config(&config) {
        Ok(s) => s,
        Err(err) => {
            log::error!("failed to initialize SNN subsystem: {err}");
            eprintln!("failed to initialize SNN subsystem: {err}");
            std::process::exit(3);
        }
    };

    log::info!("snn-service started");
    Service::new(config, subsystem, QueryRoot, MutationRoot).start_async().await;
}
