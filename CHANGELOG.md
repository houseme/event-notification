# CHANGELOG

## [Unreleased] 0.4.2

### Dependencies

- Migrated from `dotenv` to `dotenvy` 0.15.7 for improved .env file handling
    - Better error messages for malformed .env files
    - Improved performance and memory usage
    - Maintained backward compatibility with existing .env files

### Engineering

- Updated documentation to reflect the dependency change
- Ensured all environment variable loading paths now use dotenvy

## 0.4.1 (2025-04-20)

### Enhancements

- Improved test methodology:
    - Added mock adapters for webhook testing
    - Enhanced test_notification_system with dependency injection
    - Optimized test timeout handling to prevent hanging tests
- Improved documentation:
    - Added comprehensive comments in global.rs
    - Enhanced code examples in documentation

### Bug Fixes

- Fixed webhook adapter timeout handling in high concurrency scenarios
- Corrected configuration validation error messages
- Fixed potential deadlock in notification system shutdown process

### Engineering

- Increased test coverage for configuration loading edge cases
- Refactored test utilities for better reusability
- Added CI pipeline for testing across multiple platforms

## 0.4.0 (2025-04-20)

### Fixed

- Fixed HttpProducer to properly handle asynchronous event handling with clones
- Fixed Kafka adapter by adding proper import for FromClientConfig and solving
  temporary value drop issues
- Added KafkaRetryExceeded error variant for better retry failure handling
- Fixed retry mechanisms to avoid using moved futures in loops
- Fixed error display implementations for new error types

## 0.3.0 (2025-04-20)

### Added

- `HttpProducerConfig` struct for HTTP producer configuration
- `set_http_port` method for runtime port configuration
- Support for HTTP port configuration via config file

### Changed

- Unified event sending through `EventProducer::send_event`
- Refactored HTTP event handling to use `send_event` method
- Updated examples and tests to demonstrate port configuration

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