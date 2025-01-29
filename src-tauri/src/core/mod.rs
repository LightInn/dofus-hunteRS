mod config;
mod screenshot;
mod state;
// mod actions;

use std::sync::{Arc, Mutex};
use tauri::State;

pub use screenshot::{capture_region, CaptureError, ScreenRegion};
pub use state::AppState;
pub use config::AppConfig;
// pub use actions::{Action, ActionHandler};

#[tauri::command]
pub fn start_bot(state: State<'_, AppState>) -> Result<(), String> {
    let mut state = state.inner.lock().unwrap();
    state.running = true;
    println!("Bot started");
    Ok(())
}

#[tauri::command]
pub fn stop_bot(state: State<'_, AppState>) -> Result<(), String> {
    let mut state = state.inner.lock().unwrap();
    state.running = false;
    println!("Bot stopped");
    Ok(())
}

#[tauri::command]
pub fn take_screenshot(state: State<'_, AppState>) -> Result<(), String> {
    println!("Taking screenshot");
    Ok(())
}

#[tauri::command]
pub fn capture_game_region() -> Result<(), String> {
    let region: ScreenRegion = (0, 0, 1920, 1080);

    let image = capture_region(region).map_err(|e| e.to_string())?;

    image.save("image.png").unwrap();
    Ok(())
}

#[tauri::command]
async fn get_config(state: State<'_, Arc<Mutex<AppConfig>>>) -> Result<AppConfig, String> {
    let config = state.lock().unwrap().clone();
    Ok(config)
}

#[tauri::command]
async fn update_config(
    new_config: AppConfig,
    state: State<'_, Arc<Mutex<AppConfig>>>,
) -> Result<(), String> {
    let mut config = state.lock().unwrap();
    *config = new_config;
    config.save().map_err(|e| e.to_string())?;
    Ok(())
}

// #[tauri::command]
// fn execute_action(action: Action, state: State<Arc<Mutex<AppState>>>) -> Result<(), String> {
//     let state = state.lock().unwrap();
//     let mut handler = ActionHandler::new();
//     handler.execute(action)?;
//     Ok(())
// }
