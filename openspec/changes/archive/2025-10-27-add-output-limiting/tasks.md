## 1. Configuration Schema Updates
- [x] 1.1 Add `maxOutputLines` field to `StopCommand` struct in `src/config.rs`
- [x] 1.2 Update field to use `Option<u32>` type with appropriate serde attributes
- [x] 1.3 Add field to JSON schema in `conclaude-schema.json`
- [x] 1.4 Update default configuration YAML with example in `src/default-config.yaml`
- [x] 1.5 Update project configuration YAML with example in `.conclaude.yaml`

## 2. Output Limiting Implementation
- [x] 2.1 Modify `StopCommandConfig` struct in `src/hooks.rs` to include `max_output_lines` field
- [x] 2.2 Update `collect_stop_commands` function to extract and pass `maxOutputLines` value
- [x] 2.3 Implement output truncation logic in `execute_stop_commands` function
- [x] 2.4 Add line counting for stdout with truncation at configured limit
- [x] 2.5 Add line counting for stderr with truncation at configured limit
- [x] 2.6 Generate truncation indicators with accurate omitted line counts
- [x] 2.7 Ensure truncation only applies when `show_stdout` or `show_stderr` are enabled

## 3. Testing
- [x] 3.1 Add unit tests for output limiting with various line counts
- [x] 3.2 Add test for output within limit (no truncation)
- [x] 3.3 Add test for output exceeding limit (with truncation indicator)
- [x] 3.4 Add test for independent stdout and stderr limiting
- [x] 3.5 Add test for configuration validation (invalid values)
- [x] 3.6 Add test for backward compatibility (no maxOutputLines field)
- [x] 3.7 Add test for interaction with showStdout/showStderr flags
- [x] 3.8 Run existing test suite to ensure no regressions

## 4. Documentation
- [x] 4.1 Update default-config.yaml comments to explain maxOutputLines usage
- [x] 4.2 Add practical examples showing output limiting scenarios
- [x] 4.3 Document interaction between maxOutputLines and show* flags
- [x] 4.4 Update README.md if necessary to mention output limiting feature

## 5. Validation
- [x] 5.1 Run `cargo test` to verify all tests pass
- [x] 5.2 Run `cargo clippy` to check for warnings
- [x] 5.3 Test manually with sample commands producing large output
- [x] 5.4 Verify truncation indicators show correct omitted line counts
- [x] 5.5 Verify backward compatibility with existing configurations
- [x] 5.6 Run `openspec validate add-output-limiting --strict` to validate the proposal
