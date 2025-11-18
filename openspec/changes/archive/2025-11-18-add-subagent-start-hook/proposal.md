# Proposal: SubagentStart Hook Support

**Change ID:** `add-subagent-start-hook`
**Status:** Proposal
**Author:** Claude Code
**Date:** 2025-11-17

## Executive Summary

Add support for the new `SubagentStart` hook event that Claude Code now fires when subagents begin execution. This mirrors the existing `SubagentStop` hook and provides symmetry in the subagent lifecycle, enabling initialization, logging, and setup operations when subagents start their tasks.

**Key benefits:**
- Track when subagents begin work, not just when they complete
- Run initialization or setup commands when specific subagents start
- Full lifecycle visibility (SubagentStart + SubagentStop) for all subagents
- Environment preparation and context loading for subagent execution
- Symmetric hook coverage matching Claude Code's event model

## Why

Claude Code recently added the `SubagentStart` hook event to provide visibility into when subagents begin their work. conclaude currently only supports `SubagentStop`, leaving users unable to hook into the start of subagent execution for initialization, logging, or environment setup.

## Motivation

### Current State
conclaude supports the following hook events:
- `SessionStart` / `SessionEnd` - Main session lifecycle
- `SubagentStop` - When subagents complete
- `PreToolUse` / `PostToolUse` - Tool execution lifecycle
- `UserPromptSubmit`, `Notification`, `PreCompact`, `Stop`

The `SubagentStop` hook is fully supported with payload fields including `agent_id` and `agent_transcript_path`. However, there is no counterpart for when subagents **begin** execution.

### User Need
Users need visibility and control at the **start** of subagent execution to:

1. **Initialize environment** - Set up context, load configuration specific to the subagent type
2. **Log subagent starts** - Track when each subagent begins for monitoring and debugging
3. **Prepare resources** - Allocate resources, create directories, or initialize state
4. **Send notifications** - Alert users when critical subagents (like `tester` or `coder`) begin work
5. **Symmetric lifecycle tracking** - Match starts with stops for complete subagent observability

### Claude Code Update
Claude Code recently added the `SubagentStart` hook event. Based on the pattern of existing hooks and parallel to `SubagentStop`, the payload likely includes:
- Standard base fields (`session_id`, `transcript_path`, `hook_event_name`, `cwd`, `permission_mode`)
- Subagent-specific fields (`agent_id`, `subagent_type`, potentially `agent_transcript_path`)

## Dependencies

**None** - This is an independent additive feature.

**Relates To:**
- `add-subagent-stop-payload-fields` - Provided the pattern for subagent payload structure with `agent_id` and `agent_transcript_path`
- `add-subagent-stop-commands` - Provides configuration pattern for subagent-specific commands

## What Changes

1. **Type System** - Add `SubagentStartPayload` struct to `src/types.rs`
2. **Hook Handler** - Add `handle_subagent_start()` function to `src/hooks.rs`
3. **Main Entry Point** - Add `SubagentStart` command to CLI in `src/main.rs`
4. **Payload Validation** - Add validation function for `SubagentStartPayload` fields
5. **Enum Update** - Add `SubagentStart` variant to `HookPayload` enum
6. **System Event Classification** - Add "SubagentStart" to `is_system_event_hook()` for notification support
7. **Environment Variables** - Export `CONCLAUDE_AGENT_ID` and `CONCLAUDE_SUBAGENT_TYPE` when hook executes
8. **Documentation** - Update README with SubagentStart payload structure and usage examples
9. **Tests** - Add unit and integration tests for SubagentStart hook

## Scope

### What's Included

**Type Definitions:**
- `SubagentStartPayload` struct with fields:
  - Base payload fields (session_id, transcript_path, hook_event_name, cwd, permission_mode)
  - `agent_id: String` - Identifier for the subagent starting (e.g., "coder", "tester")
  - `subagent_type: String` - Type/category of the subagent
  - `agent_transcript_path: String` - Path to subagent's transcript file

**Hook Handler:**
- Read and deserialize payload from stdin
- Validate all required fields
- Log subagent start event
- Export environment variables
- Send system notification (if configured)
- Return success result

**Validation:**
- Ensure `agent_id` is non-empty
- Ensure `subagent_type` is non-empty
- Ensure `agent_transcript_path` is non-empty
- Validate base payload fields

**CLI Integration:**
- `conclaude SubagentStart` command
- Accepts JSON payload via stdin
- Returns exit code 0 on success, 1 on error, 2 on blocked

**Notifications:**
- Support SubagentStart in notifications config
- Enable notifications via `hooks: ["SubagentStart"]` or `hooks: ["*"]`
- Include agent_id and subagent_type in notification context

### What's NOT Included

- Subagent start command execution (no `subagentStart` config section for running commands)
  - This could be added in a future proposal if needed, similar to `add-subagent-stop-commands`
- Changes to existing hook behaviors
- Blocking or controlling subagent execution
- Subagent lifecycle events beyond start (pause, resume, etc.)

## Questions & Decisions

### Q: What fields should SubagentStartPayload include?
**Decision:** Mirror SubagentStop structure with fields appropriate for start context:
- `agent_id: String` - Identifier for the subagent (required, non-empty)
- `subagent_type: String` - Type of subagent (e.g., "coder", "tester", "stuck")
- `agent_transcript_path: String` - Path to agent's transcript file
- Base fields: session_id, transcript_path, hook_event_name, cwd, permission_mode

