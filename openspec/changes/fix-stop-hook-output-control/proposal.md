# Proposal: Fix Stop Hook Output Control Settings

## Why

The `showStdout` and `showStderr` settings in stop hook commands are not being respected during command execution. Currently, when a stop command fails, the implementation unconditionally logs the full stdout and stderr to the console via `eprintln!()` (lines 657-681 in `src/hooks.rs`), regardless of the `showStdout: false` and `showStderr: false` configuration settings.

This violates the existing specification in `openspec/specs/execution/spec.md` (lines 63-79), which explicitly states:

> **Requirement: Output Limiting with showStdout and showStderr**
> The system SHALL apply output limiting only when the respective output stream is being shown to the user or Claude.
>
> **Scenario: maxOutputLines with showStdout disabled**
> - **WHEN** maxOutputLines is configured but showStdout is false
> - **THEN** stdout SHALL NOT be shown to the user or Claude
> - **AND** maxOutputLines SHALL have no effect on stdout

Users who configure `showStdout: false` and `showStderr: false` expect their terminal output to be clean, with no command output leaked. This is particularly important for:
- Commands that produce verbose debugging output
- Commands with sensitive information in their output
- Clean CI/CD pipeline logs

## What Changes

- Modify `execute_stop_commands()` in `src/hooks.rs` to respect `showStdout` and `showStderr` settings when logging failures via `eprintln!()`
- Only include stdout in the eprintln diagnostic output when `showStdout` is true
- Only include stderr in the eprintln diagnostic output when `showStderr` is true
- Add integration tests that verify stdout/stderr are NOT printed to console when the respective flags are false
- Update existing unit tests if needed to cover the eprintln behavior

## Impact

- **Affected specs**: `execution` (already contains the requirements this fixes)
- **Affected code**:
  - `src/hooks.rs` (execute_stop_commands function, lines 657-681)
  - Tests in `tests/output_limiting_tests.rs` (need integration tests)
- **Breaking changes**: None - this fixes broken functionality to match the documented behavior
- **Dependencies**: None - purely fixes existing code
