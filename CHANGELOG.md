# CHANGELOG

## [Unreleased] 0.3.0

## 0.2.0 (2025-04-19)

### Features

- Added validation for `WebhookConfig` to ensure non-empty values
- Added default values for `NotificationConfig`:
    - `store_path` defaults to system temporary directory
    - `channel_capacity` defaults to 10000 (recommended for high concurrency)
- Enhanced configuration system:
    - Support for loading configuration from TOML, YAML, and environment variables
    - Support for loading from `.env` files, including environment variable formats for all adapters

### Improvements

- Improved configuration validation to ensure all required parameters are valid
- Adjusted configuration loading priority: Environment Variables > YAML > TOML

### Engineering

- Refactored configuration loading mechanism using the `figment` library for multi-format support
- Added `dotenv` support for `.env` file configuration

## 0.1.0 (2025-04-19)

### Features

- Added core functionality for a modular event notification system
- Implemented multiple notification channels:
    - Webhook adapter (enabled by default)
    - Kafka adapter (optional)
    - MQTT adapter (optional)
- Initial configuration system:
    - Support for configuring adapters through code
    - Feature flags to control adapter availability
- Event system implementation:
    - Builder pattern for events
    - Asynchronous event processing
    - Event persistence storage

### Engineering

- Feature flags for optional adapters
- Complete example code
- Asynchronous implementation based on Tokio