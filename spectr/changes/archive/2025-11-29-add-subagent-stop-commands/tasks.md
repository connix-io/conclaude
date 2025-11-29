# Tasks for add-subagent-stop-commands

This document tracks implementation tasks for the subagent stop commands feature.

## Configuration & Schema (4 tasks)

- [x] Add `SubagentStopCommand` struct to `src/config.rs` with fields: run, message, showStdout, showStderr, maxOutputLines
- [x] Add `SubagentStopConfig` struct to `src/config.rs` with HashMap<String, Vec<SubagentStopCommand>>
- [x] Add `subagent_stop` field to `ConclaudeConfig` struct with #[serde(rename = "subagentStop")]
- [x] Add validation for subagentStop configuration (empty patterns, maxOutputLines range)

## Environment Variable Setup (2 tasks)

- [x] Create `build_subagent_env_vars(payload: &SubagentStopPayload) -> HashMap<String, String>` helper
- [x] Add env vars: CONCLAUDE_AGENT_ID, CONCLAUDE_AGENT_TRANSCRIPT_PATH, CONCLAUDE_SESSION_ID, CONCLAUDE_TRANSCRIPT_PATH, CONCLAUDE_HOOK_EVENT, CONCLAUDE_CWD

## Pattern Matching (4 tasks)

- [x] Implement `match_subagent_patterns(agent_id: &str, config: &SubagentStopConfig) -> Result<Vec<&str>>` function
- [x] Add wildcard (`*`) matching logic with priority handling (wildcard first)
- [x] Add exact match logic
- [x] Add glob pattern matching using `glob::Pattern::matches`

## Command Execution (4 tasks)

- [x] Modify `handle_subagent_stop()` to load `subagentStop` config section
- [x] Match agent_id against configured patterns
- [x] Execute wildcard commands first (if configured)
- [x] Execute specific/glob pattern matched commands second

## Command Runner Integration (4 tasks)

- [x] Create `execute_subagent_stop_commands()` function
- [x] Pass environment variables to command execution
- [x] Respect showStdout, showStderr, maxOutputLines settings
- [x] Handle command failures gracefully without blocking SubagentStop completion

## Testing - Unit Tests (8 tasks)

- [x] Test `match_subagent_patterns()` with wildcard pattern (`*` matches all)
- [x] Test `match_subagent_patterns()` with exact match (`coder` matches only `coder`)
- [x] Test `match_subagent_patterns()` with prefix glob (`test*` matches `tester`, `test-runner`)
- [x] Test `match_subagent_patterns()` with suffix glob (`*coder` matches `coder`, `auto-coder`)
- [x] Test `match_subagent_patterns()` with character class glob (`agent_[0-9]*` matches `agent_1`, `agent_2x`)
- [x] Test `match_subagent_patterns()` with multiple pattern matches (wildcard first, then sorted)
- [x] Test `build_subagent_env_vars()` includes all expected environment variables
- [x] Test `collect_subagent_stop_commands()` collects commands in correct order

## Testing - Integration Tests (3 tasks)

- [x] Test command collection with single pattern
- [x] Test command collection with multiple patterns
- [x] Test command collection with no matching patterns

## Validation & Polish (3 tasks)

- [x] Run `cargo clippy` and fix all warnings
- [x] Run `cargo test` and verify all tests pass (116 tests passing)
- [x] Run `spectr validate add-subagent-stop-commands --strict`

---

**Total: 32 tasks**

**Status:** Complete (32/32)
