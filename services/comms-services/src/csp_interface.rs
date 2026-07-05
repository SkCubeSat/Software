use std::{
    collections::HashMap,
    fs::File,
    io::{self, Read},
    thread,
    time::Duration,
};

use radsat_csp::{I2cInterfaceConfig, LinuxI2cCspInterface};
use thiserror::Error;

use crate::config::{CspSettings, RadioConfig, RadioSettings};

const RECEIVE_ERROR_RETRY_DELAY: Duration = Duration::from_millis(100);

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
    #[error("uplink radio must configure `slave_rx_device` for NXTRX4 I2C slave receive")]
    MissingUplinkSlaveRxDevice,
    #[error("failed to open I2C slave frame device {path}: {source}")]
    OpenSlaveRxDevice { path: String, source: io::Error },
}

#[derive(Debug, Clone)]
struct InterfacePlan {
    bus: String,
    routes: Vec<RoutePlan>,
    rx_source: Option<FrameSourcePlan>,
}

#[derive(Debug, Clone, Copy)]
struct RoutePlan {
    node: u16,
    i2c_addr: u8,
}

#[derive(Debug, Clone)]
struct FrameSourcePlan {
    path: String,
    max_frame_bytes: usize,
}

struct SlaveFrameReceiver {
    path: String,
    file: File,
    buffer: Vec<u8>,
}

pub fn spawn_i2c_workers(csp: &CspSettings, radios: &RadioSettings) -> Result<(), InterfaceError> {
    let mut plans = build_plans(csp, radios)?;

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

        let receiver = plan.rx_source.map(SlaveFrameReceiver::open).transpose()?;

        thread::spawn(move || run_interface_worker(interface, receiver));
    }

    Ok(())
}

fn build_plans(
    csp: &CspSettings,
    radios: &RadioSettings,
) -> Result<Vec<InterfacePlan>, InterfaceError> {
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
        let path = radios
            .uplink
            .slave_rx_device
            .clone()
            .ok_or(InterfaceError::MissingUplinkSlaveRxDevice)?;

        // The NXTRX4 writes received RF frames back to the OBC as I2C master.
        // The external slave backend preserves each I2C write as one CSP frame.
        plan.rx_source = Some(FrameSourcePlan {
            path,
            max_frame_bytes: csp.max_frame_bytes,
        });
    }

    Ok(by_bus.into_values().collect())
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
            rx_source: None,
        });

    if !plan
        .routes
        .iter()
        .any(|existing| existing.node == route.node && existing.i2c_addr == route.i2c_addr)
    {
        plan.routes.push(route);
    }
}

impl SlaveFrameReceiver {
    fn open(plan: FrameSourcePlan) -> Result<Self, InterfaceError> {
        let file = File::open(&plan.path).map_err(|source| InterfaceError::OpenSlaveRxDevice {
            path: plan.path.clone(),
            source,
        })?;

        Ok(Self {
            path: plan.path,
            file,
            buffer: vec![0; plan.max_frame_bytes],
        })
    }

    fn read_frame(&mut self) -> io::Result<&[u8]> {
        loop {
            match self.file.read(&mut self.buffer) {
                Ok(0) => {
                    return Err(io::Error::new(
                        io::ErrorKind::UnexpectedEof,
                        format!("{} returned EOF", self.path),
                    ));
                }
                Ok(len) => return Ok(&self.buffer[..len]),
                Err(err) if err.kind() == io::ErrorKind::Interrupted => continue,
                Err(err) => return Err(err),
            }
        }
    }
}

fn run_interface_worker(
    mut interface: LinuxI2cCspInterface,
    mut receiver: Option<SlaveFrameReceiver>,
) {
    loop {
        match receiver.as_mut() {
            Some(receiver) => match receiver.read_frame() {
                Ok(frame) => {
                    // The backend returns raw CSP-over-I2C bytes. libcsp then
                    // strips the CSP header before the comms listener sees it.
                    if let Err(err) = interface.inject_received_frame(frame) {
                        log::warn!("failed to inject NXTRX4 CSP frame: {err}");
                    }
                }
                Err(err) => {
                    log::warn!("failed to read NXTRX4 I2C slave frame: {err}");
                    thread::sleep(RECEIVE_ERROR_RETRY_DELAY);
                }
            },
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
                slave_rx_device: Some("/dev/i2c-slave-frameq-1-01".to_string()),
                command_timeout: Duration::from_secs(1),
                nmp_keys: Default::default(),
            },
            downlink: RadioConfig {
                bus: "/dev/i2c-1".to_string(),
                csp_node: 9,
                i2c_addr: 9,
                slave_rx_device: None,
                command_timeout: Duration::from_secs(1),
                nmp_keys: Default::default(),
            },
        };

        let plans = build_plans(&csp(), &radios).unwrap();

        assert_eq!(plans.len(), 1);
        assert_eq!(plans[0].routes.len(), 3);
        assert_eq!(
            plans[0]
                .rx_source
                .as_ref()
                .map(|source| source.path.as_str()),
            Some("/dev/i2c-slave-frameq-1-01")
        );
    }

    #[test]
    fn builds_two_plans_for_separate_buses() {
        let radios = RadioSettings {
            uplink: RadioConfig {
                bus: "/dev/i2c-1".to_string(),
                csp_node: 8,
                i2c_addr: 8,
                slave_rx_device: Some("/dev/i2c-slave-frameq-1-01".to_string()),
                command_timeout: Duration::from_secs(1),
                nmp_keys: Default::default(),
            },
            downlink: RadioConfig {
                bus: "/dev/i2c-2".to_string(),
                csp_node: 9,
                i2c_addr: 9,
                slave_rx_device: None,
                command_timeout: Duration::from_secs(1),
                nmp_keys: Default::default(),
            },
        };

        let plans = build_plans(&csp(), &radios).unwrap();

        assert_eq!(plans.len(), 2);
        assert_eq!(
            plans.iter().filter(|plan| plan.rx_source.is_some()).count(),
            1
        );
    }

    #[test]
    fn requires_uplink_slave_rx_device() {
        let radios = RadioSettings {
            uplink: RadioConfig {
                bus: "/dev/i2c-1".to_string(),
                csp_node: 8,
                i2c_addr: 8,
                slave_rx_device: None,
                command_timeout: Duration::from_secs(1),
                nmp_keys: Default::default(),
            },
            downlink: RadioConfig {
                bus: "/dev/i2c-2".to_string(),
                csp_node: 9,
                i2c_addr: 9,
                slave_rx_device: None,
                command_timeout: Duration::from_secs(1),
                nmp_keys: Default::default(),
            },
        };

        assert!(matches!(
            build_plans(&csp(), &radios),
            Err(InterfaceError::MissingUplinkSlaveRxDevice)
        ));
    }
}
