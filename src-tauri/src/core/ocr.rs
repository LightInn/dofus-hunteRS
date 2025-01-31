use tauri::{AppHandle, Emitter, State};
use crate::composent::ocr::{ocr_coordinates, ocr_hunt_panel};
use crate::composent::screenshot::capture_region;
use crate::core::ocr;
use crate::models::{AppState, Coord, ScreenRegion};

#[tauri::command]
pub fn call_capture_game_region(state: State<'_, AppState>) -> Result<(), String> {
    let state = state.inner.lock().unwrap();

    let region: ScreenRegion = state.config.regions.coordinates.into();

    let image = capture_region(region).map_err(|e| e.to_string())?;

    let infos = ocr_coordinates(&image).map_err(|e| e.to_string())?;

    Ok(())
}



#[tauri::command]
pub fn call_capture_analyse(state: State<'_, AppState>, app: AppHandle) -> Result<(), String> {
    let mut state = state.inner.lock().unwrap();

    let region: ScreenRegion = state.config.regions.hunt_panel.into();

    let image = capture_region(region).map_err(|e| e.to_string())?;

    let infos = ocr_hunt_panel(&image).map_err(|e| e.to_string())?;

    state.bot_data.coords.start = Coord {
        x: infos.start_x,
        y: infos.start_y,
    };
    state.bot_data.coords.target = Coord {
        x: infos.start_x,
        y: infos.start_y,
    };
    state.bot_data.current_hint = infos.current_hint;
    state.bot_data.steps.current = infos.step_current;
    state.bot_data.steps.total = infos.step_total;

    app.emit("state_changed", &*state).unwrap();

    Ok(())
}