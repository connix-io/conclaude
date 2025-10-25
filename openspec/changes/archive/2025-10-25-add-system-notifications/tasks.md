# Implementation Tasks

## 1. Add Dependency

- [x] 1.1 Add `notify-rust` crate to `Cargo.toml` dependencies
- [x] 1.2 Run `cargo build` to verify dependency resolution

## 2. Configuration Updates

- [x] 2.1 Add `NotificationsConfig` struct to `src/config.rs` with fields:
  - `enabled: bool` (default: false)
  - `hooks: Vec<String>` (default: empty)
- [x] 2.2 Add `notifications` field to `ConclaudeConfig` struct
- [x] 2.3 Derive `JsonSchema` for `NotificationsConfig` for schema generation
- [x] 2.4 Update `src/default-config.yaml` with notifications section and examples
- [x] 2.5 Add helper method to check if notifications are enabled for a specific hook

## 3. Notification Module

- [x] 3.1 Create notification helper function that accepts hook name and status
- [x] 3.2 Implement notification title formatting: "Conclaude - [HookName]"
- [x] 3.3 Implement notification body formatting based on hook type and status
- [x] 3.4 Add error handling that logs failures but doesn't propagate errors
- [x] 3.5 Add debug logging for notification events

## 4. Hook Integration

- [x] 4.1 Update `handle_stop` in `src/hooks.rs` to send notification on completion
- [x] 4.2 Update `handle_pre_tool_use` to send notification when configured
- [x] 4.3 Update `handle_post_tool_use` to send notification when configured
- [x] 4.4 Update `handle_session_start` to send notification when configured
- [x] 4.5 Update `handle_user_prompt_submit` to send notification when configured
- [x] 4.6 Update `handle_notification` to send notification when configured
- [x] 4.7 Update `handle_subagent_stop` to send notification when configured
- [x] 4.8 Update `handle_pre_compact` to send notification when configured

## 5. Testing

- [x] 5.1 Create integration test for notification configuration loading
- [x] 5.2 Test notification with `enabled: false` (should not send)
- [x] 5.3 Test notification with `enabled: true` and specific hooks
- [x] 5.4 Test notification with `enabled: true` and wildcard `"*"`
- [x] 5.5 Manual testing: Run hooks with notifications enabled and verify system notifications appear
- [x] 5.6 Test graceful degradation when notifications fail

## 6. Documentation

- [x] 6.1 Update README.md with notifications configuration section
- [x] 6.2 Add example configuration showing notification setup
- [x] 6.3 Document supported hook names for `notifications.hooks` array
- [x] 6.4 Regenerate JSON schema with `conclaude generate-schema`

## 7. Validation

- [x] 7.1 Run `cargo fmt` to format all changes
- [x] 7.2 Run `cargo clippy` to check for linting issues
- [x] 7.3 Run `cargo test` to ensure all tests pass
- [x] 7.4 Run `cargo build --release` to verify release build succeeds
- [x] 7.5 Test end-to-end with sample `.conclaude.yaml` enabling notifications
