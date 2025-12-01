# Change: Add SeaORM-based Hook Storage

## Why

Conclaude needs persistent storage for SubagentStart and SubagentStop hook executions to enable session tracking, audit trails, and analytics across CLI invocations. The current in-memory state management loses data between process executions. SeaORM provides a type-safe, async ORM with excellent SQLite support and cleaner entity-based abstractions compared to raw SQL.

## What Changes

- **Add SeaORM dependency**: Integrate SeaORM with SQLite runtime for async database operations
- **Create database module**: New `src/database/` module with entities, migrations, and connection management
- **Implement hook storage**: Persist SubagentStart and SubagentStop hook payloads and execution results
- **Platform-aware data directory**: Store database in XDG_DATA_HOME (Linux), ~/Library/Application Support (macOS), %LOCALAPPDATA% (Windows)
- **Add database configuration**: Extend config with optional database path override via `CONCLAUDE_DATA_DIR`
- **CLI commands**: Add `conclaude db` subcommands for database management

## Impact

- **Affected specs**:
  - `database` (new capability for SeaORM-based persistence)
  - `hook-execution` (enhanced with persistent audit trail)
  - `configuration` (extended with database settings)
- **Affected code**:
  - `src/database/` (new module with entities and migrations)
  - `src/config.rs` (database configuration)
  - `src/hooks.rs` (hook execution logging)
  - `src/main.rs` (new CLI subcommands)
  - `Cargo.toml` (SeaORM dependency)
- **Breaking changes**: None - fully backward compatible
- **Dependencies**: Add sea-orm, sea-orm-migration with SQLite runtime
- **Supersedes**: This change supersedes `old-state` proposal (uses SeaORM instead of SQLx)
