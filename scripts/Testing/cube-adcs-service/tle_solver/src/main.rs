mod data;
mod frames;
mod solver;
mod exporter;

use nalgebra::DVector;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("--- BeagleBone TLE Solver ---");

    // 1. Load GNSS Data
    let csv_file = "merged_og_tle_1s_polling.csv";
    println!("Loading GNSS data from: {}", csv_file);
    let observations = data::load_gnss_csv(csv_file)?;
    
    if observations.is_empty() {
        return Err("CSV file is empty or unreadable.".into());
    }
    
    println!("Successfully loaded {} observations.", observations.len());

    // 2. Downsample data (Fit Step)
    // Running LM on 10,000 rows on a BeagleBone will take too long.
    // We take a sample every ~30 seconds (assuming 1Hz data).
    let fit_step = 30; 
    let mut fit_observations = Vec::new();
    for (i, obs) in observations.iter().enumerate() {
        if i % fit_step == 0 {
            fit_observations.push(obs);
        }
    }
    println!("Downsampled to {} points for optimization.", fit_observations.len());

    // 3. Convert ITRF to TEME
    let epoch_datetime = fit_observations[fit_observations.len() / 2].datetime;
    let epoch_time = fit_observations[fit_observations.len() / 2].epoch;
    
    let mut obs_times_mins = Vec::new();
    let mut obs_positions_teme = Vec::new();

    for obs in &fit_observations {
        let dt_seconds = (obs.epoch.to_unix_seconds() - epoch_time.to_unix_seconds()) as f64;
        obs_times_mins.push(dt_seconds / 60.0);
        let teme_pos = frames::itrf_to_teme_pos(obs.itrf_pos, obs.epoch);
        obs_positions_teme.push(teme_pos);
    }

    // 4. Initial Guess Vector
    let initial_guess = DVector::from_vec(vec![
        0.9, 1.0, 0.001, 0.0, 0.0, 0.065, 1e-5
    ]);

    // 5. Initial Guess Vector
    let fitter = solver::OrbitFitter {
        obs_times_mins,
        obs_positions_teme,
        epoch_datetime,
        params: initial_guess, // The LM solver now holds its own state parameters
    };

    println!("Starting Levenberg-Marquardt Optimization Loop...");
    
    // 6. Run the optimization
    let best_elements = solver::fit_tle_elements(fitter)?;

    println!("Solver converged!");

    // 7. Export TLE
    let (l1, l2) = exporter::generate_tle(&best_elements, 99999, "1800100");
    
    println!("\nGenerated TLE:");
    println!("{}", l1);
    println!("{}", l2);

    Ok(())
}