use chrono::{DateTime, Utc};
use nalgebra::{Rotation3, Vector3};
use std::f64::consts::PI;

const EARTH_ROTATION_RAD_S: f64 = 7.292_115_146_7e-5;

#[derive(Debug, Clone)]
pub struct TemeObservation {
    pub datetime: DateTime<Utc>,
    pub pos_km: Vector3<f64>,
    pub vel_km_s: Vector3<f64>,
}

/// Simplified ITRF/ECEF -> TEME-like inertial conversion.
///
/// This intentionally does not use IERS polar motion, UT1-UTC, precession, or nutation.
/// It is suitable for an onboard/offline approximation, not precision orbit determination.
pub fn itrf_to_teme_state(
    itrf_pos_km: Vector3<f64>,
    itrf_vel_km_s: Vector3<f64>,
    datetime: DateTime<Utc>,
) -> (Vector3<f64>, Vector3<f64>) {
    let theta = gmst_rad(datetime);
    let rotation = Rotation3::from_axis_angle(&Vector3::z_axis(), theta);
    let omega = Vector3::new(0.0, 0.0, EARTH_ROTATION_RAD_S);

    let pos_teme = rotation * itrf_pos_km;
    let vel_teme = rotation * (itrf_vel_km_s + omega.cross(&itrf_pos_km));

    (pos_teme, vel_teme)
}

pub fn gmst_rad(datetime: DateTime<Utc>) -> f64 {
    let unix_seconds = datetime.timestamp() as f64
        + datetime.timestamp_subsec_nanos() as f64 * 1.0e-9;
    let jd = unix_seconds / 86_400.0 + 2_440_587.5;
    let d = jd - 2_451_545.0;
    let t = d / 36_525.0;

    let mut gmst_deg = 280.460_618_37
        + 360.985_647_366_29 * d
        + 0.000_387_933 * t * t
        - (t * t * t) / 38_710_000.0;

    gmst_deg %= 360.0;
    if gmst_deg < 0.0 {
        gmst_deg += 360.0;
    }

    gmst_deg * PI / 180.0
}
