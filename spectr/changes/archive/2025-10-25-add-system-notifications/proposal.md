# Add System Notifications for Command Execution

## Why

Users running conclaude need better visibility into when hooks are executing, especially during long-running validation tasks or when working in different windows. System notifications provide immediate, non-intrusive feedback about conclaude's status without requiring the user to monitor terminal output.

## What Changes

- Add opt-in system notification support using the `notifica` Rust crate
- Add configuration option `notifications.enabled` to enable/disable notifications
- Add configuration option `notifications.hooks` to specify which hooks trigger notifications
- Send system notifications when configured hooks execute, showing:
  - Hook name (e.g., "Stop", "PreToolUse")
  - Execution status (success/failure)
  - Brief context (e.g., "All checks passed", "Command failed")
- Gracefully handle notification failures (log but don't block hook execution)

## Impact

- **Affected specs**: New capability - `notifications`
- **Affected code**:
  - `Cargo.toml` - Add `notifica` dependency
  - `src/config.rs` - Add `NotificationsConfig` struct
  - `src/hooks.rs` - Add notification sending logic to hook handlers
  - `src/default-config.yaml` - Add notifications section with defaults
- **Breaking changes**: None - feature is opt-in with `enabled: false` by default
- **User experience**: Improved visibility and feedback when notifications are enabled
