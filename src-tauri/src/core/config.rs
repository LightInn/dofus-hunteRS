use serde::{Deserialize, Serialize};
use config::{Config, File, FileFormat, Environment};
use std::collections::HashMap;
use std::path::PathBuf;



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


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub api: ApiConfig,
    pub window: WindowConfig,
    pub regions: RegionConfig,
    pub database: DatabaseConfig,
    pub logging: LoggingConfig,
    pub shortcuts: ShortcutConfig,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub api: ApiConfig,
    pub window: WindowConfig,
    pub regions: RegionConfig,
    pub database: DatabaseConfig,
    pub logging: LoggingConfig,
    pub shortcuts: ShortcutConfig,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiConfig {
    pub url: String,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WindowConfig {
    pub title: String,
    pub focus_chat_binding: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegionConfig {
    pub coordinates: (i32, i32, u32, u32),
    pub hunt_panel: (i32, i32, u32, u32),
    pub chat: (i32, i32, u32, u32),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub name: String,
    pub user: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoggingConfig {
    pub file: PathBuf,
    pub level: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShortcutConfig {
    pub auto_detect: String,
    pub use_proxy: bool,
    pub keyboard_shortcuts: bool,
}

impl AppConfig {
    pub fn new() -> Result<Self, config::ConfigError> {
        let mut cfg = Config::builder()
            .add_source(File::with_name("config").format(FileFormat::Json))
            .add_source(Environment::with_prefix("DOFUS").separator("__"))
            .build()?;

        cfg.try_deserialize()
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        let contents = serde_json::to_string_pretty(self)?;
        std::fs::write("config.json", contents)?;
        Ok(())
    }
}