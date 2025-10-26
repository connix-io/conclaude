# Notification System Capability

## ADDED Requirements

### Requirement: Enhanced NotificationsConfig

The system SHALL provide enhanced notification configuration options including show_errors, show_success, and show_system_events boolean fields to control different types of notifications.

#### Scenario: User configures notification system

- **WHEN** the user has a .conclaude.yaml configuration file with `notifications.enabled: true`
- **THEN** they SHALL receive desktop notifications for configured events
- **AND** they SHALL be able to control which types of notifications they receive through the new configuration fields

#### Scenario: User configures specific notification types

- **WHEN** they set `notifications.show_errors: true` and `notifications.show_success: false`
- **THEN** they SHALL only receive error notifications
- **AND** they SHALL NOT receive success notifications

### Requirement: Notification System Replacement

The system SHALL completely remove all file-based logging infrastructure and replace it with desktop notifications for key events.

#### Scenario: Application starts without logging

- **WHEN** conclaude initializes
- **THEN** it SHALL NOT create any log files
- **AND** it SHALL NOT initialize env_logger
- **AND** it SHALL NOT use the log crate

### Requirement: Hook Execution Notifications

The system SHALL send desktop notifications for hook execution start, success, and failure events when notifications are enabled for the specific hook type.

#### Scenario: Hook starts execution

- **WHEN** a hook is about to execute and notifications are enabled for this hook type
- **THEN** a desktop notification SHALL appear with title "Conclaude" and body "Starting {hook_name}..."

#### Scenario: Hook completes successfully

- **WHEN** a hook has finished execution without errors and `notifications.show_success` is true
- **THEN** a desktop notification SHALL appear with title "Conclaude" and body "{hook_name} completed successfully"

#### Scenario: Hook execution fails

- **WHEN** a hook encounters an error during execution and `notifications.show_errors` is true
- **THEN** a desktop notification SHALL appear with title "Conclaude Error" and body "Hook failed: {error_details}"

### Requirement: System Event Notifications

The system SHALL send desktop notifications for system events like session start/end and configuration loading when system event notifications are enabled.

#### Scenario: Configuration loaded successfully

- **WHEN** conclaude loads a configuration file successfully and `notifications.show_system_events` is true
- **THEN** a desktop notification SHALL appear with title "Conclaude" and body "Configuration loaded from {file_path}"

#### Scenario: Session starts

- **WHEN** a new Claude session begins and `notifications.show_system_events` is true
- **THEN** a desktop notification SHALL appear with title "Conclaude" and body "Session started"

### Requirement: Graceful Notification Failure

The system SHALL handle notification failures gracefully and continue normal operation without blocking hook execution.

#### Scenario: Notification system unavailable

- **WHEN** the desktop notification system is not available and conclaude tries to send a notification
- **THEN** the notification SHALL fail silently
- **AND** the application SHALL continue normal execution
- **AND** an error message SHALL be written to stderr for debugging purposes

### Requirement: Configuration Error Notifications

The system SHALL attempt to send desktop notifications for configuration-related errors when the notification system is available.

#### Scenario: Configuration file has errors

- **WHEN** conclaude tries to load a configuration file and the file contains syntax errors
- **THEN** a desktop notification SHALL appear with title "Configuration Error" and body "Failed to parse config: {error_details}"
- **AND** if notification system is unavailable, the error SHALL be printed to stderr

## MODIFIED Requirements

### Requirement: Default Configuration Updates

The system SHALL provide a default configuration without logging options and with enhanced notification settings.

#### Scenario: Generate default configuration

- **WHEN** a user runs `conclaude init`
- **THEN** the generated configuration SHALL NOT contain any logging configuration
- **AND** it SHALL contain the enhanced notifications configuration with appropriate defaults
- **AND** file_logging SHALL NOT be mentioned anywhere in the configuration

### Requirement: Hook Event Processing Updates

The system SHALL use desktop notifications for all hook event status updates instead of writing to log files.

#### Scenario: Process hook event

- **WHEN** a hook event is received and processed
- **THEN** status updates SHALL be sent as notifications instead of log entries
- **AND** no log entries SHALL be written
- **AND** error conditions SHALL trigger error notifications

## REMOVED Requirements

### Requirement: Log File Creation Removal

The system SHALL NOT create any log files in temporary directories during operation.

#### Scenario: Application runs with session

- **WHEN** conclaude processes a session with session_id "abc123"
- **THEN** no log file SHALL be created in /tmp/
- **AND** no conclaude-*.jsonl files SHALL be generated

### Requirement: LoggingConfig Struct Removal

The system SHALL NOT contain any LoggingConfig struct or references to it in the codebase.

#### Scenario: Type system validation

- **WHEN** the codebase is compiled
- **THEN** LoggingConfig SHALL NOT exist in the type system
- **AND** no references to LoggingConfig SHALL remain in the code
- **AND** file_logging configuration option SHALL NOT exist

### Requirement: Logging Dependencies Removal

The system SHALL NOT depend on log or env_logger crates and SHALL not use any log macros.

#### Scenario: Dependency check

- **WHEN** Cargo.toml dependencies are examined
- **THEN** log crate SHALL NOT be present
- **AND** env_logger crate SHALL NOT be present
- **AND** no `log::` macro calls SHALL exist in the codebase