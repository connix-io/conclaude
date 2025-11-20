# Change: Uneditable Comment Ranges

## Why

Currently, conclaude provides file-level protection through `uneditableFiles` patterns, but users cannot protect specific sections within a file. This is problematic for files containing both auto-generated and human-editable content, or sections that should remain immutable for compliance, security, or architectural reasons.

Adding inline comment-based uneditable ranges allows users to mark specific code sections as protected while keeping the rest of the file editable, enabling fine-grained control over AI modifications.

## Problem

Developers often have files where:
- **Auto-generated sections** exist alongside hand-written code (e.g., ORM models, API clients, config files)
- **Critical logic** must not be modified without human review (e.g., authentication, authorization, encryption)
- **Template regions** need protection while customization areas remain open
- **License headers** or compliance boilerplate should stay untouched

Current workarounds are inadequate:
1. **Protect entire file** - Blocks all edits, even when only one section needs protection
2. **Manual review** - Relies on humans catching AI mistakes after the fact
3. **Post-hoc validation** - Errors discovered too late, wasting session time

## Context

The existing `uneditableFiles` rule demonstrates successful file-level protection:
```rust
pub struct RulesConfig {
    pub uneditable_files: Vec<String>,  // Glob patterns
}
```

The `PreToolUse` hook already intercepts Edit/Write operations and validates them against rules. We can extend this validation to check for protected ranges within files.

## Proposed Solution

Introduce **inline uneditable markers** that users embed in comments to define protected ranges:

```go
// <!-- conclaude-uneditable:start -->
// This section is auto-generated - DO NOT EDIT
func GeneratedMethod() {
    // ...
}
// <!-- conclaude-uneditable:end -->
```

```python
# <!-- conclaude-uneditable:start -->
# Critical authentication logic - protected
def verify_token(token):
    # ...
# <!-- conclaude-uneditable:end -->
```

### How It Works

1. **PreToolUse hook** intercepts `Edit` tool operations
2. **Parse file** for uneditable markers in language-specific comment syntax
3. **Extract line ranges** between start/end markers (supports nesting)
4. **Validate edit operation** - block if `old_string` overlaps any protected range
5. **Return custom error** with range info and optional user message

### Configuration

Add opt-in configuration:

```yaml
rules:
  uneditableRanges:
    enabled: true
    # Optional: custom message when edit is blocked
    message: "This range is protected. Review markers in file."

    # Optional: language-specific patterns (defaults provided)
    languages:
      c: '//'
      csharp: '//'
      go: '//'
      java: '//'
      python: '#'
      javascript: '//'
      rust: '//'
      ruby: '#'
      shell: '#'
      zig: '//'
      nim: '#'
      tsx: '//'
      svelte: '<!--'
      astro: '<!--'
      html: '<!--'
      md: '<!--'
```

### Marker Format

Standard format across all languages:
```
<comment-prefix> <!-- conclaude-uneditable:start -->
<protected content>
<comment-prefix> <!-- conclaude-uneditable:end -->
```

**Examples:**
- **C/C++:** `// <!-- conclaude-uneditable:start -->`
- **C#:** `// <!-- conclaude-uneditable:start -->`
- **Go:** `// <!-- conclaude-uneditable:start -->`
- **Java:** `// <!-- conclaude-uneditable:start -->`
- **Python:** `# <!-- conclaude-uneditable:start -->`
- **JavaScript:** `// <!-- conclaude-uneditable:start -->`
- **Rust:** `// <!-- conclaude-uneditable:start -->`
- **Ruby:** `# <!-- conclaude-uneditable:start -->`
- **Shell:** `# <!-- conclaude-uneditable:start -->`
- **Zig:** `// <!-- conclaude-uneditable:start -->`
- **Nim:** `# <!-- conclaude-uneditable:start -->`
- **TSX:** `// <!-- conclaude-uneditable:start -->` or `{/* <!-- conclaude-uneditable:start --> */}`
- **Svelte:** `<!-- <!-- conclaude-uneditable:start --> -->`
- **Astro:** `<!-- <!-- conclaude-uneditable:start --> -->`
- **HTML:** `<!-- <!-- conclaude-uneditable:start --> -->`
- **Markdown:** `<!-- <!-- conclaude-uneditable:start --> -->`

### Nested Ranges Support

Nested markers are supported and behave as union of ranges:

```python
# <!-- conclaude-uneditable:start --> (outer: lines 1-10)
def outer_function():
    # <!-- conclaude-uneditable:start --> (inner: lines 3-7)
    def inner_function():
        pass
    # <!-- conclaude-uneditable:end -->
    pass
# <!-- conclaude-uneditable:end -->
```

Any edit overlapping lines 1-10 is blocked.

## What Changes

