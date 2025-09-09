# Research Findings: Grep-based Prompt System Level Enforcement

## Executive Summary

The grep-based prompt system will extend conclaude's existing hook system to enable pattern-based prompt enhancement. Key findings reveal that the current architecture supports the validation aspects but requires significant extensions to enable content modification.

## Research Questions & Findings

### 1. Hook System Integration Points

**Decision**: Extend the existing `UserPromptSubmit` hook for pattern matching and context injection.

**Rationale**: 
- The `UserPromptSubmit` hook already receives user prompt text via `UserPromptSubmitPayload.prompt`
- Hook is triggered at the exact point where pattern matching should occur (before Claude processes the prompt)
- Existing infrastructure for payload handling, logging, and error management is already established

**Alternatives Considered**:
- Creating a new hook type: Rejected - adds unnecessary complexity when existing hook serves the purpose
- Pre-tool-use hooks: Rejected - triggers too late in the process, after prompt is already processed

**Implementation Considerations**:
- Hook handler location: `handle_user_prompt_submit()` in `src/hooks.rs` (lines 318-344)
- Current behavior is pass-through logging only - needs extension for pattern matching
- Must maintain backward compatibility with existing logging functionality

### 2. Configuration Schema Extension Patterns

**Decision**: Add `prePromptSubmission` section to `.conclaude.yaml` using existing patterns.

**Rationale**:
- Follows established naming convention (camelCase with serde rename)
- Reuses existing `GrepRule` struct for pattern definitions
- Consistent with other hook configuration sections (`preToolUse`, `stop`)

**Alternatives Considered**:
- New configuration file: Rejected - inconsistent with existing centralized config approach
- Extending existing sections: Rejected - would create semantic confusion

**Implementation Considerations**:
```rust
// Add to ConclaudeConfig in src/config.rs
#[serde(default, rename = "prePromptSubmission")]
pub pre_prompt_submission: PrePromptSubmissionConfig,

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct PrePromptSubmissionConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub patterns: Vec<PatternRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PatternRule {
    pub pattern: String,           // regex or glob pattern
    pub prompt: String,           // context to inject
    pub description: String,      // human-readable description
    #[serde(default)]
    pub pattern_type: PatternType, // "regex" or "glob"
}
```

### 3. File Reference Handling (@ Syntax)

**Decision**: Implement custom @ syntax file reference resolver with robust error handling.

**Rationale**:
- No existing @ syntax support in codebase - must be built from scratch
- Existing file loading patterns provide proven error handling approach
- Configuration search path logic can be adapted for context file resolution

**Alternatives Considered**:
- Using existing path resolution: Rejected - no @ syntax support exists
- Third-party file reference libraries: Rejected - adds dependency overhead for simple feature

**Implementation Considerations**:
```rust
// File reference resolution pattern
fn resolve_file_reference(reference: &str) -> anyhow::Result<String> {
    if reference.starts_with('@') {
        let file_path = &reference[1..]; // Remove @
        let cwd = std::env::current_dir()
            .context("Failed to get current working directory")?;
        let full_path = cwd.join(file_path);
        
        fs::read_to_string(full_path)
            .with_context(|| format!("Failed to read context file: {}", file_path))
    } else {
        Ok(reference.to_string()) // Direct text prompt
    }
}
```

### 4. Prompt Processing Pipeline Integration

**Decision**: Modify `UserPromptSubmit` hook to process and enhance prompts before returning results.

**Rationale**:
- Current pipeline is pure pass-through - ideal for adding enhancement layer
- Existing payload structure provides prompt text access
- Integration point is well-defined and isolated

**Alternatives Considered**:
- Separate processing stage: Rejected - would require Claude Code architecture changes
- Post-processing hooks: Rejected - too late in the pipeline

**Implementation Considerations**:
- Current flow: Payload → Validation → Logging → Success
- Enhanced flow: Payload → Validation → Pattern Matching → Context Injection → Enhanced Result
- **Critical limitation**: `HookResult` struct only supports block/allow, not content modification

### 5. Hook Trigger Mechanism & Content Modification

**Decision**: Extend `HookResult` to support content modification alongside existing block/allow behavior.

**Rationale**:
- Current system only supports binary block/allow decisions
- Content modification requires new communication mechanism
- Must maintain backward compatibility with existing hooks

**Alternatives Considered**:
- New hook type with different result handling: Rejected - breaks consistency
- External content modification service: Rejected - adds architectural complexity
- Stdout-based content return: Rejected - conflicts with existing error messaging

**Implementation Considerations**:
```rust
// Extended HookResult structure
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct HookResult {
    pub message: Option<String>,
    pub blocked: Option<bool>,
    // NEW: Support for content modification
    pub modified_content: Option<String>,
}

// Updated communication protocol
// - Exit code 0: Success (check for modified_content)
// - Exit code 2: Blocked (display message)
// - Exit code 1: Error (hook failure)
```

### 6. Pattern Matching Implementation

**Decision**: Use existing `regex` and `glob` crates with dual pattern support.

**Rationale**:
- Both crates already exist in project dependencies
- Existing `GrepRule` pattern can be extended for prompt content matching
- Clear specification for `regex:` and `glob:` pattern prefixes

**Alternatives Considered**:
- Single pattern type: Rejected - spec requires both regex and glob support
- Custom pattern engine: Rejected - reinvents proven solutions

**Implementation Considerations**:
```rust
// Pattern matching logic
fn matches_pattern(prompt: &str, pattern_rule: &PatternRule) -> anyhow::Result<bool> {
    match pattern_rule.pattern_type {
        PatternType::Regex => {
            let regex = regex::Regex::new(&pattern_rule.pattern)?;
            Ok(regex.is_match(prompt))
        }
        PatternType::Glob => {
            let glob_pattern = glob::Pattern::new(&pattern_rule.pattern)?;
            Ok(glob_pattern.matches(prompt))
        }
    }
}
```

## Key Implementation Insights

### Architecture Modifications Required

1. **HookResult Extension**: Must support content modification, not just blocking
2. **Configuration Schema**: New `prePromptSubmission` section with pattern rules
3. **File Reference System**: Complete @ syntax implementation from scratch
4. **Pattern Engine**: Dual regex/glob matching system

### Integration Complexity Assessment

- **Low Complexity**: Configuration extension, pattern matching logic
- **Medium Complexity**: File reference resolution, error handling
- **High Complexity**: HookResult modification, content modification protocol

### Performance Considerations

- Pattern matching on every prompt submission (< 100ms target)
- File I/O for context loading (caching strategy needed)
- Multiple pattern evaluation (short-circuit on first match)

### Backward Compatibility

- All changes are additive to existing configuration schema
- Existing hooks remain unchanged
- Default behavior preserves current pass-through functionality

## Next Phase Dependencies

Based on research findings, Phase 1 (Design & Contracts) requires:

1. **Data Model**: Pattern rule schema, enhanced HookResult structure
2. **API Contracts**: Configuration schema extension, file reference resolution API
3. **Integration Testing**: Hook modification behavior, file loading edge cases
4. **Error Handling**: File missing scenarios, pattern compilation failures