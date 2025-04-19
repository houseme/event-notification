use crate::Error;
use crate::Event;
use async_trait::async_trait;

/// event producer characteristics
#[allow(dead_code)]
#[async_trait]
pub trait EventProducer: Send + Sync {
    /// start producer services
    async fn start(&self) -> Result<(), Error>;
    /// stop producer services
    async fn stop(&self) -> Result<(), Error>;
    /// send a single event
    async fn send_event(&self, event: Event) -> Result<(), Error>;
}

#[cfg(feature = "http-producer")]
pub mod http {
    use super::*;
    use axum::{Json, Router, routing::post};

    pub struct HttpProducer {
        tx: mpsc::Sender<Event>,
        port: u16,
        shutdown: Arc<tokio::sync::Notify>,
    }

    impl HttpProducer {
        pub fn new(tx: mpsc::Sender<Event>, port: u16) -> Self {
            Self {
                tx,
                port,
                shutdown: Arc::new(tokio::sync::Notify::new()),
            }
        }
    }

    #[async_trait]
    impl EventProducer for HttpProducer {
        async fn start(&self) -> Result<(), Error> {
            let producer = Arc::new(self.clone());
            let app = Router::new().route(
                "/event",
                post(|event| handle_event(event, producer.clone())),
            );

            let addr = format!("0.0.0.0:{}", self.port);
            let listener = tokio::net::TcpListener::bind(&addr).await?;

            let shutdown = self.shutdown.clone();
            tokio::select! {
                result = axum::serve(listener, app) => {
                    result?;
                    Ok(())
                }
                _ = shutdown.notified() => Ok(())
            }
        }

        async fn stop(&self) -> Result<(), Error> {
            self.shutdown.notify_one();
            Ok(())
        }

        async fn send_event(&self, event: Event) -> Result<(), Error> {
            self.tx
                .send(event)
                .await
                .map_err(|e| Error::ChannelSend(Box::new(e)))?;
            Ok(())
        }
    }

    async fn handle_event(
        Json(event): Json<Event>,
        producer: Arc<HttpProducer>,
    ) -> Result<(), axum::http::StatusCode> {
        producer
            .send_event(event)
            .await
            .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)
    }
}
