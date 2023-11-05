// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod api;
pub mod models;

use crate::api::weather_api::fetch_basic_weather_data;
use crate::models::weather_response::CurrentWeatherResponse;

#[tauri::command]
async fn get_wmo_code_description(code: i32) -> String {
    models::wmo_code::create_wmo_code_map()
    .get(&code)
    .unwrap_or(&String::from("Error retrieving WMO code description"))
    .to_owned()
}

#[tauri::command]
async fn get_weather_data(_location: &str) -> Result<CurrentWeatherResponse, ()> {
    let Ok(geocoding_result) = api::geocoding::get_coordinates(_location).await else {
        return Err(());
    };
    let data = fetch_basic_weather_data(geocoding_result).await;
    match data {
        Ok(response) => Ok(response),
        Err(error) => {
            eprintln!("{error}");
            Err(())
        }
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_weather_data, get_wmo_code_description])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
