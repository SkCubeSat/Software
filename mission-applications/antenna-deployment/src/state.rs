use failure::{Error, bail, format_err};
use kubos_app::ServiceConfig;
use serde_json::Value;
use std::io::{Read, Write};
use std::net::TcpStream;
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
    let addr = cfg
        .hosturl()
        .ok_or_else(|| format_err!("Unable to fetch addr for service"))?;

    let body = serde_json::json!({ "query": request }).to_string();
    let http_request = format!(
        concat!(
            "POST /graphql HTTP/1.1\r\n",
            "Host: {addr}\r\n",
            "Content-Type: application/json\r\n",
            "Accept: application/json\r\n",
            "Connection: close\r\n",
            "Content-Length: {content_length}\r\n",
            "\r\n",
            "{body}"
        ),
        addr = addr,
        content_length = body.len(),
        body = body
    );

    let mut stream = TcpStream::connect(&addr)
        .map_err(|e| format_err!("failed to connect to fram-service at {}: {}", addr, e))?;
    stream
        .set_read_timeout(Some(FRAM_TIMEOUT))
        .map_err(|e| format_err!("failed to set fram-service read timeout: {}", e))?;
    stream
        .set_write_timeout(Some(FRAM_TIMEOUT))
        .map_err(|e| format_err!("failed to set fram-service write timeout: {}", e))?;
    stream
        .write_all(http_request.as_bytes())
        .map_err(|e| format_err!("failed to send fram-service request: {}", e))?;

    let mut response = Vec::new();
    stream
        .read_to_end(&mut response)
        .map_err(|e| format_err!("failed to read fram-service response: {}", e))?;

    parse_graphql_response(&response)
}

fn parse_graphql_response(response: &[u8]) -> Result<Value, Error> {
    let header_end = response
        .windows(4)
        .position(|window| window == b"\r\n\r\n")
        .ok_or_else(|| format_err!("invalid HTTP response from fram-service"))?;

    let headers = std::str::from_utf8(&response[..header_end])
        .map_err(|e| format_err!("fram-service response headers were not valid UTF-8: {}", e))?;
    let mut header_lines = headers.lines();
    let status_line = header_lines
        .next()
        .ok_or_else(|| format_err!("fram-service response missing HTTP status line"))?;

    if !status_line.contains(" 200 ") {
        bail!("fram-service returned non-200 response: {}", status_line);
    }

    let is_chunked = header_lines.any(|line| {
        let mut parts = line.splitn(2, ':');
        let key = parts.next().unwrap_or_default().trim();
        let value = parts.next().unwrap_or_default().trim();
        key.eq_ignore_ascii_case("Transfer-Encoding") && value.eq_ignore_ascii_case("chunked")
    });

    let body_bytes = &response[header_end + 4..];
    let body = if is_chunked {
        decode_chunked_body(body_bytes)?
    } else {
        body_bytes.to_vec()
    };

    let response: serde_json::Value = serde_json::from_slice(&body)
        .map_err(|e| format_err!("invalid JSON from fram-service: {}", e))?;

    if let Some(errs) = response.get("errors") {
        if errs.is_string() {
            let errs_str = errs.as_str().unwrap_or_default();
            if !errs_str.is_empty() {
                bail!("{}", errs_str);
            }
        } else if !errs.is_null() {
            if let Some(message) = errs.get("message").and_then(Value::as_str) {
                bail!("{}", message);
            }
            bail!("{}", serde_json::to_string(errs)?);
        }
    }

    if let Some(err) = response.get(0) {
        if let Some(message) = err.get("message").and_then(Value::as_str) {
            bail!("{}", message);
        }
    }

    response
        .get("data")
        .cloned()
        .ok_or_else(|| format_err!("No result returned in 'data' key: {}", response))
}

fn decode_chunked_body(bytes: &[u8]) -> Result<Vec<u8>, Error> {
    let mut decoded = Vec::new();
    let mut cursor = 0usize;

    while cursor < bytes.len() {
        let size_end = bytes[cursor..]
            .windows(2)
            .position(|window| window == b"\r\n")
            .ok_or_else(|| format_err!("invalid chunked fram-service response"))?;
        let size_slice = &bytes[cursor..cursor + size_end];
        let size_str = std::str::from_utf8(size_slice)
            .map_err(|e| format_err!("invalid chunk size in fram-service response: {}", e))?;
        let size_hex = size_str.split(';').next().unwrap_or_default().trim();
        let chunk_size = usize::from_str_radix(size_hex, 16)
            .map_err(|e| format_err!("invalid chunk size '{}' in fram-service response: {}", size_hex, e))?;
        cursor += size_end + 2;

        if chunk_size == 0 {
            return Ok(decoded);
        }

        let chunk_end = cursor + chunk_size;
        if chunk_end + 2 > bytes.len() {
            bail!("truncated chunked fram-service response");
        }

        decoded.extend_from_slice(&bytes[cursor..chunk_end]);
        if &bytes[chunk_end..chunk_end + 2] != b"\r\n" {
            bail!("invalid chunk terminator in fram-service response");
        }
        cursor = chunk_end + 2;
    }

    bail!("chunked fram-service response missing terminating chunk")
}
