# subagent-hooks Specification Delta

## ADDED Requirements

### Requirement: SubagentStartPayload type definition

The system MUST define a `SubagentStartPayload` struct that represents the data received when a SubagentStart hook fires.

**Type:** Rust struct with serde Serialize/Deserialize derives
**Location:** `src/types.rs`

**Fields:**
- `base: BasePayload` - Standard hook payload fields (session_id, transcript_path, hook_event_name, cwd, permission_mode)
- `agent_id: String` - Unique identifier for the subagent starting execution (required, non-empty)
- `subagent_type: String` - Type or category of the subagent (required, non-empty)
- `agent_transcript_path: String` - Path to the subagent's transcript file (required, non-empty)

#### Scenario: Valid SubagentStartPayload deserialization

**Given** Claude Code fires a SubagentStart hook with a JSON payload containing:
```json
{
  "session_id": "abc123",
  "transcript_path": "/tmp/main.json",
  "hook_event_name": "SubagentStart",
  "cwd": "/home/user/project",
  "permission_mode": "default",
  "agent_id": "coder",
  "subagent_type": "coder",
  "agent_transcript_path": "/tmp/agent_coder.json"
}
```
**When** conclaude deserializes the payload into `SubagentStartPayload`
**Then** all fields MUST be populated correctly
**And** `base.session_id` equals "abc123"
**And** `agent_id` equals "coder"
**And** `subagent_type` equals "coder"
**And** `agent_transcript_path` equals "/tmp/agent_coder.json"

#### Scenario: Missing agent_id field

**Given** a SubagentStart hook JSON payload without an `agent_id` field
**When** conclaude attempts to deserialize the payload
**Then** deserialization MUST fail with a clear error message
**And** the error indicates that `agent_id` is a required field

#### Scenario: Empty agent_id value

**Given** a SubagentStart hook JSON payload with `"agent_id": ""`
**When** conclaude validates the payload
**Then** validation MUST fail
**And** the error message indicates agent_id cannot be empty

#### Scenario: Missing subagent_type field

**Given** a SubagentStart hook JSON payload without a `subagent_type` field
**When** conclaude attempts to deserialize the payload
**Then** deserialization MUST fail with a clear error message
**And** the error indicates that `subagent_type` is a required field

### Requirement: SubagentStart payload validation function

The system MUST provide a validation function that ensures all required SubagentStartPayload fields are present and non-empty.

**Function signature:** `validate_subagent_start_payload(payload: &SubagentStartPayload) -> Result<(), String>`
**Location:** `src/types.rs`

#### Scenario: Complete valid payload passes validation

**Given** a SubagentStartPayload with all required fields populated
**When** `validate_subagent_start_payload()` is called
**Then** validation MUST return `Ok(())`
**And** no error messages are generated

#### Scenario: Empty agent_id fails validation

**Given** a SubagentStartPayload with `agent_id` set to empty string or whitespace
**When** `validate_subagent_start_payload()` is called
**Then** validation MUST return `Err("agent_id cannot be empty")`

#### Scenario: Empty subagent_type fails validation

**Given** a SubagentStartPayload with `subagent_type` set to empty string or whitespace
**When** `validate_subagent_start_payload()` is called
**Then** validation MUST return `Err("subagent_type cannot be empty")`

#### Scenario: Empty agent_transcript_path fails validation

**Given** a SubagentStartPayload with `agent_transcript_path` set to empty string or whitespace
**When** `validate_subagent_start_payload()` is called
**Then** validation MUST return `Err("agent_transcript_path cannot be empty")`

#### Scenario: Invalid base payload fails validation

**Given** a SubagentStartPayload with invalid base fields (e.g., empty session_id)
**When** `validate_subagent_start_payload()` is called
**Then** validation MUST return an error from `validate_base_payload()`
**And** the SubagentStart-specific validation is NOT executed

### Requirement: SubagentStart hook handler function

The system MUST provide a hook handler function that processes SubagentStart events.

**Function signature:** `async fn handle_subagent_start() -> Result<HookResult>`
**Location:** `src/hooks.rs`

#### Scenario: Successful SubagentStart hook execution

**Given** a valid SubagentStartPayload is provided via stdin
**When** `handle_subagent_start()` is called
**Then** the function MUST:
- Read and deserialize the payload from stdin
- Validate the payload using `validate_subagent_start_payload()`
- Log the event with session_id and agent_id
- Export environment variables (CONCLAUDE_AGENT_ID, CONCLAUDE_SUBAGENT_TYPE, CONCLAUDE_AGENT_TRANSCRIPT_PATH)
- Send a system notification (if configured)
- Return `HookResult::success()`

#### Scenario: Invalid payload causes handler failure

**Given** an invalid SubagentStartPayload (missing agent_id) is provided via stdin
**When** `handle_subagent_start()` is called
**Then** the function MUST return an error
**And** the error message indicates which field is invalid
**And** no environment variables are exported
**And** no notification is sent

#### Scenario: Environment variables exported on success

