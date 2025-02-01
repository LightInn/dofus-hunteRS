use tauri::{AppHandle, Emitter, State};
use crate::composent::arrows::detect_arrow_direction;
use crate::composent::screenshot::capture_region;
use crate::models::{AppState, ScreenRegion};

use super::error::Result;

#[tauri::command]
pub fn call_arrow_direction(state: State<'_, AppState>, app: AppHandle) -> Result<()> {
    let mut state = state.inner.lock().unwrap();

    let region: ScreenRegion = state.config.regions.hunt_panel.into();

    let image = capture_region(region)?;

    let direction = detect_arrow_direction(&image, false);

    state.bot_data.current_arrow = direction;

    app.emit("state_changed", &*state).unwrap();


    println!("Direction: {:?}", direction);

    Ok(())
}