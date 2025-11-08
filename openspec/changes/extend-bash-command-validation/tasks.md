# Implementation Tasks

## Configuration Schema Updates

- [ ] Add `command_pattern: Option<String>` field to `ToolUsageRule` struct in `src/config.rs`
- [ ] Add `match_mode: Option<String>` field to `ToolUsageRule` struct in `src/config.rs`
- [ ] Add serde rename attributes for camelCase: `#[serde(rename = "commandPattern")]` and `#[serde(rename = "matchMode")]`
- [ ] Update `schema.json` to include new optional `commandPattern` and `matchMode` fields with descriptions
- [ ] Add JSON schema validation for `matchMode` enum values ("full", "prefix")
- [ ] Add example configurations to schema demonstrating both matching modes

## Hook Execution Logic

- [ ] Add `extract_bash_command()` helper function in `src/hooks.rs` following pattern of `extract_file_path()`
- [ ] Extend `check_tool_usage_rules()` to detect Bash tool with commandPattern rules
- [ ] Implement full match mode: use `Pattern::new(pattern)?.matches(&command)` directly
- [ ] Implement prefix match mode: check command starts with pattern or matches pattern
- [ ] Add default matchMode logic: use "full" when `match_mode` is `None`
- [ ] Add validation to skip invalid matchMode values with logged warning
- [ ] Ensure commandPattern validation runs before file path validation for Bash tool
- [ ] Preserve existing file path validation logic for non-Bash tools and rules without commandPattern

## Error Handling and Messaging

- [ ] Implement custom error message display for blocked Bash commands
- [ ] Implement default error message format: "Bash command blocked by validation rule: {pattern}"
- [ ] Ensure HookResult::blocked() is returned with appropriate message
- [ ] Handle edge case: empty command string (skip validation)
- [ ] Handle edge case: missing command field in tool_input (skip validation)
- [ ] Handle invalid glob pattern compilation errors with clear error messages

## Testing - Unit Tests

- [ ] Write test for `extract_bash_command()` with valid command in tool_input
- [ ] Write test for `extract_bash_command()` with missing command field returns None
- [ ] Write test for `extract_bash_command()` with empty command string
- [ ] Write test for full match mode with exact pattern match (should block)
- [ ] Write test for full match mode with extra content (should not block)
- [ ] Write test for prefix match mode with matching prefix (should block)
- [ ] Write test for prefix match mode with non-matching prefix (should not block)
- [ ] Write test for default matchMode behavior when field is omitted
- [ ] Write test for glob wildcard patterns in commands
- [ ] Write test for multiple wildcards in single pattern

## Testing - Integration Tests

- [ ] Write integration test for blocking exact dangerous command (rm -rf /)
- [ ] Write integration test for allowing similar but safe command (rm -rf /tmp)
- [ ] Write integration test for prefix mode blocking command family (all curl commands)
- [ ] Write integration test for custom error message display
- [ ] Write integration test for default error message when not specified
- [ ] Write integration test verifying backward compatibility with existing file path rules
- [ ] Write integration test for mixed configuration (both command and file path rules)
- [ ] Write integration test for tool="*" applying to Bash commands with commandPattern
- [ ] Write integration test for non-Bash tools ignoring commandPattern rules
- [ ] Write integration test for invalid matchMode value handling

## Documentation and Validation

- [ ] Update README.md with Bash command validation examples
- [ ] Add configuration examples showing both matchMode options
- [ ] Document pattern matching behavior and glob syntax support
- [ ] Document security considerations and potential bypass scenarios
- [ ] Add migration guide for users adding command rules to existing configs
- [ ] Run `openspec validate extend-bash-command-validation --strict` and fix all issues
- [ ] Run `cargo test` and ensure all tests pass
- [ ] Run `cargo clippy` and fix any linter warnings
- [ ] Run integration tests against real conclaude.yml configurations

## Deployment Preparation

- [ ] Verify schema.json is valid JSON and matches config.rs structure
- [ ] Test configuration loading with sample YAML files containing new fields
- [ ] Test backward compatibility: ensure existing configs without new fields still work
- [ ] Verify error messages are clear and actionable for users
- [ ] Confirm all validation scenarios from spec are covered by tests
