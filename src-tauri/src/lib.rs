mod composent;
mod core;
mod models;
mod ocr;

use crate::core::api::call_send_api_request;
use crate::core::arrows::{call_arrow_direction, call_set_direction};
use crate::core::call_manual;
use crate::core::config::{call_get_config, call_save_region, call_update_config};
use crate::core::ocr::{call_capture_analyse, call_current_coord, call_set_coord, call_set_hint};
use crate::models::AppState;
use core::call_python;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState::default())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            call_current_coord,
            call_get_config,
            call_update_config,
            call_capture_analyse,
            call_arrow_direction,
            call_send_api_request,
            call_save_region,
            call_python,
            call_set_direction,
            call_set_hint,
            call_set_coord,
            call_manual
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
