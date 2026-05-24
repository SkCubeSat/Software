use chrono::{DateTime, Duration, NaiveDateTime, TimeZone, Utc};
use nalgebra::Vector3;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct GnssRow {
    #[serde(rename = "Date")]
    pub date: String,
    #[serde(rename = "Time")]
    pub time: String,
    #[serde(rename = "Time since start (s)")]
    pub elapsed_seconds: Option<f64>,
    #[serde(rename = "X (km)")]
    pub x: f64,
    #[serde(rename = "Y (km)")]
    pub y: f64,
    #[serde(rename = "Z (km)")]
    pub z: f64,
    #[serde(rename = "X (km/s)")]
    pub vx: f64,
    #[serde(rename = "Y (km/s)")]
    pub vy: f64,
    #[serde(rename = "Z (km/s)")]
    pub vz: f64,
}

#[derive(Debug, Clone)]
pub struct Observation {
    pub datetime: DateTime<Utc>,
    pub itrf_pos: Vector3<f64>,
    pub itrf_vel: Vector3<f64>,
}

fn parse_datetime_utc(date: &str, time: &str) -> Result<DateTime<Utc>, Box<dyn Error>> {
    let stamp = format!("{} {}", date.trim(), time.trim());
    let naive = NaiveDateTime::parse_from_str(&stamp, "%Y-%m-%d %I:%M:%S %p")
        .or_else(|_| NaiveDateTime::parse_from_str(&stamp, "%Y-%m-%d %H:%M:%S"))?;
    Ok(Utc.from_utc_datetime(&naive))
}

fn add_seconds(base: DateTime<Utc>, seconds: f64) -> DateTime<Utc> {
    let whole = seconds.trunc() as i64;
    let nanos = ((seconds.fract()) * 1.0e9).round() as i64;
    base + Duration::seconds(whole) + Duration::nanoseconds(nanos)
}

pub fn load_gnss_csv<P: AsRef<Path>>(path: P) -> Result<Vec<Observation>, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut reader = csv::Reader::from_reader(file);
    let mut rows: Vec<GnssRow> = Vec::new();

    for result in reader.deserialize() {
        rows.push(result?);
    }

    if rows.is_empty() {
        return Err("CSV has no data rows".into());
    }

    let base_time = parse_datetime_utc(&rows[0].date, &rows[0].time)?;
    let mut observations = Vec::with_capacity(rows.len());

    for row in rows {
        let datetime = match row.elapsed_seconds {
            Some(seconds) => add_seconds(base_time, seconds),
            None => parse_datetime_utc(&row.date, &row.time)?,
        };

        observations.push(Observation {
            datetime,
            itrf_pos: Vector3::new(row.x, row.y, row.z),
            itrf_vel: Vector3::new(row.vx, row.vy, row.vz),
        });
    }

    Ok(observations)
}

pub fn median_cadence_seconds(observations: &[Observation]) -> Result<f64, Box<dyn Error>> {
    if observations.len() < 2 {
        return Err("At least two observations are required".into());
    }

    let mut diffs: Vec<f64> = observations
        .windows(2)
        .map(|w| {
            let dt = w[1].datetime - w[0].datetime;
            dt.num_microseconds().unwrap_or(0) as f64 / 1.0e6
        })
        .collect();

    diffs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    Ok(diffs[diffs.len() / 2])
}

pub fn build_fit_indices(count: usize, cadence_seconds: f64, fit_step_seconds: f64) -> Vec<usize> {
    let stride = (fit_step_seconds / cadence_seconds).round().max(1.0) as usize;
    let mut indices: Vec<usize> = (0..count).step_by(stride).collect();
    if *indices.last().unwrap() != count - 1 {
        indices.push(count - 1);
    }
    indices
}
