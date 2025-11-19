## 1. Configuration Structure Updates
- [ ] 1.1 Add optional `timeout` field to `StopCommand` struct in `src/config.rs`
- [ ] 1.2 Update JSON schema generation to include timeout field
- [ ] 1.3 Add validation logic for timeout field values during config loading

## 2. Command Execution Implementation
- [ ] 2.1 Modify `execute_stop_commands` function in `src/hooks.rs` to handle timeouts
- [ ] 2.2 Implement timeout logic using `tokio::time::timeout` for command execution
- [ ] 2.3 Add process termination handling when timeout is exceeded
- [ ] 2.4 Update `StopCommandConfig` struct to include timeout information
- [ ] 2.5 Modify command collection logic to pass timeout values through

## 3. Error Handling and Messaging
- [ ] 3.1 Create timeout-specific error messages
- [ ] 3.2 Update logging to include timeout events
- [ ] 3.3 Ensure proper hook result blocking when timeout occurs
- [ ] 3.4 Add context to timeout error messages (duration, command)

## 4. Testing and Validation
- [ ] 4.1 Write unit tests for timeout configuration parsing
- [ ] 4.2 Write integration tests for command execution with timeouts
- [ ] 4.3 Test backward compatibility with existing configurations
- [ ] 4.4 Test timeout scenarios with long-running commands
- [ ] 4.5 Test validation of invalid timeout values

## 5. Documentation
- [ ] 5.1 Update default configuration template to show timeout field
- [ ] 5.2 Add examples to documentation showing timeout usage
- [ ] 5.3 Update schema documentation to reflect timeout field

## 6. Validation
- [ ] 6.1 Run existing tests to ensure no regressions
- [ ] 6.2 Run new timeout-specific tests
- [ ] 6.3 Validate configuration loading with timeout values
- [ ] 6.4 Test with various timeout scenarios (short, long, none)