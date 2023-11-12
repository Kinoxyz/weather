use crate::models::{geocoding::GeocodingResult, weather_response::CurrentWeatherResponse};
use anyhow::Result;

fn construct_api_url(coordinates: GeocodingResult) -> String {
    let GeocodingResult {
        longitude,
        latitude,
    } = coordinates;
    let forecast_days = 1;

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

pub async fn fetch_basic_weather_data(
    coordinates: GeocodingResult,
) -> Result<CurrentWeatherResponse> {
    let api_url = construct_api_url(coordinates);
    let response = reqwest::get(api_url)
        .await?
        .json::<CurrentWeatherResponse>()
        .await?;
    Ok(response)
}
