pub mod user;

use bangumi_api::{
    common::model::BangumiClient,
    module::user::model::{AvatarType, User, UserPublic},
};
use std::sync::{Arc, LazyLock};
use tauri::async_runtime::RwLock;

pub static BANGUMI_CLIENT: LazyLock<Arc<RwLock<BangumiClient>>> =
    LazyLock::new(|| Arc::new(RwLock::new(BangumiClient::default())));

#[tauri::command]
pub async fn bangumi_access_token_check(access_token: String) -> Result<User, String> {
    let mut bangumi_client = BangumiClient::default();
    bangumi_client.access_token = Some(access_token);
    let user = bangumi_client.get_me().await.map_err(|e| e.to_string())?;
    Ok(user)
}

#[tauri::command]
pub async fn bangumi_get_user(username: String) -> Result<UserPublic, String> {
    let bangumi_client = BANGUMI_CLIENT.read().await;
    let user = bangumi_client
        .get_user(&username)
        .await
        .map_err(|e| e.to_string())?;
    Ok(user)
}

#[tauri::command]
pub async fn bangumi_get_user_avatar(
    username: String,
    r#type: AvatarType,
) -> Result<Vec<u8>, String> {
    let bangumi_client = BANGUMI_CLIENT.read().await;
    let avatar = bangumi_client
        .get_user_avatar(&username, r#type)
        .await
        .map_err(|e| e.to_string())?;
    Ok(avatar.to_vec())
}

#[tauri::command]
pub async fn bangumi_get_me() -> Result<User, String> {
    let bangumi_client = BANGUMI_CLIENT.read().await;
    let user = bangumi_client.get_me().await.map_err(|e| e.to_string())?;
    Ok(user)
}
