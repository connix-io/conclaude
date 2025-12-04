# Change: Add Interactive TUI Mode to `conclaude init`

## Why

The current `conclaude init` command generates a complete default configuration without user input. Users must manually edit the YAML file afterward to customize settings. An interactive TUI mode allows guided configuration during initialization, reducing time-to-first-working-config and exposing available options to new users.

## What Changes

- Add `--interactive` / `-i` flag to `conclaude init` command
- Implement ratatui-based TUI with step-by-step configuration wizard
- TUI covers all major settings: core protections, notifications, subagent patterns, and tool validation rules
- Generated `.conclaude.yaml` reflects user selections from TUI
- Silent mode (current behavior) remains the default

## Impact

- **Affected specs**: `cli-init` (new capability spec)
- **Affected code**: `src/main.rs` (Init command handler), new `src/tui/` module
- **New dependencies**: `ratatui`, `crossterm`
- **No breaking changes**: Default behavior preserved; interactive mode is opt-in via flag
