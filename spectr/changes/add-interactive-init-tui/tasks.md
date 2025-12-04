# Tasks: Add Interactive TUI Mode to `conclaude init`

## 1. Setup

- [ ] 1.1 Add `ratatui` and `crossterm` dependencies to `Cargo.toml`
- [ ] 1.2 Create `src/tui/mod.rs` module structure with submodules for screens, state, widgets

## 2. Core TUI Infrastructure

- [ ] 2.1 Implement terminal setup/teardown with crossterm (raw mode, alternate screen)
- [ ] 2.2 Create `TuiState` struct mirroring `ConclaudeConfig` fields
- [ ] 2.3 Implement main event loop handling keyboard input
- [ ] 2.4 Create `Screen` enum and navigation logic (next, previous, skip to review)

## 3. Wizard Screens

- [ ] 3.1 Welcome screen with navigation instructions
- [ ] 3.2 Core Protections screen (`preventRootAdditions`, `preventGeneratedFileEdits`, `preventUpdateGitIgnored`)
- [ ] 3.3 Uneditable Files screen with list widget (add/remove patterns)
- [ ] 3.4 Stop Hook screen (commands list, `infinite` toggle, `rounds` input)
- [ ] 3.5 Notifications screen (enabled toggle, hook selector)
- [ ] 3.6 Subagent Hooks screen (pattern-to-commands mapping, advanced)
- [ ] 3.7 Review screen showing config summary with confirm/edit options

## 4. CLI Integration

- [ ] 4.1 Add `--interactive` / `-i` flag to `Commands::Init` in `main.rs`
- [ ] 4.2 Branch `handle_init` to call TUI wizard when flag is present
- [ ] 4.3 Generate YAML from TUI state using `serde_yaml`
- [ ] 4.4 Write generated config to file path (respecting `--config-path` and `--force` flags)

## 5. Testing

- [ ] 5.1 Unit tests for `TuiState` initialization and field defaults
- [ ] 5.2 Unit tests for config serialization from TUI state
- [ ] 5.3 Integration test: run TUI in headless mode with scripted input (if feasible)
- [ ] 5.4 Manual testing on Linux, macOS, Windows terminals

## 6. Documentation

- [ ] 6.1 Update README with `--interactive` flag usage example
- [ ] 6.2 Add `conclaude init --help` description for interactive mode
- [ ] 6.3 Update JSON schema if new fields are introduced
