use crate::composent::ocr::{ocr_coordinates, ocr_hunt_panel};
use crate::composent::screenshot::capture_region;
use crate::core::error::Result;
use crate::models::{AppState, Coord, ScreenRegion};
use tauri::{AppHandle, Emitter, State};

#[tauri::command]
pub fn call_current_coord(state: State<'_, AppState>, app: AppHandle) -> Result<()> {
    let mut state = state.inner.lock().unwrap();

    let region: ScreenRegion = state.config.regions.coordinates.into();

    let image = capture_region(region)?;

    let coordinates = ocr_coordinates(&image)?.unwrap_or_default();

    state.bot_data.coords.current = Coord {
        x: coordinates.0,
        y: coordinates.1,
    };

    app.emit("state_changed", &*state).unwrap();

    Ok(())
}

#[tauri::command]
pub fn call_capture_analyse(state: State<'_, AppState>, app: AppHandle) -> Result<()> {
    let mut state = state.inner.lock().unwrap();

    let region: ScreenRegion = state.config.regions.hunt_panel.into();

    let image = capture_region(region)?;

    let infos = ocr_hunt_panel(&image, state.config.api.clone())?;

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

#[tauri::command]
pub fn call_set_coord(x: i8, y: i8, state: State<'_, AppState>, app: AppHandle) -> Result<()> {
    let mut state = state.inner.lock().unwrap();

    state.bot_data.coords.current = Coord { x, y };

    println!("Coord: {:?}", state.bot_data.coords.current);

    app.emit("state_changed", &*state).unwrap();

    Ok(())
}

#[tauri::command]
pub fn call_set_hint(hint: String, state: State<'_, AppState>, app: AppHandle) -> Result<()> {
    let mut state = state.inner.lock().unwrap();

    state.bot_data.current_hint = hint;

    println!("Hint: {}", state.bot_data.current_hint);

    app.emit("state_changed", &*state).unwrap();

    Ok(())
}
