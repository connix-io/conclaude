## ADDED Requirements

### Requirement: Database Initialization and Connection Management
The system SHALL provide a SQLite database for persistent state storage across CLI invocations.

#### Scenario: Database initialization on first run
- **WHEN** conclaude is executed for the first time
- **AND** no database file exists at the configured location
- **THEN** the system SHALL create a new SQLite database
- **AND** SHALL run all pending migrations to create required tables
- **AND** SHALL establish a connection pool for efficient access

#### Scenario: Database connection pooling
- **WHEN** multiple database operations are required
- **THEN** the system SHALL use a connection pool to manage database connections
- **AND** SHALL reuse connections to reduce connection overhead
- **AND** SHALL support concurrent read operations

#### Scenario: Database fallback handling
- **WHEN** database operations fail due to permission or corruption issues
- **THEN** the system SHALL fall back to in-memory state management
- **AND** SHALL log appropriate error messages
- **AND** SHALL continue operating with reduced functionality

### Requirement: Session Lifecycle Management
The system SHALL track Claude session lifecycle in the database for persistent state across hook invocations.

#### Scenario: Session creation on start
- **WHEN** a SessionStart hook is processed
- **THEN** the system SHALL create a new session record in the database
- **AND** SHALL store session metadata (session_id, source, cwd, timestamp)
- **AND** SHALL set session status to 'active'

#### Scenario: Session update on activity
- **WHEN** any hook is processed for an active session
- **THEN** the system SHALL update the session's updated_at timestamp
- **AND** SHALL maintain the session as 'active' status

#### Scenario: Session termination
- **WHEN** a SessionEnd hook is processed
- **THEN** the system SHALL update the session status to 'ended'
- **AND** SHALL record the end timestamp
- **AND** SHALL preserve session data for audit purposes

#### Scenario: Session cleanup
- **WHEN** sessions are older than 30 days
- **THEN** the system SHALL automatically clean up old session records
- **AND** SHALL preserve session data if configured for longer retention

### Requirement: Persistent Counter Storage
The system SHALL provide database-backed storage for numeric counters that persist across CLI invocations.

#### Scenario: Counter initialization
- **WHEN** a counter is accessed for the first time in a session
- **THEN** the system SHALL create a counter record with initial value of 0
- **AND** SHALL associate the counter with the session and counter type

#### Scenario: Counter increment atomicity
- **WHEN** a counter needs to be incremented
- **THEN** the system SHALL use SQLite transactions with SERIALIZABLE isolation to prevent race conditions
- **AND** SHALL return the updated counter value
- **AND** SHALL handle concurrent access safely

#### Scenario: Counter retrieval
- **WHEN** a counter value is requested
- **THEN** the system SHALL retrieve the current value from the database
- **AND** SHALL return 0 if no counter exists for the session/type combination

#### Scenario: Rounds counter persistence
- **WHEN** the rounds counter is incremented in the Stop hook
- **THEN** the system SHALL store the value persistently in the database
- **AND** SHALL maintain the counter across multiple CLI invocations
- **AND** SHALL reset the counter when it reaches the config.stop.rounds limit

### Requirement: Hook Execution Audit Trail
The system SHALL maintain an audit trail of hook executions in the database for debugging and analytics.

#### Scenario: Hook execution logging
- **WHEN** any hook is processed
- **THEN** the system SHALL log the execution details in the database
- **AND** SHALL record session_id, hook_name, status, and timestamp
- **AND** SHALL capture tool_name for PreToolUse and PostToolUse hooks

#### Scenario: Error information capture
- **WHEN** a hook execution fails
- **THEN** the system SHALL record the error message and stack trace
- **AND** SHALL store the failure status for later analysis
- **AND** SHALL include duration metrics for performance monitoring

#### Scenario: Metadata storage
- **WHEN** additional context is available for hook execution
- **THEN** the system SHALL store JSON metadata in the audit record
- **AND** SHALL include tool input parameters, file paths, or other relevant context

### Requirement: Configuration Caching
The system SHALL cache parsed configuration files in the database to avoid repeated file I/O operations.

#### Scenario: Config cache creation
- **WHEN** a configuration file is loaded for the first time
- **THEN** the system SHALL calculate a hash of the file content
- **AND** SHALL store the parsed configuration in the database cache
- **AND** SHALL associate the cache entry with the file path and hash

#### Scenario: Config cache validation
- **WHEN** a configuration file is requested
- **THEN** the system SHALL check if a cached version exists
- **AND** SHALL validate the file hash against cached hash
- **AND** SHALL return cached data if hash matches, otherwise reload from file

#### Scenario: Config cache invalidation
- **WHEN** a configuration file is modified
- **THEN** the system SHALL detect the hash change
- **AND** SHALL invalidate the cached configuration
- **AND** SHALL reload and cache the updated configuration

### Requirement: Database Configuration Management
The system SHALL provide configuration options for database behavior and location.

#### Scenario: Custom database path
- **WHEN** CONCLAUDE_DB_PATH environment variable is set
- **THEN** the system SHALL use the specified path for the database file
- **AND** SHALL create parent directories if they don't exist

#### Scenario: Default database location
- **WHEN** no custom database path is specified
- **THEN** the system SHALL use ~/.conclaude/conclaude.db as default location
- **AND** SHALL fall back to /tmp/conclaude.db if home directory is inaccessible

#### Scenario: Database features toggle
- **WHEN** database features need to be disabled
- **THEN** the system SHALL support --no-db flag or configuration option
- **AND** SHALL fall back to in-memory state management
- **AND** SHALL provide clear indication of reduced functionality