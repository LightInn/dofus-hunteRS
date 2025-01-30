use config::Config;
use std::sync::{Arc, Mutex};

use super::config::BotConfig;

#[derive(Default, Debug, Clone, Copy, serde::Serialize)]
pub enum ArrowDirection {
    Up,
    Down,
    Left,
    Right,
    #[default]
    Unknown,
}

#[derive(Default)]
pub struct AppState {
    pub inner: Arc<Mutex<InnerAppState>>,
}

#[derive(Default)]
pub struct InnerAppState {
    pub running: bool,
    pub bot_data: BotData,
    pub config: BotConfig,
}

#[derive(Default)]
pub struct BotData {
    pub steps: Steps,
    pub current_hint: String,
    pub start_coord: Coord,
    pub current_coord: Coord,
    pub current_arrow: ArrowDirection,
}

#[derive(Default)]
pub struct Steps {
    pub current: u8,
    pub total: u8,
}

#[derive(Default)]
pub struct Coord {
    pub x: i8,
    pub y: i8,
}
