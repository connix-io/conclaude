# Change: Remove Rounds Feature

## Why

The `rounds` feature in the stop hook configuration adds complexity with minimal user value. It was designed to allow Claude to continue for a specific number of iterations, but:

1. **Unreliable Implementation**: The current implementation uses a static `AtomicU32` that resets between hook process invocations, making round counting fundamentally broken (as documented in the pending `fix-rounds-counter-tmpfs-state` change)
2. **Overlapping Functionality**: The `infinite` mode already provides the core use case (allowing Claude to continue working)
3. **Limited Use Cases**: It's unclear when users would need exactly N rounds vs infinite mode or single execution
4. **Maintenance Burden**: Fixing rounds properly requires tmpfs state management, adding complexity for marginal benefit

Since the user has explicitly stated "no worries for backwards compat", we can cleanly remove this feature.

## What Changes

- Remove `rounds` field from `StopConfig` struct in `src/config.rs`
- Remove round counting logic from `handle_stop()` in `src/hooks.rs` (static `ROUND_COUNT`, fetch_add, comparison logic)
- Remove `rounds` validation from `validate_config_constraints()` in `src/config.rs`
- Remove `rounds` from JSON Schema in `conclaude-schema.json`
- Remove `rounds` documentation from `src/default-config.yaml`
- Remove `rounds` mentions from `README.md`
- Remove `rounds` from field list generation in configuration error suggestions
- Update tests that reference rounds
- Close/supersede the `fix-rounds-counter-tmpfs-state` change as no longer needed

## Impact

- **Affected specs**:
  - `hooks-system` (stop hook behavior)
  - `configuration` (stop configuration schema)
- **Affected code**:
  - `src/config.rs` (StopConfig struct, validation, field lists)
  - `src/hooks.rs` (handle_stop function, round counting logic)
  - `src/default-config.yaml` (default configuration template)
  - `conclaude-schema.json` (JSON schema)
  - `README.md` (feature documentation)
  - Tests in `tests/hooks_tests.rs` (any round-related tests)
- **Breaking changes**: **YES** - Users with `rounds` in their config will get validation errors. Migration: remove `rounds` field, use `infinite: true` instead for continuous operation.
- **Supersedes**: `fix-rounds-counter-tmpfs-state` change (no longer needed)
