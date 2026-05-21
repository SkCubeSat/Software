use failure::{Error, bail, format_err};
use kubos_app::{ServiceConfig, query};
use serde_json::Value;
use std::time::Duration;

const FRAM_TIMEOUT: Duration = Duration::from_secs(2);

#[derive(Clone, Debug, Default)]
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

#[derive(Copy, Clone, Debug)]
pub enum MissionFlagKey {
    RemoveBeforeFlight,
    Deployed,
    SolarPanelDeployed,
    UhfAntennaDeployed,
    VhfAntennaDeployed,
    InitialSafeStateComplete,
    DetumblingComplete,
}

impl MissionFlagKey {
    fn as_graphql(self) -> &'static str {
        match self {
            Self::RemoveBeforeFlight => "REMOVE_BEFORE_FLIGHT",
            Self::Deployed => "DEPLOYED",
            Self::SolarPanelDeployed => "SOLAR_PANEL_DEPLOYED",
            Self::UhfAntennaDeployed => "UHF_ANTENNA_DEPLOYED",
            Self::VhfAntennaDeployed => "VHF_ANTENNA_DEPLOYED",
            Self::InitialSafeStateComplete => "INITIAL_SAFE_STATE_COMPLETE",
            Self::DetumblingComplete => "DETUMBLING_COMPLETE",
        }
    }
}

pub fn parse_flag_key(input: &str) -> Option<MissionFlagKey> {
    match input.trim().to_ascii_lowercase().as_str() {
        "remove_before_flight" => Some(MissionFlagKey::RemoveBeforeFlight),
        "deployed" => Some(MissionFlagKey::Deployed),
        "solar_panel_deployed" => Some(MissionFlagKey::SolarPanelDeployed),
        "uhf_antenna_deployed" => Some(MissionFlagKey::UhfAntennaDeployed),
        "vhf_antenna_deployed" => Some(MissionFlagKey::VhfAntennaDeployed),
        "initial_safe_state_complete" => Some(MissionFlagKey::InitialSafeStateComplete),
        "detumbling_complete" => Some(MissionFlagKey::DetumblingComplete),
        _ => None,
    }
}

pub fn parse_bool(input: &str) -> Option<bool> {
    match input.trim().to_ascii_lowercase().as_str() {
        "1" | "true" | "t" | "yes" | "y" => Some(true),
        "0" | "false" | "f" | "no" | "n" => Some(false),
        _ => None,
    }
}

pub fn reconcile_mission_state(dry_run: bool) -> Result<(), Error> {
    let request =
        format!("mutation {{ reconcileMissionState(dryRun: {dry_run}) {{ success errors }} }}");
    let response = graphql(&request)?;

    let reconcile = response
        .get("reconcileMissionState")
        .ok_or_else(|| format_err!("missing reconcileMissionState in FRAM response"))?;

    if reconcile
        .get("success")
        .and_then(Value::as_bool)
        .unwrap_or(false)
    {
        return Ok(());
    }

    let errors = reconcile
        .get("errors")
        .and_then(Value::as_str)
        .unwrap_or("unknown reconcileMissionState error");

    bail!("reconcileMissionState failed: {}", errors)
}

pub fn read_mission_state() -> Result<MissionState, Error> {
    let request = r#"
    {
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
    }
    "#;

    let response = graphql(request)?;
    let state = response
        .get("missionState")
        .ok_or_else(|| format_err!("missing missionState in FRAM response"))?;

    Ok(MissionState {
        remove_before_flight: state
            .get("removeBeforeFlight")
            .and_then(Value::as_bool)
            .unwrap_or(false),
        deployed: state
            .get("deployed")
            .and_then(Value::as_bool)
            .unwrap_or(false),
        deploy_start: state.get("deployStart").and_then(Value::as_i64),
        solar_panel_deployed: state
            .get("solarPanelDeployed")
            .and_then(Value::as_bool)
            .unwrap_or(false),
        uhf_antenna_deployed: state
            .get("uhfAntennaDeployed")
            .and_then(Value::as_bool)
            .unwrap_or(false),
        vhf_antenna_deployed: state
            .get("vhfAntennaDeployed")
            .and_then(Value::as_bool)
            .unwrap_or(false),
        initial_safe_state_complete: state
            .get("initialSafeStateComplete")
            .and_then(Value::as_bool)
            .unwrap_or(false),
        detumbling_complete: state
            .get("detumblingComplete")
            .and_then(Value::as_bool)
            .unwrap_or(false),
    })
}

pub fn set_deploy_start(timestamp: i64) -> Result<(), Error> {
    if timestamp < 0 {
        bail!("deploy_start timestamp must be non-negative");
    }

    let request = format!(
        "mutation {{ setDeployStart(timestamp: {timestamp}, mirrorToEnv: true) {{ success errors }} }}"
    );

    let response = graphql(&request)?;
    let result = response
        .get("setDeployStart")
        .ok_or_else(|| format_err!("missing setDeployStart in FRAM response"))?;

    if result
        .get("success")
        .and_then(Value::as_bool)
        .unwrap_or(false)
    {
        return Ok(());
    }

    let errors = result
        .get("errors")
        .and_then(Value::as_str)
        .unwrap_or("unknown setDeployStart error");

    bail!("setDeployStart failed: {}", errors)
}

pub fn clear_deploy_start() -> Result<(), Error> {
    let request = "mutation { setDeployStart(mirrorToEnv: true) { success errors } }";
    let response = graphql(request)?;
    let result = response
        .get("setDeployStart")
        .ok_or_else(|| format_err!("missing setDeployStart in FRAM response"))?;

    if result
        .get("success")
        .and_then(Value::as_bool)
        .unwrap_or(false)
    {
        return Ok(());
    }

    let errors = result
        .get("errors")
        .and_then(Value::as_str)
        .unwrap_or("unknown setDeployStart error");

    bail!("clear deploy_start failed: {}", errors)
}

pub fn set_flag(key: MissionFlagKey, value: bool) -> Result<(), Error> {
    let request = format!(
        "mutation {{ setMissionFlag(key: {}, value: {}, mirrorToEnv: true) {{ success errors }} }}",
        key.as_graphql(),
        value
    );

    let response = graphql(&request)?;
    let result = response
        .get("setMissionFlag")
        .ok_or_else(|| format_err!("missing setMissionFlag in FRAM response"))?;

    if result
        .get("success")
        .and_then(Value::as_bool)
        .unwrap_or(false)
    {
        return Ok(());
    }

    let errors = result
        .get("errors")
        .and_then(Value::as_str)
        .unwrap_or("unknown setMissionFlag error");

    bail!(
        "setMissionFlag failed ({}={}): {}",
        key.as_graphql(),
        value,
        errors
    )
}

fn graphql(request: &str) -> Result<Value, Error> {
    let cfg = ServiceConfig::new("fram-service")?;
    query(&cfg, request, Some(FRAM_TIMEOUT))
}
