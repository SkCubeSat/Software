use crate::frames::TemeObservation;
use chrono::NaiveDateTime;
use nalgebra::{DMatrix, DVector, Vector3};
use sgp4::{Classification, Constants, Elements, MinutesSinceEpoch};
use std::f64::consts::PI;

const MU_EARTH_KM3_S2: f64 = 398_600.441_8;
const NUM_PARAMS: usize = 7;
const VEL_WEIGHT_SECONDS: f64 = 60.0;

#[derive(Debug, Clone)]
pub struct FitReport {
    pub elements: Elements,
    pub iterations: usize,
    pub fit_pos_rmse_km: f64,
    pub fit_pos_max_km: f64,
    pub fit_vel_rmse_km_s: f64,
    pub fit_vel_max_km_s: f64,
    pub all_pos_rmse_km: f64,
    pub all_pos_max_km: f64,
    pub all_vel_rmse_km_s: f64,
    pub all_vel_max_km_s: f64,
}

pub fn fit_tle_elements(
    all_observations: &[TemeObservation],
    fit_indices: &[usize],
    satnum: u64,
    intldesg: &str,
    max_iter: usize,
) -> Result<FitReport, String> {
    if fit_indices.len() < 3 {
        return Err("Need at least 3 fit samples".to_string());
    }

    let fit_obs: Vec<TemeObservation> = fit_indices
        .iter()
        .map(|&i| all_observations[i].clone())
        .collect();

    let epoch_fit_index = fit_obs.len() / 2;
    let epoch_dt = fit_obs[epoch_fit_index].datetime.naive_utc();
    let epoch_state = &fit_obs[epoch_fit_index];

    let mut params = initial_guess_from_state(
        epoch_state.pos_km,
        epoch_state.vel_km_s,
        epoch_dt,
        satnum,
        intldesg,
    )?;

    let times_mins: Vec<f64> = fit_obs
        .iter()
        .map(|obs| minutes_between(obs.datetime.naive_utc(), epoch_dt))
        .collect();

    let mut lambda = 1.0e-2;
    let tolerance = 1.0e-7;
    let mut iterations = 0usize;
    let mut residual = residual_vector(&params, &fit_obs, &times_mins)?;
    let mut best_score = residual.norm_squared();

    for iter in 0..max_iter {
        iterations = iter + 1;
        let jac = numerical_jacobian(&params, &fit_obs, &times_mins, &residual)?;
        let jt = jac.transpose();
        let mut lhs = &jt * &jac;
        let rhs = -(&jt * &residual);

        for i in 0..NUM_PARAMS {
            lhs[(i, i)] += lambda * lhs[(i, i)].abs().max(1.0);
        }

        let Some(delta) = lhs.lu().solve(&rhs) else {
            lambda *= 10.0;
            continue;
        };

        if delta.norm() < tolerance {
            break;
        }

        let mut candidate = params.clone() + delta;
        normalize_params(&mut candidate);

        if let Ok(candidate_residual) = residual_vector(&candidate, &fit_obs, &times_mins) {
            let candidate_score = candidate_residual.norm_squared();
            if candidate_score < best_score {
                params = candidate;
                residual = candidate_residual;
                best_score = candidate_score;
                lambda = (lambda * 0.3).max(1.0e-12);
            } else {
                lambda = (lambda * 10.0).min(1.0e12);
            }
        } else {
            lambda = (lambda * 10.0).min(1.0e12);
        }
    }

    let elements = params_to_elements(&params, epoch_dt, satnum, intldesg)?;
    let (fit_pos_rmse_km, fit_pos_max_km, fit_vel_rmse_km_s, fit_vel_max_km_s) =
        residual_stats(&elements, &fit_obs)?;
    let (all_pos_rmse_km, all_pos_max_km, all_vel_rmse_km_s, all_vel_max_km_s) =
        residual_stats(&elements, all_observations)?;

    Ok(FitReport {
        elements,
        iterations,
        fit_pos_rmse_km,
        fit_pos_max_km,
        fit_vel_rmse_km_s,
        fit_vel_max_km_s,
        all_pos_rmse_km,
        all_pos_max_km,
        all_vel_rmse_km_s,
        all_vel_max_km_s,
    })
}

