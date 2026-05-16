use failure::{bail, Error};
use kubos_app::ServiceConfig;
use std::time::Duration;

pub struct MissionState {
    pub remove_before_flight: bool,
    pub deployed: bool,
    pub deploy_start: Option<i64>,
    pub solar_panel_deployed: bool,
    pub uhf_antenna_deployed: bool,
    pub vhf_antenna_deployed: bool,
    pub initial_safe_state_complete: bool,
    pub detumbling_complete: bool,
}

pub fn read_state() -> Result<MissionState, Error> {
    let request = r#"{
        missionState {
            removeBeforeFlight
            deployed
            deployStart
            solarPanelDeployed
            uhfAntennaDeployed
            vhfAntennaDeployed
            initialSafeStateComplete
            detumblingComplete
        }
    }"#;

    let response = kubos_app::query(
        &ServiceConfig::new("fram-service")?,
        request,
        Some(Duration::from_secs(5)),
    )?;

    let s = &response["missionState"];

    Ok(MissionState {
        remove_before_flight: s["removeBeforeFlight"].as_bool().unwrap_or(false),
        deployed:             s["deployed"].as_bool().unwrap_or(false),
        deploy_start:         s["deployStart"].as_i64(),
        solar_panel_deployed: s["solarPanelDeployed"].as_bool().unwrap_or(false),
        uhf_antenna_deployed: s["uhfAntennaDeployed"].as_bool().unwrap_or(false),
        vhf_antenna_deployed: s["vhfAntennaDeployed"].as_bool().unwrap_or(false),
        initial_safe_state_complete: s["initialSafeStateComplete"].as_bool().unwrap_or(false),
        detumbling_complete:  s["detumblingComplete"].as_bool().unwrap_or(false),
    })
}

// state.rs  (additions only — struct and read_state() unchanged)

pub fn reconcile() -> Result<(), Error> {
    let request = r#"
        mutation {
            reconcileMissionState(dryRun: false) {
                success
                errors
            }
        }
    "#;

    let response = kubos_app::query(
        &ServiceConfig::new("fram-service")?,
        request,
        Some(Duration::from_secs(5)),
    )?;

    let success = response["reconcileMissionState"]["success"]
        .as_bool()
        .unwrap_or(false);

    if !success {
        let errors = &response["reconcileMissionState"]["errors"];
        bail!("reconcileMissionState failed: {}", errors);
    }

    Ok(())
}

pub fn set_flag(key: &str, value: bool) -> Result<(), Error> {
    let request = format!(
        r#"
        mutation {{
            setMissionFlag(key: {key}, value: {value}) {{
                success
                errors
            }}
        }}
    "#,
        key = key,
        value = value,
    );

    let response = kubos_app::query(
        &ServiceConfig::new("fram-service")?,
        &request,
        Some(Duration::from_secs(5)),
    )?;

    let success = response["setMissionFlag"]["success"]
        .as_bool()
        .unwrap_or(false);

    if !success {
        let errors = &response["setMissionFlag"]["errors"];
        bail!("setMissionFlag({key}) failed: {}", errors);
    }

    Ok(())
}

pub fn set_deploy_start(timestamp: i64) -> Result<(), Error> {
    let request = format!(
        r#"
        mutation {{
            setDeployStart(timestamp: {timestamp}) {{
                success
                errors
            }}
        }}
    "#,
        timestamp = timestamp,
    );

    let response = kubos_app::query(
        &ServiceConfig::new("fram-service")?,
        &request,
        Some(Duration::from_secs(5)),
    )?;

    let success = response["setDeployStart"]["success"]
        .as_bool()
        .unwrap_or(false);

    if !success {
        let errors = &response["setDeployStart"]["errors"];
        bail!("setDeployStart failed: {}", errors);
    }

    Ok(())
}