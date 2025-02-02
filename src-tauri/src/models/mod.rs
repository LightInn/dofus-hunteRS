use crate::composent::config::BotConfig;

use std::sync::{Arc, Mutex};
use serde::Deserialize;

#[derive(Default, Debug, Clone, Copy, serde::Serialize)]
#[serde(rename_all = "camelCase")]
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

#[derive(Default, Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InnerAppState {
    pub running: bool,
    pub bot_data: BotData,
    pub config: BotConfig,
}

#[derive(Default, Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BotData {
    pub steps: Steps,
    pub coords: Coords,
    pub current_hint: String,
    pub current_arrow: ArrowDirection,
}

#[derive(Default, Debug, Clone, Copy, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Steps {
    pub current: u8,
    pub total: u8,
}

#[derive(Default, Debug, Clone, Copy, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Coord {
    pub x: i8,
    pub y: i8,
}

#[derive(Default, Debug, Clone, Copy, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Coords {
    pub start: Coord,
    pub current: Coord,
    pub target: Coord,
}



#[derive(Deserialize)]
pub struct RegionCoordinates {
    pub(crate) region: String,
    pub(crate) coordinates: [i32; 4],
}


/// Représente une région de l'écran avec des coordonnées (x, y, width, height)
pub type ScreenRegion = (i32, i32, i32, i32);


// api models
#[derive(Debug, Deserialize, Default)]
pub struct LocationData {
    pub pos_x: i8,
    pub pos_y: i8,
    pub distance: u8,
}