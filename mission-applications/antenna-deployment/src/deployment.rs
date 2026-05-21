use failure::{Error, bail, format_err};
use log::{info, warn};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::gpio::{self, GPIO_7, GPIO_66, GPIO_115, GPIO_117};
use crate::state::{self, MissionFlagKey, MissionState};

const HOLD_TIME_SECONDS: i64 = 30 * 60;
const INTER_ANTENNA_DELAY_SECONDS: u64 = 90;
const DEPLOY_PULSE_MS: u64 = 5;
const MAX_ATTEMPT_SETS: u8 = 3;
const MIN_VALID_UNIX_TIME: i64 = 1_735_689_600; // 2025-01-01T00:00:00Z

pub fn run_once() -> Result<(), Error> {
    let now = current_unix_time()?;
    ensure_time_valid(now)?;

    state::reconcile_mission_state(false)?;
    let mut mission = state::read_mission_state()?;

    if mission.remove_before_flight {
        info!("remove_before_flight=true, aborting deployment run");
        return Ok(());
    }

    if mission.deployed {
        info!("deployed=true, nothing to do");
        return Ok(());
    }

    if mission.deploy_start.is_none() {
        info!("deploy_start missing, setting deploy_start={}", now);
        state::set_deploy_start(now)?;
        return Ok(());
    }

    let deploy_start = mission.deploy_start.unwrap_or(now);
    let elapsed = now - deploy_start;
    if elapsed < HOLD_TIME_SECONDS {
        info!(
            "hold timer active: elapsed={}s hold={}s",
            elapsed, HOLD_TIME_SECONDS
        );
        return Ok(());
    }

    gpio::init_antenna_gpio()?;
    refresh_confirmed_antennas(&mut mission);

    if mission.vhf_antenna_deployed && mission.uhf_antenna_deployed {
        finalize_deployment()?;
        return Ok(());
    }

    let attempts_done = decode_attempt_count(&mission);
    if attempts_done < MAX_ATTEMPT_SETS {
        mission = run_attempt_sets(mission, attempts_done)?;
    } else {
        info!(
            "attempt sets exhausted ({}), entering check-only mode",
            MAX_ATTEMPT_SETS
        );
    }

    run_check_only_cycle(&mut mission);

    if mission.vhf_antenna_deployed && mission.uhf_antenna_deployed {
        finalize_deployment()?;
    }

    Ok(())
}

fn current_unix_time() -> Result<i64, Error> {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| format_err!("system clock is before unix epoch: {}", e))?;

    i64::try_from(duration.as_secs())
        .map_err(|e| format_err!("unix timestamp overflowed i64: {}", e))
}

fn ensure_time_valid(now: i64) -> Result<(), Error> {
    if now < MIN_VALID_UNIX_TIME {
        bail!(
            "invalid system time {} (expected >= {}); ensure RTC/system time is set",
            now,
            MIN_VALID_UNIX_TIME
        );
    }

    Ok(())
}

fn run_attempt_sets(mut mission: MissionState, attempts_done: u8) -> Result<MissionState, Error> {
    let mut count = attempts_done;

    while count < MAX_ATTEMPT_SETS {
        if mission.vhf_antenna_deployed && mission.uhf_antenna_deployed {
            break;
        }

        count += 1;
        info!("starting attempt set {}/{}", count, MAX_ATTEMPT_SETS);

        if !mission.vhf_antenna_deployed {
            info!("attempting VHF deployment via GPIO_{}", GPIO_117);
            match gpio::pulse_high(GPIO_117, Duration::from_millis(DEPLOY_PULSE_MS)) {
                Ok(()) => match gpio::read_pin(GPIO_66) {
                    Ok(true) => {
                        mission.vhf_antenna_deployed = true;
                        state::set_flag(MissionFlagKey::VhfAntennaDeployed, true)?;
                        info!(
                            "VHF deployment confirmed via GPIO_{} (active-high)",
                            GPIO_66
                        );
                    }
                    Ok(false) => info!(
                        "VHF deployment not yet confirmed via GPIO_{} (active-high)",
                        GPIO_66
                    ),
                    Err(err) => warn!("VHF sense read failed this run: {}", err),
                },
                Err(err) => warn!("VHF deploy pulse failed this run: {}", err),
            }
        }

        info!(
            "waiting {}s before UHF attempt",
            INTER_ANTENNA_DELAY_SECONDS
        );
        thread::sleep(Duration::from_secs(INTER_ANTENNA_DELAY_SECONDS));

        if !mission.uhf_antenna_deployed {
            info!("attempting UHF deployment via GPIO_{}", GPIO_115);
            match gpio::pulse_high(GPIO_115, Duration::from_millis(DEPLOY_PULSE_MS)) {
                Ok(()) => match gpio::read_pin(GPIO_7) {
                    Ok(true) => {
                        mission.uhf_antenna_deployed = true;
                        state::set_flag(MissionFlagKey::UhfAntennaDeployed, true)?;
                        info!("UHF deployment confirmed via GPIO_{} (active-high)", GPIO_7);
                    }
                    Ok(false) => info!(
                        "UHF deployment not yet confirmed via GPIO_{} (active-high)",
                        GPIO_7
                    ),
                    Err(err) => warn!("UHF sense read failed this run: {}", err),
                },
                Err(err) => warn!("UHF deploy pulse failed this run: {}", err),
            }
        }

        persist_attempt_count(count)?;
    }

    Ok(mission)
}

