# Design: Auto-Generated Field Lists

## Context

The `suggest_similar_fields` function provides user-friendly error messages when unknown configuration fields are encountered. Currently, it relies on a hardcoded mapping of section names to field lists (lines 161-202 of `src/config.rs`). This creates maintenance burden and risks drift from the actual struct definitions.

The structs involved are:
- `StopConfig` (fields: `run`, `commands`, `infinite`, `infiniteMessage`, `rounds`)
- `RulesConfig` (fields: `preventRootAdditions`, `uneditableFiles`, `toolUsageValidation`)
- `PreToolUseConfig` (fields: `preventAdditions`, `preventGeneratedFileEdits`, `generatedFileMessage`)
- `NotificationsConfig` (fields: `enabled`, `hooks`, `showErrors`, `showSuccess`, `showSystemEvents`)
- `StopCommand` (fields: `run`, `message`, `showStdout`, `showStderr`, `maxOutputLines`)

## Goals / Non-Goals

### Goals
- Eliminate manual duplication between struct field definitions and validation error suggestions
- Ensure field lists automatically update when structs change
- Maintain or improve compile-time safety
- Keep zero runtime overhead for field list lookup

### Non-Goals
- Change the user-facing error message format
- Support dynamic/runtime schema inspection beyond compile-time generation
- Refactor the broader validation system

## Decisions

### Approach Comparison

| Aspect | Procedural Macro (Option A) | Build Script (Option B) |
|--------|----------------------------|-------------------------|
| **Complexity** | Higher (new crate, proc-macro APIs) | Medium (syn parsing, code generation) |
| **Type Safety** | High (trait impl per struct) | Medium (static constant) |
| **Maintainability** | Better (standard Rust derive pattern) | Good (single build.rs file) |
| **IDE Support** | Excellent (derives are well-understood) | Limited (generated files not indexed until build) |
| **Compile Time** | Minimal impact | Adds build step (negligible for small project) |
| **Ecosystem Fit** | Idiomatic Rust (similar to serde, JsonSchema) | Common but less ergonomic |

### Recommended: Option A (Procedural Macro)

**Rationale:**
1. **Consistency**: The project already uses `#[derive(Serialize, Deserialize, JsonSchema)]` extensively. Adding `#[derive(FieldList)]` aligns with existing patterns.
2. **Type Safety**: Each struct gets its own `impl`, making it impossible to forget updating a struct without breaking compilation.
3. **Maintainability**: Developers familiar with serde will immediately understand the pattern.
4. **Clarity**: Annotation at struct definition site is clearer than implicit build script magic.

**Trade-off**: Adds a new crate and proc-macro dependency, but this is a one-time cost with negligible compile-time impact.

### Alternative: Option B (Build Script)

Use build script if:
- Team prefers avoiding new internal crates
- Need to support older Rust versions without proc-macro2 features
- Want centralized code generation logic outside workspace

**Implementation sketch:**
```rust
// build.rs
use syn::{File, Item, ItemStruct};
use std::fs;
use std::env;
use std::path::Path;

fn main() {
    let src = fs::read_to_string("src/config.rs").unwrap();
    let ast: File = syn::parse_file(&src).unwrap();

    let mut field_map = Vec::new();
    for item in ast.items {
        if let Item::Struct(s) = item {
            if let Some(fields) = extract_fields(&s) {
                let section_name = s.ident.to_string()
                    .trim_end_matches("Config")
                    .to_lowercase();
                field_map.push((section_name, fields));
            }
        }
    }

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated_fields.rs");
    fs::write(dest_path, generate_code(field_map)).unwrap();

    println!("cargo:rerun-if-changed=src/config.rs");
}
```

## Risks / Trade-offs

### Risk: Struct Rename Detection
- **Issue**: If a struct is renamed, the section name mapping might break
- **Mitigation**: Use explicit `#[field_list(section = "stop")]` attribute to decouple struct name from section name

### Risk: Nested Structs (e.g., StopCommand)
- **Issue**: `StopCommand` is nested inside `StopConfig.commands`
- **Mitigation**: Generate separate entry for `commands` section or handle via attribute: `#[field_list(section = "commands")]`

### Trade-off: Compile-Time Reflection Limitations
- Rust doesn't have full compile-time reflection, so field attributes (like `#[serde(rename)]`) must be parsed explicitly
- The macro/build script must respect `#[serde(rename = "infiniteMessage")]` to match actual YAML keys

### Trade-off: Documentation
- Generated code may be less discoverable for newcomers
- **Mitigation**: Add clear doc comments and link to this design doc

## Migration Plan

### Phase 1: Implementation
1. Choose approach (recommend Option A)
2. Implement macro/build script
3. Add to one config struct as proof-of-concept
4. Validate field extraction matches hardcoded list

### Phase 2: Rollout
1. Apply to all config structs
2. Update `suggest_similar_fields` to use generated data
3. Remove hardcoded field list
4. Run full test suite

### Phase 3: Validation
1. Test with intentionally invalid configs
2. Verify error messages still helpful
3. Confirm no performance regression

### Rollback
- If critical issues discovered, revert to hardcoded list (single commit revert)
- Generated code is additive; doesn't break existing functionality

## Open Questions

1. **Should we expose the field list API publicly?**
   - Potential future use: Schema generation, CLI help text
   - Decision: Start private (`pub(crate)`), expose if needed

2. **How to handle optional fields?**
   - Current hardcoded list treats all fields equally
   - Should optional fields be marked differently in suggestions?
   - Decision: No distinction needed for error suggestions; all fields are valid

3. **Should we validate field types too?**
   - E.g., suggest "infinite" (bool) vs "infiniteMessage" (string) differently
   - Decision: Out of scope; Levenshtein distance is sufficient for typo suggestions
