mod state;
mod screenshot;
// mod actions;

use tauri::State;
use std::sync::{Arc, Mutex};

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
pub fn take_screenshot(state: State<'_, AppState>) -> Result<(), String> {
    println!("Taking screenshot");
    Ok(())
}

// #[tauri::command]
// fn execute_action(action: Action, state: State<Arc<Mutex<AppState>>>) -> Result<(), String> {
//     let state = state.lock().unwrap();
//     let mut handler = ActionHandler::new();
//     handler.execute(action)?;
//     Ok(())
// }