This provides symmetry with SubagentStop and gives users the information needed for initialization.

### Q: Should we infer field names or wait for official documentation?
**Decision:** Use field names based on established patterns:
- `agent_id` - Already used in SubagentStop, confirmed pattern
- `subagent_type` - Logical descriptor of the agent type
- `agent_transcript_path` - Already used in SubagentStop

If Claude Code uses different field names, we'll update in a follow-up migration.

### Q: Should SubagentStart support command execution like SubagentStop?
**Decision:** Not in this initial proposal. Start with notification support only.

**Rationale:**
- Keep initial implementation focused and minimal
- Users can add command execution in a follow-up proposal (parallel to `add-subagent-stop-commands`)
- Notification support provides value for monitoring without added complexity

### Q: Which environment variables should be exported?
**Decision:** Export subagent context similar to SubagentStop:
- `CONCLAUDE_AGENT_ID` - The subagent identifier
- `CONCLAUDE_SUBAGENT_TYPE` - The type of subagent
- `CONCLAUDE_AGENT_TRANSCRIPT_PATH` - Path to agent's transcript
- Standard context: `CONCLAUDE_SESSION_ID`, `CONCLAUDE_TRANSCRIPT_PATH`, `CONCLAUDE_CWD`, `CONCLAUDE_HOOK_EVENT`

### Q: Should this be treated as a system event hook?
**Decision:** Yes, add "SubagentStart" to `is_system_event_hook()`.

**Rationale:**
- Mirrors SessionStart classification
- Represents lifecycle event, not tool execution
- Users expect system event notifications for subagent lifecycle

## Success Criteria

1. **Payload deserialization works** - SubagentStartPayload correctly deserializes from JSON stdin
2. **Validation enforces required fields** - Empty or missing agent_id/subagent_type causes validation error
3. **Handler executes successfully** - `handle_subagent_start()` processes valid payloads without error
4. **CLI command available** - `conclaude SubagentStart` command exists and processes input
5. **Environment variables exported** - Commands can access CONCLAUDE_AGENT_ID and CONCLAUDE_SUBAGENT_TYPE
6. **Notifications work** - System notifications fire when SubagentStart is in notifications.hooks config
7. **Tests pass** - Unit tests for payload validation, integration tests for handler

## Risks & Mitigations

| Risk | Impact | Likelihood | Mitigation |
|------|--------|------------|------------|
| Claude Code uses different field names | High | Medium | Use established patterns (agent_id from SubagentStop); update if needed |
| Missing fields in actual payload | High | Low | Validation fails gracefully with clear error messages |
| Notification spam from many subagents | Medium | Medium | Users control via notifications.hooks configuration |
| Environment variable conflicts | Low | Low | Use CONCLAUDE_ prefix to avoid collisions |

## Migration Path

**For existing users:**
- No changes required
- SubagentStart hook is new, purely additive
- No breaking changes to existing configurations or hooks

**For users wanting SubagentStart notifications:**
- Add "SubagentStart" to `notifications.hooks` array in `.conclaude.yaml`
- Or use `hooks: ["*"]` to include all hooks

**Example configuration:**
```yaml
notifications:
  enabled: true
  hooks: ["SessionStart", "SubagentStart", "SubagentStop", "Stop"]
  showSystemEvents: true
```

## Alternatives Considered

### Alternative 1: Wait for official Claude Code documentation
**Rejected:** The hook event was added but not yet documented. Based on established patterns (SubagentStop, SessionStart), we can implement with high confidence and update if needed.

### Alternative 2: Include command execution in initial implementation
**Rejected:** Keep initial scope minimal. Command execution can be added in follow-up proposal similar to `add-subagent-stop-commands`.

### Alternative 3: Make subagent_type optional
**Rejected:** Type information is valuable for routing and logging. Require it to ensure users have necessary context.

## Related Work

- **add-subagent-stop-payload-fields** - Established pattern for agent_id and agent_transcript_path fields
- **add-subagent-stop-commands** - Future pattern for SubagentStart command execution (if needed)
- Existing SessionStart hook - Provides pattern for "start" lifecycle hook implementation

## Implementation Notes

**Key files to modify:**
- `src/types.rs` - Add `SubagentStartPayload` struct and validation function
- `src/hooks.rs` - Add `handle_subagent_start()` function and update `is_system_event_hook()`
- `src/main.rs` - Add `SubagentStart` CLI command variant
- `tests/hooks_tests.rs` - Add tests for SubagentStart payload validation and handler
- `README.md` - Document SubagentStart hook, payload structure, and usage

**Testing approach:**
- Unit tests for `validate_subagent_start_payload()` with valid/invalid inputs
- Unit tests for payload field validation (empty agent_id, missing subagent_type, etc.)
- Integration test for `handle_subagent_start()` with mock payload
- Manual testing with `echo '{}' | conclaude SubagentStart`
- Verify environment variables are set correctly

**Example test payload:**
```json
{
  "session_id": "test-session-123",
  "transcript_path": "/tmp/main_transcript.jsonl",
  "hook_event_name": "SubagentStart",
  "cwd": "/home/user/project",
  "permission_mode": "default",
  "agent_id": "coder",
  "subagent_type": "coder",
  "agent_transcript_path": "/tmp/agent_coder_transcript.jsonl"
}
```
