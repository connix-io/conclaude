## MODIFIED Requirements

### Definitions

For clarity and to prevent implementation ambiguity, the following terms are explicitly defined:

- **Rounds mode**: An operational mode of the Stop hook that is active when `config.stop.rounds` is set to `Some(u32)` value in the user's configuration. This mode provides an alternative to infinite mode, allowing Claude to continue for a specific number of rounds before stopping naturally. Rounds mode is checked at line 793 in `src/hooks.rs` via `if let Some(max_rounds) = config.stop.rounds`.

- **Maximum rounds limit**: The numeric value of `config.stop.rounds: Option<u32>` as configured in the user's `.conclaude.yaml` file (defined in `src/config.rs:38`). This is a user-configurable positive integer (u32) that determines how many Stop hook invocations will occur before allowing the session to terminate.

- **Atomic database operations**: Database operations that use SQLite transactions with SERIALIZABLE isolation level to ensure thread-safe, race-condition-free counter increments. Implementation SHALL use SQLx's transaction API with explicit transaction boundaries to guarantee that concurrent database access from multiple CLI invocations maintains data integrity and prevents lost updates.

### Requirement: Rounds Counter Persistence
The system SHALL provide persistent rounds counter storage that maintains state across CLI invocations using a SQLite database.

#### Scenario: Database-backed rounds counting
- **WHEN** the Stop hook is processed AND config.stop.rounds is Some(value)
- **THEN** the system SHALL increment the rounds counter in the database
- **AND** SHALL use SQLite transactions with SERIALIZABLE isolation to prevent race conditions
- **AND** SHALL maintain the counter value across process restarts

#### Scenario: Cross-process rounds continuity
- **WHEN** multiple CLI invocations occur within the same session
- **THEN** the system SHALL retrieve the current rounds count from the database
- **AND** SHALL continue counting from the previous value
- **AND** SHALL maintain session-based counter isolation

#### Scenario: Rounds counter reset
- **WHEN** the rounds counter equals the config.stop.rounds limit
- **THEN** the system SHALL reset the session's rounds counter to 0 in the database
- **AND** SHALL update the session status to 'ended'
- **AND** SHALL prepare for the next session cycle

### Requirement: Session-Aware Hook Processing
The system SHALL enhance hook processing with session context stored in the database.

#### Hook Scope
The following hooks SHALL participate in session-aware processing:

**Session Creation Hooks** (create new session records):
- `SessionStart` - Primary session initialization hook

**Session Update Hooks** (update last_seen timestamp and session state):
- `UserPromptSubmit` - User input events
- `PreToolUse` - Before tool execution
- `PostToolUse` - After tool execution
- `PreCompact` - Before transcript compaction
- `Notification` - System notifications
- `SubagentStop` - Subagent termination events

**Session Termination Hooks** (finalize session state):
- `Stop` - Main session stop event
- `SessionEnd` - Explicit session termination

**Excluded from Session Processing**:
- Internal middleware hooks that operate before session initialization
- Hooks fired before database connection is established

#### Session Context Schema
The session context SHALL be stored with the following fields and types:

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| `session_id` | String (VARCHAR(128)) | PRIMARY KEY, NOT NULL | Unique identifier for the Claude session |
| `created_at` | Timestamp (DATETIME) | NOT NULL, DEFAULT CURRENT_TIMESTAMP | Session creation timestamp |
| `updated_at` | Timestamp (DATETIME) | NOT NULL, DEFAULT CURRENT_TIMESTAMP | Last modification timestamp |
| `last_seen` | Timestamp (DATETIME) | NOT NULL, DEFAULT CURRENT_TIMESTAMP | Last hook activity timestamp |
| `status` | String (VARCHAR(32)) | NOT NULL, CHECK constraint | Current session status (see below) |
| `source` | String (VARCHAR(64)) | NOT NULL | Session initiation source (e.g., "startup", "cli", "ide") |
| `cwd` | String (TEXT) | NOT NULL | Working directory at session start |
| `metadata` | JSON (TEXT) | NULL | Additional session context as JSON object |
| `owner_id` | String (VARCHAR(128)) | NULL | Optional user/process identifier |

**Session Status Values**:
- `active` - Session is currently running
- `ended` - Session terminated normally via SessionEnd or Stop
- `abandoned` - Session inactive beyond timeout threshold
- `archived` - Session marked for retention/export

