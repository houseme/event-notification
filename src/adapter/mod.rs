use crate::event::Event;
use async_trait::async_trait;

#[cfg(feature = "kafka")]
pub mod kafka;
#[cfg(feature = "mqtt")]
pub mod mqtt;
pub mod webhook;

#[async_trait]
pub trait ChannelAdapter: Send + Sync {
    fn name(&self) -> String;
    async fn send(&self, event: &Event) -> anyhow::Result<()>;
}
