use crate::config::AdapterConfig;
use crate::event::Event;
use async_trait::async_trait;
use std::sync::Arc;

#[cfg(feature = "kafka")]
pub mod kafka;
#[cfg(feature = "mqtt")]
pub mod mqtt;
#[cfg(feature = "webhook")]
pub mod webhook;

use crate::error::Error;

#[async_trait]
pub trait ChannelAdapter: Send + Sync {
    fn name(&self) -> String;
    async fn send(&self, event: &Event) -> Result<(), Error>;
}

pub fn create_adapters(configs: &[AdapterConfig]) -> Result<Vec<Arc<dyn ChannelAdapter>>, Error> {
    let mut adapters: Vec<Arc<dyn ChannelAdapter>> = Vec::new();

    for config in configs {
        match config {
            #[cfg(feature = "webhook")]
            AdapterConfig::Webhook(webhook_config) => {
                adapters.push(Arc::new(webhook::WebhookAdapter::new(
                    webhook_config.clone(),
                )));
            }
            #[cfg(feature = "kafka")]
            AdapterConfig::Kafka(kafka_config) => {
                adapters.push(Arc::new(kafka::KafkaAdapter::new(kafka_config)?));
            }
            #[cfg(feature = "mqtt")]
            AdapterConfig::Mqtt(mqtt_config) => {
                let (mqtt, event_loop) = mqtt::MqttAdapter::new(mqtt_config);
                tokio::spawn(async move { while event_loop.poll().await.is_ok() {} });
                adapters.push(Arc::new(mqtt));
            }
            #[cfg(not(feature = "webhook"))]
            AdapterConfig::Webhook(_) => return Err(Error::FeatureDisabled("webhook")),
            #[cfg(not(feature = "kafka"))]
            AdapterConfig::Kafka(_) => return Err(Error::FeatureDisabled("kafka")),
            #[cfg(not(feature = "mqtt"))]
            AdapterConfig::Mqtt(_) => return Err(Error::FeatureDisabled("mqtt")),
        }
    }

    Ok(adapters)
}
