use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct WeatherResponse {
    latitude: f64,
    longitude: f64,
    generationtime_ms: f64,
    utc_offset_seconds: i64,
    timezone: String,
    timezone_abbreviation: String,
    elevation: f64,
    hourly_units: HourlyUnits,
    hourly: Hourly,
}

#[derive(Serialize, Deserialize)]
pub struct Hourly {
    time: Vec<String>,
    #[serde(rename = "temperature_2m")]
    temperature_2_m: Vec<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct HourlyUnits {
    time: String,
    #[serde(rename = "temperature_2m")]
    temperature_2_m: String,
}
