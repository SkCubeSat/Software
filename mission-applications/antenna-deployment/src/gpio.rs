use failure::{Error, bail, format_err};
use std::fs;
use std::path::Path;
use std::thread;
use std::time::Duration;

pub const GPIO_117: u32 = 117;
pub const GPIO_66: u32 = 66;
pub const GPIO_115: u32 = 115;
pub const GPIO_7: u32 = 7;

const GPIO_ROOT: &str = "/sys/class/gpio";

#[derive(Copy, Clone)]
enum Direction {
    In,
    Out,
}

impl Direction {
    fn as_str(self) -> &'static str {
        match self {
            Self::In => "in",
            Self::Out => "out",
        }
    }
}

pub fn init_antenna_gpio() -> Result<(), Error> {
    configure_pin(GPIO_117, Direction::Out)?;
    configure_pin(GPIO_115, Direction::Out)?;
    configure_pin(GPIO_66, Direction::In)?;
    configure_pin(GPIO_7, Direction::In)?;
    Ok(())
}

pub fn pulse_high(pin: u32, pulse: Duration) -> Result<(), Error> {
    write_pin(pin, true)?;
    thread::sleep(pulse);
    write_pin(pin, false)?;
    Ok(())
}

pub fn read_pin(pin: u32) -> Result<bool, Error> {
    let raw = fs::read_to_string(value_path(pin))
        .map_err(|e| format_err!("failed to read GPIO_{} value: {}", pin, e))?;

    match raw.trim() {
        "1" => Ok(true),
        "0" => Ok(false),
        other => bail!("unexpected GPIO_{} value: '{}'", pin, other),
    }
}

fn configure_pin(pin: u32, direction: Direction) -> Result<(), Error> {
    ensure_exported(pin)?;

    fs::write(direction_path(pin), direction.as_str())
        .map_err(|e| format_err!("failed to set GPIO_{} direction: {}", pin, e))
}

fn write_pin(pin: u32, high: bool) -> Result<(), Error> {
    let value = if high { "1" } else { "0" };
    fs::write(value_path(pin), value)
        .map_err(|e| format_err!("failed to write GPIO_{} value {}: {}", pin, value, e))
}

fn ensure_exported(pin: u32) -> Result<(), Error> {
    let gpio_dir = format!("{}/gpio{}", GPIO_ROOT, pin);
    if Path::new(&gpio_dir).exists() {
        return Ok(());
    }

    fs::write(format!("{}/export", GPIO_ROOT), pin.to_string())
        .map_err(|e| format_err!("failed to export GPIO_{}: {}", pin, e))
}

fn direction_path(pin: u32) -> String {
    format!("{}/gpio{}/direction", GPIO_ROOT, pin)
}

fn value_path(pin: u32) -> String {
    format!("{}/gpio{}/value", GPIO_ROOT, pin)
}