#### Execution Status Values
Hook execution audit records SHALL use the following status values:

| Status | Meaning | Use Case |
|--------|---------|----------|
| `success` | Hook completed without errors, operation proceeded | Normal execution path |
| `failure` | Hook encountered an error, operation may be aborted | Runtime errors, exceptions |
| `timeout` | Hook exceeded configured execution time limit | Long-running commands, hung processes |
| `skipped` | Hook was bypassed due to configuration or conditions | Disabled hooks, conditional execution |
| `blocked` | Hook intentionally prevented operation from proceeding | Validation failures, policy violations |

#### Error Handling Semantics for Session Retrieval Failures

**Default Behavior (Fail-Fast)**:
- **WHEN** session retrieval fails due to database unavailability, corruption, or query errors
- **THEN** the hook execution SHALL fail immediately with status code 1
- **AND** SHALL log a critical error message to stderr
- **AND** SHALL NOT proceed with hook logic
- **AND** SHALL return error details to Claude Code for user visibility

**Fallback Behavior (Degraded Mode)**:
- **WHEN** `database.allow_degraded_mode` configuration is enabled
- **AND** session retrieval fails
- **THEN** the system SHALL create an ephemeral in-memory session marker
- **AND** SHALL log a warning about degraded operation mode
- **AND** SHALL proceed with hook execution using ephemeral state
- **AND** SHALL mark the session with `degraded: true` metadata
- **AND** SHALL disable features requiring persistent state (e.g., rounds counting)

**Configuration Toggle**:
```yaml
database:
  fail_fast_on_session_error: true  # Default: true (fail-fast)
  allow_degraded_mode: false        # Default: false (strict mode)
```

#### Audit Retention and Purge Policy

**Default Retention Period**:
- Hook execution audit records SHALL be retained for **30 days** by default
- Session records SHALL be retained for **30 days** after session status changes to `ended` or `abandoned`

**Configurable Retention**:
```yaml
database:
  retention_days: 30              # Default: 30, Range: 1-365
  enable_archiving: false         # Default: false
  archive_format: "jsonl"         # Options: "jsonl", "sqlite"
  archive_path: "~/.conclaude/archives"
```

**Soft-Delete Strategy**:
- **WHEN** records reach retention age
- **THEN** the system SHALL mark records with `deleted_at` timestamp
- **AND** SHALL change session status to `archived` if not already set
- **AND** SHALL exclude soft-deleted records from active queries
- **AND** SHALL retain soft-deleted records for additional 7 days before hard deletion

**Hard Deletion Process**:
- **WHEN** soft-deleted records exceed retention window + 7 days
- **THEN** the system SHALL permanently delete records from the database
- **AND** SHALL log deletion statistics (count, date range, session IDs)

**Archiving Strategy** (when enabled):
- **BEFORE** soft-delete operation
- **THEN** the system SHALL export records to archive file
- **AND** SHALL include all session metadata, hook executions, and counters
- **AND** SHALL compress archive files with gzip
- **AND** SHALL verify archive integrity before marking for deletion
- **AND** SHALL name archives as `session-{session_id}-{timestamp}.jsonl.gz`

**Purge Execution Schedule**:
- Automatic purge SHALL run during `Stop` hook processing
- Manual purge available via `conclaude db purge` command
- Purge operations SHALL use database transactions for atomicity

#### Scenario: Session context retrieval
- **WHEN** any hook is processed
- **THEN** the system SHALL retrieve session context from the database
- **AND** SHALL use session_id to associate all operations with the correct session
- **AND** SHALL maintain session lifecycle state across hook invocations
- **AND** SHALL handle session retrieval failures according to configured error handling semantics
- **AND** SHALL update last_seen timestamp for the session

#### Scenario: Hook execution audit logging
- **WHEN** any hook is executed
- **THEN** the system SHALL log the execution details to the database
- **AND** SHALL record execution status using defined status values (success, failure, timeout, skipped, blocked)
- **AND** SHALL record duration in milliseconds for performance monitoring
- **AND** SHALL capture error messages and stack traces for failure and timeout statuses
- **AND** SHALL maintain an audit trail for debugging and analytics
- **AND** SHALL respect retention policy for automatic cleanup