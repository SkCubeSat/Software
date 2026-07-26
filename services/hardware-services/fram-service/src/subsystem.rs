use std::sync::{Arc, Mutex};

use crate::backend::{ByteStorage, DEFAULT_CAPACITY_BYTES, FileImageBackend};
use crate::env::{CommandEnvStore, EnvStore};
use crate::layout::FramStorage;
use crate::model::{
    MISSION_KEYS, MissionFlagKey, MissionKey, MissionState, MissionValue, ReconcileResponse,
    SyncAction,
};
use crate::reconcile::{
    SourceValue, build_state, merge_value, now_timestamp, source_from_env, source_matches,
};

#[derive(Clone)]
pub struct Subsystem {
    fram: Arc<Mutex<FramStorage>>,
    env: Arc<Mutex<Box<dyn EnvStore>>>,
    backend_name: String,
    last_error: Arc<Mutex<Option<String>>>,
}

#[derive(Clone, Debug)]
pub struct Health {
    pub backend: String,
    pub capacity_bytes: u32,
    pub fram_reachable: bool,
    pub last_error: Option<String>,
}

impl Subsystem {
    pub fn from_config(config: &kubos_service::Config) -> Result<Self, String> {
        let backend = config
            .get("backend")
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .unwrap_or_else(|| "file".to_string());

        let storage: Box<dyn ByteStorage> = match backend.as_str() {
            "file" => {
                let path = config
                    .get("image_path")
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_else(|| "/tmp/fram-service.img".to_string());

                let capacity = config
                    .get("image_capacity_bytes")
                    .and_then(|v| v.as_integer())
                    .map(|v| v as u32)
                    .unwrap_or(DEFAULT_CAPACITY_BYTES);

                Box::new(FileImageBackend::new(&path, capacity).map_err(|e| e.to_string())?)
            }
            "i2c" => {
                #[cfg(feature = "i2c")]
                {
                    let bus = config
                        .get("i2c_bus")
                        .and_then(|v| v.as_str().map(|s| s.to_string()))
                        .unwrap_or_else(|| "/dev/i2c-1".to_string());
                    let addr = config
                        .get("i2c_addr")
                        .and_then(|v| v.as_str().map(parse_hex_u8))
                        .transpose()?
                        .unwrap_or(0x50);
                    let capacity = config
                        .get("capacity_bytes")
                        .and_then(|v| v.as_integer())
                        .map(|v| v as u32)
                        .unwrap_or(DEFAULT_CAPACITY_BYTES);
                    let address_width = config
                        .get("address_width_bytes")
                        .and_then(|v| v.as_integer())
                        .map(|v| v as u8)
                        .unwrap_or(2);
                    let max_transfer = config
                        .get("max_transfer_bytes")
                        .and_then(|v| v.as_integer())
                        .map(|v| v as usize)
                        .unwrap_or(crate::backend::DEFAULT_MAX_TRANSFER_BYTES);

                    Box::new(
                        crate::backend::I2cFramBackend::new(
                            &bus,
                            addr,
                            capacity,
                            address_width,
                            max_transfer,
                        )
                        .map_err(|e| e.to_string())?,
                    )
                }

                #[cfg(not(feature = "i2c"))]
                {
                    return Err(
                        "backend='i2c' requested but service was built without 'i2c' feature"
                            .to_string(),
                    );
                }
            }
            other => {
                return Err(format!(
                    "unsupported backend '{other}'. expected 'file' or 'i2c'"
                ));
            }
        };

        let printenv = config
            .get("fw_printenv")
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .unwrap_or_else(|| "/usr/sbin/fw_printenv".to_string());
        let setenv = config
            .get("fw_setenv")
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .unwrap_or_else(|| "/usr/sbin/fw_setenv".to_string());

        Self::from_parts(
            backend,
            storage,
            Box::new(CommandEnvStore::new(printenv, setenv)),
        )
    }

    pub fn from_parts(
        backend_name: String,
        storage: Box<dyn ByteStorage>,
        env: Box<dyn EnvStore>,
    ) -> Result<Self, String> {
        let fram = FramStorage::mount(storage).map_err(|err| err.to_string())?;
        Ok(Self {
            fram: Arc::new(Mutex::new(fram)),
            env: Arc::new(Mutex::new(env)),
            backend_name,
            last_error: Arc::new(Mutex::new(None)),
        })
    }

    pub fn health(&self) -> Health {
        let capacity = self.capacity();
        let fram_reachable = self
            .read_fram_source(MissionKey::RemoveBeforeFlight)
            .is_ok();

        Health {
            backend: self.backend_name.clone(),
            capacity_bytes: capacity,
            fram_reachable,
            last_error: self.last_error.lock().ok().and_then(|err| err.clone()),
        }
    }

    pub fn capacity(&self) -> u32 {
        self.fram.lock().map(|fram| fram.capacity()).unwrap_or(0)
    }

    pub fn mission_state(&self, reconcile: bool) -> Result<MissionState, String> {
        if reconcile {
            return Ok(self.reconcile(false)?.state);
        }

        let mut values = Vec::new();
        for key in MISSION_KEYS {
            let value = match self.read_fram_source(key)? {
                SourceValue::Value(value) => value,
                SourceValue::Missing | SourceValue::Invalid(_) | SourceValue::Unavailable(_) => {
                    key.default_value()
                }
            };
            values.push((key, value));
        }

        Ok(build_state(&values, Vec::new()))
    }

