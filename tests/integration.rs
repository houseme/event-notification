use event_notification::adapter::{webhook::WebhookAdapter, ChannelAdapter};
use event_notification::config::WebhookConfig;
use event_notification::event::{Bucket, Event, Identity, Metadata, Name, Object, Source};
use event_notification::NotificationSystem;
use std::collections::HashMap;
use std::sync::Arc;

#[tokio::test]
async fn test_webhook_adapter() {
    let config = WebhookConfig {
        endpoint: "http://localhost:8080/webhook".to_string(),
        auth_token: None,
        custom_headers: None,
        max_retries: 1,
        timeout: 5,
    };
    let adapter = WebhookAdapter::new(config);
    let event = Event::new(
        "2.0",
        "aws:s3",
        "us-east-1",
        "2023-10-01T12:00:00.000Z",
        Name::ObjectCreatedPut,
        Identity {
            principal_id: "user123".to_string(),
        },
        HashMap::new(),
        HashMap::new(),
        Metadata {
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
        },
        Source {
            host: "localhost".to_string(),
            port: "80".to_string(),
            user_agent: "curl/7.68.0".to_string(),
        },
        vec!["webhook".to_string()],
    );

    let result = adapter.send(&event).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_notification_system() {
    let config = event_notification::config::NotificationConfig {
        store_path: "./test_events".to_string(),
        channel_capacity: 100,
        adapters: vec![event_notification::config::AdapterConfig::Webhook(
            WebhookConfig {
                endpoint: "http://localhost:8080/webhook".to_string(),
                auth_token: None,
                custom_headers: None,
                max_retries: 1,
                timeout: 5,
            },
        )],
    };
    let mut system = NotificationSystem::new(config).await.unwrap();
    let adapters: Vec<Arc<dyn ChannelAdapter>> =
        vec![Arc::new(WebhookAdapter::new(WebhookConfig {
            endpoint: "http://localhost:8080/webhook".to_string(),
            auth_token: None,
            custom_headers: None,
            max_retries: 1,
            timeout: 5,
        }))];

    let event = Event::new(
        "2.0",
        "aws:s3",
        "us-east-1",
        "2023-10-01T12:00:00.000Z",
        Name::ObjectCreatedPut,
        Identity {
            principal_id: "user123".to_string(),
        },
        HashMap::new(),
        HashMap::new(),
        Metadata {
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
        },
        Source {
            host: "localhost".to_string(),
            port: "80".to_string(),
            user_agent: "curl/7.68.0".to_string(),
        },
        vec!["webhook".to_string()],
    );
    system.send_event(event).await.unwrap();

    let system_handle = tokio::spawn(async move { system.start(adapters).await });
    system.shutdown();
    assert!(system_handle.await.is_ok());
}
