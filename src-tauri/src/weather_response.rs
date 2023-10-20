use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ForecastWeatherResponse {
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

#[derive(Serialize, Deserialize)]
pub struct CurrentWeatherResponse {
    latitude: f64,
    longitude: f64,
    generationtime_ms: f64,
    utc_offset_seconds: i64,
    timezone: String,
    timezone_abbreviation: String,
    elevation: f64,
    current_units: CurrentUnits,
    current: Current
}

#[derive(Serialize, Deserialize)]
pub struct CurrentUnits {
    time: String,
    interval: String,
    #[serde(rename = "temperature_2m")]
    temperature_2_m: String,
    #[serde(rename = "windspeed_2m")]
    windspeed_2_m: String,
    weathercode: String
}

#[derive(Serialize, Deserialize)]
pub struct Current {
    time: String,
    interval: f64,
    #[serde(rename = "temperature_2m")]
    temperature_2_m: f64,
    #[serde(rename = "windspeed_2m")]
    windspeed_2_m: f64,
    weathercode: f64
}
