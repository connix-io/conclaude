# Design Document: Custom Messages for Uneditable Files

## Architectural Overview

This change introduces context-aware error messaging for the `uneditableFiles` validation rule. The design maintains backward compatibility while enabling per-pattern customization.

## Current Architecture

```
┌─────────────────────┐
│  RulesConfig        │
│  uneditable_files:  │
│    Vec<String>      │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│ hooks.rs            │
│ matches_uneditable_ │
│   pattern()         │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│  Generic Error:     │
│  "Blocked {op}      │
│   operation: file   │
│   matches uneditable│
│   pattern '{pat}'"  │
└─────────────────────┘
```

## Proposed Architecture

```
┌────────────────────────────┐
│  RulesConfig               │
│  uneditable_files:         │
│    Vec<UnEditableFileRule> │
│      - pattern: String     │
│      - message: Option<Str>│
└───────────────┬────────────┘
                │
                ▼
┌────────────────────────────┐
│  hooks.rs                  │
│  Check pattern match       │
│  Extract custom message    │
│  (if present)              │
└───────────────┬────────────┘
                │
         ┌──────┴──────┐
         │             │
         ▼             ▼
  ┌────────────┐  ┌───────────────┐
  │ Custom:    │  │ Generic:      │
  │ (provided) │  │ (fallback)    │
  └────────────┘  └───────────────┘
         │              │
         └──────┬───────┘
                ▼
         ┌──────────────┐
         │ Error Result │
         └──────────────┘
```

## Design Decisions

### 1. New Type: `UnEditableFileRule`

**Decision**: Create a dedicated struct instead of using a more generic approach.

```rust
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]  // Enable dual format support
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

**Rationale**:
- **Backward compatibility**: The `#[serde(untagged)]` enum allows both `"*.lock"` (string) and `{pattern: "*.lock", message: "..."}` (object) formats
- **Type safety**: Rust's enum provides compile-time guarantees
- **Single responsibility**: Each variant represents a valid configuration state
- **Future extensibility**: Can add more fields to `Detailed` variant without breaking changes

**Alternative considered**: Using `UnEditableFileRule` struct with fallback deserialization. This would require custom deserialization code, which is more complex but also type-safe. The enum approach is cleaner.

### 2. Deserialization Strategy

**Decision**: Use `#[serde(untagged)]` to support both formats transparently.

This allows serde to automatically handle:
- `uneditableFiles: ["*.lock"]` → `Simple("*.lock")`
- `uneditableFiles: [{pattern: "*.lock", message: "..."}]` → `Detailed { pattern, message }`
- Mixed arrays work automatically

**Why this works**:
1. Serde tries deserialization variants in order (Simple first, then Detailed)
2. If `Simple(String)` fails to parse, it tries `Detailed { ... }`
3. No custom code needed; leverages serde's built-in logic

### 3. Message Usage in Error Path

**Location**: `src/hooks.rs` in `validate_tool_use` function, around line 322-342

**Current code**:
```rust
let error_message = format!(
    "Blocked {} operation: file matches uneditable pattern '{}'. File: {}",
    payload.tool_name, pattern, file_path
);
```

**Updated code**:
```rust
let error_message = match &config.rules.uneditable_files[i] {
    UnEditableFileRule::Detailed { message: Some(msg), .. } => msg.clone(),
    _ => format!(
        "Blocked {} operation: file matches uneditable pattern '{}'. File: {}",
        payload.tool_name, pattern, file_path
    ),
};
```

**Rationale**:
- Pattern matching is idiomatic Rust
- Falls back to generic message seamlessly
- No string interpolation complexity
- Single point of change

### 4. Configuration Validation

**Scope**: Keep simple in v1.

What we do NOT implement yet:
- Message template substitution (e.g., `{file_path}`, `{pattern}`)
- Message length limits
- Multi-line validation (YAML handles this naturally)

**Rationale**: These can be added in future versions without breaking changes. The core feature (custom message display) is self-contained.

## Backward Compatibility Analysis

### ✓ Existing Configs Continue to Work

