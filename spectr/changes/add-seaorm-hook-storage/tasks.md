## 1. Database Infrastructure

### 1.1 Dependencies
- [x] 1.1.1 Add sea-orm dependency with sqlite, runtime-tokio-rustls features
- [x] 1.1.2 Add sea-orm-migration for schema migrations
- [x] 1.1.3 Verify Cargo.toml compiles with new dependencies

### 1.2 Module Structure
- [x] 1.2.1 Create `src/database/mod.rs` with module exports
- [x] 1.2.2 Create `src/database/connection.rs` with lazy connection singleton
- [x] 1.2.3 Create `src/database/entities/mod.rs` for entity exports
- [x] 1.2.4 Create `src/database/migrations/mod.rs` for migration runner

### 1.3 Platform Data Directories
- [x] 1.3.1 Implement `get_data_dir()` using dirs crate for platform paths
- [x] 1.3.2 Add CONCLAUDE_DATA_DIR environment variable override
- [x] 1.3.3 Create directory if it doesn't exist on first access

## 2. Entity and Migration Implementation

### 2.1 Hook Execution Entity
- [x] 2.1.1 Create `src/database/entities/hook_execution.rs` with Model derive
- [x] 2.1.2 Define fields: id, session_id, hook_type, agent_id, agent_transcript_path, cwd, status, duration_ms, error_message, payload_json, created_at
- [x] 2.1.3 Implement Relation and ActiveModelBehavior traits

### 2.2 Migrations
- [x] 2.2.1 Create initial migration for hook_executions table
- [x] 2.2.2 Add indexes on session_id, agent_id, hook_type columns
- [x] 2.2.3 Implement MigratorTrait for automatic migration running

## 3. Hook Integration

### 3.1 SubagentStart Hook
- [x] 3.1.1 Add database logging call in handle_subagent_start
- [x] 3.1.2 Capture execution start time for duration tracking
- [x] 3.1.3 Store full payload as JSON for debugging
- [x] 3.1.4 Handle database errors gracefully (log, don't fail hook)

### 3.2 SubagentStop Hook
- [x] 3.2.1 Add database logging call in handle_subagent_stop
- [x] 3.2.2 Capture execution duration
- [x] 3.2.3 Store execution status (success/failure/blocked)
- [x] 3.2.4 Store error message if execution failed

## 4. Configuration

### 4.1 Database Config
- [x] 4.1.1 Add DatabaseConfig struct to config.rs
- [x] 4.1.2 Add optional database_path field
- [x] 4.1.3 Add enabled toggle (default: true)
- [x] 4.1.4 Update ConclaudeConfig to include database section

## 5. CLI Commands

### 5.1 Database Subcommands
- [x] 5.1.1 Add `conclaude db status` to show database info and stats
- [x] 5.1.2 Add `conclaude db cleanup --older-than <days>` for data cleanup
- [x] 5.1.3 Add `conclaude db query --session <id>` to list hook executions

## 6. Testing

### 6.1 Unit Tests
- [x] 6.1.1 Test entity CRUD operations with in-memory SQLite
- [x] 6.1.2 Test migration up/down functionality
- [x] 6.1.3 Test platform path resolution for Linux/macOS/Windows

### 6.2 Integration Tests
- [x] 6.2.1 Test SubagentStart hook logs to database
- [x] 6.2.2 Test SubagentStop hook logs to database
- [x] 6.2.3 Test database graceful degradation when unavailable
