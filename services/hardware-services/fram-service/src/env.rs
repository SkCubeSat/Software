use std::collections::HashMap;
use std::process::Command;

pub trait EnvStore: Send {
    fn read(&mut self, name: &str) -> Result<Option<String>, String>;
    fn write(&mut self, name: &str, value: Option<&str>) -> Result<(), String>;
}

pub struct CommandEnvStore {
    printenv_path: String,
    setenv_path: String,
}

impl CommandEnvStore {
    pub fn new(printenv_path: String, setenv_path: String) -> Self {
        Self {
            printenv_path,
            setenv_path,
        }
    }
}

impl EnvStore for CommandEnvStore {
    fn read(&mut self, name: &str) -> Result<Option<String>, String> {
        let output = Command::new(&self.printenv_path)
            .args(["-n", name])
            .output()
            .map_err(|err| format!("failed to execute {}: {err}", self.printenv_path))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
            if is_env_backend_error(&stderr) {
                return Err(stderr);
            }
            return Ok(None);
        }

        Ok(Some(
            String::from_utf8_lossy(&output.stdout).trim().to_string(),
        ))
    }

    fn write(&mut self, name: &str, value: Option<&str>) -> Result<(), String> {
        let mut command = Command::new(&self.setenv_path);
        command.arg(name);
        if let Some(value) = value {
            command.arg(value);
        }

        let output = command
            .output()
            .map_err(|err| format!("failed to execute {}: {err}", self.setenv_path))?;

        if output.status.success() {
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
            Err(if stderr.is_empty() {
                format!("{} failed for {name}", self.setenv_path)
            } else {
                stderr
            })
        }
    }
}

fn is_env_backend_error(stderr: &str) -> bool {
    stderr.contains("Cannot open")
        || stderr.contains("No such file or directory")
        || stderr.contains("Permission denied")
        || stderr.contains("Bad CRC")
}

#[derive(Default)]
pub struct MemoryEnvStore {
    values: HashMap<String, String>,
}

impl MemoryEnvStore {
    pub fn new(values: impl IntoIterator<Item = (String, String)>) -> Self {
        Self {
            values: values.into_iter().collect(),
        }
    }

    pub fn get(&self, name: &str) -> Option<&str> {
        self.values.get(name).map(String::as_str)
    }
}

impl EnvStore for MemoryEnvStore {
    fn read(&mut self, name: &str) -> Result<Option<String>, String> {
        Ok(self.values.get(name).cloned())
    }

    fn write(&mut self, name: &str, value: Option<&str>) -> Result<(), String> {
        match value {
            Some(value) => {
                self.values.insert(name.to_string(), value.to_string());
            }
            None => {
                self.values.remove(name);
            }
        }

        Ok(())
    }
}
