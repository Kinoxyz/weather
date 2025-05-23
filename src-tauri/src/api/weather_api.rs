use crate::models::{geocoding::GeocodingResult, weather_response::WeatherApiResponse};
use anyhow::Result;

fn construct_api_url(coordinates: &GeocodingResult) -> String {
    let GeocodingResult {
        longitude,
        latitude,
        name: _,
        country: _,
    } = coordinates;
    let forecast_days = 2;

    format!(
        "https://api.open-meteo.com/v1/forecast?\
    latitude={latitude}&\
    longitude={longitude}&\
    forecast_days={forecast_days}&\
    current=temperature_2m,\
    windspeed_10m,\
    weathercode&\
    daily=temperature_2m_max,\
    temperature_2m_min,\
    weathercode&\
    timezone=Europe%2FBerlin"
    )
}

pub async fn fetch_basic_weather_data(coordinates: &GeocodingResult) -> Result<WeatherApiResponse> {
    let api_url = construct_api_url(coordinates);
    let response = reqwest::get(api_url)
        .await?
        .json::<WeatherApiResponse>()
        .await?;
    Ok(response)
}
