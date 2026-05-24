mod data;
mod exporter;
mod frames;
mod solver;

use crate::frames::TemeObservation;
use std::env;
use std::error::Error;
use std::fs;

#[derive(Debug)]
struct Args {
    csv_path: String,
    output_path: Option<String>,
    fit_step_seconds: f64,
    satnum: u64,
    intldesg: String,
    max_iter: usize,
    name: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = parse_args()?;

    println!("--- Offline GNSS -> TLE Solver ---");
    println!("Input CSV: {}", args.csv_path);

    let observations = data::load_gnss_csv(&args.csv_path)?;
    let cadence = data::median_cadence_seconds(&observations)?;
    let fit_indices = data::build_fit_indices(observations.len(), cadence, args.fit_step_seconds);

    println!("Rows: {}", observations.len());
    println!("Median cadence: {:.6} s", cadence);
    println!("Fit step: {:.3} s", args.fit_step_seconds);
    println!("Fit rows: {}", fit_indices.len());

    let teme_observations: Vec<TemeObservation> = observations
        .iter()
        .map(|obs| {
            let (pos_km, vel_km_s) = frames::itrf_to_teme_state(obs.itrf_pos, obs.itrf_vel, obs.datetime);
            TemeObservation { datetime: obs.datetime, pos_km, vel_km_s }
        })
        .collect();

    let report = solver::fit_tle_elements(
        &teme_observations,
        &fit_indices,
        args.satnum,
        &args.intldesg,
        args.max_iter,
    )?;

    let (line1, line2) = exporter::generate_tle(&report.elements, args.satnum, &args.intldesg);

    println!("Iterations: {}", report.iterations);
    println!("Fit position RMSE: {:.6} km", report.fit_pos_rmse_km);
    println!("Fit position max:  {:.6} km", report.fit_pos_max_km);
    println!("Fit velocity RMSE: {:.9} km/s", report.fit_vel_rmse_km_s);
    println!("Fit velocity max:  {:.9} km/s", report.fit_vel_max_km_s);
    println!("All-row position RMSE: {:.6} km", report.all_pos_rmse_km);
    println!("All-row position max:  {:.6} km", report.all_pos_max_km);
    println!("All-row velocity RMSE: {:.9} km/s", report.all_vel_rmse_km_s);
    println!("All-row velocity max:  {:.9} km/s", report.all_vel_max_km_s);

    println!("\nGenerated TLE:");
    if let Some(name) = &args.name {
        println!("{}", name);
    }
    println!("{}", line1);
    println!("{}", line2);

    if let Some(path) = args.output_path {
        let mut text = String::new();
        if let Some(name) = &args.name {
            text.push_str(name);
            text.push('\n');
        }
        text.push_str(&line1);
        text.push('\n');
        text.push_str(&line2);
        text.push('\n');
        fs::write(&path, text)?;
        println!("Wrote {}", path);
    }

    Ok(())
}

fn parse_args() -> Result<Args, Box<dyn Error>> {
    let mut csv_path: Option<String> = None;
    let mut output_path: Option<String> = None;
    let mut fit_step_seconds = 30.0;
    let mut satnum = 99999u64;
    let mut intldesg = "26001A".to_string();
    let mut max_iter = 25usize;
    let mut name: Option<String> = None;

    let args: Vec<String> = env::args().skip(1).collect();
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--output" => { i += 1; output_path = Some(args.get(i).ok_or("Missing --output value")?.clone()); }
            "--fit-step-seconds" => { i += 1; fit_step_seconds = args.get(i).ok_or("Missing --fit-step-seconds value")?.parse()?; }
            "--satnum" => { i += 1; satnum = args.get(i).ok_or("Missing --satnum value")?.parse()?; }
            "--intldesg" => { i += 1; intldesg = args.get(i).ok_or("Missing --intldesg value")?.clone(); }
            "--max-iter" => { i += 1; max_iter = args.get(i).ok_or("Missing --max-iter value")?.parse()?; }
            "--name" => { i += 1; name = Some(args.get(i).ok_or("Missing --name value")?.clone()); }
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            value if value.starts_with('-') => return Err(format!("Unknown option: {value}").into()),
            value => csv_path = Some(value.to_string()),
        }
        i += 1;
    }

    let csv_path = csv_path.ok_or("Usage: tle_solver <csv_path> [--output out.tle]")?;
    Ok(Args { csv_path, output_path, fit_step_seconds, satnum, intldesg, max_iter, name })
}

fn print_help() {
    println!("Usage: tle_solver <csv_path> [options]");
    println!("Options:");
    println!("  --output <path>             Write generated TLE to a file");
    println!("  --fit-step-seconds <sec>    Downsample spacing for fit points, default 30");
    println!("  --satnum <number>           TLE satellite number, default 99999");
    println!("  --intldesg <designator>     International designator, default 26001A");
    println!("  --max-iter <n>              LM iterations, default 25");
    println!("  --name <line0>              Optional TLE line 0 name");
}
