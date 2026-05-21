use failure::Error;
use log::{error, info};
use std::time::{SystemTime, UNIX_EPOCH};

mod state;

fn main() -> Result<(), Error> {
    logging_setup!("antenna-deployment")?;

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        print_usage();
        return Ok(());
    }

    match args[1].as_str() {
        "show-state" => {
            let mission = state::read_mission_state()?;
            info!("remove_before_flight={}", mission.remove_before_flight);
            info!("deployed={}", mission.deployed);
            info!(
                "deploy_start={}",
                mission
                    .deploy_start
                    .map(|x| x.to_string())
                    .unwrap_or_else(|| "null".to_string())
            );
            info!("solar_panel_deployed={}", mission.solar_panel_deployed);
            info!("uhf_antenna_deployed={}", mission.uhf_antenna_deployed);
            info!("vhf_antenna_deployed={}", mission.vhf_antenna_deployed);
            info!(
                "initial_safe_state_complete={}",
                mission.initial_safe_state_complete
            );
            info!("detumbling_complete={}", mission.detumbling_complete);
            Ok(())
        }
        "reconcile" => {
            let dry_run = matches!(args.get(2).map(String::as_str), Some("dry-run"));
            state::reconcile_mission_state(dry_run)?;
            info!("reconcile complete (dry_run={})", dry_run);
            Ok(())
        }
        "set-flag" => {
            if args.len() < 4 {
                print_usage();
                return Ok(());
            }

            let key = match state::parse_flag_key(&args[2]) {
                Some(key) => key,
                None => {
                    error!("unknown flag key: {}", args[2]);
                    print_usage();
                    return Ok(());
                }
            };

            let value = match state::parse_bool(&args[3]) {
                Some(value) => value,
                None => {
                    error!("invalid bool value: {}", args[3]);
                    print_usage();
                    return Ok(());
                }
            };

            state::set_flag(key, value)?;
            info!("set-flag {}={}", args[2], value);
            Ok(())
        }
        "set-deploy-start" => {
            if args.len() < 3 {
                print_usage();
                return Ok(());
            }

            match args[2].as_str() {
                "clear" => {
                    state::clear_deploy_start()?;
                    info!("deploy_start cleared");
                    Ok(())
                }
                "now" => {
                    let now = now_unix()?;
                    state::set_deploy_start(now)?;
                    info!("deploy_start set to now ({})", now);
                    Ok(())
                }
                raw => {
                    let timestamp = match raw.parse::<i64>() {
                        Ok(value) => value,
                        Err(_) => {
                            error!("invalid unix timestamp: {}", raw);
                            print_usage();
                            return Ok(());
                        }
                    };
                    state::set_deploy_start(timestamp)?;
                    info!("deploy_start set to {}", timestamp);
                    Ok(())
                }
            }
        }
        "help" | "--help" | "-h" => {
            print_usage();
            Ok(())
        }
        other => {
            error!("unknown command: {}", other);
            print_usage();
            Ok(())
        }
    }
}

fn now_unix() -> Result<i64, Error> {
    let duration = SystemTime::now().duration_since(UNIX_EPOCH)?;
    Ok(i64::try_from(duration.as_secs())?)
}

fn print_usage() {
    info!("FRAM manual interface commands:");
    info!("  show-state");
    info!("  reconcile [dry-run]");
    info!("  set-flag <key> <true|false>");
    info!("  set-deploy-start <now|clear|unix_timestamp>");
    info!("Flag keys:");
    info!("  remove_before_flight");
    info!("  deployed");
    info!("  solar_panel_deployed");
    info!("  uhf_antenna_deployed");
    info!("  vhf_antenna_deployed");
    info!("  initial_safe_state_complete");
    info!("  detumbling_complete");
    info!("Example:");
    info!("  antenna-deployment set-flag vhf_antenna_deployed true");
    info!("  antenna-deployment set-deploy-start now");
    info!("  antenna-deployment show-state");
}
