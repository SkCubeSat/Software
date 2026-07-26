use std::time::{SystemTime, UNIX_EPOCH};

use crate::model::{
    MISSION_KEYS, MissionKey, MissionState, MissionValue, SyncAction, parse_env_value,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SourceValue {
    Missing,
    Invalid(String),
    Unavailable(String),
    Value(MissionValue),
}

impl SourceValue {
    pub fn display_value(&self) -> Option<String> {
        match self {
            Self::Value(value) => value.display_value(),
            Self::Missing | Self::Invalid(_) | Self::Unavailable(_) => None,
        }
    }

    pub fn error(&self) -> Option<String> {
        match self {
            Self::Invalid(err) | Self::Unavailable(err) => Some(err.clone()),
            Self::Missing | Self::Value(_) => None,
        }
    }
}

pub fn source_from_env(key: MissionKey, value: Option<String>) -> SourceValue {
    match value {
        Some(value) => match parse_env_value(key, &value) {
            Ok(value) => SourceValue::Value(value),
            Err(err) => SourceValue::Invalid(err),
        },
        None => SourceValue::Missing,
    }
}

pub fn merge_value(
    key: MissionKey,
    fram: &SourceValue,
    env: &SourceValue,
    now_timestamp: u64,
) -> MissionValue {
    match key {
        MissionKey::RemoveBeforeFlight => {
            let value = [fram, env]
                .iter()
                .any(|source| matches!(source, SourceValue::Value(MissionValue::Bool(true))));
            MissionValue::Bool(value)
        }
        MissionKey::DeployStart => merge_deploy_start(fram, env, now_timestamp),
        key if key.is_completion_flag() => {
            let values: Vec<bool> = [fram, env]
                .iter()
                .filter_map(|source| match source {
                    SourceValue::Value(MissionValue::Bool(value)) => Some(*value),
                    _ => None,
                })
                .collect();

            if values.iter().any(|value| !*value) {
                MissionValue::Bool(false)
            } else if values.iter().any(|value| *value) {
                MissionValue::Bool(true)
            } else {
                MissionValue::Bool(false)
            }
        }
        _ => key.default_value(),
    }
}

pub fn now_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0)
}

pub fn build_state(values: &[(MissionKey, MissionValue)], sync: Vec<SyncAction>) -> MissionState {
    let mut state = MissionState {
        sync,
        ..MissionState::default()
    };

    for (key, value) in values {
        state.set(*key, value);
    }

    state
}

pub fn all_default_values() -> Vec<(MissionKey, MissionValue)> {
    MISSION_KEYS
        .iter()
        .map(|key| (*key, key.default_value()))
        .collect()
}

fn merge_deploy_start(fram: &SourceValue, env: &SourceValue, now_timestamp: u64) -> MissionValue {
    let mut timestamps = Vec::new();
    let mut saw_invalid = false;

    for source in [fram, env] {
        match source {
            SourceValue::Value(MissionValue::Timestamp(Some(value))) => timestamps.push(*value),
            SourceValue::Value(MissionValue::Timestamp(None)) | SourceValue::Missing => {}
            SourceValue::Unavailable(_) => {}
            SourceValue::Invalid(_) => saw_invalid = true,
            SourceValue::Value(MissionValue::Bool(_)) => saw_invalid = true,
        }
    }

    if let Some(max) = timestamps.into_iter().max() {
        MissionValue::Timestamp(Some(max))
    } else if saw_invalid {
        MissionValue::Timestamp(Some(now_timestamp))
    } else {
        MissionValue::Timestamp(None)
    }
}

pub fn source_matches(source: &SourceValue, merged: &MissionValue) -> bool {
    matches!(source, SourceValue::Value(value) if value == merged)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_before_flight_or_merges() {
        let merged = merge_value(
            MissionKey::RemoveBeforeFlight,
            &SourceValue::Value(MissionValue::Bool(true)),
            &SourceValue::Value(MissionValue::Bool(false)),
            10,
        );
        assert_eq!(merged, MissionValue::Bool(true));
    }

    #[test]
    fn completion_flag_false_beats_true() {
        let merged = merge_value(
            MissionKey::Deployed,
            &SourceValue::Value(MissionValue::Bool(true)),
            &SourceValue::Value(MissionValue::Bool(false)),
            10,
        );
        assert_eq!(merged, MissionValue::Bool(false));
    }

    #[test]
    fn deploy_start_latest_timestamp_wins() {
        let merged = merge_value(
            MissionKey::DeployStart,
            &SourceValue::Value(MissionValue::Timestamp(Some(100))),
            &SourceValue::Value(MissionValue::Timestamp(Some(200))),
            10,
        );
        assert_eq!(merged, MissionValue::Timestamp(Some(200)));
    }

    #[test]
    fn invalid_deploy_start_restarts_hold_timer() {
        let merged = merge_value(
            MissionKey::DeployStart,
            &SourceValue::Invalid("bad".to_string()),
            &SourceValue::Missing,
            123,
        );
        assert_eq!(merged, MissionValue::Timestamp(Some(123)));
    }
}
