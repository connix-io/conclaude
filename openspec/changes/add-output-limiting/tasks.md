## 1. Configuration Schema Updates
- [ ] 1.1 Add `maxOutputLines` field to `StopCommand` struct in `src/config.rs`
- [ ] 1.2 Update field to use `Option<u32>` type with appropriate serde attributes
- [ ] 1.3 Add field to JSON schema in `conclaude-schema.json`
- [ ] 1.4 Update default configuration YAML with example in `src/default-config.yaml`
- [ ] 1.5 Update project configuration YAML with example in `.conclaude.yaml`

## 2. Output Limiting Implementation
- [ ] 2.1 Modify `StopCommandConfig` struct in `src/hooks.rs` to include `max_output_lines` field
- [ ] 2.2 Update `collect_stop_commands` function to extract and pass `maxOutputLines` value
- [ ] 2.3 Implement output truncation logic in `execute_stop_commands` function
- [ ] 2.4 Add line counting for stdout with truncation at configured limit
- [ ] 2.5 Add line counting for stderr with truncation at configured limit
- [ ] 2.6 Generate truncation indicators with accurate omitted line counts
- [ ] 2.7 Ensure truncation only applies when `show_stdout` or `show_stderr` are enabled

## 3. Testing
- [ ] 3.1 Add unit tests for output limiting with various line counts
- [ ] 3.2 Add test for output within limit (no truncation)
- [ ] 3.3 Add test for output exceeding limit (with truncation indicator)
- [ ] 3.4 Add test for independent stdout and stderr limiting
- [ ] 3.5 Add test for configuration validation (invalid values)
- [ ] 3.6 Add test for backward compatibility (no maxOutputLines field)
- [ ] 3.7 Add test for interaction with showStdout/showStderr flags
- [ ] 3.8 Run existing test suite to ensure no regressions

## 4. Documentation
- [ ] 4.1 Update default-config.yaml comments to explain maxOutputLines usage
- [ ] 4.2 Add practical examples showing output limiting scenarios
- [ ] 4.3 Document interaction between maxOutputLines and show* flags
- [ ] 4.4 Update README.md if necessary to mention output limiting feature

## 5. Validation
- [ ] 5.1 Run `cargo test` to verify all tests pass
- [ ] 5.2 Run `cargo clippy` to check for warnings
- [ ] 5.3 Test manually with sample commands producing large output
- [ ] 5.4 Verify truncation indicators show correct omitted line counts
- [ ] 5.5 Verify backward compatibility with existing configurations
- [ ] 5.6 Run `openspec validate add-output-limiting --strict` to validate the proposal
