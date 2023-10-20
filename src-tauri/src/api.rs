use crate::weather_response::ForecastWeatherResponse;

const API_URL: &str =
    "https://api.open-meteo.com/v1/forecast?latitude=52.52&longitude=13.41&current=temperature_2m,windspeed_10m,weathercode&timezone=Europe%2FBerlin";

pub async fn fetch_basic_weather_data() -> Result<ForecastWeatherResponse, Box<dyn std::error::Error>> {
    let response = reqwest::get(API_URL)
        .await?
        .json::<ForecastWeatherResponse>()
        .await?;
    Ok(response)
}