fn run_check_only_cycle(mission: &mut MissionState) {
    if !mission.vhf_antenna_deployed {
        match gpio::read_pin(GPIO_66) {
            Ok(true) => {
                mission.vhf_antenna_deployed = true;
                if let Err(err) = state::set_flag(MissionFlagKey::VhfAntennaDeployed, true) {
                    warn!("failed to persist VHF confirmation: {}", err);
                } else {
                    info!("VHF became confirmed during check-only cycle");
                }
            }
            Ok(false) => {}
            Err(err) => warn!("VHF check-only read failed this run: {}", err),
        }
    }

    if !mission.uhf_antenna_deployed {
        match gpio::read_pin(GPIO_7) {
            Ok(true) => {
                mission.uhf_antenna_deployed = true;
                if let Err(err) = state::set_flag(MissionFlagKey::UhfAntennaDeployed, true) {
                    warn!("failed to persist UHF confirmation: {}", err);
                } else {
                    info!("UHF became confirmed during check-only cycle");
                }
            }
            Ok(false) => {}
            Err(err) => warn!("UHF check-only read failed this run: {}", err),
        }
    }
}

fn refresh_confirmed_antennas(mission: &mut MissionState) {
    if !mission.vhf_antenna_deployed {
        match gpio::read_pin(GPIO_66) {
            Ok(true) => {
                mission.vhf_antenna_deployed = true;
                if let Err(err) = state::set_flag(MissionFlagKey::VhfAntennaDeployed, true) {
                    warn!("failed to persist VHF confirmation during refresh: {}", err);
                }
            }
            Ok(false) => {}
            Err(err) => warn!("VHF refresh read failed this run: {}", err),
        }
    }

    if !mission.uhf_antenna_deployed {
        match gpio::read_pin(GPIO_7) {
            Ok(true) => {
                mission.uhf_antenna_deployed = true;
                if let Err(err) = state::set_flag(MissionFlagKey::UhfAntennaDeployed, true) {
                    warn!("failed to persist UHF confirmation during refresh: {}", err);
                }
            }
            Ok(false) => {}
            Err(err) => warn!("UHF refresh read failed this run: {}", err),
        }
    }
}

fn finalize_deployment() -> Result<(), Error> {
    state::set_flag(MissionFlagKey::Deployed, true)?;
    persist_attempt_count(MAX_ATTEMPT_SETS)?;
    info!("both antennas confirmed, deployed=true");
    Ok(())
}

fn decode_attempt_count(mission: &MissionState) -> u8 {
    let mut value = 0u8;
    if mission.initial_safe_state_complete {
        value |= 0b01;
    }
    if mission.detumbling_complete {
        value |= 0b10;
    }
    value.min(MAX_ATTEMPT_SETS)
}

fn persist_attempt_count(count: u8) -> Result<(), Error> {
    let value = count.min(MAX_ATTEMPT_SETS);
    let bit0 = (value & 0b01) != 0;
    let bit1 = (value & 0b10) != 0;

    state::set_flag(MissionFlagKey::InitialSafeStateComplete, bit0)?;
    state::set_flag(MissionFlagKey::DetumblingComplete, bit1)?;

    Ok(())
}
