use tauri::State;
use crate::composent::arrows::detect_arrow_direction;
use crate::composent::screenshot::capture_region;
use crate::models::{AppState, ScreenRegion};

#[tauri::command]
pub fn call_arrow_direction(state: State<'_, AppState>) -> Result<(), String> {
    let mut state = state.inner.lock().unwrap();

    let region: ScreenRegion = state.config.regions.hunt_panel.into();

    let image = capture_region(region).map_err(|e| e.to_string())?;

    let direction = detect_arrow_direction(&image, false);

    state.bot_data.current_arrow = direction;
    println!("Direction: {:?}", direction);

    Ok(())
}