```yaml
# Old format still works
rules:
  uneditableFiles:
    - "*.lock"
    - ".env"
```

Will deserialize as:
```rust
vec![
    UnEditableFileRule::Simple("*.lock".to_string()),
    UnEditableFileRule::Simple(".env".to_string()),
]
```

Error messages use the generic format (no custom message provided).

### ✓ No Runtime Breaking Changes

The enum `Detailed` variant includes `pattern` and optional `message`. Extracting the pattern is identical for both variants:

```rust
fn get_pattern(rule: &UnEditableFileRule) -> &str {
    match rule {
        UnEditableFileRule::Simple(p) => p,
        UnEditableFileRule::Detailed { pattern, .. } => pattern,
    }
}
```

### ✓ Schema Evolution

The generated JSON schema will reflect both formats, allowing validators and IDE autocomplete to guide users to either format.

## Implementation Roadmap

### Phase 1: Type System (Low Risk)

1. Define `UnEditableFileRule` enum in `src/config.rs`
2. Update `RulesConfig.uneditable_files` type
3. Ensure code compiles

**Impact**: Deserialization automatically works due to serde.

### Phase 2: Message Handling (Localized Change)

1. Update `hooks.rs` to extract custom message
2. Use message in error reporting

**Impact**: Single function updated, no cascade effects.

### Phase 3: Testing & Validation

1. Test backward compatibility (string format)
2. Test new format (object with pattern + message)
3. Test mixed arrays
4. Verify error messages appear correctly

**Impact**: Builds confidence in implementation.

### Phase 4: Schema & Documentation

1. Regenerate JSON schema (automatic)
2. Update README with examples
3. Update default config template

**Impact**: Users can leverage IDE autocomplete.

## Risks and Mitigation

| Risk | Severity | Mitigation |
|------|----------|-----------|
| Serde deserialization order causes unexpected variant matching | Low | Test with mixed arrays; document that variant order matters |
| Large messages slow down error logging | Low | No length validation yet; users responsible for reasonable lengths |
| Future message templating breaks old configs | Low | Design with raw strings now; templating is opt-in feature |
| Schema regeneration breaks consumers | Low | Schema is always re-published; consumers use latest |

## Testing Strategy

### Unit Tests

1. **Deserialization Tests**:
   - Parse `"*.lock"` as Simple variant
   - Parse `{pattern: "*.lock", message: "..."}` as Detailed variant
   - Parse mixed array
   - Verify pattern extraction

2. **Error Message Tests**:
   - Custom message used when present
   - Generic message used when absent
   - File path and pattern included in generic message

3. **Round-trip Tests**:
   - Serialize and deserialize config
   - Verify patterns and messages preserved

### Integration Tests

1. Pre-tool-use hook with uneditable file match
2. Verify custom message in response (when provided)
3. Verify generic message in response (when not provided)

## Future Enhancements (Out of Scope)

1. **Message templates**: Support `{file_path}`, `{pattern}` substitution
2. **Message validation**: Enforce length limits or no-newline rules
3. **Message localization**: Conditional messages based on environment
4. **Rule grouping**: Group related patterns with shared messages
5. **Regex support**: Use regex instead of glob patterns (larger change)

These can be added in subsequent versions without breaking this foundation.

## Open Design Questions

1. **Detailed variant field names**: Should we use `pattern` and `message` or `file` and `reason`?
   - **Decision**: Use `pattern` and `message` for consistency with the broader config structure.

2. **Variant ordering**: If both Simple and Detailed fail to deserialize, what error message does serde show?
   - **Decision**: Test and document the actual behavior. Serde will show the last error (Detailed), which is the more specific format and thus more helpful.

3. **Default message behavior**: Should empty string `message: ""` be treated as "no custom message"?
   - **Decision**: No - if user provides empty string, we use it (which shows no additional context). This is user's choice.

## Conclusion

This design maintains backward compatibility while enabling powerful new functionality. The use of serde's `#[serde(untagged)]` keeps the implementation clean and leverages existing patterns from the codebase.

The change is localized, well-tested, and extensible for future enhancements.
