use std::{
    process,
    sync::{Arc, Mutex},
    time::Duration,
};

use comms_services::{
    config::{SERVICE_NAME, ServiceSettings},
    csp_interface::spawn_i2c_workers,
    model::Subsystem,
    nxtrx_comms::{NxtrxComms, read, write},
    schema::{MutationRoot, QueryRoot},
};
use kubos_comms::{CommsControlBlock, CommsService, CommsTelemetry, SpacePacket};
use kubos_service::{Config, Logger, Service};
use log::info;
use radsat_csp::{CspListener, ReservedServiceWorker, RouterWorker};

fn main() {
    if let Err(err) = run() {
        log::error!("{err}");
        eprintln!("{err}");
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    Logger::init(SERVICE_NAME)?;

    let config = Config::new(SERVICE_NAME)?;
    let settings = ServiceSettings::from_config(&config)?;

    // CSP is the outer transport layer. It moves packets between CSP nodes
    // such as the ground station, OBC, and each NXTRX4 radio.
    let _router = RouterWorker::start();
    let _ping_service = ReservedServiceWorker::start_ping_service();

    // Inbound RF traffic arrives on two explicit CSP ports: one for a single
    // CSP packet and one for SFP-fragmented SpacePackets.
    let packet_listener =
        CspListener::bind(settings.csp.uplink_packet_csp_port, settings.csp.backlog)
            .with_accept_timeout(Duration::from_millis(100))
            .with_read_timeout(Duration::from_millis(100));
    let sfp_listener = CspListener::bind(settings.csp.uplink_sfp_csp_port, settings.csp.backlog)
        .with_accept_timeout(Duration::from_millis(100))
        .with_read_timeout(settings.csp.sfp_read_timeout);

    // Register the I2C buses as CSP interfaces and install routes such as
    // ground_node -> downlink radio I2C address.
    spawn_i2c_workers(&settings.csp, &settings.radios)?;

    let comms = NxtrxComms::new(
        packet_listener,
        sfp_listener,
        &settings.csp,
        &settings.radios,
    );
    let controls = CommsControlBlock::new(
        Some(Arc::new(read)),
        vec![Arc::new(write)],
        comms.clone(),
        comms.clone(),
        settings.comms.clone(),
    )?;
    let telemetry = Arc::new(Mutex::new(CommsTelemetry::default()));

    info!(
        "NXTRX4 communications service starting: uplink node {} on {}, downlink node {} on {}",
        settings.radios.uplink.csp_node,
        settings.radios.uplink.bus,
        settings.radios.downlink.csp_node,
        settings.radios.downlink.bus
    );
    // kubos-comms handles the inner SpacePacket: parse it, choose GraphQL or
    // UDP by payload type, dispatch by destination port, and call write().
    CommsService::start::<NxtrxComms, SpacePacket>(controls, &telemetry)?;

    let subsystem = Subsystem::new(telemetry, comms, settings);
    Service::new(config, subsystem, QueryRoot, MutationRoot).start();

    Ok(())
}
