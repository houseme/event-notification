use crate::error::Error;
use crate::event::Event;
use axum::{routing::post, Json, Router};
use tokio::sync::mpsc;

pub async fn handle_event(
    Json(event): Json<Event>,
    tx: mpsc::Sender<Event>,
) -> Result<(), axum::http::StatusCode> {
    tx.send(event)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(())
}

pub async fn start_producer(tx: mpsc::Sender<Event>) -> Result<(), Error> {
    let app = Router::new().route("/event", post(|event| handle_event(event, tx)));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
