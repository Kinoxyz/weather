use ts_rs::TS;
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
pub struct WeatherApiResponse {
    latitude: f64,
    longitude: f64,
    generationtime_ms: f64,
    utc_offset_seconds: i64,
    timezone: String,
    timezone_abbreviation: String,
    elevation: f64,
    pub current_units: CurrentUnits,
    pub current: Current,
    pub daily_units: DailyUnits,
    pub daily: Daily
}

#[derive(Serialize, Deserialize, TS)]
#[ts(export, export_to = "../src/bindings/")]
pub struct Location {
    pub name: String,
    pub country: String
}

#[derive(Serialize, Deserialize, TS)]
#[ts(export, export_to = "../src/bindings/")]
pub struct WeatherData {
    pub location: Location,
    pub current_units: CurrentUnits,
    pub current: Current,
    pub daily_units: DailyUnits,
    pub daily: Daily
}

#[derive(Serialize, Deserialize, TS)]
#[ts(export, export_to = "../src/bindings/")]
pub struct CurrentUnits {
    time: String,
    interval: String,
    #[serde(rename = "temperature_2m")]
    temperature_2_m: String,
    #[serde(rename = "windspeed_10m")]
    windspeed_10_m: String,
    weathercode: String
}

#[derive(Serialize, Deserialize, TS)]
#[ts(export, export_to = "../src/bindings/")]
pub struct Current {
    time: String,
    interval: f64,
    #[serde(rename = "temperature_2m")]
    temperature_2_m: f64,
    #[serde(rename = "windspeed_10m")]
    windspeed_10_m: f64,
    weathercode: f64
}

#[derive(Serialize, Deserialize, TS)]
#[ts(export, export_to = "../src/bindings/")]
pub struct DailyUnits {
    time: String,
    #[serde(rename = "temperature_2m_max")]
    temperature_2_m_max: String,
    #[serde(rename = "temperature_2m_min")]
    temperature_2_m_min: String
}

#[derive(Serialize, Deserialize, TS)]
#[ts(export, export_to = "../src/bindings/")]
pub struct Daily {
    time: Vec<String>,
    #[serde(rename = "temperature_2m_max")]
    temperature_2_m_max: Vec<f64>,
    #[serde(rename = "temperature_2m_min")]
    temperature_2_m_min: Vec<f64>
}