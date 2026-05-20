use sgp4::Elements;
use std::f64::consts::PI;

/// Computes the standard TLE modulo-10 checksum for a string line.
fn compute_checksum(line: &str) -> u8 {
    let mut sum = 0;
    for c in line.chars() {
        if c.is_ascii_digit() {
            sum += c.to_digit(10).unwrap() as u32;
        } else if c == '-' {
            sum += 1;
        }
    }
    (sum % 10) as u8
}

/// Converts an exponential value (like B*) into the strict TLE format (e.g., +12345-4).
fn format_bstar(bstar: f64) -> String {
    if bstar == 0.0 {
        return " 00000-0".to_string();
    }
    let sign = if bstar < 0.0 { "-" } else { " " };
    let exp = bstar.abs().log10().floor() as i32 + 1;
    let mantissa = bstar.abs() / 10f64.powi(exp);
    let mantissa_int = (mantissa * 100_000.0).round() as i32;
    
    // TLEs format exponent 10^-4 as -4, etc.
    format!("{}{:05}{:+1}", sign, mantissa_int, exp - 1)
}

/// Converts SGP4 Elements into a Two-Line Element (TLE) string.
pub fn generate_tle(elements: &Elements, sat_num: u32, int_designator: &str) -> (String, String) {
    // Convert Radians to Degrees
    let inc_deg = elements.inclination * (180.0 / PI);
    let raan_deg = elements.right_ascension * (180.0 / PI);
    let argp_deg = elements.argument_of_perigee * (180.0 / PI);
    let ma_deg = elements.mean_anomaly * (180.0 / PI);
    
    // Mean motion is usually revs/day in TLEs (elements.mean_motion is rad/min in SGP4 crate)
    let mean_motion_rev_day = elements.mean_motion * (1440.0 / (2.0 * PI));
    
    // Format Epoch year and days
    // This is a simplified epoch string formatter.
    let epoch_str = format!("{:02}{:012.8}", 24, 100.5); // Placeholder: Calculate exact year and day of year from JD
    
    let bstar_str = format_bstar(elements.drag_term);

    // Build Line 1 (without checksum)
    let l1_base = format!(
        "1 {:05}U {:<8} {}  .00000000  00000-0 {} 0  999",
        sat_num, int_designator, epoch_str, bstar_str
    );
    let l1 = format!("{}{}", l1_base, compute_checksum(&l1_base));

    // Build Line 2 (without checksum)
    let l2_base = format!(
        "2 {:05} {:8.4} {:8.4} {:07.0} {:8.4} {:8.4} {:11.8}{:05}",
        sat_num,
        inc_deg,
        raan_deg,
        elements.eccentricity * 1e7,
        argp_deg,
        ma_deg,
        mean_motion_rev_day,
        1 // Revolution number
    );
    let l2 = format!("{}{}", l2_base, compute_checksum(&l2_base));

    (l1, l2)
}