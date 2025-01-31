use tauri::State;
use crate::composent::api::find_next_location;
use crate::models::{AppState, ArrowDirection};

#[tauri::command]
pub fn call_send_api_request(state: State<'_, AppState>) -> Result<(), String> {
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
        find_next_location(config, x, y, direction, &hint).map_err(|e| e.to_string())?;

    println!("{:?}", response);

    Ok(())
}