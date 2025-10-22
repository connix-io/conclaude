# Proposal: Fix Rounds Counter Using tmpfs File State

## Why

The current rounds counter implementation in `src/hooks.rs:560-609` uses a static `AtomicU32` that persists across hook invocations but resets when the hook binary is re-executed. This creates unreliable round counting, as each hook invocation creates a fresh process with a reset counter. For proper session-based round counting, state must persist across process invocations using tmpfs file-based storage.

## What Changes

- Replace the static `AtomicU32` rounds counter with tmpfs file-based state tracking
- Use `/tmp/conclaude-{session_id}.rounds` or similar tmpfs location to store the current round count
- Implement atomic file operations to read, increment, and write round counts
- Ensure proper cleanup of tmpfs state files when sessions end or max rounds are reached
- Add error handling for file I/O operations
- Update tests to verify tmpfs-based round counting behavior

## Impact

- **Affected specs**: `hook-execution` (rounds mode functionality)
- **Affected code**:
  - `src/hooks.rs` (handle_stop function, lines 560-609)
  - Tests in `tests/hooks_tests.rs` (new tests for tmpfs state persistence)
- **Breaking changes**: None - this fixes broken functionality to match the documented behavior
- **Dependencies**: Standard library `fs` module (already in use)
