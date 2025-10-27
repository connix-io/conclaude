# Proposal: Add SQLx SQLite Database for Shared State

## Why

The current conclaude implementation uses in-memory state management (`AtomicU32` static variables) that doesn't persist across CLI invocations. Each hook execution creates a fresh process with reset state, making features like rounds counting unreliable. While a pending change proposes using tmpfs files, this approach has limitations for scalability, queryability, and data integrity. A proper database solution will provide persistent, queryable, and reliable state management across all CLI calls.

## What Changes

- **Add SQLx SQLite dependency**: Integrate SQLx for async database operations with SQLite
- **Create database module**: New `src/database.rs` with connection management and migrations
- **Implement persistent counters**: Replace `AtomicU32` rounds counter with database-backed storage
- **Add session tracking**: Track session lifecycle across hook invocations
- **Database configuration**: Extend `ConclaudeConfig` with database settings
- **Connection pooling**: Use SQLx connection pooling for efficient access
- **Schema migrations**: Automatic database initialization and schema updates
- **Audit trail**: Log hook executions for debugging and analytics

## Impact

- **Affected specs**:
  - `database` (new capability for data persistence)
  - `hook-execution` (enhanced with persistent state)
  - `configuration` (extended with database settings)
- **Affected code**:
  - `src/database.rs` (new module)
  - `src/config.rs` (database configuration)
  - `src/hooks.rs` (state management updates)
  - `src/types.rs` (database-related types)
  - `Cargo.toml` (SQLx dependency)
  - Integration tests for database operations
- **Breaking changes**: None - fully backward compatible
- **Dependencies**: Add SQLx with SQLite runtime
- **Performance**: Improved due to connection pooling and reduced file I/O