use std::sync::{Arc, Mutex};
use config::Config;

use super::config::BotConfig;


#[derive(Default)]
pub struct AppState {
    pub inner: Arc<Mutex<InnerAppState>>
}

#[derive(Default)]
pub struct InnerAppState {
    pub running: bool,
    pub config: BotConfig,
}