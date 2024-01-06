use anyhow::Result;

use crate::models::geocoding::{GeocodingResponse, GeocodingResult};
use crate::api::errors;

fn construct_api_url(location: &str) -> String {
    format!(
        "https://geocoding-api.open-meteo.com/v1/search?\
    name={location}&\
    count=1&\
    language=en&\
    format=json"
    )
}

pub async fn get_coordinates(location: &str) -> Result<GeocodingResult, errors::GeoCodingError> {
    let api_url = construct_api_url(location.trim());

    let response = match reqwest::get(api_url).await {
        Ok(resp) => resp,
        Err(_e) => {
            return Err(errors::GeoCodingError::NetworkError);
        }
    };

    let mut geocoding_response = match response.json::<GeocodingResponse>().await {
        Ok(resp) => resp,
        Err(_e) => {
            return Err(errors::GeoCodingError::InvalidLocationError);
        }
    };

    match geocoding_response.results.pop() {
        Some(result) => Ok(result),
        None => {
            Err(errors::GeoCodingError::MissingCoordinatesError)
        }
    }
}

