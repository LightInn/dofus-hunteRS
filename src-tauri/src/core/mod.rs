pub mod config;
mod screenshot;
mod state;
// mod actions;

use std::sync::{Arc, Mutex};
use tauri::State;

pub use config::BotConfig;
pub use screenshot::{capture_region, CaptureError, ScreenRegion};
pub use state::AppState;
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
pub fn take_screenshot() -> Result<(), String> {
    println!("Taking screenshot");
    Ok(())
}

#[tauri::command]
pub fn capture_game_region(state: State<'_, AppState> ) -> Result<(), String> {

    let state = state.inner.lock().unwrap();

    let region: ScreenRegion = state.config.regions.hunt_panel.into();

    let image = capture_region(region).map_err(|e| e.to_string())?;

    image.save("image.png").unwrap();
    Ok(())
}


#[tauri::command]
pub async fn get_config(state: State<'_, AppState>) -> Result<BotConfig, String> {
    let state = state.inner.lock().unwrap();
    let config = state.config.clone();

    Ok(config)
}

#[tauri::command]
pub async fn update_config(new_config: BotConfig, state: State<'_, AppState>) -> Result<(), String> {
    let mut app_state  = state.inner.lock().unwrap();
    app_state.config = new_config;
    app_state.config.save().map_err(|e| e.to_string())?;
    Ok(())
}


#[tauri::command]
pub fn capture_analyse(state: State<'_, AppState> ) -> Result<(), String> {

    let state = state.inner.lock().unwrap();

    let region: ScreenRegion = state.config.regions.hunt_panel.into();

    let image = capture_region(region).map_err(|e| e.to_string())?;



    image.save("image.png").unwrap();
    Ok(())
}

// #[tauri::command]
// fn execute_action(action: Action, state: State<Arc<Mutex<AppState>>>) -> Result<(), String> {
//     let state = state.lock().unwrap();
//     let mut handler = ActionHandler::new();
//     handler.execute(action)?;
//     Ok(())
// }
