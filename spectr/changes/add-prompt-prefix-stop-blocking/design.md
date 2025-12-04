## Context

This feature requires tracking state across multiple hook events (UserPromptSubmit and Stop) within the same session. The existing codebase already uses `OnceLock` for singleton state, atomic counters for round tracking, and SeaORM for database persistence, providing patterns to follow.

**Stakeholders**: Users who want Claude to persist on complex tasks signaled by prompt prefixes.

**Constraints**:
- Must be thread-safe for concurrent hook invocations
- Must persist state across process restarts (database-backed)
- Must integrate cleanly with existing stop hook flow (commands, infinite, rounds)

## Goals / Non-Goals

**Goals**:
- Track initial prompt (first 100 chars) per session with database persistence
- Provide glob pattern matching for flexible prefix configuration (case-sensitive)
- Support message queuing with configurable repeat counts
- Clean integration with existing stop hook infrastructure

**Non-Goals**:
- Tracking prompts for subagents (main session only)
- Case-insensitive matching (always case-sensitive)
- Automatic cleanup of database records (manual only for audit/debugging)

## Decisions

### Decision: Match against first 100 characters only

For performance, glob pattern matching runs against only the first 100 characters of the initial prompt. This is sufficient for detecting prefix keywords like "ULTRATHINK" while avoiding overhead for long prompts.

### Decision: Persist state to new database table

Create a `prompt_prefix_sessions` table with:
- `session_id` (primary key)
- `initial_prompt` (first 100 chars)
- `queue_position` (current message index)
- `times_remaining` (remaining sends for current message)
- `created_at`, `updated_at` timestamps

This enables session continuity across process restarts.

**Alternatives considered**:
- In-memory `DashMap`: Simpler but state lost on restart
- Extend `hook_executions` table: Would conflate concerns and complicate queries

### Decision: No automatic cleanup (manual only)

Records are kept indefinitely for debugging and audit purposes. Users can manually clean up old records if needed.

**Rationale**: This provides visibility into prefix blocking behavior and helps diagnose issues without silent data loss.

### Decision: Always case-sensitive matching

Glob pattern matching is case-sensitive. "ULTRATHINK*" matches "ULTRATHINK help me" but not "ultrathink help me".

**Rationale**: Case-sensitivity is more predictable and aligns with how most users expect prefix keywords to work.

### Decision: Silent completion when messages exhausted

When all messages have been sent, the Stop hook proceeds normally without any special notification or log message.

**Rationale**: Keeps the behavior simple and predictable; users who want visibility can check the database records.

### Decision: Process message queue in Stop hook, not UserPromptSubmit

The UserPromptSubmit hook only stores the initial prompt. All queue logic (matching, iteration, decrementing) happens in the Stop hook. This keeps UserPromptSubmit fast and simple.

### Decision: Prompt prefix blocking runs before commands

When prefix blocking is active and messages remain, return immediately without running stop.commands. This prevents wasted command execution when the stop will be blocked anyway.

**Rationale**: Commands may have side effects (running tests, builds). If we're going to block the stop, there's no point running potentially expensive commands.

### Decision: Default `times` to 1 for ergonomic config

If `times` is omitted, default to 1 so users can write simple configs:
```yaml
messages:
  - text: "Keep working"  # Sent once
  - text: "Document decisions"  # Sent once
```

## Risks / Trade-offs

**Risk**: Database overhead for every Stop hook
- **Mitigation**: Only query database when `promptPrefixBlocking` is configured and initial prompt was stored

**Risk**: Stale records accumulating in database
- **Mitigation**: Records are small; users can manually purge old records if needed

**Trade-off**: Persistence vs simplicity
- Chose persistence for session continuity; adds database dependency but enables resume scenarios

## Migration Plan

1. Run SeaORM migration to create `prompt_prefix_sessions` table
2. No changes needed to existing configs - feature is opt-in via `promptPrefixBlocking`

## Open Questions

None - all clarified via user questions.
