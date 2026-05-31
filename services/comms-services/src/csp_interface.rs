use std::{collections::HashMap, thread};

use radsat_csp::{I2cInterfaceConfig, LinuxI2cCspInterface};
use thiserror::Error;

use crate::config::{CspSettings, RadioConfig, RadioSettings};

#[derive(Debug, Error)]
pub enum InterfaceError {
    #[error("failed to open CSP I2C interface on {bus}: {source}")]
    Open {
        bus: String,
        source: radsat_csp::Error,
    },
    #[error("failed to route CSP node {node} through I2C address 0x{i2c_addr:02X}: {source}")]
    Route {
        node: u16,
        i2c_addr: u8,
        source: radsat_csp::Error,
    },
}

#[derive(Debug, Clone)]
struct InterfacePlan {
    bus: String,
    routes: Vec<RoutePlan>,
    poll: Option<PollPlan>,
}

#[derive(Debug, Clone, Copy)]
struct RoutePlan {
    node: u16,
    i2c_addr: u8,
}

#[derive(Debug, Clone, Copy)]
struct PollPlan {
    i2c_addr: u8,
    frame_len: usize,
    interval: std::time::Duration,
}

pub fn spawn_i2c_workers(csp: &CspSettings, radios: &RadioSettings) -> Result<(), InterfaceError> {
    let mut plans = build_plans(csp, radios);

    for (index, plan) in plans.drain(..).enumerate() {
        // Each Linux I2C bus becomes one libcsp interface owned by this
        // process. Routes below tell libcsp which I2C slave address reaches
        // each CSP node; this is separate from KubOS service-port routing.
        let mut interface = LinuxI2cCspInterface::open(
            I2cInterfaceConfig::new(plan.bus.clone(), csp.obc_node)
                .with_name(format!("I2C{index}"))
                .with_default_route(false),
        )
        .map_err(|source| InterfaceError::Open {
            bus: plan.bus.clone(),
            source,
        })?;

        for route in plan.routes {
            interface
                .route_node_via_i2c_addr(route.node, route.i2c_addr)
                .map_err(|source| InterfaceError::Route {
                    node: route.node,
                    i2c_addr: route.i2c_addr,
                    source,
                })?;
        }

        thread::spawn(move || run_interface_worker(interface, plan.poll));
    }

    Ok(())
}

fn build_plans(csp: &CspSettings, radios: &RadioSettings) -> Vec<InterfacePlan> {
    let mut by_bus: HashMap<String, InterfacePlan> = HashMap::new();

    add_radio_route(&mut by_bus, &radios.uplink);
    add_radio_route(&mut by_bus, &radios.downlink);

    // Anything addressed to the ground station should leave the OBC through
    // the downlink radio. The radio then handles the RF hop to the ground.
    add_route(
        &mut by_bus,
        &radios.downlink.bus,
        RoutePlan {
            node: csp.ground_node,
            i2c_addr: radios.downlink.i2c_addr,
        },
    );

    if let Some(plan) = by_bus.get_mut(&radios.uplink.bus) {
        // I2C devices cannot push frames to us, so the uplink radio is polled.
        // A successful read injects the raw CSP frame into libcsp routing.
        plan.poll = Some(PollPlan {
            i2c_addr: radios.uplink.i2c_addr,
            frame_len: csp.max_frame_bytes,
            interval: csp.poll_interval,
        });
    }

    by_bus.into_values().collect()
}

fn add_radio_route(by_bus: &mut HashMap<String, InterfacePlan>, radio: &RadioConfig) {
    add_route(
        by_bus,
        &radio.bus,
        RoutePlan {
            node: radio.csp_node,
            i2c_addr: radio.i2c_addr,
        },
    );
}

fn add_route(by_bus: &mut HashMap<String, InterfacePlan>, bus: &str, route: RoutePlan) {
    let plan = by_bus
        .entry(bus.to_string())
        .or_insert_with(|| InterfacePlan {
            bus: bus.to_string(),
            routes: Vec::new(),
            poll: None,
        });

    if !plan
        .routes
        .iter()
        .any(|existing| existing.node == route.node && existing.i2c_addr == route.i2c_addr)
    {
        plan.routes.push(route);
    }
}

fn run_interface_worker(mut interface: LinuxI2cCspInterface, poll: Option<PollPlan>) {
    loop {
        match poll {
            Some(poll) => {
                // This reads a complete CSP frame from the radio. libcsp then
                // strips the CSP header before the comms listener sees it.
                if let Err(err) = interface.read_frame_from(poll.i2c_addr, poll.frame_len) {
                    log::debug!("NXTRX4 I2C poll did not inject a frame: {err}");
                }
                thread::sleep(poll.interval);
            }
            None => thread::park(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;
    use crate::config::{CspSettings, UplinkCrypto};

    fn csp() -> CspSettings {
        CspSettings {
            obc_node: 1,
            uplink_packet_csp_port: 10,
            uplink_sfp_csp_port: 12,
            ground_node: 2,
            ground_packet_csp_port: 11,
            ground_sfp_csp_port: 13,
            backlog: 10,
            max_frame_bytes: 260,
            sfp_mtu: 240,
            sfp_max_space_packet_bytes: 4096,
            sfp_use_rdp: true,
            poll_interval: Duration::from_millis(100),
            uplink_crypto: UplinkCrypto::None,
        }
    }

    #[test]
    fn builds_one_plan_for_shared_bus() {
        let radios = RadioSettings {
            uplink: RadioConfig {
                bus: "/dev/i2c-1".to_string(),
                csp_node: 8,
                i2c_addr: 8,
                command_timeout: Duration::from_secs(1),
            },
            downlink: RadioConfig {
                bus: "/dev/i2c-1".to_string(),
                csp_node: 9,
                i2c_addr: 9,
                command_timeout: Duration::from_secs(1),
            },
        };

        let plans = build_plans(&csp(), &radios);

        assert_eq!(plans.len(), 1);
        assert_eq!(plans[0].routes.len(), 3);
        assert!(plans[0].poll.is_some());
    }

    #[test]
    fn builds_two_plans_for_separate_buses() {
        let radios = RadioSettings {
            uplink: RadioConfig {
                bus: "/dev/i2c-1".to_string(),
                csp_node: 8,
                i2c_addr: 8,
                command_timeout: Duration::from_secs(1),
            },
            downlink: RadioConfig {
                bus: "/dev/i2c-2".to_string(),
                csp_node: 9,
                i2c_addr: 9,
                command_timeout: Duration::from_secs(1),
            },
        };

        let plans = build_plans(&csp(), &radios);

        assert_eq!(plans.len(), 2);
        assert_eq!(plans.iter().filter(|plan| plan.poll.is_some()).count(), 1);
    }
}
