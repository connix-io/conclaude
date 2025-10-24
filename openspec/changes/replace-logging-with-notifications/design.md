# Design: Replace Logging with Notify-Rust

## Architecture Overview

This change transforms conclaude from a file-based logging system to a desktop notification system. The design leverages the existing notify_rust crate that's already included in the dependencies.

## Current State Analysis

### Existing Logging Infrastructure
- **File**: `src/logger.rs` (219 lines) - Complete logger implementation
- **Dependencies**: `log = "0.4"`, `env_logger = "0.11"`
- **Configuration**: `LoggingConfig` struct with file_logging boolean
- **Integration**: Logger initialization in main.rs, used throughout hooks.rs

### Existing Notification System
- **Dependency**: `notify-rust = "4.10"` (already present)
- **Current Usage**: System notifications in hooks.rs
- **Configuration**: `NotificationsConfig` struct with enabled flag and hooks list

## Notification Strategy

### Notification Categories

1. **Hook Execution Notifications**
   - Hook start: "Starting {hook_name}..."
   - Hook success: "{hook_name} completed successfully"
   - Hook failure: "Error in {hook_name}: {error_message}"

2. **System Status Notifications**
   - Session start: "Conclaude session started"
   - Session end: "Conclaude session ended"
   - Configuration loaded: "Configuration loaded from {path}"

3. **Error Notifications**
   - Configuration errors: "Configuration error: {details}"
   - Hook execution errors: "Hook failed: {details}"
   - System errors: "System error: {details}"

### Notification Configuration Integration

The existing `NotificationsConfig` will be enhanced to control the new notification system:

```rust
pub struct NotificationsConfig {
    pub enabled: bool,
    pub hooks: Vec<String>,
    // New fields to be added:
    pub show_errors: bool,
    pub show_success: bool,
    pub show_system_events: bool,
}
```

## Implementation Plan

### Phase 1: Remove Logging Infrastructure
1. Delete `src/logger.rs` entirely
2. Remove `LoggingConfig` from `src/types.rs`
3. Remove logging dependencies from `Cargo.toml`
4. Remove logger initialization from `src/main.rs`

### Phase 2: Enhance Notification System
1. Extend `NotificationsConfig` with new notification categories
2. Create notification helper functions in `hooks.rs`
3. Replace all `log::*` calls with appropriate notifications
4. Update configuration schema and defaults

### Phase 3: Update Tests
1. Remove all logging-related tests
2. Add notification tests (mocking notify_rust)
3. Update integration tests to expect notifications instead of logs

## Error Handling

Notifications will use non-blocking sends with error logging to stderr if notification fails. This ensures the application continues functioning even if the notification system is unavailable.

## Benefits

1. **Immediate User Feedback**: Users see notifications in real-time
2. **Reduced File I/O**: No more writing to temporary log files
3. **Simplified Configuration**: Single notification system instead of dual logging/notification
4. **Better User Experience**: Desktop notifications are more visible than log files

## Trade-offs

1. **No Persistent Logs**: Users won't have a written record of events
2. **Dependency on Desktop Environment**: Notifications require a working desktop environment
3. **Limited History**: Notifications are ephemeral and cannot be reviewed later

## Migration Notes

Users who need persistent logging should:
1. Use external system logging tools that capture desktop notifications
2. Modify their hook scripts to provide custom logging if needed
3. Use session management tools that capture notification events