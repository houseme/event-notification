use crate::Error;
use crate::config::KafkaConfig;
use crate::event::Event;
use async_trait::async_trait;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;
use std::time::Duration;
use tokio::time::sleep;

/// Kafka adapter for sending events to a Kafka topic.
pub struct KafkaAdapter {
    producer: FutureProducer,
    topic: String,
    max_retries: u32,
}

impl KafkaAdapter {
    /// Creates a new Kafka adapter.
    pub fn new(config: &KafkaConfig) -> Result<Self, Error> {
        let producer = rdkafka::producer::FutureProducer::from_config(
            &rdkafka::config::ClientConfig::new()
                .set("bootstrap.servers", &config.brokers)
                .create()
                .map_err(Error::Kafka)?,
        )
        .map_err(Error::Kafka)?;
        Ok(Self {
            producer,
            topic: config.topic.clone(),
            max_retries: config.max_retries,
        })
    }
}

#[async_trait]
impl ChannelAdapter for KafkaAdapter {
    fn name(&self) -> String {
        "kafka".to_string()
    }

    async fn send(&self, event: &Event) -> Result<(), Error> {
        let payload = serde_json::to_string(event).map_err(Error::Serde)?;
        let record = FutureRecord::to(&self.topic)
            .payload(&payload)
            .key(&event.id.to_string());
        let mut attempt = 0;
        loop {
            match self.producer.send(record, Timeout::Never).await {
                Ok(()) => return Ok(()),
                Err((e, _)) if attempt < self.max_retries => {
                    attempt += 1;
                    tracing::warn!("Kafka attempt {} failed: {}. Retrying...", attempt, e);
                    sleep(Duration::from_secs(2u64.pow(attempt))).await;
                }
                Err((e, _)) => return Err(Error::Kafka(e)),
            }
        }
    }
}
