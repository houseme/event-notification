use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookConfig {
    pub endpoint: String,
    pub auth_token: Option<String>,
    pub custom_headers: Option<HashMap<String, String>>,
    pub max_retries: u32,
    pub timeout: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KafkaConfig {
    pub brokers: String,
    pub topic: String,
    pub max_retries: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MqttConfig {
    pub broker: String,
    pub port: u16,
    pub client_id: String,
    pub topic: String,
    pub max_retries: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AdapterConfig {
    Webhook(WebhookConfig),
    Kafka(KafkaConfig),
    Mqtt(MqttConfig),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationConfig {
    pub store_path: String,
    pub channel_capacity: usize,
    pub adapters: Vec<AdapterConfig>,
}
