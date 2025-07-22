use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{
    path::Path,
    sync::{Arc, LazyLock},
};
use tauri::async_runtime::RwLock;

use crate::common::{service::ServiceConfig, theme::ThemeConfig};

use super::model::AutoSave;

pub static CONFIG: LazyLock<Arc<RwLock<AutoSave<Config>>>> = LazyLock::new(|| {
    Arc::new(RwLock::new(AutoSave::new(
        Config::load_default_config_file().unwrap(),
        "config.toml",
    )))
});

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub service: ServiceConfig,
    pub theme: ThemeConfig,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            service: ServiceConfig::default(),
            theme: ThemeConfig::default(),
        }
    }
}

impl Config {
    pub fn load_default_config_file() -> Result<Self> {
        let config_path = Path::new("config.toml");

        if !config_path.exists() {
            let config_default = Config::default();
            let config_default_string = toml::to_string(&config_default)?;
            std::fs::write(config_path, config_default_string)?;
        }

        let config_string = std::fs::read_to_string(config_path)?;
        let config = toml::from_str(&config_string)?;

        Ok(config)
    }

    pub fn set_config(&mut self, config: Config) {
        *self = config;
    }

    pub fn set_service_config(&mut self, service_config: ServiceConfig) {
        self.service = service_config;
    }

    pub fn set_theme_config(&mut self, theme_config: ThemeConfig) {
        self.theme = theme_config;
    }
}
