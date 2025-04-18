use event_notification::NotificationSystem;
use event_notification::create_adapters;
use event_notification::{AdapterConfig, NotificationConfig, WebhookConfig};
use event_notification::{Bucket, Event, Identity, Metadata, Name, Object, Source};
use std::collections::HashMap;
use std::error;
use std::sync::Arc;
use tokio::signal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    tracing_subscriber::fmt::init();

    let mut config = NotificationConfig {
        store_path: "./events".to_string(),
        channel_capacity: 100,
        adapters: vec![AdapterConfig::Webhook(WebhookConfig {
            endpoint: "http://localhost:8080/webhook".to_string(),
            auth_token: Some("secret-token".to_string()),
            custom_headers: Some(HashMap::from([(
                "X-Custom".to_string(),
                "value".to_string(),
            )])),
            max_retries: 3,
            timeout: 10,
        })],
        http: Default::default(),
    };
    config.http.port = 8080;

    // loading configuration from specific env files
    let _config = NotificationConfig::from_env_file(".env.example")?;

    // loading from a specific file
    let _config = NotificationConfig::from_file("event.toml")?;

    // Automatically load from multiple sources (Priority: Environment Variables > YAML > TOML)
    let _config = NotificationConfig::load()?;

    let system = Arc::new(tokio::sync::Mutex::new(
        NotificationSystem::new(config.clone()).await?,
    ));
    let adapters = create_adapters(&config.adapters)?;

    // create an s3 metadata object
    let metadata = Metadata {
        schema_version: "1.0".to_string(),
        configuration_id: "test-config".to_string(),
        bucket: Bucket {
            name: "my-bucket".to_string(),
            owner_identity: Identity {
                principal_id: "owner123".to_string(),
            },
            arn: "arn:aws:s3:::my-bucket".to_string(),
        },
        object: Object {
            key: "test.txt".to_string(),
            size: Some(1024),
            etag: Some("abc123".to_string()),
            content_type: Some("text/plain".to_string()),
            user_metadata: None,
            version_id: None,
            sequencer: "1234567890".to_string(),
        },
    };

    // create source object
    let source = Source {
        host: "localhost".to_string(),
        port: "80".to_string(),
        user_agent: "curl/7.68.0".to_string(),
    };

    // create events using builder mode
    let event = Event::builder()
        .event_time("2023-10-01T12:00:00.000Z")
        .event_name(Name::ObjectCreatedPut)
        .user_identity(Identity {
            principal_id: "user123".to_string(),
        })
        .s3(metadata)
        .source(source)
        .channels(vec!["webhook".to_string()])
        .build()
        .expect("failed to create event");

    {
        let system = system.lock().await;
        system.send_event(event).await?;
    }

    let system_clone = Arc::clone(&system);
    let system_handle = tokio::spawn(async move {
        let mut system = system_clone.lock().await;
        system.start(adapters).await
    });

    signal::ctrl_c().await?;
    tracing::info!("Received shutdown signal");
    {
        let system = system.lock().await;
        system.shutdown();
    }

    system_handle.await??;
    Ok(())
}
