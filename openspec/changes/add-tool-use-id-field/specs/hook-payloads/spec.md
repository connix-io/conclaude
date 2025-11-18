# hook-payloads Specification

## Purpose
Define the structure of PreToolUse and PostToolUse hook payloads to support tool invocation tracking via unique tool_use_id identifiers.

## ADDED Requirements

### Requirement: PreToolUse Payload Structure
The system SHALL include a required tool_use_id field in PreToolUsePayload to identify the specific tool invocation.

#### Scenario: PreToolUse payload with tool_use_id
- **GIVEN** Claude Code sends a PreToolUse hook event with a tool_use_id field
- **WHEN** the payload is deserialized into PreToolUsePayload
- **THEN** the tool_use_id field SHALL be populated with the provided value
- **AND** hook handlers SHALL be able to access the tool_use_id

#### Scenario: PreToolUse payload missing tool_use_id
- **GIVEN** Claude Code sends a PreToolUse hook event without a tool_use_id field
- **WHEN** the payload is deserialized into PreToolUsePayload
- **THEN** deserialization SHALL fail with an error
- **AND** an appropriate error message SHALL indicate the missing required field

#### Scenario: Tool invocation correlation
- **GIVEN** a PreToolUse event with tool_use_id "abc123"
- **WHEN** a hook handler processes the payload
- **THEN** the handler SHALL be able to store or log the tool_use_id
- **AND** the tool_use_id SHALL be available for correlation with PostToolUse events

### Requirement: PostToolUse Payload Structure
The system SHALL include a required tool_use_id field in PostToolUsePayload to correlate with the corresponding PreToolUse event.

#### Scenario: PostToolUse payload with tool_use_id
- **GIVEN** Claude Code sends a PostToolUse hook event with a tool_use_id field
- **WHEN** the payload is deserialized into PostToolUsePayload
- **THEN** the tool_use_id field SHALL be populated with the provided value
- **AND** the tool_use_id SHALL match the corresponding PreToolUse event's tool_use_id

#### Scenario: PostToolUse payload missing tool_use_id
- **GIVEN** Claude Code sends a PostToolUse hook event without a tool_use_id field
- **WHEN** the payload is deserialized into PostToolUsePayload
- **THEN** deserialization SHALL fail with an error
- **AND** an appropriate error message SHALL indicate the missing required field

#### Scenario: Cross-event correlation
- **GIVEN** a PreToolUse event with tool_use_id "xyz789"
- **AND** a subsequent PostToolUse event with the same tool_use_id "xyz789"
- **WHEN** hook handlers process both events
- **THEN** handlers SHALL be able to correlate the two events via matching tool_use_id values
- **AND** this enables audit trails, performance tracking, and debug workflows

### Requirement: Field Validation
The system SHALL validate that tool_use_id is present and non-empty in all tool use hook payloads.

#### Scenario: Valid tool_use_id
- **GIVEN** a hook payload with tool_use_id containing a non-empty string
- **WHEN** the payload is deserialized
- **THEN** deserialization SHALL succeed
- **AND** the tool_use_id SHALL be accessible to hook handlers

#### Scenario: Empty tool_use_id
- **GIVEN** a hook payload with tool_use_id as an empty string
- **WHEN** the payload is deserialized
- **THEN** deserialization SHALL succeed (serde allows empty strings)
- **AND** the empty value SHALL be preserved
- **AND** hook handlers MAY choose to validate non-emptiness if needed

### Requirement: Type Safety and Serialization
The system SHALL correctly serialize and deserialize tool_use_id using Rust's type system and serde.

#### Scenario: Serialization with tool_use_id
- **GIVEN** a PreToolUsePayload with tool_use_id = "test123"
- **WHEN** the payload is serialized to JSON
- **THEN** the JSON SHALL include `"tool_use_id": "test123"`
- **AND** the field SHALL be at the appropriate nesting level

#### Scenario: Round-trip serialization
- **GIVEN** a PreToolUsePayload with tool_use_id = "abc-xyz-123"
- **WHEN** the payload is serialized to JSON and then deserialized back
- **THEN** the deserialized payload SHALL have tool_use_id = "abc-xyz-123"
- **AND** no data SHALL be lost in the round-trip

#### Scenario: Deserialization with extra fields
- **GIVEN** a JSON payload with tool_use_id and all required fields
- **WHEN** deserialized into PreToolUsePayload or PostToolUsePayload
- **THEN** deserialization SHALL succeed
- **AND** all fields SHALL have correct values
- **AND** no data SHALL be lost
