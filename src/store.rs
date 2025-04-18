use crate::event::Event;
use chrono::Utc;
use parking_lot::RwLock;
use std::sync::Arc;
use tokio::fs::{create_dir_all, File, OpenOptions};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};

pub struct EventStore {
    path: String,
    lock: Arc<RwLock<()>>,
}

impl EventStore {
    pub async fn new(path: &str) -> anyhow::Result<Self> {
        create_dir_all(path).await?;
        Ok(Self {
            path: path.to_string(),
            lock: Arc::new(RwLock::new(())),
        })
    }

    pub async fn save_events(&self, events: &[Event]) -> anyhow::Result<()> {
        let _guard = self.lock.write();
        let file_path = format!("{}/events_{}.jsonl", self.path, Utc::now().timestamp());
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&file_path)
            .await?;
        let mut writer = BufWriter::new(file);
        for event in events {
            let line = serde_json::to_string(event)?;
            writer.write_all(line.as_bytes()).await?;
            writer.write_all(b"\n").await?;
        }
        writer.flush().await?;
        Ok(())
    }

    pub async fn load_events(&self) -> anyhow::Result<Vec<Event>> {
        let _guard = self.lock.read();
        let mut events = Vec::new();
        let mut entries = tokio::fs::read_dir(&self.path).await?;
        while let Some(entry) = entries.next_entry().await? {
            let file = File::open(entry.path()).await?;
            let reader = BufReader::new(file);
            let mut lines = reader.lines();
            while let Some(line) = lines.next_line().await? {
                let event: Event = serde_json::from_str(&line)?;
                events.push(event);
            }
        }
        Ok(events)
    }
}
