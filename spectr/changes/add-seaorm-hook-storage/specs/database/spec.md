## ADDED Requirements

### Requirement: SeaORM SQLite Database

The system SHALL provide persistent storage using SeaORM with SQLite backend for hook execution data.

#### Scenario: Database initialization on first access
- **WHEN** a hook handler first accesses the database
- **THEN** the system creates the database file in the platform data directory
- **AND** runs all pending migrations automatically

#### Scenario: Connection reuse within process
- **WHEN** multiple database operations occur within the same CLI invocation
- **THEN** the system reuses the same database connection

### Requirement: Platform-Aware Data Directory

The system SHALL store the database in platform-appropriate data directories.

#### Scenario: Linux data directory
- **WHEN** running on Linux
- **THEN** the database is stored at `$XDG_DATA_HOME/conclaude/conclaude.db`
- **OR** `~/.local/share/conclaude/conclaude.db` if XDG_DATA_HOME is unset

#### Scenario: macOS data directory
- **WHEN** running on macOS
- **THEN** the database is stored at `~/Library/Application Support/conclaude/conclaude.db`

#### Scenario: Windows data directory
- **WHEN** running on Windows
- **THEN** the database is stored at `%LOCALAPPDATA%\conclaude\conclaude.db`

#### Scenario: Custom data directory override
- **WHEN** the CONCLAUDE_DATA_DIR environment variable is set
- **THEN** the database is stored at `$CONCLAUDE_DATA_DIR/conclaude.db`

### Requirement: Hook Execution Entity

The system SHALL store hook execution records with the following fields: id, session_id, hook_type, agent_id, agent_transcript_path, cwd, status, duration_ms, error_message, payload_json, created_at.

#### Scenario: Store SubagentStart execution
- **WHEN** a SubagentStart hook executes
- **THEN** a hook_execution record is created with hook_type='SubagentStart'
- **AND** the agent_id and agent_transcript_path from the payload are stored

#### Scenario: Store SubagentStop execution
- **WHEN** a SubagentStop hook executes
- **THEN** a hook_execution record is created with hook_type='SubagentStop'
- **AND** the execution status and duration are recorded

### Requirement: Database Graceful Degradation

The system SHALL continue hook execution even when database operations fail.

#### Scenario: Database unavailable
- **WHEN** the database cannot be accessed or created
- **THEN** the hook execution continues without database logging
- **AND** a warning is logged to stderr
