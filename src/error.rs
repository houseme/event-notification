use thiserror::Error;
use tokio::task::JoinError;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Join error: {0}")]
    JoinError(#[from] JoinError),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[cfg(feature = "kafka")]
    #[error("Kafka error: {0}")]
    Kafka(#[from] rdkafka::error::KafkaError),
    #[cfg(feature = "mqtt")]
    #[error("MQTT error: {0}")]
    Mqtt(#[from] rumqttc::ClientError),
    #[error("Channel send error: {0}")]
    ChannelSend(#[from] tokio::sync::mpsc::error::SendError<crate::event::Event>),
    #[error("Feature disabled: {0}")]
    FeatureDisabled(&'static str),
    #[error("Event bus already started")]
    EventBusStarted,
}
