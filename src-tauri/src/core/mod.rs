mod api;
mod arrows;
pub mod config;
mod ocr;
mod screenshot;
mod state;
mod window;

use tauri::State;

use crate::core::state::Coord;
pub use state::ArrowDirection;
pub use config::BotConfig;
pub use screenshot::{capture_region, CaptureError, ScreenRegion};
pub use state::AppState;

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
pub fn capture_game_region(state: State<'_, AppState>) -> Result<(), String> {
    let mut state = state.inner.lock().unwrap();

    let region: ScreenRegion = state.config.regions.coordinates.into();

    let image = capture_region(region).map_err(|e| e.to_string())?;

    let infos = ocr::ocr_coordinates(&image).map_err(|e| e.to_string())?;


    Ok(())
}

#[tauri::command]
pub async fn get_config(state: State<'_, AppState>) -> Result<BotConfig, String> {
    let state = state.inner.lock().unwrap();
    let config = state.config.clone();

    Ok(config)
}

#[tauri::command]
pub async fn update_config(
    new_config: BotConfig,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut app_state = state.inner.lock().unwrap();
    app_state.config = new_config;
    app_state.config.save().map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn capture_analyse(state: State<'_, AppState>) -> Result<(), String> {
    let mut state = state.inner.lock().unwrap();

    let region: ScreenRegion = state.config.regions.hunt_panel.into();

    let image = capture_region(region).map_err(|e| e.to_string())?;

    let infos = ocr::ocr_hunt_panel(&image).map_err(|e| e.to_string())?;

    state.bot_data.start_coord = Coord {
        x: infos.start_x,
        y: infos.start_y,
    };
    state.bot_data.current_hint = infos.current_hint;
    state.bot_data.steps.current = infos.step_current;
    state.bot_data.steps.total = infos.step_total;

    Ok(())
}

#[tauri::command]
pub fn detect_arrow_direction(state: State<'_, AppState>) -> Result<(), String> {
    let mut state = state.inner.lock().unwrap();

    let region: ScreenRegion = state.config.regions.hunt_panel.into();

    let image = capture_region(region).map_err(|e| e.to_string())?;

    let direction = arrows::detect_arrow_direction(&image, false);

    state.bot_data.current_arrow = direction;
    println!("Direction: {:?}", direction);

    Ok(())
}

#[tauri::command]
pub fn send_api_request(state: State<'_, AppState>) -> Result<(), String> {
    let state = state.inner.lock().unwrap();
    let config = state.config.api.clone();

    let x = state.bot_data.start_coord.x as i32;
    let y = state.bot_data.start_coord.y as i32;
    let direction = match state.bot_data.current_arrow {
        ArrowDirection::Up => "up",
        ArrowDirection::Down => "down",
        ArrowDirection::Left => "left",
        ArrowDirection::Right => "right",
        ArrowDirection::Unknown => "unknown",
    };

    let hint = state.bot_data.current_hint.clone();

    println!("Sending request with x: {}, y: {}, direction: {}, hint: {}", x, y, direction, hint);

    let response =
        api::find_next_location(config, x, y, direction, &hint).map_err(|e| e.to_string())?;

    println!("{:?}", response);

    Ok(())
}

// #[tauri::command]
// fn execute_action(action: Action, state: State<Arc<Mutex<AppState>>>) -> Result<(), String> {
//     let state = state.lock().unwrap();
//     let mut handler = ActionHandler::new();
//     handler.execute(action)?;
//     Ok(())
// }
