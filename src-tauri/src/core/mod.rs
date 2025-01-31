pub mod api;
pub mod arrows;
pub mod config;
pub mod ocr;

use crate::composent::api::find_next_location;
use crate::composent::arrows::detect_arrow_direction;
use crate::composent::ocr::ocr_hunt_panel;
use crate::composent::screenshot::capture_region;
use crate::composent::window::WindowManager;
use crate::models::{AppState, ArrowDirection, Coord, ScreenRegion};

use serde::Deserialize;
use tauri::State;
use tauri::{Emitter, Manager};

#[tauri::command]
pub fn call_python(state: State<'_, AppState>) -> Result<(), String> {
    let mut state = state.inner.lock().unwrap();
    let config = state.config.clone();

    let region: ScreenRegion = state.config.regions.hunt_panel.into();
    let image = capture_region(region).map_err(|e| e.to_string())?;

    let infos = ocr_hunt_panel(&image).map_err(|e| e.to_string())?;

    state.bot_data.coords.start = Coord {
        x: infos.start_x,
        y: infos.start_y,
    };
    println!("Hint : {}", infos.current_hint);
    state.bot_data.current_hint = infos.current_hint;
    println!("Hint: {}", state.bot_data.current_hint);
    state.bot_data.steps.current = infos.step_current;
    state.bot_data.steps.total = infos.step_total;

    let direction = detect_arrow_direction(&image, false);

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

    let response = find_next_location(config.api, x, y, direction, &hint)
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
