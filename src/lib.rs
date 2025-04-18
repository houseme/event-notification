pub mod adapter;
mod bus;
pub mod event;
mod producer;
mod store;

use adapter::ChannelAdapter;
use event::Event;
use std::sync::Arc;
use store::EventStore;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

pub struct NotificationSystem {
    tx: mpsc::Sender<Event>,
    rx: mpsc::Receiver<Event>,
    store: Arc<EventStore>,
    shutdown: CancellationToken,
}

impl NotificationSystem {
    pub async fn new(store_path: &str) -> anyhow::Result<Self> {
        let (tx, rx) = mpsc::channel::<Event>(100);
        let store = Arc::new(EventStore::new(store_path).await?);
        let shutdown = CancellationToken::new();

        // Load persisted events
        let restored_events = store.load_events().await?;
        for event in restored_events {
            tx.send(event).await?;
        }

        Ok(Self {
            tx,
            rx,
            store,
            shutdown,
        })
    }

    pub async fn start(self, adapters: Vec<Arc<dyn ChannelAdapter>>) -> anyhow::Result<()> {
        let bus_handle = {
            let rx = self.rx;
            let adapters = adapters;
            let store = self.store.clone();
            let shutdown = self.shutdown.clone();
            tokio::spawn(async move { bus::event_bus(rx, adapters, store, shutdown).await })
        };
        let producer_handle = tokio::spawn(producer::start_producer(self.tx.clone()));

        if let Err(e) = bus_handle.await {
            anyhow::bail!("Event bus task failed: {}", e);
        }
        if let Err(e) = producer_handle.await? {
            anyhow::bail!("Producer task failed: {}", e);
        }
        Ok(())
    }

    pub fn shutdown(&self) {
        self.shutdown.cancel();
    }
}