    pub fn reconcile(&self, dry_run: bool) -> Result<ReconcileResponse, String> {
        let now = now_timestamp();
        let mut actions = Vec::new();
        let mut values = Vec::new();
        let mut errors = Vec::new();

        for key in MISSION_KEYS {
            let fram_source = match self.read_fram_source(key) {
                Ok(source) => source,
                Err(err) => {
                    errors.push(format!("{} FRAM read: {err}", key.env_name()));
                    SourceValue::Unavailable(err)
                }
            };
            let env_source = match self.read_env_source(key) {
                Ok(source) => source,
                Err(err) => {
                    errors.push(format!("{} env read: {err}", key.env_name()));
                    SourceValue::Invalid(err)
                }
            };

            let merged = merge_value(key, &fram_source, &env_source, now);
            let mut action = Vec::new();
            let mut action_errors = Vec::new();

            if matches!(fram_source, SourceValue::Unavailable(_)) {
                action.push("fram_unavailable");
            } else if !source_matches(&fram_source, &merged) {
                action.push("write_fram");
                if !dry_run {
                    if let Err(err) = self.write_fram_value(key, &merged) {
                        action_errors.push(err.clone());
                        errors.push(format!("{} FRAM write: {err}", key.env_name()));
                    }
                }
            }

            if !source_matches(&env_source, &merged) {
                action.push("write_env");
                if !dry_run {
                    if let Err(err) = self.write_env_value(key, &merged) {
                        action_errors.push(err.clone());
                        errors.push(format!("{} env write: {err}", key.env_name()));
                    }
                }
            }

            actions.push(SyncAction {
                key: key.env_name().to_string(),
                fram_value: fram_source.display_value(),
                env_value: env_source.display_value(),
                merged_value: merged.display_value(),
                action: if action.is_empty() {
                    "none".to_string()
                } else if dry_run {
                    format!("dry_run:{}", action.join(","))
                } else {
                    action.join(",")
                },
                errors: collect_source_errors(&fram_source, &env_source, &action_errors),
            });
            values.push((key, merged));
        }

        let state = build_state(&values, actions.clone());
        Ok(ReconcileResponse {
            success: errors.is_empty(),
            errors: errors.join("; "),
            actions,
            state,
        })
    }

    pub fn set_flag(
        &self,
        key: MissionFlagKey,
        value: bool,
        mirror_to_env: bool,
    ) -> Result<MissionState, String> {
        let key = MissionKey::from(key);
        let value = MissionValue::Bool(value);
        self.write_fram_value(key, &value)?;
        if mirror_to_env {
            self.write_env_value(key, &value)?;
        }

        self.mission_state(false)
    }

    pub fn set_deploy_start(
        &self,
        timestamp: Option<u64>,
        mirror_to_env: bool,
    ) -> Result<MissionState, String> {
        let value = MissionValue::Timestamp(timestamp);
        self.write_fram_value(MissionKey::DeployStart, &value)?;
        if mirror_to_env {
            self.write_env_value(MissionKey::DeployStart, &value)?;
        }

        self.mission_state(false)
    }

    pub fn initialize_flight_state(&self, confirm: bool) -> Result<(), String> {
        if !confirm {
            return Err("initializeFlightState is destructive: pass confirm=true".to_string());
        }

        let mut errors = Vec::new();
        for key in MISSION_KEYS {
            let value = key.default_value();
            if let Err(err) = self.write_fram_value(key, &value) {
                errors.push(format!("{} FRAM write: {err}", key.env_name()));
            }
            if let Err(err) = self.write_env_value(key, &value) {
                errors.push(format!("{} env write: {err}", key.env_name()));
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors.join("; "))
        }
    }

    fn read_fram_source(&self, key: MissionKey) -> Result<SourceValue, String> {
        let mut fram = self.fram.lock().map_err(lock_error)?;
        match fram.read_key(key) {
            Ok(Some(value)) => Ok(SourceValue::Value(value)),
            Ok(None) => Ok(SourceValue::Missing),
            Err(err) => {
                let err = err.to_string();
                self.set_last_error(err.clone());
                Err(err)
            }
        }
    }

    fn read_env_source(&self, key: MissionKey) -> Result<SourceValue, String> {
        let mut env = self.env.lock().map_err(lock_error)?;
        let raw = env.read(key.env_name())?;
        Ok(source_from_env(key, raw))
    }

    fn write_fram_value(&self, key: MissionKey, value: &MissionValue) -> Result<(), String> {
        let mut fram = self.fram.lock().map_err(lock_error)?;
        fram.write_key(key, value).map_err(|err| {
            let err = err.to_string();
            self.set_last_error(err.clone());
            err
        })
    }

    fn write_env_value(&self, key: MissionKey, value: &MissionValue) -> Result<(), String> {
        let mut env = self.env.lock().map_err(lock_error)?;
        let env_value = value.to_env_value();
        env.write(key.env_name(), env_value.as_deref())
    }

    fn set_last_error(&self, err: String) {
        if let Ok(mut last_error) = self.last_error.lock() {
            *last_error = Some(err);
        }
    }
}

fn collect_source_errors(
    fram: &SourceValue,
    env: &SourceValue,
    action_errors: &[String],
) -> String {
    let mut errors = Vec::new();
    if let Some(err) = fram.error() {
        errors.push(format!("fram: {err}"));
    }
    if let Some(err) = env.error() {
        errors.push(format!("env: {err}"));
    }
    errors.extend(action_errors.iter().cloned());
    errors.join("; ")
}

fn lock_error<E>(_: E) -> String {
    "subsystem lock poisoned".to_string()
}

#[cfg(feature = "i2c")]
fn parse_hex_u8(value: &str) -> Result<u8, String> {
    let value = value.trim().strip_prefix("0x").unwrap_or(value.trim());
    u8::from_str_radix(value, 16).map_err(|err| err.to_string())
}
