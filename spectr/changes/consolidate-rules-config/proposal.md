# Consolidate File Protection Rules into preToolUse Configuration

## Why

The current configuration has `preventRootAdditions` and `uneditableFiles` spread across the `rules` section, while `preventAdditions` (glob patterns for file protection) exists in the `preToolUse` section. This creates a confusing two-section approach to file protection that duplicates concerns:

- `rules.preventRootAdditions` - boolean flag for root-level file protection
- `rules.uneditableFiles` - glob patterns for protected files
- `preToolUse.preventAdditions` - glob patterns for protected file additions

These three fields all relate to preventing Claude from modifying certain files, making them natural candidates for consolidation. By moving all file protection logic under `preToolUse`, we:

1. **Simplify configuration** - All file protection rules in one section
2. **Reduce confusion** - Users find all related settings together
3. **Improve consistency** - Name patterns align (prevent* fields all in one place)
4. **Enable future enhancements** - Consolidation enables better validation and documentation

## What Changes

### Configuration Structure Changes
- Move `rules.preventRootAdditions` → `preToolUse.preventRootAdditions`
- Move `rules.uneditableFiles` → `preToolUse.uneditableFiles`
- Move `rules.toolUsageValidation` → `preToolUse.toolUsageValidation`
- Remove the entire `rules` section from the configuration schema

### Files Affected
- **src/config.rs** - Update struct definitions and defaults
- **src/default-config.yaml** - Update default configuration template
- **conclaude-schema.json** - Update JSON schema to reflect new structure
- **Documentation** - Update examples and defaults

### Schema Changes
- Remove `RulesConfig` struct from Rust code
- Add `preventRootAdditions` and `uneditableFiles` to `PreToolUseConfig`
- Update JSON schema definitions
- Maintain backward compatibility through migration guide (not in this spec)

### Impact
- **Breaking change**: Yes, users will need to update their configurations to move fields to `preToolUse`
- **Migration path**: Will be documented with clear before/after examples
- **Backward compatibility**: None - old `rules` section is not supported; configuration loading will fail with clear error messages

## Architecture Notes

The `preToolUse` section currently handles preventing additions to certain paths. Consolidating root addition prevention and uneditable file protection into this section creates a cohesive "file protection" policy within the `preToolUse` hook configuration. This reflects the semantic reality: all these rules are validation constraints applied before tool execution to protect files.

## Clarifications Resolved

**toolUsageValidation handling**: The `rules.toolUsageValidation` field (per-tool restrictions) is moved to `preToolUse` as part of this consolidation. This completes the migration of all rule-based validations into the pre-tool-use hook configuration.

**Default behavior**: `preventRootAdditions` defaults to `true` (secure by default).

**Error handling**: When file violations occur, the first matched protection rule is reported in the error message.

**Backward compatibility**: This is a hard break - the old `rules` section is not supported. Configuration loading will fail with clear error messages if the old format is detected.
