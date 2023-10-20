// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod api;
pub mod weather_response;

use crate::weather_response::ForecastWeatherResponse;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn get_weather_data() -> ForecastWeatherResponse {
    let data = api::fetch_basic_weather_data().await;
    match data {
        Ok(response) => return response,
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
