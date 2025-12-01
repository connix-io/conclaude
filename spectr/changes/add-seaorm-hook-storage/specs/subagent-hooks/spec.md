## ADDED Requirements

### Requirement: SubagentStart Database Logging

The system SHALL log SubagentStart hook executions to the SeaORM SQLite database for audit trail purposes.

#### Scenario: Successful SubagentStart logs to database
- **WHEN** a SubagentStart hook executes successfully
- **THEN** a hook_execution record SHALL be created
- **AND** the record includes session_id, hook_type='SubagentStart', agent_id, agent_transcript_path, cwd
- **AND** status is set to 'success'
- **AND** payload_json contains the full serialized payload

#### Scenario: SubagentStart logging does not block execution
- **WHEN** database logging fails during SubagentStart processing
- **THEN** the hook execution SHALL continue normally
- **AND** a warning SHALL be written to stderr
- **AND** the hook result is unaffected by database failure

### Requirement: SubagentStop Database Logging

The system SHALL log SubagentStop hook executions to the SeaORM SQLite database with execution results.

#### Scenario: Successful SubagentStop logs to database
- **WHEN** a SubagentStop hook executes successfully
- **THEN** a hook_execution record SHALL be created
- **AND** the record includes session_id, hook_type='SubagentStop', agent_id, agent_transcript_path, cwd
- **AND** status is set to 'success'
- **AND** duration_ms contains execution time in milliseconds

#### Scenario: Failed SubagentStop logs failure details
- **WHEN** a SubagentStop hook execution fails (command error)
- **THEN** a hook_execution record SHALL be created
- **AND** status is set to 'failure'
- **AND** error_message contains the failure reason

#### Scenario: SubagentStop logging does not block execution
- **WHEN** database logging fails during SubagentStop processing
- **THEN** the hook execution result SHALL be returned normally
- **AND** a warning SHALL be written to stderr
