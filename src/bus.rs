use crate::adapter::ChannelAdapter;
use crate::event::Event;
use crate::store::EventStore;
use std::sync::Arc;
use tokio::sync::mpsc::Receiver;
use tokio_util::sync::CancellationToken;

pub async fn event_bus(
    rx: Receiver<Event>,
    adapters: Vec<Arc<dyn ChannelAdapter>>,
    store: Arc<EventStore>,
    shutdown: CancellationToken,
) -> anyhow::Result<()> {
    let mut pending_events = Vec::new();
    let mut rx = rx;
    loop {
        tokio::select! {
            Some(event) = rx.recv() => {
                pending_events.push(event.clone());
                for adapter in &adapters {
                    if event.channels.contains(&adapter.name()) {
                        if let Err(e) = adapter.send(&event).await {
                            tracing::error!("Failed to send event to {}: {}", adapter.name(), e);
                        } else {
                            pending_events.retain(|e| e.id != event.id);
                        }
                    }
                }
            }
            _ = shutdown.cancelled() => {
                tracing::info!("Shutting down event bus, saving pending events...");
                store.save_events(&pending_events).await?;
                break;
            }
            else => break,
        }
    }
    Ok(())
}
