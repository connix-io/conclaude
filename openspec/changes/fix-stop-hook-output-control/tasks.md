# Tasks for fix-stop-hook-output-control

## Implementation Tasks

### 1. Fix eprintln diagnostic output to respect showStdout/showStderr flags
- [ ] Modify `execute_stop_commands()` in `src/hooks.rs` (lines 657-681)
- [ ] Add conditional logic to check `cmd_config.show_stdout` before including stdout in eprintln
- [ ] Add conditional logic to check `cmd_config.show_stderr` before including stderr in eprintln
- [ ] Consider adding placeholder text like "(output hidden by configuration)" when flags are false
- [ ] Ensure command name and exit code are always logged regardless of flags

### 2. Add integration tests for console output behavior
- [ ] Create integration test that runs a failing stop command with `showStdout: false`
- [ ] Verify stdout is NOT present in the console output (eprintln)
- [ ] Create integration test that runs a failing stop command with `showStderr: false`
- [ ] Verify stderr is NOT present in the console output (eprintln)
- [ ] Create integration test with both flags false and verify no output leaks
- [ ] Create integration test with both flags true and verify output is shown

### 3. Update existing unit tests
- [ ] Review unit tests in `tests/output_limiting_tests.rs`
- [ ] Add assertions to verify eprintln behavior matches flag settings
- [ ] Ensure tests cover all combinations of showStdout/showStderr flags

## Validation Tasks

### 4. Manual testing
- [ ] Create test configuration with `showStdout: false` and `showStderr: false`
- [ ] Run a failing stop command and verify no output appears in console
- [ ] Create test configuration with flags true and verify output appears
- [ ] Test edge cases (empty output, very long output, special characters)

### 5. Run existing test suite
- [ ] Run `cargo test` to ensure no regressions
- [ ] Run `cargo test output_limiting` to verify output limiting tests
- [ ] Fix any broken tests

### 6. Code review and cleanup
- [ ] Review code changes for readability and maintainability
- [ ] Add code comments explaining the output flag logic
- [ ] Run `cargo clippy` and address any warnings
- [ ] Run `cargo fmt` to ensure consistent formatting

## Documentation Tasks

### 7. Update documentation if needed
- [ ] Check if README or other docs reference this behavior
- [ ] Update examples if they show incorrect behavior
- [ ] Consider adding a troubleshooting section if users report confusion