fn residual_vector(
    params: &DVector<f64>,
    observations: &[TemeObservation],
    times_mins: &[f64],
) -> Result<DVector<f64>, String> {
    let epoch_dt = observations[observations.len() / 2].datetime.naive_utc();
    let elements = params_to_elements(params, epoch_dt, 99999, "26001A")?;
    let constants = Constants::from_elements_afspc_compatibility_mode(&elements)
        .map_err(|err| format!("Invalid SGP4 constants: {err:?}"))?;

    let mut residuals = DVector::zeros(observations.len() * 6);
    for (i, obs) in observations.iter().enumerate() {
        let prediction = constants
            .propagate_afspc_compatibility_mode(MinutesSinceEpoch(times_mins[i]))
            .map_err(|err| format!("SGP4 propagation failed: {err:?}"))?;

        let pred_pos = Vector3::new(
            prediction.position[0],
            prediction.position[1],
            prediction.position[2],
        );
        let pred_vel = Vector3::new(
            prediction.velocity[0],
            prediction.velocity[1],
            prediction.velocity[2],
        );

        let pos_diff = pred_pos - obs.pos_km;
        let vel_diff = pred_vel - obs.vel_km_s;
        let row = i * 6;
        residuals[row] = pos_diff.x;
        residuals[row + 1] = pos_diff.y;
        residuals[row + 2] = pos_diff.z;
        residuals[row + 3] = vel_diff.x * VEL_WEIGHT_SECONDS;
        residuals[row + 4] = vel_diff.y * VEL_WEIGHT_SECONDS;
        residuals[row + 5] = vel_diff.z * VEL_WEIGHT_SECONDS;
    }

    Ok(residuals)
}

fn numerical_jacobian(
    params: &DVector<f64>,
    observations: &[TemeObservation],
    times_mins: &[f64],
    base_residual: &DVector<f64>,
) -> Result<DMatrix<f64>, String> {
    let mut jac = DMatrix::zeros(base_residual.len(), NUM_PARAMS);
    let steps = [1.0e-4, 1.0e-4, 1.0e-7, 1.0e-4, 1.0e-4, 1.0e-6, 1.0e-7];

    for col in 0..NUM_PARAMS {
        let mut plus = params.clone();
        plus[col] += steps[col];
        normalize_params(&mut plus);

        let mut minus = params.clone();
        minus[col] -= steps[col];
        normalize_params(&mut minus);

        let plus_res = residual_vector(&plus, observations, times_mins);
        let minus_res = residual_vector(&minus, observations, times_mins);

        match (plus_res, minus_res) {
            (Ok(rp), Ok(rm)) => {
                for row in 0..base_residual.len() {
                    jac[(row, col)] = (rp[row] - rm[row]) / (2.0 * steps[col]);
                }
            }
            (Ok(rp), Err(_)) => {
                for row in 0..base_residual.len() {
                    jac[(row, col)] = (rp[row] - base_residual[row]) / steps[col];
                }
            }
            _ => return Err(format!("Could not perturb parameter {col}")),
        }
    }

    Ok(jac)
}

fn params_to_elements(
    p: &DVector<f64>,
    epoch_dt: NaiveDateTime,
    satnum: u64,
    intldesg: &str,
) -> Result<Elements, String> {
    let inclination = p[0];
    let eccentricity = p[2];
    let mean_motion = p[5];

    if !(0.0..180.0).contains(&inclination) {
        return Err(format!("Inclination out of range: {inclination}"));
    }
    if !(0.0..0.25).contains(&eccentricity) {
        return Err(format!("Eccentricity out of range: {eccentricity}"));
    }
    if !(1.0..25.0).contains(&mean_motion) {
        return Err(format!("Mean motion out of LEO range: {mean_motion}"));
    }

    Ok(Elements {
        object_name: None,
        international_designator: Some(intldesg.to_string()),
        norad_id: satnum,
        classification: Classification::Unclassified,
        datetime: epoch_dt,
        mean_motion_dot: 0.0,
        mean_motion_ddot: 0.0,
        drag_term: p[6],
        element_set_number: 999,
        inclination,
        right_ascension: wrap_deg(p[1]),
        eccentricity,
        argument_of_perigee: wrap_deg(p[3]),
        mean_anomaly: wrap_deg(p[4]),
        mean_motion,
        revolution_number: 1,
        ephemeris_type: 0,
    })
}

