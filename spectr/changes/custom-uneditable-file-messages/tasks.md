# Implementation Tasks

## Phase 1: Type System Updates (Low Risk)

### Task 1.1: Define UnEditableFileRule enum

- **Description**: Create the `UnEditableFileRule` enum to support both string and detailed formats
- **Location**: `src/config.rs` - Add after imports or near other rule types
- **Implementation**:
  ```rust
  #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
  #[serde(untagged)]
  pub enum UnEditableFileRule {
      #[serde(rename_all = "camelCase")]
      Detailed {
          pattern: String,
          #[serde(default)]
          message: Option<String>,
      },
      Simple(String),
  }
  ```
- **Validation**: Code compiles without errors
- **Dependencies**: None - must complete before Task 1.2
- **Estimated effort**: 5 minutes

### Task 1.2: Update RulesConfig.uneditable_files type

- **Description**: Change type from `Vec<String>` to `Vec<UnEditableFileRule>`
- **Location**: `src/config.rs` - `RulesConfig` struct (around line 47)
- **Implementation**:
  ```rust
  #[serde(default, rename = "uneditableFiles")]
  pub uneditable_files: Vec<UnEditableFileRule>,
  ```
- **Validation**: Code compiles without errors
- **Dependencies**: Requires Task 1.1 complete
- **Estimated effort**: 2 minutes

### Task 1.3: Verify serde deserialization works

- **Description**: Test that serde automatically handles both string and detailed formats
- **Implementation**:
  - Load existing config with string patterns
  - Verify deserialization succeeds
  - Test with new detailed format
  - Test with mixed array
- **Validation**: All three formats deserialize correctly
- **Dependencies**: Tasks 1.1 and 1.2
- **Estimated effort**: 10 minutes (includes manual testing)

## Phase 2: Message Handling in Error Path (Localized)

### Task 2.1: Add pattern extraction helper

- **Description**: Create helper function to extract pattern from either enum variant
- **Location**: `src/hooks.rs` - near `matches_uneditable_pattern` function
- **Implementation**:
  ```rust
  fn get_pattern(rule: &UnEditableFileRule) -> &str {
      match rule {
          UnEditableFileRule::Simple(pattern) => pattern,
          UnEditableFileRule::Detailed { pattern, .. } => pattern,
      }
  }
  ```
- **Validation**: Helper function works with both variants
- **Dependencies**: Phase 1 complete
- **Estimated effort**: 5 minutes

### Task 2.2: Add custom message extraction helper

- **Description**: Create helper to get custom message from rule if present
- **Location**: `src/hooks.rs` - near `get_pattern` helper
- **Implementation**:
  ```rust
  fn get_custom_message(rule: &UnEditableFileRule) -> Option<&str> {
      match rule {
          UnEditableFileRule::Detailed { message: Some(msg), .. } => Some(msg),
          _ => None,
      }
  }
  ```
- **Validation**: Returns Some for detailed with message, None otherwise
- **Dependencies**: Phase 1 complete
- **Estimated effort**: 5 minutes

### Task 2.3: Update validate_tool_use function

- **Description**: Modify uneditable file validation to use custom message when available
- **Location**: `src/hooks.rs` - around line 322-342 in `validate_tool_use`
- **Current code**:
  ```rust
  // Check uneditableFiles rule
  for pattern in &config.rules.uneditable_files {
      if matches_uneditable_pattern(...)? {
          let error_message = format!(
              "Blocked {} operation: file matches uneditable pattern '{}'. File: {}",
              payload.tool_name, pattern, file_path
          );
          return Ok(Some(HookResult::blocked(error_message)));
      }
  }
  ```
- **Updated code**:
  ```rust
  // Check uneditableFiles rule
  for rule in &config.rules.uneditable_files {
      let pattern = get_pattern(rule);
      if matches_uneditable_pattern(...)? {
          let error_message = if let Some(msg) = get_custom_message(rule) {
              msg.to_string()
          } else {
              format!(
                  "Blocked {} operation: file matches uneditable pattern '{}'. File: {}",
                  payload.tool_name, pattern, file_path
              )
          };
          return Ok(Some(HookResult::blocked(error_message)));
      }
  }
  ```
- **Validation**: Code compiles and logic is correct
- **Dependencies**: Tasks 2.1, 2.2
- **Estimated effort**: 10 minutes

### Task 2.4: Update matches_uneditable_pattern call signature

- **Description**: Adjust call to `matches_uneditable_pattern` to pass pattern string
- **Location**: `src/hooks.rs` - where `matches_uneditable_pattern` is called
- **Current signature**: Takes pattern from loop
- **Implementation**: Pass `pattern` variable (from helper) instead of pattern object
- **Validation**: Function accepts the correct parameter type
- **Dependencies**: Task 2.1
- **Estimated effort**: 3 minutes

