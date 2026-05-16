use std::fmt;
use std::str::FromStr;

use async_graphql::{Enum, SimpleObject};

pub const MISSION_KEYS: [MissionKey; 8] = [
    MissionKey::RemoveBeforeFlight,
    MissionKey::Deployed,
    MissionKey::DeployStart,
    MissionKey::SolarPanelDeployed,
    MissionKey::UhfAntennaDeployed,
    MissionKey::VhfAntennaDeployed,
    MissionKey::InitialSafeStateComplete,
    MissionKey::DetumblingComplete,
];

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MissionKey {
    RemoveBeforeFlight,
    Deployed,
    DeployStart,
    SolarPanelDeployed,
    UhfAntennaDeployed,
    VhfAntennaDeployed,
    InitialSafeStateComplete,
    DetumblingComplete,
}

impl MissionKey {
    pub fn id(self) -> u8 {
        match self {
            Self::RemoveBeforeFlight => 1,
            Self::Deployed => 2,
            Self::DeployStart => 3,
            Self::SolarPanelDeployed => 4,
            Self::UhfAntennaDeployed => 5,
            Self::VhfAntennaDeployed => 6,
            Self::InitialSafeStateComplete => 7,
            Self::DetumblingComplete => 8,
        }
    }

    pub fn from_id(id: u8) -> Option<Self> {
        match id {
            1 => Some(Self::RemoveBeforeFlight),
            2 => Some(Self::Deployed),
            3 => Some(Self::DeployStart),
            4 => Some(Self::SolarPanelDeployed),
            5 => Some(Self::UhfAntennaDeployed),
            6 => Some(Self::VhfAntennaDeployed),
            7 => Some(Self::InitialSafeStateComplete),
            8 => Some(Self::DetumblingComplete),
            _ => None,
        }
    }

    pub fn env_name(self) -> &'static str {
        match self {
            Self::RemoveBeforeFlight => "remove_before_flight",
            Self::Deployed => "deployed",
            Self::DeployStart => "deploy_start",
            Self::SolarPanelDeployed => "solar_panel_deployed",
            Self::UhfAntennaDeployed => "uhf_antenna_deployed",
            Self::VhfAntennaDeployed => "vhf_antenna_deployed",
            Self::InitialSafeStateComplete => "initial_safe_state_complete",
            Self::DetumblingComplete => "detumbling_complete",
        }
    }

    pub fn is_completion_flag(self) -> bool {
        matches!(
            self,
            Self::Deployed
                | Self::SolarPanelDeployed
                | Self::UhfAntennaDeployed
                | Self::VhfAntennaDeployed
                | Self::InitialSafeStateComplete
                | Self::DetumblingComplete
        )
    }

    pub fn default_value(self) -> MissionValue {
        match self {
            Self::DeployStart => MissionValue::Timestamp(None),
            _ => MissionValue::Bool(false),
        }
    }
}

impl fmt::Display for MissionKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.env_name())
    }
}

#[derive(Enum, Copy, Clone, Debug, Eq, PartialEq)]
#[graphql(rename_items = "SCREAMING_SNAKE_CASE")]
pub enum MissionFlagKey {
    RemoveBeforeFlight,
    Deployed,
    SolarPanelDeployed,
    UhfAntennaDeployed,
    VhfAntennaDeployed,
    InitialSafeStateComplete,
    DetumblingComplete,
}

