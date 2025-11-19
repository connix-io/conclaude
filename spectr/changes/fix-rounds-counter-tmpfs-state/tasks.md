# Implementation Tasks

## 1. Core Implementation

- [ ] 1.1 Remove static `ROUND_COUNT: AtomicU32` from `src/hooks.rs:560-561`
- [ ] 1.2 Create helper function `get_rounds_state_path(session_id: &str) -> PathBuf` to generate tmpfs file path
- [ ] 1.3 Create function `read_round_count(session_id: &str) -> Result<u32>` to read current round from tmpfs
- [ ] 1.4 Create function `write_round_count(session_id: &str, count: u32) -> Result<()>` to write round count to tmpfs
- [ ] 1.5 Create function `cleanup_rounds_state(session_id: &str) -> Result<()>` to remove tmpfs state file
- [ ] 1.6 Update `handle_stop()` function to use tmpfs-based round counting (lines 601-610)
- [ ] 1.7 Implement error handling with fail-open behavior (log errors but allow operation)

## 2. Testing

- [ ] 2.1 Write unit test for `get_rounds_state_path()` function
- [ ] 2.2 Write unit test for `read_round_count()` with non-existent file (should return 0)
- [ ] 2.3 Write unit test for `write_round_count()` and `read_round_count()` round trip
- [ ] 2.4 Write integration test simulating multiple hook invocations with same session ID
- [ ] 2.5 Write test for cleanup behavior when max rounds reached
- [ ] 2.6 Write test for concurrent sessions with different session IDs
- [ ] 2.7 Write test for graceful handling of tmpfs I/O errors

## 3. Validation

- [ ] 3.1 Run `cargo test` to verify all tests pass
- [ ] 3.2 Run `cargo build` to ensure no compilation errors
- [ ] 3.3 Manual testing with actual `.conclaude.yaml` rounds configuration
- [ ] 3.4 Verify tmpfs files are created in correct location
- [ ] 3.5 Verify tmpfs files are cleaned up after max rounds or session end

## 4. Documentation

- [ ] 4.1 Update inline documentation for rounds counter implementation
- [ ] 4.2 Add doc comments to new helper functions
- [ ] 4.3 Update error messages to be clear and actionable
