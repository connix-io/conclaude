# Architectural Design: Consolidate Rules Configuration

## Problem Statement

The current configuration structure splits file protection rules across two sections:

```yaml
# Current structure (fragmented)
rules:
  preventRootAdditions: true
  uneditableFiles: ["*.lock", "package.json"]
  toolUsageValidation: [...]  # Future work

preToolUse:
  preventAdditions: ["src/generated/**"]
  preventGeneratedFileEdits: true
  generatedFileMessage: null
```

This creates confusion because:
1. **Semantic overlap** - Both `rules.uneditableFiles` and `preToolUse.preventAdditions` are about protecting files
2. **Conceptual mismatch** - File protection is fundamentally a pre-tool-use validation concern, not a separate "rules" domain
3. **Naming inconsistency** - Three different field names for related concepts (`preventRootAdditions`, `uneditableFiles`, `preventAdditions`)
4. **Documentation burden** - Users must understand two sections to manage file protection

## Proposed Solution

Move all file protection fields into the `preToolUse` section, creating a cohesive "file protection policy" configuration:

```yaml
# Proposed structure (consolidated)
preToolUse:
  # Root-level file protection
  preventRootAdditions: true

  # Pattern-based file protection
  uneditableFiles: ["*.lock", "package.json"]

  # Path-based addition prevention (keep existing name for now)
  preventAdditions: ["src/generated/**"]

  # Generated file protection
  preventGeneratedFileEdits: true
  generatedFileMessage: null

# rules section is removed entirely
# (toolUsageValidation deferred to future spec)
```

## Design Rationale

### 1. Semantic Cohesion
All fields in `preToolUse` are validation constraints applied **before** tool execution to prevent unintended file modifications. Moving file protection rules here aligns configuration structure with actual execution semantics.

### 2. Single Responsibility
Rather than having `rules` as a catch-all for various validation concerns, `preToolUse` becomes the authoritative place for pre-execution file validation policies.

### 3. Progressive Enhancement
The `preToolUse` section already exists and handles file path exclusions. Consolidation is an incremental improvement rather than introducing entirely new sections.

### 4. Future Extensibility
Consolidation creates a clear path for future enhancements:
- Adding new file protection strategies to `preToolUse`
- Introducing inheritance/composition of file protection rules
- Better error messages by centralizing validation logic

## Implementation Approach

### Phase 1: Configuration Structure
1. Add `preventRootAdditions` and `uneditableFiles` to `PreToolUseConfig` struct
2. Update JSON schema to reflect new structure
3. Remove `RulesConfig` struct and related definitions
4. Update default configuration template

### Phase 2: Migration and Deprecation
1. Implement deprecation detection (old `rules` section still loads with warning)
2. Update all documentation with before/after examples
3. Add migration helpers if needed

### Phase 3: Cleanup
1. Remove deprecated `rules` section support after deprecation period
2. Clean up migration code and internal references

## Field Consolidation Details

### preventRootAdditions
- **Current location**: `rules.preventRootAdditions`
- **New location**: `preToolUse.preventRootAdditions`
- **Type**: `boolean`
- **Default**: `true`
- **Semantics**: Prevents Claude from creating/modifying files without directory separators (root-level files)
- **Rationale**: Root-level protection is a pre-tool-use validation constraint

### uneditableFiles
- **Current location**: `rules.uneditableFiles`
- **New location**: `preToolUse.uneditableFiles`
- **Type**: `Vec<String>` (glob patterns)
- **Default**: `[]`
- **Semantics**: Glob patterns for files Claude cannot edit
- **Rationale**: File pattern protection is fundamentally a pre-tool-use concern, complements existing `preventAdditions`

### preventAdditions (no movement)
- **Location**: `preToolUse.preventAdditions` (already consolidated)
- **Type**: `Vec<String>` (glob patterns)
- **Default**: `[]`
- **Semantics**: Glob patterns for paths Claude cannot create files in
- **Rationale**: Already in the correct location; no change needed

### toolUsageValidation (Movement)
- **Current location**: `rules.toolUsageValidation`
- **New location**: `preToolUse.toolUsageValidation`
- **Type**: `Vec<ToolUsageRule>`
- **Default**: `[]`
- **Semantics**: Per-tool restrictions that control which tools can operate on which files
- **Rationale**: Tool usage validation is fundamentally a pre-tool-use validation concern

## Validation Strategy

### Configuration Loading
```
1. Load YAML configuration
2. If `rules` section detected:
   - FAIL with clear error message
   - Indicate that rules section is no longer supported
   - Provide specific field-by-field migration instructions
3. Validate preToolUse structure (including all new fields)
4. Build runtime config
```

### Runtime Validation
- File protection checks occur in the `preToolUse` hook handler
- Consolidated fields allow for composite validation across all protection rules
- Error messages clearly reference the consolidated configuration location
- When multiple rules match, first matched rule is reported (clear singular error)


## Migration Examples

### Before (Old Configuration)
```yaml
rules:
  preventRootAdditions: true
  uneditableFiles:
    - "package.json"
    - "Cargo.toml"
    - "*.lock"
  toolUsageValidation:
    - tool: "bash"
      pattern: "*.md"
      action: "block"

preToolUse:
  preventAdditions:
    - "src/generated/**"
  preventGeneratedFileEdits: true
```

### After (New Configuration)
```yaml
preToolUse:
  preventRootAdditions: true
  uneditableFiles:
    - "package.json"
    - "Cargo.toml"
    - "*.lock"
  toolUsageValidation:
    - tool: "bash"
      pattern: "*.md"
      action: "block"
  preventAdditions:
    - "src/generated/**"
  preventGeneratedFileEdits: true
```

## Clarifications and Decisions

### toolUsageValidation
The `rules.toolUsageValidation` field (per-tool restrictions) is **moved to `preToolUse`** as part of this consolidation. This completes the migration of all validation rules into the pre-tool-use hook configuration, creating a unified validation policy section.

### Default Behavior
- `preventRootAdditions` defaults to `true` (secure by default)
- `uneditableFiles` defaults to empty array `[]` (no files protected by default other than via root prevention)
- `toolUsageValidation` defaults to empty array `[]` (no per-tool restrictions by default)

### Error Handling Strategy
When a file operation violates multiple protection rules, the first matched rule is reported in the error message. This prevents message confusion and provides a clear, singular reason for the rejection.

### Backward Compatibility
This is a **hard break** - no support for old `rules` section. Configuration loading fails immediately with clear error messages if the old format is detected. This approach:
- Simplifies code (no migration logic)
- Forces users to update configs (clean slate)
- Provides clear feedback (not silent failures)

## Risk Assessment

### Low Risk
- Configuration loading is isolated; changes don't affect runtime logic
- Clear deprecation path; old configs can still work with warnings
- Well-defined migration examples

### Medium Risk
- Users must update configurations (breaking change)
- Documentation must be comprehensive to avoid confusion
- Migration tooling may be needed for large projects

### Mitigation
- Clear deprecation warnings with migration examples
- Updated documentation with before/after comparisons
- Optional migration helper command if needed
