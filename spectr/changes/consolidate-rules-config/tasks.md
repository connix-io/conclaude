## 1. Configuration Structure Updates
- [ ] 1.1 Update `PreToolUseConfig` struct with `preventRootAdditions`, `uneditableFiles`, and `toolUsageValidation` fields in `src/config.rs`
- [ ] 1.2 Remove `RulesConfig` struct and `rules` field from `ConclaudeConfig` in `src/config.rs`
- [ ] 1.3 Move configuration fields from `rules:` to `preToolUse:` section in `src/default-config.yaml`
- [ ] 1.4 Update JSON schema files (`conclaude-schema.json` and `schema.json`) to reflect new structure

## 2. Code Migration and Hook Integration
- [ ] 2.1 Update hook execution logic in `src/hooks.rs` to use `config.pre_tool_use.*` instead of `config.rules.*`
- [ ] 2.2 Add detection for old `rules` section with clear error message and migration instructions
- [ ] 2.3 Update all error messages to reference `preToolUse` instead of `rules`

## 3. Testing and Validation
- [ ] 3.1 Add unit tests for `preToolUse` configuration loading (preventRootAdditions, uneditableFiles, toolUsageValidation)
- [ ] 3.2 Add integration tests for pre-tool-use hook with consolidated configuration
- [ ] 3.3 Add end-to-end tests for full workflow with YAML configuration
- [ ] 3.4 Validate JSON schema against sample configurations

## 4. Documentation Updates
- [ ] 4.1 Update `README.md` with new configuration structure examples
- [ ] 4.2 Update inline comments in `src/default-config.yaml` to explain new structure

## 5. Build and Release
- [ ] 5.1 Verify project builds without errors (`cargo build`)
- [ ] 5.2 Update `CHANGELOG.md` to document breaking change and migration path
