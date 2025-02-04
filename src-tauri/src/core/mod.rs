pub mod api;
pub mod arrows;
pub mod config;
pub mod error;
pub mod ocr;

use crate::composent::api::find_next_location;
use crate::composent::arrows::detect_arrow_direction;
use crate::composent::ocr::{ocr_coordinates, ocr_hunt_panel};
use crate::composent::screenshot::capture_region;
use crate::composent::window::WindowManager;
use crate::models::{AppState, ArrowDirection, Coord, HistoryPoint, HistoryType, ScreenRegion};
use error::Result;
use std::cmp::PartialEq;
use std::panic;

use crate::core::error::ApiError;
use serde::Deserialize;
use tauri::{AppHandle, State};
use tauri::{Emitter, Manager};

#[tauri::command]
pub fn call_python(state: State<'_, AppState>, app: AppHandle) -> Result<()> {
    let mut state = state.inner.lock().unwrap();
    let config = state.config.clone();

    let imageHuntPanel = capture_region(state.config.regions.hunt_panel.into())?;
    let imageCoordinates = capture_region(state.config.regions.coordinates.into())?;

    let infos = ocr_hunt_panel(&imageHuntPanel, state.config.api.clone())?;

    state.bot_data.coords.start = Coord {
        x: infos.start_x,
        y: infos.start_y,
    };

    state.bot_data.current_hint = infos.current_hint;
    println!("Hint: {}", state.bot_data.current_hint);
    state.bot_data.steps.current = infos.step_current;
    state.bot_data.steps.total = infos.step_total;

    let direction = detect_arrow_direction(&imageHuntPanel, false);

    let current_co = ocr_coordinates(&imageCoordinates)?.unwrap_or_default();
    state.bot_data.coords.current = Coord {
        x: current_co.0,
        y: current_co.1,
    };

    println!("Direction: {:?}", direction);
    state.bot_data.current_arrow = direction;
    println!("Direction: {:?}", state.bot_data.current_arrow);

    let is_start = state.bot_data.coords.target == state.bot_data.coords.start;

    let x = (if is_start {
        state.bot_data.coords.start.x
    } else {
        state.bot_data.coords.current.x
    }) as i32;
    let y = (if is_start {
        state.bot_data.coords.start.y
    } else {
        state.bot_data.coords.current.y
    }) as i32;

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

    let response = find_next_location(config.api, x, y, direction, &hint).map_err(|e| {
        state.api_status = crate::models::ApiStatus::Error;
        app.emit("state_changed", &*state).unwrap();
        e
    })?;

    if response.is_none() {
        state.api_status = crate::models::ApiStatus::Error;
        app.emit("state_changed", &*state).unwrap();
        return Ok(());
    }

    let api_data = response.unwrap();

    let history_type_entry: HistoryType =
        if state.bot_data.coords.target == state.bot_data.coords.start {
            HistoryType::Start
        } else {
            HistoryType::Normal
        };

    let history_entry = HistoryPoint {
        coord: Coord {
            x: api_data.pos_x,
            y: api_data.pos_y,
        },
        history_type: history_type_entry,
    };

    state.bot_data.history.push(history_entry);
    state.api_status = crate::models::ApiStatus::Active;
    state.bot_data.coords.target.x = api_data.pos_x;
    state.bot_data.coords.target.y = api_data.pos_y;

    let mut window_manager = WindowManager::new();

    window_manager.find_window("Latte").unwrap();
    window_manager.bring_to_front().unwrap();
    window_manager
        .send_travel_command(api_data.pos_x, api_data.pos_y)
        .unwrap();

    app.emit("state_changed", &*state).unwrap();

    Ok(())
}

#[tauri::command]
pub fn call_manual(state: State<'_, AppState>, app: AppHandle) -> Result<()> {
    let mut state = state.inner.lock().unwrap();
    let config = state.config.clone();

    let x = state.bot_data.coords.current.x as i32;
    let y = state.bot_data.coords.current.y as i32;
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
        .map_err(|e| {
            state.api_status = crate::models::ApiStatus::Error;
            app.emit("state_changed", &*state).unwrap();
            e
        })?
        .unwrap();

    let history_type_entry: HistoryType =
        if state.bot_data.coords.target == state.bot_data.coords.start {
            HistoryType::Start
        } else {
            HistoryType::Normal
        };

    let history_entry = HistoryPoint {
        coord: Coord {
            x: response.pos_x,
            y: response.pos_y,
        },
        history_type: history_type_entry,
    };

    state.bot_data.history.push(history_entry);
    state.api_status = crate::models::ApiStatus::Active;
    state.bot_data.coords.target.x = response.pos_x;
    state.bot_data.coords.target.y = response.pos_y;

    let mut window_manager = WindowManager::new();

    window_manager.find_window("Latte").unwrap();
    window_manager.bring_to_front().unwrap();
    window_manager
        .send_travel_command(response.pos_x, response.pos_y)
        .unwrap();

    app.emit("state_changed", &*state).unwrap();

    Ok(())
}