fn initial_guess_from_state(
    r: Vector3<f64>,
    v: Vector3<f64>,
    epoch_dt: NaiveDateTime,
    satnum: u64,
    intldesg: &str,
) -> Result<DVector<f64>, String> {
    let h = r.cross(&v);
    let h_norm = h.norm();
    let r_norm = r.norm();
    let v_norm = v.norm();
    let k = Vector3::new(0.0, 0.0, 1.0);
    let n = k.cross(&h);
    let n_norm = n.norm();
    let e_vec = v.cross(&h) / MU_EARTH_KM3_S2 - r / r_norm;
    let e = e_vec.norm().clamp(1.0e-7, 0.1);
    let energy = 0.5 * v_norm * v_norm - MU_EARTH_KM3_S2 / r_norm;
    let a = -MU_EARTH_KM3_S2 / (2.0 * energy);

    if !a.is_finite() || a <= 0.0 {
        return Err("Initial state does not look like an elliptical Earth orbit".to_string());
    }

    let inc = (h.z / h_norm).acos();
    let raan = if n_norm > 1.0e-12 { n.y.atan2(n.x) } else { 0.0 };

    let argp = if n_norm > 1.0e-12 && e > 1.0e-12 {
        (n.cross(&e_vec).dot(&h) / h_norm).atan2(n.dot(&e_vec))
    } else {
        0.0
    };

    let true_anomaly = if e > 1.0e-12 {
        (e_vec.cross(&r).dot(&h) / h_norm).atan2(e_vec.dot(&r))
    } else if n_norm > 1.0e-12 {
        (n.cross(&r).dot(&h) / h_norm).atan2(n.dot(&r))
    } else {
        0.0
    };

    let eccentric_anomaly = 2.0
        * ((1.0 - e).sqrt() * (true_anomaly / 2.0).sin())
            .atan2((1.0 + e).sqrt() * (true_anomaly / 2.0).cos());
    let mean_anomaly = eccentric_anomaly - e * eccentric_anomaly.sin();
    let mean_motion_rev_day = (MU_EARTH_KM3_S2 / (a * a * a)).sqrt() * 86_400.0 / (2.0 * PI);

    let mut params = DVector::from_vec(vec![
        inc.to_degrees(),
        raan.to_degrees(),
        e,
        argp.to_degrees(),
        mean_anomaly.to_degrees(),
        mean_motion_rev_day,
        1.0e-6,
    ]);
    normalize_params(&mut params);

    // Verify that the initial guess is accepted by sgp4 before running LM.
    let _ = Constants::from_elements_afspc_compatibility_mode(&params_to_elements(
        &params, epoch_dt, satnum, intldesg,
    )?)
    .map_err(|err| format!("Initial SGP4 constants failed: {err:?}"))?;

    Ok(params)
}

fn residual_stats(elements: &Elements, observations: &[TemeObservation]) -> Result<(f64, f64, f64, f64), String> {
    let constants = Constants::from_elements_afspc_compatibility_mode(elements)
        .map_err(|err| format!("Invalid SGP4 constants: {err:?}"))?;

    let epoch = elements.datetime;
    let mut pos_sq = 0.0;
    let mut vel_sq = 0.0;
    let mut pos_max = 0.0;
    let mut vel_max = 0.0;

    for obs in observations {
        let t = minutes_between(obs.datetime.naive_utc(), epoch);
        let prediction = constants
            .propagate_afspc_compatibility_mode(MinutesSinceEpoch(t))
            .map_err(|err| format!("SGP4 propagation failed: {err:?}"))?;
        let pred_pos = Vector3::new(prediction.position[0], prediction.position[1], prediction.position[2]);
        let pred_vel = Vector3::new(prediction.velocity[0], prediction.velocity[1], prediction.velocity[2]);
        let pos_err = (pred_pos - obs.pos_km).norm();
        let vel_err = (pred_vel - obs.vel_km_s).norm();
        pos_sq += pos_err * pos_err;
        vel_sq += vel_err * vel_err;
        if pos_err > pos_max { pos_max = pos_err; }
        if vel_err > vel_max { vel_max = vel_err; }
    }

    let n = observations.len() as f64;
    Ok(((pos_sq / n).sqrt(), pos_max, (vel_sq / n).sqrt(), vel_max))
}

fn minutes_between(t: NaiveDateTime, epoch: NaiveDateTime) -> f64 {
    let dt = t - epoch;
    dt.num_microseconds().unwrap_or(0) as f64 / 60.0e6
}

fn normalize_params(p: &mut DVector<f64>) {
    p[1] = wrap_deg(p[1]);
    p[3] = wrap_deg(p[3]);
    p[4] = wrap_deg(p[4]);
    if p[2] < 1.0e-8 { p[2] = 1.0e-8; }
}

fn wrap_deg(mut x: f64) -> f64 {
    x %= 360.0;
    if x < 0.0 { x += 360.0; }
    x
}
