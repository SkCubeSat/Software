use chrono::{Datelike, NaiveDateTime, Timelike};
use sgp4::Elements;

fn checksum(line_without_checksum: &str) -> u8 {
    let mut sum = 0u32;
    for c in line_without_checksum.chars() {
        if c.is_ascii_digit() {
            sum += c.to_digit(10).unwrap();
        } else if c == '-' {
            sum += 1;
        }
    }
    (sum % 10) as u8
}

fn tle_exp(value: f64) -> String {
    if value == 0.0 || !value.is_finite() {
        return " 00000-0".to_string();
    }

    let sign = if value < 0.0 { '-' } else { ' ' };
    let abs_value = value.abs();
    let mut exponent = abs_value.log10().floor() as i32 + 1;
    let mut mantissa = (abs_value / 10f64.powi(exponent) * 100_000.0).round() as i32;

    if mantissa >= 100_000 {
        mantissa /= 10;
        exponent += 1;
    }

    format!("{}{:05}{:+1}", sign, mantissa, exponent)
}

fn epoch_field(dt: NaiveDateTime) -> String {
    let yy = dt.year().rem_euclid(100);
    let day_start = dt.date().and_hms_opt(0, 0, 0).unwrap();
    let seconds_today = (dt - day_start).num_nanoseconds().unwrap() as f64 / 1.0e9;
    let doy = dt.ordinal() as f64 + seconds_today / 86_400.0;
    format!("{:02}{:012.8}", yy, doy)
}

pub fn generate_tle(elements: &Elements, satnum: u64, int_designator: &str) -> (String, String) {
    let sat = satnum % 100_000;
    let epoch = epoch_field(elements.datetime);
    let bstar = tle_exp(elements.drag_term);
    let nddot = tle_exp(elements.mean_motion_ddot);
    let intldesg = if int_designator.trim().is_empty() {
        "        ".to_string()
    } else {
        format!("{:<8}", int_designator.trim())
    };

    let l1_base = format!(
        "1 {:05}U {} {}  .{:08} {} {} 0  {:>3}",
        sat,
        intldesg,
        epoch,
        0,
        nddot,
        bstar,
        elements.element_set_number % 1000,
    );
    let l1 = format!("{}{}", l1_base, checksum(&l1_base));

    let ecc_digits = (elements.eccentricity.clamp(0.0, 0.999_999_9) * 10_000_000.0).round() as u64;
    let l2_base = format!(
        "2 {:05} {:8.4} {:8.4} {:07} {:8.4} {:8.4} {:11.8}{:5}",
        sat,
        elements.inclination,
        elements.right_ascension,
        ecc_digits,
        elements.argument_of_perigee,
        elements.mean_anomaly,
        elements.mean_motion,
        elements.revolution_number % 100_000,
    );
    let l2 = format!("{}{}", l2_base, checksum(&l2_base));

    (l1, l2)
}
