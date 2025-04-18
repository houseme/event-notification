use crate::adapter::ChannelAdapter;
use crate::event::{Event, WebhookConfig};
use async_trait::async_trait;
use reqwest::{Client, RequestBuilder};
use std::time::Duration;
use tokio::time::sleep;

pub struct WebhookAdapter {
    config: WebhookConfig,
    client: Client,
}

impl WebhookAdapter {
    pub fn new(config: WebhookConfig) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout))
            .build()
            .expect("Failed to build reqwest client");
        Self { config, client }
    }

    fn build_request(&self, event: &Event) -> RequestBuilder {
        let mut request = self.client.post(&self.config.endpoint).json(event);
        if let Some(token) = &self.config.auth_token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }
        if let Some(headers) = &self.config.custom_headers {
            for (key, value) in headers {
                request = request.header(key, value);
            }
        }
        request
    }
}

#[async_trait]
impl ChannelAdapter for WebhookAdapter {
    fn name(&self) -> String {
        "webhook".to_string()
    }

    async fn send(&self, event: &Event) -> anyhow::Result<()> {
        let mut attempt = 0;
        loop {
            match self.build_request(event).send().await {
                Ok(response) => {
                    response.error_for_status()?;
                    return Ok(());
                }
                Err(e) if attempt < self.config.max_retries => {
                    attempt += 1;
                    tracing::warn!("Webhook attempt {} failed: {}. Retrying...", attempt, e);
                    sleep(Duration::from_secs(2u64.pow(attempt))).await;
                }
                Err(e) => {
                    return Err(anyhow::anyhow!(
                        "Webhook failed after {} retries: {}",
                        attempt,
                        e
                    ));
                }
            }
        }
    }
}
