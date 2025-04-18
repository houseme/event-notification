use crate::adapter::ChannelAdapter;
use crate::event::Event;
use async_trait::async_trait;
use rumqttc::{AsyncClient, MqttOptions, QoS};

pub struct MqttAdapter {
    client: AsyncClient,
    topic: String,
}

impl MqttAdapter {
    pub fn new(
        broker: &str,
        port: u16,
        client_id: &str,
        topic: &str,
    ) -> (Self, rumqttc::EventLoop) {
        let mqtt_options = MqttOptions::new(client_id, broker, port);
        let (client, event_loop) = rumqttc::AsyncClient::new(mqtt_options, 10);
        (
            Self {
                client,
                topic: topic.to_string(),
            },
            event_loop,
        )
    }
}

#[async_trait]
impl ChannelAdapter for MqttAdapter {
    fn name(&self) -> String {
        "mqtt".to_string()
    }

    async fn send(&self, event: &Event) -> anyhow::Result<()> {
        let payload = serde_json::to_string(event)?;
        self.client
            .publish(&self.topic, QoS::AtLeastOnce, false, payload)
            .await?;
        Ok(())
    }
}
