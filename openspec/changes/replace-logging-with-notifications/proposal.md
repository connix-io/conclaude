# Replace Logging with Notify-Rust Proposal

## Why

The current logging implementation has several critical limitations that impact user experience:

1. **Poor Visibility**: Log files are written to temporary directories that users rarely check or even know exist
2. **Redundant Infrastructure**: Maintaining both console and file logging creates unnecessary complexity
3. **Limited Real-time Feedback**: Users have no immediate indication of hook execution status or errors
4. **Resource Overhead**: File I/O operations create performance overhead and disk usage
5. **Maintenance Burden**: The logging code requires maintenance while providing limited value

Desktop notifications provide immediate, visible feedback that users actually see, eliminating the need for persistent log files while improving the user experience through real-time status updates.

## Summary

This change removes all existing logging and file logging infrastructure from the conclaude codebase and replaces it with desktop notifications using the notify_rust crate. The goal is to provide users with immediate desktop feedback for important events instead of writing to log files.

## Rationale

The current logging implementation has several limitations:
- File logging to temp directories that users rarely check
- Redundant logging with both console and file output
- Limited visibility into hook execution status
- No real-time feedback for users

By replacing this with desktop notifications, users will receive immediate visual feedback about hook events, errors, and important status changes directly on their desktop.

## Files to be Modified

- `src/logger.rs` - Remove entire file
- `src/config.rs` - Remove LoggingConfig references
- `src/types.rs` - Remove LoggingConfig struct
- `src/hooks.rs` - Replace log calls with notify_rust notifications
- `src/main.rs` - Remove logger initialization
- `Cargo.toml` - Remove log and env_logger dependencies
- Various test files - Update to remove logging tests

## Design Approach

1. **Complete Removal**: All existing logging code will be removed
2. **Notification Integration**: Use existing notify_rust dependency for desktop notifications
3. **Event-Driven Notifications**: Send notifications for key events like:
   - Hook execution start/end
   - Errors and failures
   - Important status changes
4. **Configurable Notifications**: Leverage existing NotificationsConfig for control

## Backwards Compatibility

This is a breaking change that removes all logging functionality. Users who relied on log files will need to use the new notification system instead.