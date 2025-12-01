# Design: SeaORM SQLite Hook Storage

## Context

Conclaude executes as a CLI tool invoked by Claude Code hooks. Each invocation is a fresh process, making in-memory state (like `AtomicU32` counters) unreliable across hook executions. Persistent storage enables session tracking, audit trails, and analytics for SubagentStart/SubagentStop hooks.

## Goals / Non-Goals

**Goals:**
- Persist SubagentStart and SubagentStop hook payloads and execution results
- Use platform-appropriate data directories (XDG_DATA_HOME, etc.)
- Provide type-safe database operations via SeaORM entities
- Enable querying hook execution history
- Maintain backward compatibility (database is optional enhancement)

**Non-Goals:**
- Store all hook types (focus on subagent hooks initially)
- Real-time analytics dashboard
- Multi-database support (SQLite only)
- Distributed database clustering

## Decisions

### ORM Choice: SeaORM over SQLx

**Decision**: Use SeaORM with SQLite backend.

**Rationale**:
- Entity-based abstractions are cleaner than raw SQL strings
- Compile-time checked queries via derive macros
- Built-in migration system with versioning
- Active Record and ActiveModel patterns fit CLI use case
- Async-first design integrates with tokio runtime

**Alternatives considered**:
- **SQLx**: Lower-level, requires manual SQL; good but more boilerplate
- **Diesel**: Sync-only, doesn't fit async architecture
- **rusqlite**: No async support, manual connection management

### Database Location Strategy

**Decision**: Use platform-specific data directories with environment override.

| Platform | Default Path | Environment Override |
|----------|--------------|---------------------|
| Linux | `$XDG_DATA_HOME/conclaude/conclaude.db` or `~/.local/share/conclaude/conclaude.db` | `CONCLAUDE_DATA_DIR` |
| macOS | `~/Library/Application Support/conclaude/conclaude.db` | `CONCLAUDE_DATA_DIR` |
| Windows | `%LOCALAPPDATA%\conclaude\conclaude.db` | `CONCLAUDE_DATA_DIR` |

**Rationale**:
- Follows OS conventions for application data
- `dirs` crate already in dependencies handles platform detection
- Environment variable allows custom paths for testing/CI

### Schema Design

```sql
-- Hook executions table for SubagentStart/SubagentStop audit trail
CREATE TABLE hook_executions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id TEXT NOT NULL,
    hook_type TEXT NOT NULL,           -- 'SubagentStart' or 'SubagentStop'
    agent_id TEXT NOT NULL,
    agent_transcript_path TEXT NOT NULL,
    cwd TEXT NOT NULL,
    status TEXT NOT NULL,              -- 'success', 'failure', 'blocked'
    duration_ms INTEGER,
    error_message TEXT,
    payload_json TEXT,                 -- Full payload for debugging
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_hook_executions_session ON hook_executions(session_id);
CREATE INDEX idx_hook_executions_agent ON hook_executions(agent_id);
CREATE INDEX idx_hook_executions_type ON hook_executions(hook_type);
```

### Entity Structure

```rust
// src/database/entities/hook_execution.rs
#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "hook_executions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub session_id: String,
    pub hook_type: String,
    pub agent_id: String,
    pub agent_transcript_path: String,
    pub cwd: String,
    pub status: String,
    pub duration_ms: Option<i64>,
    pub error_message: Option<String>,
    pub payload_json: Option<String>,
    pub created_at: DateTimeUtc,
}
```

### Connection Management

**Decision**: Lazy singleton connection with automatic initialization.

```rust
static DB_CONN: OnceLock<DatabaseConnection> = OnceLock::new();

pub async fn get_connection() -> Result<&'static DatabaseConnection> {
    if let Some(conn) = DB_CONN.get() {
        return Ok(conn);
    }

    let db_path = get_database_path()?;
    ensure_parent_dir_exists(&db_path)?;

    let conn = Database::connect(format!("sqlite://{}?mode=rwc", db_path)).await?;
    Migrator::up(&conn, None).await?;  // Auto-run migrations

    Ok(DB_CONN.get_or_init(|| conn))
}
```

## Risks / Trade-offs

### Performance Impact
**Risk**: Database operations add latency to hook execution.

**Mitigation**:
- Use async operations to avoid blocking
- SQLite with WAL mode for fast writes
- Database operations are non-blocking to hook success/failure

### Database Corruption
**Risk**: SQLite file corruption could lose audit data.

**Mitigation**:
- SQLite has excellent durability (ACID)
- WAL mode provides crash recovery
- Audit data is supplementary, not critical path

### Dependency Size
**Risk**: SeaORM adds binary size and compile time.

**Mitigation**:
- Use minimal feature set (sqlite, runtime-tokio-native-tls)
- SeaORM is well-maintained with active development
- Trade-off acceptable for type safety benefits

## Migration Plan

1. Add SeaORM dependencies to Cargo.toml
2. Create `src/database/` module structure
3. Implement entities and migrations
4. Add connection management with platform paths
5. Integrate logging into SubagentStart/SubagentStop handlers
6. Add CLI commands for database management
7. Update configuration for database path override

## Open Questions

None - design decisions are complete.
