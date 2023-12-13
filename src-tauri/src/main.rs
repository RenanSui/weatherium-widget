// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dotenvy::dotenv;
use std::env;

fn main() {
    dotenv().expect(".env file not found");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_env])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_env(name: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| "".to_string())
}
