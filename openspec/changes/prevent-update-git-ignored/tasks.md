# Implementation Tasks: preventUpdateGitIgnored Setting

## Phase 1: Configuration and Schema Updates

### Task 1.1: Add field to PreToolUseConfig struct
- **Description**: Add `prevent_update_git_ignored: bool` field to `PreToolUseConfig` in `src/config.rs`
- **Default**: `false`
- **Serde attribute**: `#[serde(default, rename = "preventUpdateGitIgnored")]`
- **Validation**: Ensure type is boolean during config loading
- **Status**: `- [ ]`

### Task 1.2: Update default-config.yaml
- **Description**: Add `preventUpdateGitIgnored: false` to the default configuration template
- **Location**: `src/default-config.yaml` in the `preToolUse` section
- **Documentation**: Add inline comment explaining the setting
- **Status**: `- [ ]`

### Task 1.3: Update JSON schema
- **Description**: Add `preventUpdateGitIgnored` field to `conclaude-schema.json`
- **Type**: `boolean`
- **Default**: `false`
- **Description**: "Block Claude from modifying or creating files that match .gitignore patterns"
- **Status**: `- [ ]`

### Task 1.4: Update field lists in config.rs
- **Description**: Add `"preventUpdateGitIgnored"` to field name lists in `config.rs`
- **Location**: `PreToolUseConfig::field_names()` and related help text
- **Status**: `- [ ]`

## Phase 2: Git-Ignore Detection Implementation

### Task 2.1: Create git-ignore parsing module
- **Description**: Create new module `src/gitignore.rs` to handle git-ignore parsing and matching
- **Functionality**:
  - Load `.gitignore` from repository root and subdirectories
  - Parse gitignore patterns respecting git semantics
  - Support negation patterns (`!`)
  - Support comments (`#`)
- **Dependencies**: Use `gitignore` crate (or similar) for pattern matching
- **Status**: `- [ ]`

### Task 2.2: Implement pattern matching against git-ignore rules
- **Description**: Implement pattern evaluation function `matches_gitignore(path: &Path) -> bool`
- **Behavior**:
  - Check if file path matches any git-ignore pattern
  - Respect git semantics (anchoring, wildcards, negation)
  - Handle nested `.gitignore` files
  - Return `true` if file is ignored
- **Status**: `- [ ]`

### Task 2.3: Integrate git-ignore check into preToolUse hook handler
- **Description**: Update `handle_pre_tool_use()` in `src/hooks.rs`
- **Logic**:
  - Check if `config.pre_tool_use.prevent_update_git_ignored` is `true`
  - Only process Read, Write, Edit operations (Glob operations skip this check)
  - If enabled, check if requested file path matches git-ignore patterns
  - Block operation if matched with clear error message
  - Allow operation if not matched
- **Status**: `- [ ]`

## Phase 3: Error Handling and Messages

### Task 3.1: Create error message for git-ignored file operations
- **Description**: Add clear, actionable error messages for blocked operations
- **Message should include**:
  - The file path that was blocked
  - Indication that it's git-ignored
  - The matching `.gitignore` pattern(s)
  - Suggestion to update `.gitignore` or disable setting
- **Status**: `- [ ]`

### Task 3.2: Implement error propagation to Claude
- **Description**: Ensure error messages are properly returned to Claude via hook result
- **Integration**: Update `HookResult` structures to include detailed error messages
- **Status**: `- [ ]`

## Phase 4: Testing

### Task 4.1: Unit tests for git-ignore pattern matching
- **Description**: Add tests in `tests/gitignore_tests.rs`
- **Test cases**:
  - Simple patterns (`node_modules`, `*.log`)
  - Glob patterns (`**/*.test.ts`, `src/**/`)
  - Negation patterns (`!important.log`)
  - Directory patterns (`dist/`)
  - Anchored patterns (`/build`)
  - Nested `.gitignore` files
  - Comments and empty lines
- **Status**: `- [ ]`

### Task 4.2: Integration tests for preToolUse hook blocking
- **Description**: Add tests in `tests/hooks_tests.rs`
- **Test cases**:
  - Block Read operations on git-ignored files
  - Block Write operations (file creation) for git-ignored paths
  - Block Edit operations (file modification) for git-ignored paths
  - Allow Glob operations even on git-ignored files
  - Allow file operations on non-ignored files
  - Verify error messages include correct information
  - Test combined with other file protection rules
  - Test with `preventUpdateGitIgnored: false` (no blocking)
- **Status**: `- [ ]`

### Task 4.3: Configuration validation tests
- **Description**: Add tests for configuration loading
- **Test cases**:
  - Valid boolean values (`true`, `false`)
  - Invalid non-boolean values
  - Missing field defaults to `false`
  - Multiple protection rules together
- **Status**: `- [ ]`

## Phase 5: Documentation

### Task 5.1: Update default config examples
- **Description**: Add documented example of `preventUpdateGitIgnored` setting
- **Location**: Inline comments in `src/default-config.yaml`
- **Status**: `- [ ]`

### Task 5.2: Create migration/usage documentation
- **Description**: Document how users can use this setting
- **Content**:
  - Explanation of when to use `preventUpdateGitIgnored`
  - Comparison with `uneditableFiles` approach
  - Example configurations
  - Performance considerations
- **Location**: Documentation files (TBD based on project docs structure)
- **Status**: `- [ ]`

### Task 5.3: Update schema documentation
- **Description**: Ensure JSON schema includes helpful descriptions
- **Status**: `- [ ]`

## Phase 6: Validation and Integration

### Task 6.1: Run configuration schema validation
- **Description**: Validate configurations against updated schema
- **Command**: `openspec validate prevent-update-git-ignored --strict`
- **Status**: `- [ ]`

### Task 6.2: Manual testing with real repositories
- **Description**: Test setting with actual repositories containing `.gitignore`
- **Test scenarios**:
  - Repository with standard `.gitignore` (node_modules, dist, .env)
  - Repository with complex nested patterns
  - Verify no false positives or false negatives
- **Status**: `- [ ]`

### Task 6.3: Verify no performance regression
- **Description**: Test that `preventUpdateGitIgnored: false` has zero performance impact
- **Measurement**: Profile file operation performance
- **Status**: `- [ ]`

## Notes

- Tasks should be completed in order (Phase 1 → Phase 2 → Phase 3 → Phase 4 → Phase 5 → Phase 6)
- Some tasks can be parallelized within a phase (e.g., tests can be written while implementation progresses)
- Configuration validation must be done before schema finalization
- All tests must pass before final integration
