# Consolidate Rules Configuration - Implementation Tasks

## Task Breakdown

### Phase 1: Configuration Structure Updates

#### Task 1.1: Update PreToolUseConfig Struct
- **What**: Add file protection and tool validation fields to `PreToolUseConfig` in `src/config.rs`
- **Details**:
  - Add `#[serde(default, rename = "preventRootAdditions")]` field with type `bool`, default `true`
  - Add `#[serde(default, rename = "uneditableFiles")]` field with type `Vec<String>`, default empty
  - Add `#[serde(default, rename = "toolUsageValidation")]` field with type `Vec<ToolUsageRule>`, default empty
  - Update `FieldList` derive macro to include new fields
  - Update struct documentation comments
  - Ensure `ToolUsageRule` struct is available to `PreToolUseConfig`
- **Validation**: Code compiles, new fields serialize/deserialize correctly

#### Task 1.2: Remove RulesConfig from ConclaudeConfig
- **What**: Remove `rules` field from `ConclaudeConfig` struct in `src/config.rs`
- **Details**:
  - Remove the field declaration `pub rules: RulesConfig,`
  - Delete the entire `RulesConfig` struct definition
  - Update related code that accesses `config.rules.*` to use `config.pre_tool_use.*` instead
  - Remove `RulesConfig` from serde deny_unknown_fields handling if needed
- **Validation**: Code compiles, no dangling references to `config.rules`

#### Task 1.3: Update Default Configuration Template
- **What**: Move configuration fields in `src/default-config.yaml`
- **Details**:
  - Move `preventRootAdditions`, `uneditableFiles`, and `toolUsageValidation` from `rules:` section to `preToolUse:` section
  - Remove entire `rules:` section header
  - Update comments to reflect new organization
  - Keep examples of glob patterns for all three fields
  - Add comments explaining the consolidation rationale
- **Validation**: YAML parses correctly, defaults match code expectations

#### Task 1.4: Update JSON Schema
- **What**: Regenerate or manually update `conclaude-schema.json` and `schema.json`
- **Details**:
  - Update `PreToolUseConfig` definition to include `preventRootAdditions`, `uneditableFiles`, and `toolUsageValidation` properties
  - Delete entire `RulesConfig` definition from schema
  - Remove `rules` property from main schema properties
  - Update property descriptions and examples for new fields
  - Run schema generation to ensure consistency with Rust code
- **Validation**: Schema validates against sample configurations, no undefined properties

### Phase 2: Code Migration and Hook Integration

#### Task 2.1: Update Hook Execution Logic
- **What**: Update pre-tool-use hook handler to use consolidated configuration
- **Details**:
  - Review `src/hooks.rs` for references to `config.rules.prevent_root_additions`
  - Update to use `config.pre_tool_use.prevent_root_additions`
  - Review validation logic for `uneditableFiles`
  - Update to use `config.pre_tool_use.uneditable_files`
  - Review tool usage validation logic
  - Update to use `config.pre_tool_use.tool_usage_validation`
  - Ensure all three validation checks are applied in pre-tool-use hook
- **Validation**: Hook behavior unchanged, tests pass for all three validations

#### Task 2.2: Add Old Configuration Detection and Error Handling
- **What**: Detect old `rules` section and provide clear error messages
- **Details**:
  - Implement custom deserializer or post-load check for old `rules` section
  - When detected, fail configuration loading immediately
  - Provide error message that:
    - Clearly states `rules` section is no longer supported
    - Lists each field and its new location (e.g., `preventRootAdditions` → `preToolUse.preventRootAdditions`)
    - Provides example migration for common cases
  - Update field validation functions for new fields
  - Update `field_names()` helper if used
- **Validation**: Old configs are rejected with helpful migration instructions

#### Task 2.3: Update Error Messages
- **What**: Update error messages to reference consolidated configuration
- **Details**:
  - Find all error messages referencing `rules.preventRootAdditions`
  - Find all error messages referencing `rules.uneditableFiles`
  - Find all error messages referencing `rules.toolUsageValidation`
  - Update to reference `preToolUse.preventRootAdditions`, `preToolUse.uneditableFiles`, and `preToolUse.toolUsageValidation`
  - Ensure consistency across all error outputs
  - Update runtime error messages that indicate which rule blocked an operation
- **Validation**: Error messages reflect new configuration structure and are helpful for users

### Phase 4: Testing and Validation