## Phase 3: Testing (Comprehensive)

### Task 3.1: Add deserialization test for simple string format

- **Description**: Test that simple string patterns deserialize correctly
- **Location**: `src/config.rs` in #[cfg(test)] module or `tests/config_tests.rs`
- **Implementation**:
  ```rust
  #[test]
  fn test_uneditable_simple_string_format() {
      let yaml = r#"
      rules:
        uneditableFiles:
          - "*.lock"
          - ".env"
      "#;
      let config: Config = serde_yaml::from_str(yaml).unwrap();
      assert_eq!(config.rules.uneditable_files.len(), 2);
      // Verify patterns extracted correctly
  }
  ```
- **Validation**: Test passes
- **Dependencies**: Phase 1 complete
- **Estimated effort**: 10 minutes

### Task 3.2: Add deserialization test for detailed object format

- **Description**: Test that detailed objects with pattern and message deserialize correctly
- **Location**: `src/config.rs` test module
- **Implementation**:
  ```rust
  #[test]
  fn test_uneditable_detailed_object_format() {
      let yaml = r#"
      rules:
        uneditableFiles:
          - pattern: "*.lock"
            message: "Auto-generated lock file"
      "#;
      let config: Config = serde_yaml::from_str(yaml).unwrap();
      assert_eq!(config.rules.uneditable_files.len(), 1);
      // Verify pattern and message extracted correctly
  }
  ```
- **Validation**: Test passes, custom message is accessible
- **Dependencies**: Phase 1 complete
- **Estimated effort**: 10 minutes

### Task 3.3: Add test for mixed format (strings and objects)

- **Description**: Test that arrays can mix both simple strings and detailed objects
- **Location**: `src/config.rs` test module
- **Implementation**:
  ```rust
  #[test]
  fn test_uneditable_mixed_format() {
      let yaml = r#"
      rules:
        uneditableFiles:
          - "*.lock"
          - pattern: ".env"
            message: "Secrets"
      "#;
      let config: Config = serde_yaml::from_str(yaml).unwrap();
      assert_eq!(config.rules.uneditable_files.len(), 2);
      // Verify first is simple, second has custom message
  }
  ```
- **Validation**: Test passes, mixed format works
- **Dependencies**: Phase 1 complete
- **Estimated effort**: 10 minutes

### Task 3.4: Add test for custom message in error

- **Description**: Test that custom message appears in blocked error when file matches
- **Location**: `src/hooks.rs` test module or integration tests
- **Implementation**:
  - Create config with uneditable rule with custom message
  - Simulate PreToolUse with Write to matching file
  - Verify error message equals custom message
- **Validation**: Error message matches custom message from config
- **Dependencies**: Phase 2 complete
- **Estimated effort**: 15 minutes

### Task 3.5: Add test for generic message fallback

- **Description**: Test that generic message is used when no custom message provided
- **Location**: `src/hooks.rs` test module
- **Implementation**:
  - Create config with uneditable rule without custom message
  - Simulate PreToolUse with Write to matching file
  - Verify error message uses generic format
- **Validation**: Error message follows generic format pattern
- **Dependencies**: Phase 2 complete
- **Estimated effort**: 15 minutes

### Task 3.6: Add test for pattern extraction from both formats

- **Description**: Test that pattern extraction helper works with both enum variants
- **Location**: `src/hooks.rs` test module
- **Implementation**:
  - Test `get_pattern()` with Simple variant
  - Test `get_pattern()` with Detailed variant
  - Verify both return correct pattern
- **Validation**: Both variants extract pattern correctly
- **Dependencies**: Task 2.1
- **Estimated effort**: 10 minutes

### Task 3.7: Add test for message extraction

- **Description**: Test that custom message extraction handles all cases
- **Location**: `src/hooks.rs` test module
- **Implementation**:
  - Test `get_custom_message()` with Simple variant (should return None)
  - Test `get_custom_message()` with Detailed + Some(msg) (should return Some)
  - Test `get_custom_message()` with Detailed + None (should return None)
- **Validation**: All three cases work correctly
- **Dependencies**: Task 2.2
- **Estimated effort**: 10 minutes

## Phase 4: Validation and Documentation

### Task 4.1: Run cargo test

- **Description**: Verify all tests pass including new tests
- **Command**: `cargo test`
- **Validation**: All tests pass, no failures
- **Dependencies**: Phase 3 complete
- **Estimated effort**: 2 minutes

