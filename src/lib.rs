mod adapter;
mod bus;
mod config;
mod error;
mod event;
mod global;
mod producer;
mod store;

pub use adapter::ChannelAdapter;
pub use adapter::create_adapters;
#[cfg(feature = "kafka")]
pub use adapter::kafka::KafkaAdapter;
#[cfg(feature = "mqtt")]
pub use adapter::mqtt::MqttAdapter;
#[cfg(feature = "webhook")]
pub use adapter::webhook::WebhookAdapter;
pub use bus::event_bus;
#[cfg(feature = "http-producer")]
pub use config::HttpProducerConfig;
#[cfg(feature = "kafka")]
pub use config::KafkaConfig;
#[cfg(feature = "mqtt")]
pub use config::MqttConfig;
#[cfg(feature = "webhook")]
pub use config::WebhookConfig;
pub use config::{AdapterConfig, NotificationConfig};
pub use error::Error;

pub use event::{Bucket, Event, EventBuilder, Identity, Log, Metadata, Name, Object, Source};
pub use global::{initialize, initialize_and_start, send_event, shutdown, start};
pub use store::EventStore;

#[cfg(feature = "http-producer")]
pub use producer::EventProducer;
#[cfg(feature = "http-producer")]
pub use producer::http::HttpProducer;

use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

/// The `NotificationSystem` struct represents the notification system.
/// It manages the event bus and the adapters.
/// It is responsible for sending and receiving events.
/// It also handles the shutdown process.
pub struct NotificationSystem {
    tx: mpsc::Sender<Event>,
    rx: Option<mpsc::Receiver<Event>>,
    store: Arc<EventStore>,
    shutdown: CancellationToken,
    #[cfg(feature = "http-producer")]
    http_config: HttpProducerConfig,
}

impl NotificationSystem {
    /// Creates a new `NotificationSystem` instance.
    pub async fn new(config: NotificationConfig) -> Result<Self, Error> {
        let (tx, rx) = mpsc::channel::<Event>(config.channel_capacity);
        let store = Arc::new(EventStore::new(&config.store_path).await?);
        let shutdown = CancellationToken::new();

        let restored_logs = store.load_logs().await?;
        for log in restored_logs {
            for event in log.records {
                // For example, where the send method may return a SendError when calling it
                tx.send(event)
                    .await
                    .map_err(|e| Error::ChannelSend(Box::new(e)))?;
            }
        }

        Ok(Self {
            tx,
            rx: Some(rx),
            store,
            shutdown,
            #[cfg(feature = "http-producer")]
            http_config: config.http,
        })
    }

    /// Starts the notification system.
    /// It initializes the event bus and the producer.
    pub async fn start(&mut self, adapters: Vec<Arc<dyn ChannelAdapter>>) -> Result<(), Error> {
        let rx = self.rx.take().ok_or_else(|| Error::EventBusStarted)?;

        let shutdown_clone = self.shutdown.clone();
        let store_clone = self.store.clone();
        let bus_handle = tokio::spawn(async move {
            if let Err(e) = event_bus(rx, adapters, store_clone, shutdown_clone).await {
                tracing::error!("Event bus failed: {}", e);
            }
        });

        #[cfg(feature = "http-producer")]
        {
            let producer = HttpProducer::new(self.tx.clone(), self.http_config.port);
            producer.start().await?;
        }

        tokio::select! {
            result = bus_handle => {
                result.map_err(Error::JoinError)?;
                Ok(())
            },
            _ = self.shutdown.cancelled() => {
                tracing::info!("System shutdown triggered");
                Ok(())
            }
        }
    }

    /// Sends an event to the notification system.
    /// This method is used to send events to the event bus.
    pub async fn send_event(&self, event: Event) -> Result<(), Error> {
        self.tx
            .send(event)
            .await
            .map_err(|e| Error::ChannelSend(Box::new(e)))?;
        Ok(())
    }

    /// Shuts down the notification system.
    /// This method is used to cancel the event bus and producer tasks.
    pub fn shutdown(&self) {
        self.shutdown.cancel();
    }

    /// Sets the HTTP port for the notification system.
    /// This method is used to change the port for the HTTP producer.
    #[cfg(feature = "http-producer")]
    pub fn set_http_port(&mut self, port: u16) {
        self.http_config.port = port;
    }
}
