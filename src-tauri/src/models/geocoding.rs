use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GeocodingResponse {
    pub results: Vec<GeocodingResult>,
    // there is also generationtime, but that it is not needed
}

#[derive(Serialize, Deserialize)]
pub struct GeocodingResult {
    // there are more fields for this response, but we only really need the coordinates, name and country
    pub latitude: f64,
    pub longitude: f64,
    pub name: String,
    pub country: String,
}
