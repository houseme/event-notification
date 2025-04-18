use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Event {
    pub id: Uuid,
    pub event_type: String,
    pub payload: Value,
    pub timestamp: DateTime<Utc>,
    pub channels: Vec<String>,
}

impl Event {
    pub fn new(event_type: &str, payload: Value, channels: Vec<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            event_type: event_type.to_string(),
            payload,
            timestamp: Utc::now(),
            channels,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookConfig {
    pub endpoint: String,
    pub auth_token: Option<String>,
    pub custom_headers: Option<HashMap<String, String>>,
    pub max_retries: u32,
    pub timeout: u64,
}
