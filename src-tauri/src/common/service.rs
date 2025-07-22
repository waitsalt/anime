use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ServiceConfig {
    pub base_url: String,
    pub user_agent: Option<String>,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
}

impl Default for ServiceConfig {
    fn default() -> Self {
        ServiceConfig {
            base_url: "https://api.bgm.tv".to_string(),
            user_agent: None,
            access_token: None,
            refresh_token: None,
        }
    }
}

impl ServiceConfig {
    pub fn set(&mut self, new_config: ServiceConfig) {
        self.base_url = new_config.base_url;
        self.user_agent = new_config.user_agent;
        self.access_token = new_config.access_token;
        self.refresh_token = new_config.refresh_token;
    }
}
