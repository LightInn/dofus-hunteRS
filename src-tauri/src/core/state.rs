use config::Config;
use std::sync::{Arc, Mutex};

use super::config::BotConfig;

#[derive(Default)]
enum ArrowDirection {
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
    pub config: BotConfig,
}

#[derive(Default)]
pub struct BotData {
    steps: Steps,
    current_hint: String,
    start_coord: Coord,
    current_coord: Coord,
    current_arrow: ArrowDirection,
}

#[derive(Default)]
struct Steps {
    current: u8,
    total: u8,
}

#[derive(Default)]
struct Coord {
    x: i8,
    y: i8,
}