### Task 4.2: Run cargo clippy

- **Description**: Verify no new linting warnings introduced
- **Command**: `cargo clippy -- -D warnings`
- **Validation**: No warnings or errors
- **Dependencies**: Phase 2 complete
- **Estimated effort**: 2 minutes

### Task 4.3: Verify backward compatibility with existing configs

- **Description**: Test that existing configs still work without modifications
- **Implementation**:
  - Load `.conclaude.yaml` from repo (or test fixture)
  - Verify deserialization succeeds
  - Verify behavior unchanged
- **Validation**: Existing configs work without changes
- **Dependencies**: Phase 1 complete
- **Estimated effort**: 10 minutes

### Task 4.4: Update JSON schema

- **Description**: Regenerate schema to reflect new UnEditableFileRule type
- **Implementation**:
  - Derive or generate schema from Rust code
  - Verify schema includes both string and object formats
  - Update `conclaude-schema.json`
- **Validation**: Schema is valid and reflects implementation
- **Dependencies**: Phase 1 complete
- **Estimated effort**: 5 minutes

### Task 4.5: Update example configuration

- **Description**: Add examples of custom message configurations to docs/examples
- **Location**: `src/schema.rs` (example template) or separate `examples/` directory
- **Implementation**:
  ```yaml
  rules:
    uneditableFiles:
      - pattern: "*.lock"
        message: "Lock files are auto-generated. Run 'npm install' to update."
      - pattern: ".env*"
        message: "Environment files contain secrets. Use .env.example instead."
      - "package.json"  # Can still use simple format
  ```
- **Validation**: Examples are clear and correct
- **Dependencies**: Phase 2 complete
- **Estimated effort**: 10 minutes

### Task 4.6: Update inline documentation

- **Description**: Add doc comments to UnEditableFileRule variants
- **Location**: `src/config.rs` - on `UnEditableFileRule` enum
- **Implementation**:
  ```rust
  /// Configuration for an uneditable file rule.
  ///
  /// Supports two formats:
  /// - Simple: `"*.lock"` - Matches files with generic error message
  /// - Detailed: `{pattern: "*.lock", message: "..."}` - Custom error message
  #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
  pub enum UnEditableFileRule { ... }
  ```
- **Validation**: Documentation is clear and helpful
- **Dependencies**: Phase 1 complete
- **Estimated effort**: 10 minutes

### Task 4.7: Validate with openspec

- **Description**: Run openspec validation to ensure proposal is well-formed
- **Command**: `openspec validate custom-uneditable-file-messages --strict`
- **Validation**: No validation errors
- **Dependencies**: All phases complete
- **Estimated effort**: 2 minutes

## Task Dependencies and Parallelization

```
Phase 1 (Sequential):
  1.1 → 1.2 → 1.3

Phase 2 (Sequential):
  2.1 → 2.2 → 2.3, 2.4

Phase 3 (Mostly parallel):
  3.1, 3.2, 3.3 (can run in parallel)
  → 3.4, 3.5, 3.6, 3.7 (need Phase 2, can run in parallel)

Phase 4 (Mostly sequential):
  4.1, 4.2 (depend on Phase 2 + 3)
  → 4.3 (after deserialization proven)
  → 4.4, 4.5, 4.6 (independent)
  → 4.7 (last, all other tasks)

Critical Path: 1.1 → 1.2 → 2.1 → 2.2 → 2.3 → 4.1, 4.2 → 4.7
```

## Total Estimated Effort

- Phase 1 (Type System): 12 minutes
- Phase 2 (Message Handling): 23 minutes
- Phase 3 (Testing): 80 minutes
- Phase 4 (Validation): 41 minutes

**Total: ~156 minutes (~2.6 hours)**

## Success Criteria

- [ ] UnEditableFileRule enum defined correctly
- [ ] RulesConfig updated to use new type
- [ ] Code compiles without errors
- [ ] Both simple and detailed formats deserialize correctly
- [ ] Mixed format arrays work
- [ ] Custom messages display when provided
- [ ] Generic messages display when not provided
- [ ] All tests pass
- [ ] cargo clippy passes with no warnings
- [ ] Backward compatibility verified with existing configs
- [ ] JSON schema updated and valid
- [ ] Example configurations documented
- [ ] Doc comments added
- [ ] openspec validate passes
- [ ] No functional regressions in existing behavior

## Notes

- Tests must actually be run and verified to pass (per CLAUDE.md requirements)
- The change is self-contained with minimal risk of side effects
- Serde's `#[serde(untagged)]` handles deserialization automatically - no custom code needed
- Error path changes are localized to one function
- Backward compatibility is guaranteed by the enum design
