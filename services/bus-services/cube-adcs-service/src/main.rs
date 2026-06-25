use cube_adcs_service::schema::{MutationRoot, QueryRoot};
use cube_adcs_service::subsystem::Subsystem;
use kubos_service::{Config, Logger, Service};

fn main() {
    if let Err(err) = Logger::init("cube-adcs-service") {
        eprintln!("failed to initialize logger: {err:?}");
        std::process::exit(1);
    }

    let config = match Config::new("cube-adcs-service") {
        Ok(config) => config,
        Err(err) => {
            log::error!("failed to load service config: {err:?}");
            eprintln!("failed to load service config: {err}");
            std::process::exit(2);
        }
    };

    let subsystem = match Subsystem::from_config(&config) {
        Ok(subsystem) => subsystem,
        Err(err) => {
            log::error!("failed to initialize Cube ADCS subsystem: {err}");
            eprintln!("failed to initialize Cube ADCS subsystem: {err}");
            std::process::exit(3);
        }
    };

    log::info!("cube-adcs-service started");
    Service::new(
        config,
        subsystem,
        QueryRoot::default(),
        MutationRoot::default(),
    )
    .start();
}
