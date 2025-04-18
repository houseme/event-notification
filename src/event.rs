use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
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


