# Tasks for fix-stop-hook-output-control

## Implementation Tasks

### 1. Fix eprintln diagnostic output to respect showStdout/showStderr flags
- [x] Modify `execute_stop_commands()` in `src/hooks.rs` (lines 657-681)
- [x] Add conditional logic to check `cmd_config.show_stdout` before including stdout in eprintln
- [x] Add conditional logic to check `cmd_config.show_stderr` before including stderr in eprintln
- [x] Consider adding placeholder text like "(output hidden by configuration)" when flags are false
- [x] Ensure command name and exit code are always logged regardless of flags

### 2. Add integration tests for console output behavior
- [x] Create integration test that runs a failing stop command with `showStdout: false`
- [x] Verify stdout is NOT present in the console output (eprintln)
- [x] Create integration test that runs a failing stop command with `showStderr: false`
- [x] Verify stderr is NOT present in the console output (eprintln)
- [x] Create integration test with both flags false and verify no output leaks
- [x] Create integration test with both flags true and verify output is shown

### 3. Update existing unit tests
- [x] Review unit tests in `tests/output_limiting_tests.rs`
- [x] Add assertions to verify eprintln behavior matches flag settings
- [x] Ensure tests cover all combinations of showStdout/showStderr flags

## Validation Tasks

### 4. Manual testing
- [x] Create test configuration with `showStdout: false` and `showStderr: false`
- [x] Run a failing stop command and verify no output appears in console
- [x] Create test configuration with flags true and verify output appears
- [x] Test edge cases (empty output, very long output, special characters)

### 5. Run existing test suite
- [x] Run `cargo test` to ensure no regressions
- [x] Run `cargo test output_limiting` to verify output limiting tests
- [x] Fix any broken tests

### 6. Code review and cleanup
- [x] Review code changes for readability and maintainability
- [x] Add code comments explaining the output flag logic
- [x] Run `cargo clippy` and address any warnings
- [x] Run `cargo fmt` to ensure consistent formatting

## Documentation Tasks

### 7. Update documentation if needed
- [x] Check if README or other docs reference this behavior
- [x] Update examples if they show incorrect behavior
- [x] Consider adding a troubleshooting section if users report confusion
