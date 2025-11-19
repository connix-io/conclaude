# Tasks for add-subagent-stop-commands

This document tracks implementation tasks for the subagent stop commands feature.

## Configuration & Schema (4 tasks)

- [x] Add `SubagentStopCommand` struct to `src/config.rs` with fields: run, message, showStdout, showStderr, maxOutputLines
- [x] Add `SubagentStopConfig` struct to `src/config.rs` with HashMap<String, Vec<SubagentStopCommand>>
- [x] Add `subagent_stop` field to `ConclaudeConfig` struct with #[serde(rename = "subagentStop")]
- [x] Verify schema validation accepts subagentStop configuration with test YAML

## Agent ID Usage (2 tasks)

- [x] Use `agent_id` field from SubagentStopPayload (provided by add-subagent-stop-payload-fields)
- [x] Add validation for subagentStop maxOutputLines in validate_config_constraints

## Environment Variable Setup (2 tasks)

- [x] Create `build_subagent_env_vars(name: &str, payload: &SubagentStopPayload) -> HashMap<String, String>` helper
- [x] Add env vars: CONCLAUDE_AGENT_ID, CONCLAUDE_AGENT_TRANSCRIPT_PATH, CONCLAUDE_SESSION_ID, CONCLAUDE_TRANSCRIPT_PATH, CONCLAUDE_HOOK_EVENT, CONCLAUDE_CWD

## Pattern Matching (4 tasks)

- [x] Implement `match_subagent_patterns(agent_id: &str, config: &SubagentStopConfig) -> Vec<&String>` function
- [x] Add wildcard (`*`) matching logic with priority handling
- [x] Add exact match logic
- [x] Add glob pattern matching using `glob::Pattern::matches`

## Command Execution (4 tasks)

- [x] Modify `handle_subagent_stop()` to load `subagentStop` config section
- [x] Use agent_id from payload directly (no transcript parsing needed)
- [x] Match agent_id against configured patterns
- [x] Execute matched commands in order (wildcard first, then specific patterns)

## Command Runner Integration (4 tasks)

- [x] Create `execute_subagent_stop_commands()` function similar to `execute_stop_commands()`
- [x] Pass environment variables to command execution
- [x] Respect showStdout, showStderr, maxOutputLines settings
- [x] Handle command failures gracefully without blocking SubagentStop completion

## Testing - Unit Tests (8 tasks)

- [x] Test `match_subagent_patterns()` with wildcard pattern (`*` matches all)
- [x] Test `match_subagent_patterns()` with exact match (`coder` matches only `coder`)
- [x] Test `match_subagent_patterns()` with prefix glob (`test*` matches `tester`, `test-runner`)
- [x] Test `match_subagent_patterns()` with suffix glob (`*coder` matches `coder`, `auto-coder`)
- [x] Test `match_subagent_patterns()` with character class glob (`agent_[0-9]*` matches `agent_1`, `agent_2x`)
- [x] Test `match_subagent_patterns()` with both wildcard and specific patterns
- [x] Test `build_subagent_env_vars()` includes all expected environment variables (AGENT_ID, AGENT_TRANSCRIPT_PATH, SESSION_ID, etc.)
- [x] Test `collect_subagent_stop_commands()` correctly collects commands for matching patterns

## Testing - Integration Tests (2 tasks)

- [x] Verify schema validation accepts subagentStop configuration
- [x] Verify maxOutputLines validation for subagentStop commands

## Validation & Polish (3 tasks)

- [x] Run `cargo clippy` and fix all warnings
- [x] Run `cargo test` and verify all tests pass
- [x] Update error messages to include subagentStop in field suggestions

---

**Total: 33 tasks**

**Status:** Complete (33/33 complete)
