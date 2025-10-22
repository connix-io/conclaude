# Hook Execution Spec Deltas

## MODIFIED Requirements

### Requirement: Rounds Mode State Persistence

The rounds counter SHALL persist state across hook invocations using tmpfs file storage to ensure accurate round counting throughout a Claude Code session.

#### Scenario: Round count persists across hook invocations

- **GIVEN** a configuration with `stop.rounds` set to 3
- **WHEN** the stop hook is invoked multiple times within the same session
- **THEN** the round count increments correctly across separate process invocations
- **AND** state is maintained in `/tmp/conclaude-{session_id}.rounds` file

#### Scenario: Round count resets when max rounds reached

- **GIVEN** a configuration with `stop.rounds` set to 3
- **AND** the current round count is 3 (maximum reached)
- **WHEN** the stop hook completes successfully
- **THEN** the tmpfs state file is removed
- **AND** the next session starts with round count 1

#### Scenario: Concurrent sessions maintain separate round counts

- **GIVEN** multiple concurrent Claude Code sessions
- **WHEN** each session has rounds mode enabled
- **THEN** each session maintains its own round count in separate tmpfs files
- **AND** round counts do not interfere with each other

#### Scenario: Tmpfs state file handles I/O errors gracefully

- **GIVEN** tmpfs is unavailable or file operations fail
- **WHEN** attempting to read or write round state
- **THEN** the system logs an appropriate error
- **AND** falls back to allowing the operation (fail-open)

### Requirement: Rounds Mode Message Format

The system SHALL display progress messages showing current round and maximum rounds when in rounds mode.

#### Scenario: Progress message includes round information

- **GIVEN** rounds mode is enabled with max rounds 5
- **AND** current round is 2
- **WHEN** the stop hook blocks to continue
- **THEN** the message format is "Round {current}/{max} completed, continuing..."
- **AND** the message is logged at INFO level

## REMOVED Requirements

### Requirement: Static Atomic Counter for Rounds

**Reason**: Static atomic counters reset across process invocations, making them unsuitable for persistent round counting in a hook-based architecture where each invocation is a separate process.

**Migration**: Replace all uses of `static ROUND_COUNT: AtomicU32` with tmpfs file-based state management using the session ID as the key.