#### Task 4.1: Unit Tests - Configuration Loading
- **What**: Add/update unit tests for configuration loading
- **Details**:
  - Test loading `preToolUse` with `preventRootAdditions: true` and `false`
  - Test loading `preToolUse` with `uneditableFiles` glob patterns
  - Test loading `preToolUse` with `toolUsageValidation` rules
  - Test default values when fields omitted (true, [], [])
  - Test type validation for invalid inputs
  - Test detection of old `rules` section with helpful error message
  - Test error message includes migration instructions
- **Validation**: All tests pass, coverage includes all three consolidated fields

#### Task 4.2: Integration Tests - Pre-Tool-Use Hook
- **What**: Test pre-tool-use hook with consolidated configuration
- **Details**:
  - Test that root file additions are blocked when `preventRootAdditions: true`
  - Test that root file additions are allowed when `preventRootAdditions: false`
  - Test that uneditable files are protected via glob patterns
  - Test that glob patterns match correctly (exact, wildcard, nested)
  - Test that tool usage validation blocks/allows operations as configured
  - Test tool validation with command patterns
  - Test interaction between all three rules (`preventRootAdditions`, `uneditableFiles`, `toolUsageValidation`)
  - Test that first matched rule is reported in error message
- **Validation**: Hook behavior matches all requirements

#### Task 4.3: End-to-End Tests
- **What**: Test full workflow with consolidated configuration
- **Details**:
  - Load config from YAML file
  - Run hooks with actual file operations
  - Verify restrictions are enforced
  - Test error messages are helpful
- **Validation**: E2E tests pass, user experience is good

#### Task 4.4: Schema Validation Tests
- **What**: Validate JSON schema against sample configurations
- **Details**:
  - Test schema validates correct new format
  - Test schema rejects invalid field types
  - Test schema validates glob patterns
  - Generate test config files
- **Validation**: Schema validation works as expected

### Phase 5: Documentation Updates

#### Task 5.1: Update README
- **What**: Update primary documentation in `README.md`
- **Details**:
  - Update configuration examples
  - Move file protection section to preToolUse
  - Update table of contents if needed
  - Link to migration guide
- **Validation**: Documentation is accurate and clear

#### Task 5.2: Update Default Config Comments
- **What**: Ensure inline comments in `src/default-config.yaml` explain new structure
- **Details**:
  - Add comments explaining file protection philosophy
  - Document each field clearly
  - Provide usage examples
  - Add migration notes
- **Validation**: Comments are helpful for users

### Phase 6: Build and Release

#### Task 6.1: Verify Build
- **What**: Ensure entire project builds without errors
- **Details**:
  - Run `cargo build`
  - Check for compiler warnings
  - Verify schema generation (if automated)
  - Test CLI help output
- **Validation**: Clean build, no warnings

#### Task 6.2: Update Changelog
- **What**: Document breaking change in changelog
- **Details**:
  - Note configuration structure change
  - Explain migration path
  - Reference documentation
  - Mark as breaking change
- **Validation**: Changelog entry is clear

## Task Dependencies

```
1.1 PreToolUseConfig → 1.2 Remove RulesConfig
1.3 Update defaults → 1.4 Update schema
1.2, 1.3, 1.4 → 2.1 Update hooks
2.1 → 2.2 Add old config detection
2.2 → 2.3 Update error messages
2.3 → 4.1 Unit tests
4.1 → 4.2 Integration tests
4.2 → 4.3 E2E tests
4.3 → 4.4 Schema validation tests
4.4 → 5.1, 5.2 Documentation
5.1, 5.2 → 6.1 Build verification
6.1 → 6.2 Changelog
```

## Parallelizable Work

- Tasks 1.1 and 1.3 can be developed in parallel
- Tasks 1.2 and 1.4 can be developed in parallel
- Tasks 4.1, 4.2, 4.3, 4.4 can be developed in parallel after Phase 2 completes
- Tasks 5.1 and 5.2 can be developed in parallel

## Estimated Effort

- **Phase 1** (Structure): 2-3 hours
- **Phase 2** (Integration & Error Handling): 2-3 hours
- **Phase 4** (Testing): 3-4 hours
- **Phase 5** (Documentation): 1-2 hours
- **Phase 6** (Release): 0.5-1 hour

**Total**: 8.5-13 hours (reduced from original estimate due to removing deprecation phase)
