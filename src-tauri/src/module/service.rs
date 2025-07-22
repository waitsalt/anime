use crate::common::service::ServiceConfig;

use super::bangumi::BANGUMI_CLIENT;

pub async fn service_load(service_config: &ServiceConfig) -> anyhow::Result<()> {
    let mut bangumi_client = BANGUMI_CLIENT.write().await;

    bangumi_client.base_path = service_config.base_url.clone();
    bangumi_client.user_agent = service_config.user_agent.clone();
    bangumi_client.access_token = service_config.access_token.clone();

    Ok(())
}
