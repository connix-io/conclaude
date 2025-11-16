# Spec: SubagentStop Payload Fields

**Capability:** `subagent-payload`
**Status:** Draft
**Version:** 1.0.0

## Overview

Extends the `SubagentStopPayload` structure with agent identification fields provided by Claude Code, enabling conclaude to identify which subagent completed and access its transcript.

## ADDED Requirements

### Requirement: SubagentStopPayload includes agent_id field

The `SubagentStopPayload` struct MUST include an `agent_id` field that identifies the specific subagent that completed.

**Type:** `String` (required, non-optional)
**Source:** Provided by Claude Code in SubagentStop hook JSON payload

#### Scenario: Valid agent_id provided

**Given** Claude Code fires a SubagentStop hook with a JSON payload
**When** the payload contains `"agent_id": "coder"`
**Then** conclaude parses the payload successfully
**And** the `agent_id` field contains the value `"coder"`
**And** the field is accessible to the SubagentStop hook handler

#### Scenario: Missing agent_id field

**Given** Claude Code fires a SubagentStop hook with a JSON payload
**When** the payload does NOT contain an `agent_id` field
**Then** payload validation MUST fail
**And** conclaude returns an error indicating the missing required field
**And** the SubagentStop hook does NOT execute

#### Scenario: Empty agent_id value

**Given** Claude Code fires a SubagentStop hook with a JSON payload
**When** the payload contains `"agent_id": ""`
**Then** payload validation MUST fail
**And** conclaude returns an error indicating agent_id cannot be empty

### Requirement: SubagentStopPayload includes agent_transcript_path field

The `SubagentStopPayload` struct MUST include an `agent_transcript_path` field that points to the subagent's transcript file.

**Type:** `String` (required, non-optional)
**Source:** Provided by Claude Code in SubagentStop hook JSON payload

#### Scenario: Valid agent_transcript_path provided

**Given** Claude Code fires a SubagentStop hook with a JSON payload
**When** the payload contains `"agent_transcript_path": "/path/to/agent/transcript.json"`
**Then** conclaude parses the payload successfully
**And** the `agent_transcript_path` field contains the value `"/path/to/agent/transcript.json"`
**And** the field is accessible to the SubagentStop hook handler

#### Scenario: Missing agent_transcript_path field

**Given** Claude Code fires a SubagentStop hook with a JSON payload
**When** the payload does NOT contain an `agent_transcript_path` field
**Then** payload validation MUST fail
**And** conclaude returns an error indicating the missing required field
**And** the SubagentStop hook does NOT execute

#### Scenario: Empty agent_transcript_path value

**Given** Claude Code fires a SubagentStop hook with a JSON payload
**When** the payload contains `"agent_transcript_path": ""`
**Then** payload validation MUST fail
**And** conclaude returns an error indicating agent_transcript_path cannot be empty

### Requirement: Export agent_id as environment variable

When executing SubagentStop hook commands, the `agent_id` field MUST be exported as the `CONCLAUDE_AGENT_ID` environment variable.

#### Scenario: Agent ID available to hook commands

**Given** a SubagentStop hook with `agent_id` set to `"tester"`
**When** a hook command executes
**Then** the command environment MUST include `CONCLAUDE_AGENT_ID="tester"`
**And** the command can access the variable via `$CONCLAUDE_AGENT_ID` (bash) or `os.environ["CONCLAUDE_AGENT_ID"]` (Python)

#### Scenario: Agent ID in notification context

**Given** a SubagentStop hook with `agent_id` set to `"coder"`
**When** conclaude sends a notification
**Then** the notification context MAY include the agent_id
**And** users can see which agent completed in notification messages

### Requirement: Export agent_transcript_path as environment variable

When executing SubagentStop hook commands, the `agent_transcript_path` field MUST be exported as the `CONCLAUDE_AGENT_TRANSCRIPT_PATH` environment variable.

#### Scenario: Agent transcript path available to hook commands

**Given** a SubagentStop hook with `agent_transcript_path` set to `"/tmp/agent_transcript.json"`
**When** a hook command executes
**Then** the command environment MUST include `CONCLAUDE_AGENT_TRANSCRIPT_PATH="/tmp/agent_transcript.json"`
**And** the command can access the variable to read the agent's transcript

