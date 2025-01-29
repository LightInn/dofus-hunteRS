mod core;

use core::{start_bot, stop_bot,take_screenshot,capture_game_region};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {

    let config = load_config("settings.json").expect("failed to load config");

    let config = Arc::new(Mutex::new(config));

    tauri::Builder::default()
        .manage(core::AppState::default())
        .manage(config)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, start_bot, stop_bot,take_screenshot,capture_game_region])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
