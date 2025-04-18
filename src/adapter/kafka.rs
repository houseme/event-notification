use crate::adapter::ChannelAdapter;
use crate::event::Event;
use async_trait::async_trait;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;

pub struct KafkaAdapter {
    producer: FutureProducer,
    topic: String,
}

impl KafkaAdapter {
    pub fn new(brokers: &str, topic: &str) -> anyhow::Result<Self> {
        let producer = rdkafka::producer::FutureProducer::from_config(
            &rdkafka::config::ClientConfig::new()
                .set("bootstrap.servers", brokers)
                .create()?,
        )?;
        Ok(Self {
            producer,
            topic: topic.to_string(),
        })
    }
}

#[async_trait]
impl ChannelAdapter for KafkaAdapter {
    fn name(&self) -> String {
        "kafka".to_string()
    }

    async fn send(&self, event: &Event) -> anyhow::Result<()> {
        let payload = serde_json::to_string(event)?;
        let record = FutureRecord::to(&self.topic)
            .payload(&payload)
            .key(&event.id.to_string());
        self.producer
            .send(record, Timeout::Never)
            .await
            .map_err(|(e, _)| anyhow::anyhow!(e))?;
        Ok(())
    }
}