### New Capabilities
- **uneditable-ranges**: Core protection mechanism for inline ranges
- **comment-syntax-c**: C/C++ comment pattern detection
- **comment-syntax-csharp**: C# comment pattern detection
- **comment-syntax-go**: Go-specific comment pattern detection
- **comment-syntax-java**: Java comment pattern detection
- **comment-syntax-python**: Python-specific comment pattern detection
- **comment-syntax-javascript**: JavaScript/TypeScript comment pattern detection
- **comment-syntax-rust**: Rust comment pattern detection
- **comment-syntax-ruby**: Ruby comment pattern detection
- **comment-syntax-shell**: Shell/Bash comment pattern detection
- **comment-syntax-zig**: Zig comment pattern detection
- **comment-syntax-nim**: Nim comment pattern detection
- **comment-syntax-tsx**: TSX (TypeScript + JSX) comment pattern detection
- **comment-syntax-svelte**: Svelte component comment pattern detection
- **comment-syntax-astro**: Astro component comment pattern detection
- **comment-syntax-html**: HTML comment pattern detection
- **comment-syntax-md**: Markdown comment pattern detection

### Configuration Changes
- New `rules.uneditableRanges` section with opt-in enablement
- Language-specific comment prefix mappings
- Custom error message support

### Code Changes
- `src/hooks.rs`: Add range detection and validation in `handle_pre_tool_use`
- `src/config.rs`: Add `UneditableRangesConfig` struct
- `src/types.rs`: Add types for parsed ranges
- `conclaude-schema.json`: Updated schema (auto-generated)

### Validation Logic
- Scan file for markers when `Edit` tool is used
- Parse markers into line ranges per detected comment syntax
- Check if `old_string` (the text being replaced) overlaps any protected range
- **Block entire edit** if any overlap detected (partial overlaps are blocked)

## Impact

- **Affected specs**: New capabilities for uneditable-ranges and comment-syntax-* patterns
- **Affected code**: `src/hooks.rs`, `src/config.rs`, `src/types.rs`, `src/schema.rs`
- **Breaking changes**: None - this is an opt-in feature
- **Performance**: Minimal - file scanning only occurs on Edit operations for files with enabled ranges

## Files Affected

### Implementation
- `src/config.rs` - Add `UneditableRangesConfig`, language mappings
- `src/hooks.rs` - Parse markers, validate Edit operations against ranges
- `src/types.rs` - Define `UneditableRange`, `RangeParser` types
- `src/schema.rs` - Update example configuration
- `conclaude-schema.json` - Regenerated schema

### Tests
- `tests/hooks_tests.rs` - Range detection, nesting, overlap validation
- `tests/config_tests.rs` - Configuration deserialization
- `tests/integration_tests.rs` - End-to-end Edit blocking scenarios

### Documentation
- README examples with marker usage
- Configuration guide for language patterns

## Dependencies

- No new external dependencies
- Leverages existing `PreToolUse` hook infrastructure
- Uses standard file I/O and string parsing

## Alternatives Considered

1. **Per-line annotations** (e.g., `// @conclaude-protected`)
   - **Rejected**: Verbose for multi-line regions, harder to visually scan

2. **Regex-based patterns in config**
   - **Rejected**: Complex to configure, error-prone, no inline context

3. **AST-based protection** (protect specific functions/classes by name)
   - **Rejected**: Requires language-specific parsers, high complexity, out of scope

4. **Start marker only** (no end marker, protect to EOF)
   - **Rejected**: Less flexible, harder to protect multiple ranges in one file

## Success Criteria

- [ ] Configuration schema supports `uneditableRanges.enabled` and language mappings
- [ ] Parser detects markers in C/C++, C#, Go, Java, Python, JavaScript, Rust, Ruby, Shell, Zig, Nim, TSX, Svelte, Astro, HTML, Markdown files
- [ ] Nested ranges correctly merge into protected line sets
- [ ] Edit operations overlapping protected ranges are blocked
- [ ] Custom error messages display range info and user message
- [ ] Partial overlaps are blocked (not just fully-contained edits)
- [ ] All tests pass
- [ ] `spectr validate --strict` passes
- [ ] Documentation includes examples for each supported language

## Open Questions

1. **Custom marker text**: Should users be able to customize the marker (e.g., `conclaude-protected` vs `conclaude-uneditable`)? *Suggest: defer to v2, keep v1 simple with fixed marker*

2. **Block-style comments**: Should we support `/* ... */` style markers (C/Java)? *Suggest: yes, add in design.md*

3. **Whitespace sensitivity**: Should markers require exact whitespace match or trim? *Suggest: trim both ends for flexibility*

4. **Mismatched markers**: How to handle start without end? *Suggest: treat as error, block file entirely with clear message*

5. **Performance optimization**: Should we cache parsed ranges per file? *Suggest: defer to v2 if performance issues arise*

6. **IDE integration**: Should we provide language server hints for protected ranges? *Suggest: future enhancement, out of scope*
