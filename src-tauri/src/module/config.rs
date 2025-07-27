use crate::common::config::{CONFIG, Config};

use super::theme::theme_load;

#[tauri::command]
pub async fn config_set(config: Config) -> Result<(), String> {
    let mut app_config = CONFIG.write().await;

    // if config.service == app_config.service {
    //     // 只将当前 Bangumi 客户端配置修改
    //     service_load(&config.service)
    //         .await
    //         .map_err(|e| e.to_string())?;
    // }

    if config.theme == app_config.theme {
        // 只是检查主题配置是否正确
        theme_load(&config.theme).map_err(|e| e.to_string())?;
    }

    app_config.set_config(config);

    Ok(())
}

#[tauri::command]
pub async fn config_get() -> Result<Config, String> {
    let config = CONFIG.read().await;
    Ok(config.clone())
}
