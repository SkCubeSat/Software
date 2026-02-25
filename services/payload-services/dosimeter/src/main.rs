use env_logger;
use kubos_service::Config;
use kubos_service::Service;
use log;
use rust_i2c::Connection;

mod i2c_reader;
mod schema;

use crate::schema::{MutationRoot, QueryRoot, Subsystem};

fn main() {
    // Initialize logging
    env_logger::init();

    // Get configuration
    let config = Config::new("dosimeter").unwrap_or_else(|err| {
        log::error!("Failed to load service config: {:?}", err);
        log::warn!("Using default configuration");
        
        // Create a default config with required fields
        let default_config = Config::default();
        // Note: You may need to manually set the hosturl if Config::default() doesn't provide one
        // This depends on your kubos_service implementation
        default_config
    });

    // Get I2C bus path from config or use default
    let i2c_bus = config
        .get("i2c_bus")
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .unwrap_or_else(|| "/dev/i2c-1".to_string());

    // Get device address from config or use default
    let device_addr = config
        .get("device_addr")
        .and_then(|v| v.as_integer())
        .unwrap_or(0x4A) as u16;

    log::info!("Connecting to I2C bus: {} at address: 0x{:02X}", &i2c_bus, device_addr);

    // Set up I2C connection
    let connection = Connection::from_path(&i2c_bus, device_addr);

    // Create and start the service
    // Note: If this still fails, you'll need to ensure the config has a valid hosturl
    // or check the kubos_service documentation for proper Config initialization
    Service::new(
        config,
        Subsystem::new(connection),
        QueryRoot,
        MutationRoot,
    )
    .start();
}