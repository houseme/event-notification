pub mod adapter;
pub mod bus;
pub mod config;
pub mod error;
pub mod event;
pub mod producer;
pub mod store;

use adapter::ChannelAdapter;
use config::NotificationConfig;
use error::Error;
use event::Event;
use std::sync::Arc;
use store::EventStore;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

pub struct NotificationSystem {
    tx: mpsc::Sender<Event>,
    rx: Option<mpsc::Receiver<Event>>,
    store: Arc<EventStore>,
    shutdown: CancellationToken,
}

impl NotificationSystem {
    pub async fn new(config: NotificationConfig) -> Result<Self, Error> {
        let (tx, rx) = mpsc::channel::<Event>(config.channel_capacity);
        let store = Arc::new(EventStore::new(&config.store_path).await?);
        let shutdown = CancellationToken::new();

        let restored_logs = store.load_logs().await?;
        for log in restored_logs {
            for event in log.records {
                tx.send(event).await?;
            }
        }

        Ok(Self {
            tx,
            rx: Some(rx),
            store,
            shutdown,
        })
    }

    pub async fn start(&mut self, adapters: Vec<Arc<dyn ChannelAdapter>>) -> Result<(), Error> {
        let rx = self.rx.take().ok_or_else(|| Error::EventBusStarted)?;

        let shutdown_clone = self.shutdown.clone();
        let store_clone = self.store.clone();
        let bus_handle = tokio::spawn(async move {
            if let Err(e) = bus::event_bus(rx, adapters, store_clone, shutdown_clone).await {
                tracing::error!("Event bus failed: {}", e);
            }
        });

        let producer_handle = tokio::spawn(async {
            if let Err(e) = producer::start_producer(self.tx.clone()).await {
                tracing::error!("Producer failed: {}", e);
            }
        });
        tokio::select! {
            result = bus_handle => {
                result.map_err(Error::JoinError)?;
                Ok(())
            },
            result = producer_handle => {
               result.map_err(Error::JoinError)?;
                Ok(())
            },
            _ = self.shutdown.cancelled() => {
                tracing::info!("System shutdown triggered");
                Ok(())
            }
        }
    }

    pub async fn send_event(&self, event: Event) -> Result<(), Error> {
        self.tx.send(event).await?;
        Ok(())
    }

    pub fn shutdown(&self) {
        self.shutdown.cancel();
    }
}
