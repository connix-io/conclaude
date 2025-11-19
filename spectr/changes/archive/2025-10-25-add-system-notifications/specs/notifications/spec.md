# System Notifications Capability

## ADDED Requirements

### Requirement: System Notification Configuration

The system SHALL provide configuration options for enabling and customizing system notifications.

#### Scenario: Default configuration disables notifications

- **WHEN** no notification configuration is specified
- **THEN** the system SHALL NOT send any system notifications

#### Scenario: Enable notifications globally

- **WHEN** `notifications.enabled` is set to `true` in configuration
- **THEN** the system SHALL send notifications for all configured hooks

#### Scenario: Configure per-hook notifications

- **WHEN** `notifications.hooks` array specifies hook names (e.g., `["Stop", "PreToolUse"]`)
- **THEN** the system SHALL only send notifications for the specified hooks
- **AND** SHALL NOT send notifications for hooks not in the array

#### Scenario: All hooks notification

- **WHEN** `notifications.hooks` contains the wildcard `"*"`
- **THEN** the system SHALL send notifications for all hook types

### Requirement: Notification Content

The system SHALL send informative notification messages that include relevant context about hook execution.

#### Scenario: Successful hook execution notification

- **WHEN** a configured hook executes successfully
- **THEN** the notification title SHALL be "Conclaude - [HookName]"
- **AND** the notification body SHALL indicate success with brief context (e.g., "All checks passed")

#### Scenario: Failed hook execution notification

- **WHEN** a configured hook fails
- **THEN** the notification title SHALL be "Conclaude - [HookName] Failed"
- **AND** the notification body SHALL indicate the failure reason or context

#### Scenario: Stop hook with validation details

- **WHEN** the Stop hook runs validation commands
- **THEN** the notification SHALL indicate the validation status (e.g., "Tests passed", "Build failed")

### Requirement: Graceful Notification Failure Handling

The system SHALL handle notification failures gracefully without impacting hook execution.

#### Scenario: Notification library unavailable

- **WHEN** system notifications are not supported on the platform
- **THEN** the system SHALL log a warning message
- **AND** SHALL continue hook execution normally
- **AND** SHALL NOT return an error status

#### Scenario: Notification send failure

- **WHEN** sending a notification fails for any reason
- **THEN** the system SHALL log the error
- **AND** SHALL continue hook execution normally
- **AND** SHALL NOT block the hook from completing

### Requirement: Configuration Schema

The system SHALL include notification configuration in the YAML schema and default configuration.

#### Scenario: Default configuration file includes notifications section

- **WHEN** generating a default configuration with `conclaude init`
- **THEN** the configuration SHALL include a `notifications` section
- **AND** SHALL set `enabled` to `false` by default
- **AND** SHALL include commented examples showing available options

#### Scenario: Configuration validation

- **WHEN** loading configuration with notifications section
- **THEN** the system SHALL validate that `enabled` is a boolean
- **AND** SHALL validate that `hooks` is an array of strings (if provided)
- **AND** SHALL reject invalid hook names with a helpful error message

### Requirement: Notification Timing

The system SHALL send notifications at appropriate times during hook execution.

#### Scenario: Notification sent after hook completion

- **WHEN** a configured hook completes execution
- **THEN** the system SHALL send the notification after the hook result is determined
- **AND** SHALL NOT delay the hook response to Claude Code

#### Scenario: No notification spam

- **WHEN** multiple hooks execute in rapid succession
- **THEN** each hook SHALL generate at most one notification
- **AND** the system SHALL NOT batch or suppress notifications
