#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use rustypass::config;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
fn start_scanning() {
    // let _config_file = config::get_config();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![start_scanning])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
