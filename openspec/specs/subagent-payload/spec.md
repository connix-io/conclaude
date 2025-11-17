# subagent-payload Specification

## Purpose
TBD - created by archiving change add-subagent-stop-payload-fields. Update Purpose after archive.
## Requirements
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