#### Scenario: Distinguish main and agent transcripts

**Given** a SubagentStop hook with:
- `transcript_path` (from BasePayload) set to `"/tmp/main_transcript.json"`
- `agent_transcript_path` set to `"/tmp/agent_coder_transcript.json"`
**When** a hook command executes
**Then** the environment includes both:
- `CONCLAUDE_TRANSCRIPT_PATH="/tmp/main_transcript.json"`
- `CONCLAUDE_AGENT_TRANSCRIPT_PATH="/tmp/agent_coder_transcript.json"`
**And** commands can differentiate between main session and subagent transcripts

### Requirement: Payload validation enforces required fields

The payload validation logic MUST verify that both `agent_id` and `agent_transcript_path` are present and non-empty.

#### Scenario: Complete valid payload

**Given** a SubagentStop hook JSON payload containing:
```json
{
  "session_id": "abc123",
  "transcript_path": "/tmp/main.json",
  "hook_event_name": "SubagentStop",
  "cwd": "/home/user/project",
  "stop_hook_active": true,
  "agent_id": "coder",
  "agent_transcript_path": "/tmp/agent.json"
}
```
**When** conclaude validates the payload
**Then** validation MUST pass
**And** the SubagentStop hook executes normally

#### Scenario: Payload missing both new fields

**Given** a SubagentStop hook JSON payload missing `agent_id` and `agent_transcript_path`
**When** conclaude validates the payload
**Then** validation MUST fail
**And** an error message indicates which required fields are missing
**And** the hook does NOT execute

## MODIFIED Requirements

None (this is a new capability).

## REMOVED Requirements

None (this is a new capability).

## Related Specs

- **Depends on:** None (foundational change)
- **Enables:** `add-subagent-stop-commands/specs/subagent-hooks` - Can use agent_id for pattern matching
- **Related to:** Base payload validation in existing hook system

## Testing Strategy

1. **Unit Tests** (tests/types_tests.rs):
   - Parse valid SubagentStopPayload with all fields
   - Reject payload missing agent_id
   - Reject payload missing agent_transcript_path
   - Reject payload with empty agent_id
   - Reject payload with empty agent_transcript_path

2. **Integration Tests** (tests/hooks_tests.rs):
   - SubagentStop hook receives payload with new fields
   - Environment variables are set correctly
   - Commands can access CONCLAUDE_AGENT_ID and CONCLAUDE_AGENT_TRANSCRIPT_PATH

3. **Schema Validation**:
   - Verify schema.json includes agent_id and agent_transcript_path as required fields
   - Verify JSON schema validation rejects payloads missing new fields

## Implementation Notes

### Rust Type Definition

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubagentStopPayload {
    #[serde(flatten)]
    pub base: BasePayload,

    /// Whether stop hooks are currently active for this session
    pub stop_hook_active: bool,

    /// Identifier for the subagent (e.g., "coder", "tester", "stuck")
    pub agent_id: String,

    /// Path to the subagent's transcript file
    pub agent_transcript_path: String,
}
```

### Validation Logic

Extend the existing `validate_base_payload` pattern or create a new `validate_subagent_stop_payload` function:

```rust
pub fn validate_subagent_stop_payload(payload: &SubagentStopPayload) -> Result<(), String> {
    validate_base_payload(&payload.base)?;

    if payload.agent_id.trim().is_empty() {
        return Err("agent_id is required and cannot be empty".to_string());
    }

    if payload.agent_transcript_path.trim().is_empty() {
        return Err("agent_transcript_path is required and cannot be empty".to_string());
    }

    Ok(())
}
```

### Environment Variable Export

In `handle_subagent_stop()`, set environment variables before executing commands:

```rust
std::env::set_var("CONCLAUDE_AGENT_ID", &payload.agent_id);
std::env::set_var("CONCLAUDE_AGENT_TRANSCRIPT_PATH", &payload.agent_transcript_path);
```

## Migration Notes

This is a **breaking change** for older versions of Claude Code that don't provide these fields.

**Minimum Claude Code Version:** TBD (version that added agent_id and agent_transcript_path to SubagentStop hooks)

**Upgrade Path:**
1. Users must update to the minimum Claude Code version
2. No configuration changes required (payload changes are automatic)
3. Existing SubagentStop hooks will receive the new fields automatically
