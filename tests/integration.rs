use event_notification::adapter::{webhook::WebhookAdapter, ChannelAdapter};
use event_notification::event::{Event, WebhookConfig};
use serde_json::json;

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
    let event = Event::new("test", json!({"key": "value"}), vec!["webhook".to_string()]);

    // Simulate a failed request
    let result = adapter.send(&event).await;
    assert!(result.is_err());
}
