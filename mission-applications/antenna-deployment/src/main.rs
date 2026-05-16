use failure::{bail, Error};
use getopts::Options;
use kubos_app::*;
use log::*;
use std::time::{SystemTime, UNIX_EPOCH};

mod deployment;
mod gpio;
mod state;

// Hold time before deployment begins
const HOLD_TIME_SECONDS: i64 = 30 * 60;

// Minimum valid unix timestamp (2020-01-01) — rejects unset RTC
const MIN_VALID_UNIX: i64 = 1_577_836_800;

fn main() -> Result<(), Error> {
    logging_setup!("antenna-deployment")?;

    let args: Vec<String> = ::std::env::args().collect();
    let mut opts = Options::new();
    opts.optflagopt("c", "config", "System config file", "CONFIG");
    opts.optflag("h", "help", "Print this help menu");

    let _matches = match opts.parse(&args[1..]) {
        Ok(r) => r,
        Err(f) => panic!("{}", f.to_string()),
    };

    run()
}

fn current_time() -> Result<i64, Error> {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs() as i64;

    if ts < MIN_VALID_UNIX {
        bail!("System time invalid (unix={}): RTC may not be set", ts);
    }

    Ok(ts)
}

fn run() -> Result<(), Error> {
    info!("Antenna deployment app started");

    let now = match current_time() {
        Ok(t) => t,
        Err(e) => {
            error!("RTC check failed: {} — exiting without action", e);
            return Ok(());
        }
    };

    info!("System time valid: unix={}", now);
    Ok(())
}