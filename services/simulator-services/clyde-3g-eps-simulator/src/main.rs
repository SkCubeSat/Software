mod model;
mod schema;

use crate::model::Subsystem;
use crate::schema::{mutation, query, Schema};
use juniper::EmptySubscription;
use kubos_service::{Config, Logger, Service};
use log::error;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    Logger::init("clyde-3g-eps-simulator").unwrap();
    let config = Config::new("clyde-3g-eps-simulator")
        .map_err(|err| {
            error!("Failed to load service config: {:?}", err);
            err
        })
        .unwrap();

    // Create the simulator subsystem
    let subsystem = match Subsystem::new() {
        Ok(sub) => sub,
        Err(err) => {
            error!("Failed to create simulator subsystem: {:?}", err);
            std::process::exit(1);
        }
    };

    println!("Starting Clyde-3G-EPS simulator service");
    println!("Simulator will cycle battery level from 0-100% and back in 60 seconds");
    println!("Use the GraphQL endpoint to query the simulated EPS");

    // Create the service with the subsystem
    Service::new(config, subsystem, query::Root, mutation::Root).start();
}
