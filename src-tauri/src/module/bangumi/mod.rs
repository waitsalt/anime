#[tauri::command]
pub async fn login(
    access_token: String,
) -> Result<bangumi_api::models::GetMyself200Response, String> {
    let mut bangumi_client = bangumi_api::apis::configuration::Configuration::default();
    bangumi_client.bearer_access_token = Some(access_token);
    let a = bangumi_api::apis::default_api::get_myself(&bangumi_client)
        .await
        .map_err(|e| e.to_string())?;
    Ok(a)
}
