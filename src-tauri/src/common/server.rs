pub struct ServerConfig {
    pub name: String,
    pub version: String,

    pub base_url: String,
    pub user_agent: String,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            name: "anime".to_string(),
            version: "0.0.1".to_string(),
            base_url: "https://api.bgm.tv".to_string(),
            user_agent: "waitsalt/anime (https://github.com/waitsalt/anime)".to_string(),
            access_token: None,
            refresh_token: None,
        }
    }
}
