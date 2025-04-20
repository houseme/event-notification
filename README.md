# Event Notification

<div align="center">

[English](./README.md) | [简体中文](./README-zh.md)

A modular event notification system with multi-channel support for Rust applications.

[![Crates.io](https://img.shields.io/crates/v/event-notification.svg)](https://crates.io/crates/event-notification)
[![Docs.rs](https://docs.rs/event-notification/badge.svg)](https://docs.rs/event-notification)
[![License](https://img.shields.io/badge/license-Apache%202.0%20or%20MIT-blue.svg)](LICENSE-APACHE)
[![Rust](https://github.com/houseme/event-notification/workflows/Rust/badge.svg)](https://github.com/houseme/event-notification/actions)

</div>

## Features

- Modular notification system with pluggable channel adapters
- Supports multiple delivery channels (Webhook, Kafka, MQTT)
- Asynchronous event processing with Tokio
- Simple global initialization pattern for cross-crate usage
- Event persistence and history management
- Flexible configuration options

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
event-notification = "0.4.0"
```

Enable specific adapters with features:

```toml
[dependencies]
event-notification = { version = "0.4.0", features = ["webhook", "kafka", "mqtt"] }
```

## Quick Start

```rust
use event_notification::{initialize, send_event, start, Event, Identity, Name, NotificationConfig};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 1. Configure the notification system
    let config = NotificationConfig {
        store_path: "./events".to_string(),
        channel_capacity: 100,
        adapters: vec![
            // Configure your adapters here
        ],
    };

    // 2. Initialize the global notification system
    initialize(config.clone()).await?;

    // 3. Start the system with adapters
    let adapters = event_notification::create_adapters(&config.adapters)?;
    start(adapters).await?;

    // 4. Send events from anywhere in your application
    let event = Event::builder()
        .event_time("2023-10-01T12:00:00.000Z")
        .event_name(Name::ObjectCreatedPut)
        .user_identity(Identity {
            principal_id: "user123".to_string(),
        })
        .build()?;

    send_event(event).await?;

    // 5. Shutdown gracefully when done
    tokio::signal::ctrl_c().await?;
    event_notification::shutdown()?;

    Ok(())
}
```

## Supported Adapters

### Webhook

Send events to HTTP endpoints:

```rust
use event_notification::{AdapterConfig, WebhookConfig};

let webhook_config = WebhookConfig {
endpoint: "https://example.com/webhook".to_string(),
headers: Some(vec![
    ("Authorization".to_string(), "Bearer token123".to_string()),
    ("Content-Type".to_string(), "application/json".to_string()),
]),
timeout_ms: Some(5000),
};

let adapter_config = AdapterConfig::Webhook(webhook_config);
```

### Kafka

Publish events to Kafka topics:

```rust
use event_notification::{AdapterConfig, KafkaConfig};

let kafka_config = KafkaConfig {
bootstrap_servers: "localhost:9092".to_string(),
topic: "notifications".to_string(),
client_id: Some("my-app".to_string()),
};

let adapter_config = AdapterConfig::Kafka(kafka_config);
```

### MQTT

Publish events to MQTT topics:

```rust
use event_notification::{AdapterConfig, MqttConfig};

let mqtt_config = MqttConfig {
broker_url: "mqtt://localhost:1883".to_string(),
client_id: "event-system".to_string(),
topic: "events".to_string(),
qos: 1,
};

let adapter_config = AdapterConfig::Mqtt(mqtt_config);
```

## Using Across Multiple Crates

Initialize once in your main application:

```rust
// main.rs or lib.rs
event_notification::initialize(config).await?;
let adapters = event_notification::create_adapters( & config.adapters) ?;
event_notification::start(adapters).await?;
```

Then use from any other crate:

```rust
// any other module or crate
use event_notification::{send_event, Event};

pub async fn process() -> Result<(), Box<dyn std::error::Error>> {
    let event = Event::builder()
        // Configure event
        .build()?;

    send_event(event).await?;
    Ok(())
}
```

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as
defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.