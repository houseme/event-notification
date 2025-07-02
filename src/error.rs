use thiserror::Error;
use tokio::sync::mpsc::error;
use tokio::task::JoinError;

/// The `Error` enum represents all possible errors that can occur in the application.
/// It implements the `std::error::Error` trait and provides a way to convert various error types into a single error type.
#[derive(Error, Debug)]
pub enum Error {
    #[error("Join error: {0}")]
    JoinError(#[from] JoinError),
    #[error("IO error: {0}")]
    Io(#[from] Box<std::io::Error>),
    #[error("Serialization error: {0}")]
    Serde(#[from] Box<serde_json::Error>),
    #[error("HTTP error: {0}")]
    Http(#[from] Box<reqwest::Error>),
    #[cfg(feature = "kafka")]
    #[error("Kafka error: {0}")]
    Kafka(#[from] Box<rdkafka::error::KafkaError>),
    #[cfg(feature = "mqtt")]
    #[error("MQTT error: {0}")]
    Mqtt(#[from] Box<rumqttc::ClientError>),
    #[error("Channel send error: {0}")]
    ChannelSend(#[from] Box<error::SendError<crate::event::Event>>),
    #[error("Feature disabled: {0}")]
    FeatureDisabled(&'static str),
    #[error("Event bus already started")]
    EventBusStarted,
    #[error("necessary fields are missing:{0}")]
    MissingField(&'static str),
    #[error("field verification failed:{0}")]
    ValidationError(&'static str),
    #[error("{0}")]
    Custom(String),
    #[error("Configuration error: {0}")]
    ConfigError(String),
    #[error("Configuration loading error: {0}")]
    Figment(#[from] Box<figment::Error>),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(Box::new(err))
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Serde(Box::new(err))
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Http(Box::new(err))
    }
}

#[cfg(feature = "kafka")]
impl From<rdkafka::error::KafkaError> for Error {
    fn from(err: rdkafka::error::KafkaError) -> Self {
        Error::Kafka(Box::new(err))
    }
}

#[cfg(feature = "mqtt")]
impl From<rumqttc::ClientError> for Error {
    fn from(err: rumqttc::ClientError) -> Self {
        Error::Mqtt(Box::new(err))
    }
}

impl From<figment::Error> for Error {
    fn from(err: figment::Error) -> Self {
        Error::Figment(Box::new(err))
    }
}

impl Error {
    pub(crate) fn custom(msg: &str) -> Error {
        Self::Custom(msg.to_string())
    }
}
