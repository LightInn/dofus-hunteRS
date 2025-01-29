use tauri::State;
use std::sync::{Arc, Mutex};

mod state;
mod actions;
mod state;
mod actions;

pub use state::AppState;
pub use actions::{Action, ActionHandler};

#[tauri::command]
fn start_bot(state: State<Arc<Mutex<AppState>>>) -> Result<(), String> {
    let mut state = state.lock().unwrap();
    state.running = true;
    println!("Bot started");
    Ok(())
}

#[tauri::command]
fn stop_bot(state: State<Arc<Mutex<AppState>>>) -> Result<(), String> {
    let mut state = state.lock().unwrap();
    state.running = false;
    println!("Bot stopped");
    Ok(())
}

#[tauri::command]
fn execute_action(action: Action, state: State<Arc<Mutex<AppState>>>) -> Result<(), String> {
    let state = state.lock().unwrap();
    let mut handler = ActionHandler::new();
    handler.execute(action)?;
    Ok(())
}