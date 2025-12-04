## 1. Configuration Schema
- [x] 1.1 Add `workingDir` field to `StopCommand` struct in `src/config.rs`
- [x] 1.2 Add `workingDir` field to `SubagentStopCommand` struct in `src/config.rs`
- [x] 1.3 Add `workingDir` to `StopCommandConfig` internal struct in `src/hooks.rs`
- [x] 1.4 Add `workingDir` to `SubagentStopCommandConfig` internal struct in `src/hooks.rs`

## 2. Bash Interpolation
- [x] 2.1 Create helper function to perform bash interpolation on `workingDir` string
- [x] 2.2 Handle environment variable expansion, command substitution, and tilde expansion
- [x] 2.3 Add error handling for interpolation failures
- [x] 2.4 Add validation for empty interpolation results

## 3. Path Resolution
- [x] 3.1 Create helper function to resolve interpolated path (absolute or relative to config)
- [x] 3.2 Add logic to detect if path is absolute vs relative after interpolation
- [x] 3.3 Resolve relative paths relative to config file directory
- [x] 3.4 Validate that resolved path exists and is a directory
- [x] 3.5 Integrate full resolution pipeline into `collect_stop_commands` function
- [x] 3.6 Integrate full resolution pipeline into `collect_subagent_stop_commands` function

## 4. Command Execution
- [x] 4.1 Modify `execute_stop_commands` to use `.current_dir()` when workingDir is set
- [x] 4.2 Modify `execute_subagent_stop_commands` to use `.current_dir()` when workingDir is set
- [x] 4.3 Ensure resolved path is used (after interpolation and validation)

## 5. JSON Schema Update
- [x] 5.1 Regenerate JSON schema to include the new field
- [x] 5.2 Verify schema validation accepts `workingDir`

## 6. Testing
- [x] 6.1 Add unit tests for bash interpolation helper
- [x] 6.2 Add unit tests for path resolution helper
- [x] 6.3 Add tests for stop commands with workingDir
- [x] 6.4 Add tests for subagent stop commands with workingDir
- [x] 6.5 Add tests for relative vs absolute paths
- [x] 6.6 Add tests for environment variable expansion
- [x] 6.7 Add tests for command substitution (e.g., git rev-parse)
- [x] 6.8 Add tests for tilde expansion
- [x] 6.9 Add tests for interpolation failure error handling
- [x] 6.10 Add tests for empty interpolation result error handling
- [x] 6.11 Add tests for non-existent directory error handling
- [x] 6.12 Add tests for file instead of directory error handling

## 7. Documentation
- [x] 7.1 Update default configuration example if applicable
- [x] 7.2 Document bash interpolation requirement (bash must be available)
