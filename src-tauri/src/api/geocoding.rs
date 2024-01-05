use anyhow::anyhow;
use anyhow::Result;

use crate::models::geocoding::{GeocodingResponse, GeocodingResult};

fn construct_api_url(location: &str) -> String {
    format!(
        "https://geocoding-api.open-meteo.com/v1/search?\
    name={location}&\
    count=1&\
    language=en&\
    format=json"
    )
}

pub async fn get_coordinates(location: &str) -> Result<GeocodingResult> {
    let api_url = construct_api_url(location.trim());

    let response = reqwest::get(api_url)
        .await?
        .json::<GeocodingResponse>()
        .await?
        .results
        .pop();

    match response {
        Some(result) => Ok(result),
        None => Err(anyhow!("Coordinates missing")),
    }
}
