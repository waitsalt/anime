use crate::common::{server::ServerConfig, theme::ThemeConfig};

pub struct Config {
    pub server: ServerConfig,
    pub theme: ThemeConfig,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            server: ServerConfig::default(),
            theme: ThemeConfig::default(),
        }
    }
}
