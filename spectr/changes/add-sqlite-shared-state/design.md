# Design: SQLx SQLite Database Integration

## Context

Conclaude currently uses in-memory state management that doesn't persist across CLI invocations. Each hook execution spawns a fresh process, losing state between calls. The existing rounds counter uses `AtomicU32` which resets on every process start, making session-based functionality unreliable. While tmpfs files were proposed as an alternative, they lack queryability, transaction support, and scalability.

## Goals / Non-Goals

**Goals:**
- Provide persistent state across CLI invocations
- Enable session lifecycle tracking
- Support efficient concurrent access
- Maintain backward compatibility
- Add queryable audit trail for hook executions
- Enable future analytics and reporting features

**Non-Goals:**
- Multi-database support (SQLite only)
- Distributed/database clustering
- Complex data migrations
- Real-time analytics dashboard
- API for external database access

## Decisions

### Database Choice: SQLite with SQLx
**Decision**: Use SQLite as the database with SQLx for async operations.

**Rationale**:
- SQLite provides file-based persistence with ACID compliance
- No external database server required (simplifies deployment)
- SQLx offers compile-time query verification and async support
- Excellent performance for read-heavy workloads
- Mature ecosystem with great Rust integration

**Alternatives considered**:
- **PostgreSQL**: Overkill for single-machine CLI tool, requires external service
- **File-based JSON**: Lacks transaction support, concurrent access issues
- **tmpfs files**: Limited queryability, no ACID guarantees
- **Redis**: External dependency, persistence complexity

### Schema Design

```sql
-- Sessions table for tracking Claude session lifecycle
CREATE TABLE sessions (
    id TEXT PRIMARY KEY,
    source TEXT NOT NULL,
    cwd TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    status TEXT NOT NULL DEFAULT 'active', -- active, ended, expired
    metadata TEXT -- JSON for additional session data
);

-- Counters table for persistent numeric state
CREATE TABLE counters (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id TEXT NOT NULL,
    counter_type TEXT NOT NULL, -- 'rounds', 'tool_executions', etc.
    value INTEGER NOT NULL DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (session_id) REFERENCES sessions(id),
    UNIQUE(session_id, counter_type)
);

-- Hook executions for audit trail
CREATE TABLE hook_executions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id TEXT NOT NULL,
    hook_name TEXT NOT NULL,
    tool_name TEXT,
    status TEXT NOT NULL, -- success, failure, blocked
    duration_ms INTEGER,
    error_message TEXT,
    metadata TEXT, -- JSON for additional execution data
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (session_id) REFERENCES sessions(id)
);

-- Config cache to avoid repeated file parsing
CREATE TABLE config_cache (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    config_path TEXT NOT NULL UNIQUE,
    config_hash TEXT NOT NULL, -- SHA256 of file content
    config_data TEXT NOT NULL, -- Serialized YAML content
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

### Connection Management

**Decision**: Use SQLx connection pooling with SQLite.

**Rationale**:
- Connection pooling reduces overhead of repeated database connections
- SQLite supports concurrent reads properly with pooling
- Automatic connection lifecycle management
- Built-in connection health checking

**Implementation**:
```rust
static DB_POOL: OnceLock<SqlitePool> = OnceLock::new();

async fn get_db_pool() -> Result<&'static SqlitePool> {
    if let Some(pool) = DB_POOL.get() {
        return Ok(pool);
    }

    let pool = SqlitePoolOptions::new()
        .max_connections(10)
        .connect(&database_url())
        .await?;

    Ok(DB_POOL.get_or_init(|| pool))
}
```

### Database Location Strategy

**Decision**: Use `~/.conclaude/conclaude.db` as default database location.

**Rationale**:
- Follows XDG Base Directory specification
- User-writable location that persists across reboots
- Separate from project-specific configurations
- Supports multiple users on the same system

**Fallback strategy**:
- If home directory not accessible, fall back to `/tmp/conclaude.db`
- Use environment variable `CONCLAUDE_DB_PATH` for customization
- Create directory structure if it doesn't exist

## Risks / Trade-offs

### Performance Considerations
**Risk**: Database operations may introduce latency compared to in-memory counters.

**Mitigation**:
- Use connection pooling to reduce connection overhead
- Batch write operations where possible
- Cache frequently accessed data in memory
- Use async operations to avoid blocking

### Data Integrity
**Risk**: Database corruption could break conclaude functionality.

**Mitigation**:
- SQLite has excellent durability guarantees
- Implement periodic database backups
- Use WAL mode for better concurrent access
- Add database integrity checks on startup

### Migration Complexity
**Risk**: Database schema changes may require complex migrations.

**Mitigation**:
- Use SQLx migrations for versioned schema changes
- Design schema with extensibility in mind
- Provide migration rollback capability
- Test migrations thoroughly in CI

### Dependency Bloat
**Risk**: Adding SQLx increases binary size and dependency surface.

**Mitigation**:
- SQLx is a mature, well-maintained dependency
- Compile-time query verification reduces runtime errors
- Use feature flags to limit SQLx features
- Monitor dependency updates regularly

## Migration Plan

### Phase 1: Database Infrastructure
1. Add SQLx dependency with SQLite features
2. Create `src/database.rs` with connection management
3. Implement database initialization and migrations
4. Add basic database configuration options

### Phase 2: State Migration
1. Replace `AtomicU32` rounds counter with database implementation
2. Add session tracking for new sessions
3. Implement database-backed config caching
4. Add hook execution logging

### Phase 3: Enhanced Features
1. Add database cleanup utilities
2. Implement session analytics
3. Add database backup/restore functionality
4. Performance optimization and monitoring

### Rollback Strategy
- Disable database features with `--no-db` flag
- Fall back to in-memory state if database unavailable
- Provide database reset utility `conclaude db reset`
- Export/import functionality for data preservation

## Open Questions

1. **Database size limits**: Should we implement automatic cleanup of old sessions?
2. **Concurrent access**: How to handle multiple conclaude instances accessing the same database?
3. **Backup strategy**: Should we implement automatic database backups?
4. **Performance monitoring**: How to track database performance impact?
5. **Error handling**: What should happen when database operations fail?

## Testing Strategy

### Unit Tests
- Database connection management
- Migration application and rollback
- Individual database operations (CRUD)
- Error handling scenarios

### Integration Tests
- End-to-end session tracking
- Concurrent database access
- Database recovery from corruption
- Performance benchmarks

### Test Database
- Use in-memory SQLite for unit tests
- Separate test database file for integration tests
- Automated test database cleanup
- Test data fixtures for consistent testing