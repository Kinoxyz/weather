// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod api;
pub mod models;

use crate::api::weather_api::fetch_basic_weather_data;
use crate::models::weather_response::WeatherData;
use crate::api::errors;

#[tauri::command]
async fn get_weather_data(_location: &str) -> Result<WeatherData, String> {
    let geocoding_result = match api::geocoding::get_coordinates(_location).await {
        Ok(result) => result,
        Err(e) => {
            return match e {
                errors::GeoCodingError::NetworkError => {
                    Err(format!("{}.Please try again.", e))
                },
                errors::GeoCodingError::InvalidLocationError => {
                    eprintln!("Error: {} is not a valid location", _location);
                    Err(format!("{} is not a valid location", e))
                },
                _ => {
                    eprintln!("Error: {}", e);
                    Err(format!("An error occurred"))
                }
            }
        }
    };
    
    let data = fetch_basic_weather_data(&geocoding_result).await;
    match data {
        Ok(response) => {
            Ok(WeatherData::new(response, geocoding_result))
        },
        Err(error) => {
            eprintln!("{error}");
            Err(format!("Invalid weather data"))
        }
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_weather_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
