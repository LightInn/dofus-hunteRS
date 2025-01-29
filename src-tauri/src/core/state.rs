use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct AppState {
    pub inner: Arc<Mutex<InnerAppState>>
}

#[derive(Default)]
pub struct InnerAppState {
    pub running: bool,
    pub config: BotConfig,
}

#[derive(Default)]
pub struct BotConfig {
    pub window_title: String,
    pub api_url: String,
}
