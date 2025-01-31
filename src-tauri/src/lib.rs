mod ocr;
mod core;

use core::{capture_game_region, get_config, start_bot, stop_bot, take_screenshot, update_config, capture_analyse,detect_arrow_direction,send_api_request};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(core::AppState::default())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            start_bot,
            stop_bot,
            take_screenshot,
            capture_game_region,
            get_config,
            update_config,
            capture_analyse,
            detect_arrow_direction,
            send_api_request
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
