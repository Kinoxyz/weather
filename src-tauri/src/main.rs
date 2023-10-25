// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod api;
pub mod models;

use crate::models::weather_response::CurrentWeatherResponse;
use crate::api::weather_api::fetch_basic_weather_data;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn get_weather_data(_location: &str) -> Result<CurrentWeatherResponse, String> {
    //TODO: fetch weather with location
    let data = fetch_basic_weather_data().await;
    match data {
        Ok(response) => Ok(response),
        Err(error) => {
            println!("{error}");
            todo!();
        }
    }
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, get_weather_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