**Given** a valid SubagentStartPayload with:
- `agent_id` = "tester"
- `subagent_type` = "tester"
- `agent_transcript_path` = "/tmp/tester.json"
**When** `handle_subagent_start()` executes successfully
**Then** the following environment variables MUST be set:
- `CONCLAUDE_AGENT_ID` = "tester"
- `CONCLAUDE_SUBAGENT_TYPE` = "tester"
- `CONCLAUDE_AGENT_TRANSCRIPT_PATH` = "/tmp/tester.json"
**And** standard environment variables are also set (SESSION_ID, TRANSCRIPT_PATH, CWD, HOOK_EVENT)

### Requirement: HookPayload enum includes SubagentStart variant

The `HookPayload` enum MUST include a `SubagentStart` variant for type-safe payload handling.

**Location:** `src/types.rs`

#### Scenario: SubagentStart variant added to enum

**Given** the HookPayload enum in `src/types.rs`
**When** a SubagentStart hook event occurs
**Then** the enum MUST have a variant defined as:
```rust
#[serde(rename = "SubagentStart")]
SubagentStart(SubagentStartPayload)
```
**And** the variant supports serde serialization/deserialization
**And** helper methods (session_id(), transcript_path(), hook_event_name()) work for SubagentStart

### Requirement: CLI command for SubagentStart hook

The CLI MUST provide a `SubagentStart` command that accepts JSON payloads via stdin.

**Command:** `conclaude SubagentStart`
**Location:** `src/main.rs`

#### Scenario: SubagentStart command processes valid input

**Given** a valid SubagentStart JSON payload is piped to stdin
**When** user executes `conclaude SubagentStart`
**Then** the command MUST invoke `handle_subagent_start()`
**And** return exit code 0 on success
**And** return exit code 1 on validation error
**And** return exit code 2 if blocked (though SubagentStart doesn't block)

#### Scenario: SubagentStart command with invalid JSON

**Given** invalid JSON is piped to stdin
**When** user executes `conclaude SubagentStart`
**Then** the command MUST return exit code 1
**And** error message indicates JSON parsing failed

### Requirement: SubagentStart classified as system event hook

The `is_system_event_hook()` function MUST classify "SubagentStart" as a system event hook for notification routing.

**Location:** `src/hooks.rs`

#### Scenario: SubagentStart recognized as system event

**Given** the `is_system_event_hook()` function
**When** called with hook_name "SubagentStart"
**Then** the function MUST return `true`
**And** notifications with `showSystemEvents: true` will fire for SubagentStart

#### Scenario: SubagentStart notifications respect showSystemEvents flag

**Given** notifications config with `showSystemEvents: false`
**When** a SubagentStart hook executes successfully
**Then** no system notification is sent
**And** the hook still completes successfully

### Requirement: Notification support for SubagentStart events

System notifications MUST support SubagentStart hooks when configured.

**Configuration:** `notifications.hooks` array in `.conclaude.yaml`

#### Scenario: SubagentStart in hooks list enables notifications

**Given** notifications config with:
```yaml
notifications:
  enabled: true
  hooks: ["SubagentStart"]
  showSystemEvents: true
```
**When** a SubagentStart hook executes
**Then** a system notification MUST be sent
**And** the notification title includes "SubagentStart"
**And** the notification body includes the agent_id

#### Scenario: Wildcard hooks config includes SubagentStart

**Given** notifications config with `hooks: ["*"]`
**When** a SubagentStart hook executes
**Then** a system notification MUST be sent (subject to showSystemEvents flag)

#### Scenario: SubagentStart not in hooks list suppresses notifications

**Given** notifications config with `hooks: ["Stop", "PreToolUse"]` (SubagentStart not listed)
**When** a SubagentStart hook executes
**Then** no system notification is sent
**And** the hook still executes successfully

### Requirement: SubagentStart logging includes agent context

Hook execution logs MUST include agent_id and subagent_type for debugging and monitoring.

#### Scenario: SubagentStart log message includes agent details

**Given** a SubagentStart payload with agent_id "coder"
**When** `handle_subagent_start()` processes the payload
**Then** the log output MUST include:
- "Processing SubagentStart hook"
- session_id value
- agent_id value
**And** the log format is consistent with other hook handlers

### Requirement: Test coverage for SubagentStart functionality

The test suite MUST include comprehensive tests for SubagentStart hook processing.

**Location:** `tests/hooks_tests.rs` or `src/types.rs` (for unit tests)

#### Scenario: Unit tests validate payload fields

**Given** the test suite in `src/types.rs`
**When** tests run
**Then** the following test cases MUST exist and pass:
- Valid payload passes validation
- Empty agent_id fails validation
- Whitespace-only agent_id fails validation
- Empty subagent_type fails validation
- Empty agent_transcript_path fails validation
- Invalid base payload fails validation
- Payload with leading/trailing spaces in agent_id passes (after trim check)

#### Scenario: Integration tests verify handler execution

**Given** integration tests for SubagentStart hook
**When** tests run
**Then** the following scenarios MUST be tested:
- Valid payload returns HookResult::success()
- Invalid payload returns error
- Environment variables are set correctly
- Notifications fire when configured
- Different agent types (coder, tester, stuck) are handled correctly
