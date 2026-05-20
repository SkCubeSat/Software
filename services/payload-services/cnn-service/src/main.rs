use kubos_service::{Config, Logger, Service};

use cnn_service::schema::{MutationRoot, QueryRoot};
use cnn_service::subsystem::Subsystem;

#[tokio::main]
async fn main() {
    if let Err(err) = Logger::init("cnn-service") {
        eprintln!("failed to initialize logger: {err:?}");
        std::process::exit(1);
    }

    let config = match Config::new("cnn-service") {
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
            log::error!("failed to initialize CNN subsystem: {err}");
            eprintln!("failed to initialize CNN subsystem: {err}");
            std::process::exit(3);
        }
    };

    log::info!("cnn-service started");
    Service::new(config, subsystem, QueryRoot, MutationRoot).start_async().await;
}
