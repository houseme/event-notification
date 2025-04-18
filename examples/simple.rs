use event_notification::adapter::{webhook::{WebhookAdapter, WebhookConfig}, ChannelAdapter};
use event_notification::NotificationSystem;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::signal;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let system = Arc::new(NotificationSystem::new("./events").await?);

    let webhook_config = WebhookConfig {
        endpoint: "http://localhost:8080/webhook".to_string(),
        auth_token: Some("secret-token".to_string()),
        custom_headers: Some(HashMap::from([(
            "X-Custom".to_string(),
            "value".to_string(),
        )])),
        max_retries: 3,
        timeout: 10,
    };
    let webhook = WebhookAdapter::new(webhook_config);

    let adapters: Vec<Arc<dyn ChannelAdapter>> = vec![Arc::new(webhook)];
    let system_clone = Arc::clone(&system);
    let system_handle = tokio::spawn((*system_clone).start(adapters));

    signal::ctrl_c().await?;
    tracing::info!("Received shutdown signal");
    system.shutdown();

    system_handle.await??;
    Ok(())
}
