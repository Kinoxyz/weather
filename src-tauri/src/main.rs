// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod api;
pub mod models;

use crate::api::weather_api::fetch_basic_weather_data;
use crate::models::weather_response::WeatherData;

#[tauri::command]
async fn get_weather_data(_location: &str) -> Result<WeatherData, String> {
    let Ok(geocoding_result) = api::geocoding::get_coordinates(_location).await else {
        return Err(_location.to_string() + " is not a valid location");
    };
    let data = fetch_basic_weather_data(&geocoding_result).await;
    match data {
        Ok(response) => {
            Ok(WeatherData::new(response, geocoding_result))
        },
        Err(error) => {
            eprintln!("{error}");
            Err("Invalid weather data".to_string())
        }
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_weather_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
