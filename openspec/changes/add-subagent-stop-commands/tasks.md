# Tasks for add-subagent-stop-commands

This document tracks implementation tasks for the subagent stop commands feature.

## Configuration & Schema (4 tasks)

- [ ] Add `SubagentStopCommand` struct to `src/config.rs` with fields: run, message, showStdout, showStderr, maxOutputLines
- [ ] Add `SubagentStopConfig` struct to `src/config.rs` with HashMap<String, Vec<SubagentStopCommand>>
- [ ] Add `subagent_stop` field to `ConclaudeConfig` struct with #[serde(rename = "subagentStop")]
- [ ] Verify schema validation accepts subagentStop configuration with test YAML

## Transcript Parsing (3 tasks)

- [ ] Implement `extract_subagent_name_from_transcript(path: &str) -> Result<String>` in `src/hooks.rs`
- [ ] Add JSONL parsing logic to find most recent `Task` tool invocation with subagent_type
- [ ] Add fallback to "unknown" when subagent name cannot be determined

## Environment Variable Setup (2 tasks)

- [ ] Create `build_subagent_env_vars(name: &str, payload: &SubagentStopPayload) -> HashMap<String, String>` helper
- [ ] Add env vars: CONCLAUDE_SUBAGENT_NAME, CONCLAUDE_SESSION_ID, CONCLAUDE_TRANSCRIPT_PATH, CONCLAUDE_HOOK_EVENT, CONCLAUDE_CWD

## Pattern Matching (4 tasks)

- [ ] Implement `match_subagent_patterns(name: &str, config: &SubagentStopConfig) -> Vec<&str>` function
- [ ] Add wildcard (`*`) matching logic with priority handling
- [ ] Add exact match logic
- [ ] Add glob pattern matching using `glob::Pattern::matches`

## Command Execution (5 tasks)

- [ ] Modify `handle_subagent_stop()` to load `subagentStop` config section
- [ ] Extract subagent name from transcript using parsing function
- [ ] Match subagent name against configured patterns
- [ ] Execute wildcard commands first (if configured)
- [ ] Execute specific/glob pattern matched commands second

## Command Runner Integration (4 tasks)

- [ ] Create `execute_subagent_stop_commands()` function similar to `execute_stop_commands()`
- [ ] Pass environment variables to command execution
- [ ] Respect showStdout, showStderr, maxOutputLines settings
- [ ] Handle command failures gracefully without blocking SubagentStop completion

## Testing - Unit Tests (8 tasks)

- [ ] Test `extract_subagent_name_from_transcript()` with valid transcript containing subagent invocation
- [ ] Test `extract_subagent_name_from_transcript()` with transcript missing subagent data (returns "unknown")
- [ ] Test `match_subagent_patterns()` with wildcard pattern (`*` matches all)
- [ ] Test `match_subagent_patterns()` with exact match (`coder` matches only `coder`)
- [ ] Test `match_subagent_patterns()` with prefix glob (`test*` matches `tester`, `test-runner`)
- [ ] Test `match_subagent_patterns()` with suffix glob (`*coder` matches `coder`, `auto-coder`)
- [ ] Test `match_subagent_patterns()` with character class glob (`agent_[0-9]*` matches `agent_1`, `agent_2x`)
- [ ] Test `build_subagent_env_vars()` includes all expected environment variables

## Testing - Integration Tests (5 tasks)

- [ ] Create test YAML config with subagentStop section
- [ ] Test command execution with wildcard pattern
- [ ] Test command execution with exact match pattern
- [ ] Test command execution with glob pattern
- [ ] Test both wildcard and specific commands execute when both match

## Testing - End-to-End Tests (3 tasks)

- [ ] Create mock SubagentStop payload with transcript file
- [ ] Test full hook execution flow: parse → match → execute
- [ ] Verify environment variables available in executed commands

## Documentation (4 tasks)

- [ ] Add subagentStop configuration examples to README or docs
- [ ] Document glob pattern syntax and matching behavior
- [ ] Document environment variables available in subagent commands
- [ ] Add troubleshooting guide for transcript parsing issues

## Validation & Polish (3 tasks)

- [ ] Run `cargo clippy` and fix all warnings
- [ ] Run `cargo test` and verify all tests pass
- [ ] Run `openspec validate add-subagent-stop-commands --strict` and fix issues

---

**Total: 45 tasks**

**Status:** Not started (0/45 complete)