impl From<MissionFlagKey> for MissionKey {
    fn from(key: MissionFlagKey) -> Self {
        match key {
            MissionFlagKey::RemoveBeforeFlight => Self::RemoveBeforeFlight,
            MissionFlagKey::Deployed => Self::Deployed,
            MissionFlagKey::SolarPanelDeployed => Self::SolarPanelDeployed,
            MissionFlagKey::UhfAntennaDeployed => Self::UhfAntennaDeployed,
            MissionFlagKey::VhfAntennaDeployed => Self::VhfAntennaDeployed,
            MissionFlagKey::InitialSafeStateComplete => Self::InitialSafeStateComplete,
            MissionFlagKey::DetumblingComplete => Self::DetumblingComplete,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ValueType {
    Bool,
    Timestamp,
}

impl ValueType {
    pub fn id(self) -> u8 {
        match self {
            Self::Bool => 1,
            Self::Timestamp => 2,
        }
    }

    pub fn from_id(id: u8) -> Option<Self> {
        match id {
            1 => Some(Self::Bool),
            2 => Some(Self::Timestamp),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MissionValue {
    Bool(bool),
    Timestamp(Option<u64>),
}

impl MissionValue {
    pub fn value_type(&self) -> ValueType {
        match self {
            Self::Bool(_) => ValueType::Bool,
            Self::Timestamp(_) => ValueType::Timestamp,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(value) => Some(*value),
            Self::Timestamp(_) => None,
        }
    }

    pub fn as_timestamp(&self) -> Option<Option<u64>> {
        match self {
            Self::Timestamp(value) => Some(*value),
            Self::Bool(_) => None,
        }
    }

    pub fn to_env_value(&self) -> Option<String> {
        match self {
            Self::Bool(true) => Some("true".to_string()),
            Self::Bool(false) => Some("false".to_string()),
            Self::Timestamp(Some(value)) => Some(value.to_string()),
            Self::Timestamp(None) => None,
        }
    }

    pub fn display_value(&self) -> Option<String> {
        self.to_env_value()
    }
}

pub fn parse_env_value(key: MissionKey, value: &str) -> Result<MissionValue, String> {
    match key {
        MissionKey::DeployStart => {
            let value = value.trim();
            if value.is_empty() {
                Ok(MissionValue::Timestamp(None))
            } else {
                u64::from_str(value)
                    .map(|value| MissionValue::Timestamp(Some(value)))
                    .map_err(|_| format!("invalid deploy_start timestamp '{value}'"))
            }
        }
        _ => parse_bool(value).map(MissionValue::Bool),
    }
}

pub fn parse_bool(value: &str) -> Result<bool, String> {
    match value.trim().to_ascii_lowercase().as_str() {
        "t" | "true" | "1" | "y" | "yes" => Ok(true),
        "f" | "false" | "0" | "n" | "no" => Ok(false),
        other => Err(format!("invalid bool value '{other}'")),
    }
}

#[derive(SimpleObject, Clone, Debug, Default, Eq, PartialEq)]
pub struct MissionState {
    pub remove_before_flight: bool,
    pub deployed: bool,
    pub deploy_start: Option<i64>,
    pub solar_panel_deployed: bool,
    pub uhf_antenna_deployed: bool,
    pub vhf_antenna_deployed: bool,
    pub initial_safe_state_complete: bool,
    pub detumbling_complete: bool,
    pub sync: Vec<SyncAction>,
}

impl MissionState {
    pub fn set(&mut self, key: MissionKey, value: &MissionValue) {
        match (key, value) {
            (MissionKey::RemoveBeforeFlight, MissionValue::Bool(value)) => {
                self.remove_before_flight = *value
            }
            (MissionKey::Deployed, MissionValue::Bool(value)) => self.deployed = *value,
            (MissionKey::DeployStart, MissionValue::Timestamp(value)) => {
                self.deploy_start = value.map(|value| value as i64)
            }
            (MissionKey::SolarPanelDeployed, MissionValue::Bool(value)) => {
                self.solar_panel_deployed = *value
            }
            (MissionKey::UhfAntennaDeployed, MissionValue::Bool(value)) => {
                self.uhf_antenna_deployed = *value
            }
            (MissionKey::VhfAntennaDeployed, MissionValue::Bool(value)) => {
                self.vhf_antenna_deployed = *value
            }
            (MissionKey::InitialSafeStateComplete, MissionValue::Bool(value)) => {
                self.initial_safe_state_complete = *value
            }
            (MissionKey::DetumblingComplete, MissionValue::Bool(value)) => {
                self.detumbling_complete = *value
            }
            _ => {}
        }
    }
}

#[derive(SimpleObject, Clone, Debug, Default, Eq, PartialEq)]
pub struct SyncAction {
    pub key: String,
    pub fram_value: Option<String>,
    pub env_value: Option<String>,
    pub merged_value: Option<String>,
    pub action: String,
    pub errors: String,
}

#[derive(SimpleObject, Clone, Debug, Default, Eq, PartialEq)]
pub struct ReconcileResponse {
    pub success: bool,
    pub errors: String,
    pub actions: Vec<SyncAction>,
    pub state: MissionState,
}
