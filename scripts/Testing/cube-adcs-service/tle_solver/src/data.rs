use chrono::{NaiveDateTime, TimeZone, Utc};
use hifitime::Epoch;
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

pub struct Observation {
    pub epoch: Epoch,
    pub datetime: NaiveDateTime,
    pub itrf_pos: Vector3<f64>,
    pub itrf_vel: Vector3<f64>,
}

/// Loads GNSS data from the CSV file and converts it into physical observations.
pub fn load_gnss_csv<P: AsRef<Path>>(path: P) -> Result<Vec<Observation>, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut reader = csv::Reader::from_reader(file);
    let mut observations = Vec::new();

    for result in reader.deserialize() {
        let row: GnssRow = result?;

        // Parse Date and Time (Assuming "YYYY-MM-DD HH:MM:SS" format)
        let datetime_str = format!("{} {}", row.date, row.time);
        let naive_dt = NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H:%M:%S")
            .or_else(|_| NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %I:%M:%S %p"))?;
            
        let utc_dt = Utc.from_utc_datetime(&naive_dt);
        
        // Convert to hifitime Epoch
        let epoch = Epoch::from_unix_seconds(utc_dt.timestamp() as f64);

        let itrf_pos = Vector3::new(row.x, row.y, row.z);
        let itrf_vel = Vector3::new(row.vx, row.vy, row.vz);

        observations.push(Observation {
            epoch,
            datetime: naive_dt,
            itrf_pos,
            itrf_vel,
        });
    }

    Ok(observations)
}