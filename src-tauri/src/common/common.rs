use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CommonConfig {
    pub name: String,
    pub version: String,
}

impl Default for CommonConfig {
    fn default() -> Self {
        CommonConfig {
            name: "anime".to_string(),
            version: "0.1.0".to_string(),
        }
    }
}
