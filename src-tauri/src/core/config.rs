use config::{Config, Environment, File, FileFormat};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

use std::sync::{Arc, Mutex};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BotConfig {
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

impl BotConfig {
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

impl Default for BotConfig {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| BotConfig {
            api: ApiConfig {
                url: "http://localhost".to_string(),
                token: "default_token".to_string(),
            },
            window: WindowConfig {
                title: "Default Title".to_string(),
                focus_chat_binding: "Ctrl+Enter".to_string(),
            },
            regions: RegionConfig {
                coordinates: (0, 0, 100, 100),
                hunt_panel: (0, 0, 50, 50),
                chat: (0, 0, 50, 50),
            },
            database: DatabaseConfig {
                host: "localhost".to_string(),
                port: 5432,
                name: "default_db".to_string(),
                user: "user".to_string(),
                password: "password".to_string(),
            },
            logging: LoggingConfig {
                file: PathBuf::from("log.txt"),
                level: "info".to_string(),
            },
            shortcuts: ShortcutConfig {
                auto_detect: "F5".to_string(),
                use_proxy: false,
                keyboard_shortcuts: true,
            },
        })
    }
}
