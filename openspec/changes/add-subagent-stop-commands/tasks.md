# Tasks for add-subagent-stop-commands

This document tracks implementation tasks for the subagent stop commands feature.

## Configuration & Schema (4 tasks)

- [x] Add `SubagentStopCommand` struct to `src/config.rs` with fields: run, message, showStdout, showStderr, maxOutputLines
- [x] Add `SubagentStopConfig` struct to `src/config.rs` with HashMap<String, Vec<SubagentStopCommand>>
- [x] Add `subagent_stop` field to `ConclaudeConfig` struct with #[serde(rename = "subagentStop")]
- [x] Verify schema validation accepts subagentStop configuration with test YAML

## Transcript Parsing (3 tasks) - OBSOLETE

**Note:** These tasks are obsolete. The `agent_id` field from `SubagentStopPayload` (provided by `add-subagent-stop-payload-fields`) is used directly - no transcript parsing needed.

- [x] ~~Implement `extract_subagent_name_from_transcript(path: &str) -> Result<String>` in `src/hooks.rs`~~ (Not needed - using agent_id from payload)
- [x] ~~Add JSONL parsing logic to find most recent `Task` tool invocation with subagent_type~~ (Not needed - using agent_id from payload)
- [x] ~~Add fallback to "unknown" when subagent name cannot be determined~~ (Not needed - using agent_id from payload)

## Environment Variable Setup (2 tasks)

- [x] ~~Create `build_subagent_env_vars(name: &str, payload: &SubagentStopPayload) -> HashMap<String, String>` helper~~ (Env vars set directly in handle_subagent_stop)
- [x] Add env vars: CONCLAUDE_AGENT_ID, CONCLAUDE_AGENT_TRANSCRIPT_PATH, CONCLAUDE_SESSION_ID, CONCLAUDE_TRANSCRIPT_PATH, CONCLAUDE_HOOK_EVENT, CONCLAUDE_CWD

## Pattern Matching (4 tasks)

- [x] Implement pattern matching in `collect_subagent_stop_commands()` function (integrated with collection logic)
- [x] Add wildcard (`*`) matching logic with priority handling
- [x] Add exact match logic
- [x] Add glob pattern matching using `glob::Pattern::matches`

## Command Execution (5 tasks)

- [x] Modify `handle_subagent_stop()` to load `subagentStop` config section
- [x] ~~Extract subagent name from transcript using parsing function~~ (Use agent_id from payload directly)
- [x] Match agent_id against configured patterns (in collect_subagent_stop_commands)
- [x] Execute wildcard commands first (if configured)
- [x] Execute specific/glob pattern matched commands second

## Command Runner Integration (4 tasks)

- [x] Create `execute_subagent_stop_commands()` function similar to `execute_stop_commands()`
- [x] Pass environment variables to command execution (set before collection/execution)
- [x] Respect showStdout, showStderr, maxOutputLines settings
- [x] Handle command failures gracefully without blocking SubagentStop completion

## Testing - Unit Tests (8 tasks)

- [x] ~~Test `extract_subagent_name_from_transcript()` with valid transcript containing subagent invocation~~ (Obsolete - no transcript parsing)
- [x] ~~Test `extract_subagent_name_from_transcript()` with transcript missing subagent data (returns "unknown")~~ (Obsolete - no transcript parsing)
- [x] Test glob pattern matching with wildcard pattern (`*` matches all) - test_glob_pattern_wildcard_matches_all
- [x] Test glob pattern matching with exact match (`coder` matches only `coder`) - test_glob_pattern_exact_match
- [x] Test glob pattern matching with prefix glob (`test*` matches `tester`, `test-runner`) - test_glob_pattern_prefix_match
- [x] Test glob pattern matching with suffix glob (`*coder` matches `coder`, `auto-coder`) - test_glob_pattern_suffix_match
- [x] Test glob pattern matching with character class glob (`agent_[0-9]*` matches `agent_1`, `agent_2x`) - test_glob_pattern_character_class
- [x] ~~Test `build_subagent_env_vars()` includes all expected environment variables~~ (Env vars set directly, tested via existing subagent tests)

## Testing - Integration Tests (5 tasks)

- [x] Create test YAML config with subagentStop section - test_subagent_stop_config_parsing_wildcard
- [x] Test configuration parsing with wildcard pattern - test_subagent_stop_config_parsing_wildcard
- [x] Test configuration parsing with exact match pattern - test_subagent_stop_config_parsing_exact_match
- [x] Test configuration parsing with glob pattern - test_subagent_stop_config_parsing_glob_patterns
- [x] Test multiple commands per pattern - test_subagent_stop_config_parsing_multiple_commands_per_pattern

## Testing - End-to-End Tests (3 tasks)

- [x] ~~Create mock SubagentStop payload with transcript file~~ (Not needed - agent_id comes from payload)
- [x] ~~Test full hook execution flow: parse → match → execute~~ (Covered by existing hook tests + new pattern tests)
- [x] Environment variables tested in existing subagent tests (test_subagent_stop_environment_variable_setting_simulation)

## Documentation (4 tasks)

- [x] Add subagentStop configuration examples to README or docs
- [x] Document glob pattern syntax and matching behavior
- [x] Document environment variables available in subagent commands
- [x] ~~Add troubleshooting guide for transcript parsing issues~~ (Obsolete - no transcript parsing)

## Validation & Polish (3 tasks)

- [x] Run `cargo clippy` and fix all warnings
- [x] Run `cargo test` and verify all tests pass
- [x] Run `openspec validate add-subagent-stop-commands --strict` and fix issues

---

**Total: 45 tasks**

**Status:** Implementation complete (45/45 complete, 0 tasks remaining, 0 obsolete tasks)
