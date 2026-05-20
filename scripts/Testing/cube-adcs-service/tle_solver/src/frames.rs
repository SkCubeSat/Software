use hifitime::Epoch;
use nalgebra::{Rotation3, Vector3};
use std::f64::consts::PI;

/// Converts an ITRF position vector to a TEME position vector.
///
/// * `itrf_pos` - Earth-Fixed position vector (X, Y, Z) in km.
/// * `epoch` - The precise time of the GNSS observation.
pub fn itrf_to_teme_pos(itrf_pos: Vector3<f64>, epoch: Epoch) -> Vector3<f64> {
    let gmst_rad = calculate_gmst_rad(epoch);
    
    // To go from Earth-Fixed to Inertial, we rotate around the Z axis 
    // by the negative GMST angle.
    let rotation = Rotation3::from_axis_angle(&Vector3::z_axis(), -gmst_rad);
    
    rotation * itrf_pos
}

/// Converts an ITRF velocity vector to a TEME velocity vector.
///
/// Velocity requires an extra step! We must account for the Coriolis effect
/// caused by the Earth's rotation rate beneath the satellite.
/// 
/// * `itrf_pos` - Earth-Fixed position vector (km).
/// * `itrf_vel` - Earth-Fixed velocity vector (km/s).
/// * `epoch` - The precise time of the GNSS observation.
pub fn itrf_to_teme_vel(
    itrf_pos: Vector3<f64>,
    itrf_vel: Vector3<f64>,
    epoch: Epoch,
) -> Vector3<f64> {
    // Earth's rotation rate in radians per second (WGS84 constant)
    let earth_omega = Vector3::new(0.0, 0.0, 7.2921151467e-5);
    
    let gmst_rad = calculate_gmst_rad(epoch);
    let rotation = Rotation3::from_axis_angle(&Vector3::z_axis(), -gmst_rad);
    
    // v_inertial = rotation * (v_fixed + (omega x r_fixed))
    let coriolis_vel = itrf_vel + earth_omega.cross(&itrf_pos);
    
    rotation * coriolis_vel
}

/// Calculates the Greenwich Mean Sidereal Time (GMST) in radians 
/// using the IAU-82 analytical model.
fn calculate_gmst_rad(epoch: Epoch) -> f64 {
    // 1. Get Julian Date
    let jd = epoch.to_jde_utc_days();
    
    // 2. Days since J2000 epoch (2000-01-01 12:00:00 UTC)
    let d = jd - 2451545.0;
    
    // 3. Julian centuries since J2000
    let t = d / 36525.0;
    
    // 4. Calculate GMST in degrees
    let mut gmst_deg = 280.46061837 
        + 360.98564736629 * d 
        + 0.000387933 * (t * t) 
        - (t * t * t) / 38710000.0;
        
    // 5. Modulo 360 to keep it within a single circle (0 to 360)
    gmst_deg %= 360.0;
    if gmst_deg < 0.0 {
        gmst_deg += 360.0;
    }
    
    // 6. Convert to radians
    gmst_deg * (PI / 180.0)
}