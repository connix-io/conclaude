## MODIFIED Requirements

### Requirement: Rounds Counter Persistence
The system SHALL provide persistent rounds counter storage that maintains state across CLI invocations using a SQLite database.

#### Scenario: Database-backed rounds counting
- **WHEN** the Stop hook processes rounds mode
- **THEN** the system SHALL increment the rounds counter in the database
- **AND** SHALL use atomic database operations to prevent race conditions
- **AND** SHALL maintain the counter value across process restarts

#### Scenario: Cross-process rounds continuity
- **WHEN** multiple CLI invocations occur within the same session
- **THEN** the system SHALL retrieve the current rounds count from the database
- **AND** SHALL continue counting from the previous value
- **AND** SHALL maintain session-based counter isolation

#### Scenario: Rounds counter reset
- **WHEN** the maximum rounds limit is reached
- **THEN** the system SHALL reset the session's rounds counter to 0 in the database
- **AND** SHALL update the session status accordingly
- **AND** SHALL prepare for the next session cycle

### Requirement: Session-Aware Hook Processing
The system SHALL enhance hook processing with session context stored in the database.

#### Scenario: Session context retrieval
- **WHEN** any hook is processed
- **THEN** the system SHALL retrieve session context from the database
- **AND** SHALL use session_id to associate all operations with the correct session
- **AND** SHALL maintain session lifecycle state across hook invocations

#### Scenario: Hook execution audit logging
- **WHEN** any hook is executed
- **THEN** the system SHALL log the execution details to the database
- **AND** SHALL record execution status, duration, and any errors
- **AND** SHALL maintain an audit trail for debugging and analytics