# Change: Add prompt-prefix-based stop blocking with message queue

## Why

Users want Claude to continue working longer on complex tasks without manual intervention. By detecting certain prefixes in the initial prompt (e.g., "ULTRATHINK"), conclaude can automatically block stop hooks and send configurable reminder messages to keep Claude working. This provides fine-grained control over session persistence based on user intent signaled through prompt prefixes.

## What Changes

- Add `promptPrefixBlocking` configuration section under `stop` config
- Track the initial prompt (first 100 characters) from the first `UserPromptSubmit` hook event per session
- Persist session state to new `prompt_prefix_sessions` database table for process restart continuity
- Match initial prompt against configurable glob patterns (case-sensitive)
- When a matching prompt triggers the Stop hook, block it and send messages from a queue
- Each message in the queue can specify how many times (`times`) to be sent before advancing (default: 1)
- Once all messages are exhausted, allow the Stop hook to proceed normally (silent completion)
- Database records retained indefinitely for audit/debugging (manual cleanup only)

## Key Design Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Match scope | First 100 chars | Performance; sufficient for prefix keywords |
| Persistence | Database (new table) | Session continuity across restarts |
| Case sensitivity | Always case-sensitive | Predictable behavior for keywords |
| Cleanup | Manual only | Audit trail, debugging visibility |
| Completion | Silent | Simple, predictable; check DB for status |
| Execution order | Before stop.commands | Avoid wasted command execution |

## Impact

- Affected specs: `hooks-system`
- Affected code:
  - `src/config.rs` (new config structs)
  - `src/hooks.rs` (stop hook logic, UserPromptSubmit enhancement)
  - `migration/` (new SeaORM migration)
  - `entity/` (new SeaORM entity)
- New database table: `prompt_prefix_sessions`
