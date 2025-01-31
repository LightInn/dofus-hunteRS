mod api;
mod arrows;
pub mod config;
mod ocr;
mod screenshot;
mod state;
mod window;

use serde::Deserialize;
use tauri::State;
use tauri::{AppHandle, Emitter, Manager};

use crate::core::state::Coord;
pub use config::BotConfig;
pub use screenshot::{capture_region, ScreenRegion};
pub use state::AppState;
pub use state::ArrowDirection;
use window::WindowManager;

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
    let state = state.inner.lock().unwrap();

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
pub fn capture_analyse(state: State<'_, AppState>, app: AppHandle) -> Result<(), String> {
    let mut state = state.inner.lock().unwrap();

    let region: ScreenRegion = state.config.regions.hunt_panel.into();

    let image = capture_region(region).map_err(|e| e.to_string())?;

    let infos = ocr::ocr_hunt_panel(&image).map_err(|e| e.to_string())?;

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

    let x = state.bot_data.coords.start.x as i32;
    let y = state.bot_data.coords.start.y as i32;
    let direction = match state.bot_data.current_arrow {
        ArrowDirection::Up => "up",
        ArrowDirection::Down => "down",
        ArrowDirection::Left => "left",
        ArrowDirection::Right => "right",
        ArrowDirection::Unknown => "unknown",
    };

    let hint = state.bot_data.current_hint.clone();

    println!(
        "Sending request with x: {}, y: {}, direction: {}, hint: {}",
        x, y, direction, hint
    );

    let response =
        api::find_next_location(config, x, y, direction, &hint).map_err(|e| e.to_string())?;

    println!("{:?}", response);

    Ok(())
}

#[derive(Deserialize)]
pub struct RegionCoordinates {
    region: String,
    coordinates: [i32; 4],
}

#[tauri::command]
pub async fn save_region(
    region_data: RegionCoordinates,
    state: State<'_, AppState>, // Supposons que `AppState` contient votre configuration
) -> Result<(), String> {
    let region = region_data.region;
    let coordinates = region_data.coordinates;

    let mut state = state.inner.lock().unwrap();

    // Convertir [i32; 4] en (i32, i32, u32, u32)
    let (x1, y1, x2, y2) = (
        coordinates[0],        // x1
        coordinates[1],        // y1
        coordinates[2] as u32, // x2 (convertir en u32)
        coordinates[3] as u32, // y2 (convertir en u32)
    );

    // Assigner les coordonnées à la région correspondante
    match region.as_str() {
        "coordinates" => state.config.regions.coordinates = (x1, y1, x2, y2),
        "hunt_panel" => state.config.regions.hunt_panel = (x1, y1, x2, y2),
        "chat" => state.config.regions.chat = (x1, y1, x2, y2),
        _ => return Err(format!("Région inconnue : {}", region)),
    }

    state.config.save().map_err(|e| e.to_string())?;

    println!(
        "Sauvegarde de la région '{}' avec les coordonnées : {:?}",
        region,
        (x1, y1, x2, y2)
    );

    Ok(())
}

#[tauri::command]
pub fn python(state: State<'_, AppState>) -> Result<(), String> {
    let mut state = state.inner.lock().unwrap();
    let config = state.config.clone();

    let region: ScreenRegion = state.config.regions.hunt_panel.into();
    let image = capture_region(region).map_err(|e| e.to_string())?;

    let infos = ocr::ocr_hunt_panel(&image).map_err(|e| e.to_string())?;

    state.bot_data.coords.start = Coord {
        x: infos.start_x,
        y: infos.start_y,
    };
    println!("Hint : {}", infos.current_hint);
    state.bot_data.current_hint = infos.current_hint;
    println!("Hint: {}", state.bot_data.current_hint);
    state.bot_data.steps.current = infos.step_current;
    state.bot_data.steps.total = infos.step_total;

    let direction = arrows::detect_arrow_direction(&image, false);

    println!("Direction: {:?}", direction);
    state.bot_data.current_arrow = direction;
    println!("Direction: {:?}", state.bot_data.current_arrow);

    let x = state.bot_data.coords.target.x as i32;
    let y = state.bot_data.coords.target.y as i32;
    let direction = match state.bot_data.current_arrow {
        ArrowDirection::Up => "up",
        ArrowDirection::Down => "down",
        ArrowDirection::Left => "left",
        ArrowDirection::Right => "right",
        ArrowDirection::Unknown => "unknown",
    };

    let hint = state.bot_data.current_hint.clone();

    println!(
        "Sending request with x: {}, y: {}, direction: {}, hint: {}",
        x, y, direction, hint
    );

    let response = api::find_next_location(config.api, x, y, direction, &hint)
        .map_err(|e| e.to_string())?
        .unwrap();

    state.bot_data.coords.target.x = response.pos_x as i8;
    state.bot_data.coords.target.y = response.pos_y as i8;

    let mut window_manager = WindowManager::new();

    window_manager.find_window("Latte").unwrap();
    window_manager.bring_to_front().unwrap();
    window_manager
        .send_travel_command(response.pos_x, response.pos_y)
        .unwrap();

    Ok(())
}
