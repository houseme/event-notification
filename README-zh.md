# 事件通知系统

<div align="center">

[English](./README.md) | [简体中文](./README-zh.md)

一个支持多通道的模块化事件通知系统，专为 Rust 应用设计。

[![Crates.io](https://img.shields.io/crates/v/event-notification.svg)](https://crates.io/crates/event-notification)
[![Docs.rs](https://docs.rs/event-notification/badge.svg)](https://docs.rs/event-notification)
[![License](https://img.shields.io/badge/license-Apache%202.0%20or%20MIT-blue.svg)](LICENSE-APACHE)
[![Rust](https://github.com/houseme/event-notification/workflows/Rust/badge.svg)](https://github.com/houseme/event-notification/actions)

</div>

## 特性

- 模块化通知系统，支持可插拔式通道适配器
- 支持多种传输通道（Webhook、Kafka、MQTT）
- 基于 Tokio 的异步事件处理
- 简单的全局初始化模式，便于跨 crate 使用
- 事件持久化和历史记录管理
- 灵活的配置选项

## 安装

在 `Cargo.toml` 中添加：

```toml
[dependencies]
event-notification = "0.1.0"
```

启用特定适配器功能：

```toml
[dependencies]
event-notification = { version = "0.1.0", features = ["webhook", "kafka", "mqtt"] }
```

## 快速开始

```rust
use event_notification::{initialize, send_event, start, Event, Identity, Name, NotificationConfig};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 1. 配置通知系统
    let config = NotificationConfig {
        store_path: "./events".to_string(),
        channel_capacity: 100,
        adapters: vec![
            // 在此配置适配器
        ],
    };

    // 2. 初始化全局通知系统
    initialize(config.clone()).await?;

    // 3. 启动系统和适配器
    let adapters = event_notification::create_adapters(&config.adapters)?;
    start(adapters).await?;

    // 4. 在应用程序的任何位置发送事件
    let event = Event::builder()
        .event_time("2023-10-01T12:00:00.000Z")
        .event_name(Name::ObjectCreatedPut)
        .user_identity(Identity {
            principal_id: "user123".to_string(),
        })
        .build()?;

    send_event(event).await?;

    // 5. 优雅关闭
    tokio::signal::ctrl_c().await?;
    event_notification::shutdown()?;

    Ok(())
}
```

## 支持的适配器

### Webhook

发送事件到 HTTP 端点：

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

发布事件到 Kafka 主题：

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

发布事件到 MQTT 主题：

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

## 跨多个 Crate 使用

在主应用程序中初始化一次：

```rust
// main.rs 或 lib.rs
event_notification::initialize(config).await?;
let adapters = event_notification::create_adapters( & config.adapters) ?;
event_notification::start(adapters).await?;
```

然后从任何其他 crate 中使用：

```rust
// 任何其他模块或 crate
use event_notification::{send_event, Event};

pub async fn process() -> Result<(), Box<dyn std::error::Error>> {
    let event = Event::builder()
        // 配置事件
        .build()?;

    send_event(event).await?;
    Ok(())
}
```

## 许可证

本项目基于 [Apache License, Version 2.0](LICENSE-APACHE) 或 [MIT license](LICENSE-MIT) 授权，可任选其一。

除非您另有明确声明，否则您对本项目提交的任何贡献（按 Apache-2.0 许可证定义）均应按上述方式进行双重许可，不附加任何其他条款或条件。
