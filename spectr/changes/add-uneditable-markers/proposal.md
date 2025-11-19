# Proposal: Uneditable Code Range Markers

## Why

Currently, conclaude can protect entire files from modification using `uneditableFiles` glob patterns. However, there are many scenarios where only *specific ranges* within a file should be protected while allowing edits to other parts:

- **Generated code blocks** within otherwise editable files (e.g., Prisma schema sections, OpenAPI spec sections)
- **Critical configuration sections** that should remain stable (e.g., security settings, database connection strings)
- **Template boilerplate** that must remain intact while allowing customization elsewhere
- **License headers** or copyright notices that cannot be modified
- **Auto-generated imports** or type definitions managed by tooling

Protecting these ranges requires a mechanism to mark specific line ranges as uneditable using in-code markers that work across all programming languages.

## What Changes

Add support for **in-code uneditable range markers** that protect specific line ranges from being modified by Claude Code. The markers use HTML-comment-style syntax embedded in language-specific comments:

**Go example:**
```go
package main

// <!-- conclaude-uneditable:start -->
const CriticalConfig = "DO_NOT_MODIFY"
// <!-- conclaude-uneditable:end -->

// This code can be edited
func main() {
    // ...
}
```

**Python example:**
```python
# -*- coding: utf-8 -*-

# <!-- conclaude-uneditable:start -->
SECRET_KEY = "production-secret-key-do-not-change"
# <!-- conclaude-uneditable:end -->

# This code can be edited
def main():
    pass
```

**JavaScript/TypeScript example:**
```typescript
// <!-- conclaude-uneditable:start -->
export const API_VERSION = "v1.0.0";
export const API_ENDPOINT = "https://api.example.com";
// <!-- conclaude-uneditable:end -->

// This code can be edited
export function fetchData() {
  // ...
}
```

**Rust example:**
```rust
// <!-- conclaude-uneditable:start -->
const DATABASE_URL: &str = "postgresql://prod-db";
// <!-- conclaude-uneditable:end -->

// This code can be edited
fn main() {
    // ...
}
```

## Key Features

1. **Language-agnostic marker syntax**: `<!-- conclaude-uneditable:start -->` and `<!-- conclaude-uneditable:end -->` work in any comment
2. **Multi-language support**: Automatic detection of comment syntax for major languages (Go, Python, JavaScript, TypeScript, Rust, Java, C/C++, Ruby, etc.)
3. **PreToolUse validation**: Blocks `Edit` and `Write` operations that attempt to modify protected ranges
4. **Clear error messages**: Informs Claude Code which ranges are protected and why the operation was blocked
5. **Nested range handling**: Detects and reports errors for improperly nested markers
6. **File-based detection**: Scans target files for markers before applying edits

## Impact

- **Affected specs**:
  - New capability: `validation-rules` (range-based protection logic)
  - New capability: `language-support` (comment syntax detection)
- **Affected code**:
  - `src/hooks.rs` - Add range validation logic to `handle_pre_tool_use` hook
  - `src/types.rs` - Add marker detection and range extraction types/functions
  - New module: `src/markers.rs` - Language detection and marker parsing
  - `src/config.rs` - Optional configuration for custom marker patterns (future)
- **Breaking changes**: **None** - This is an additive feature
- **Performance**: Minimal impact - only scans files that are being edited/written

## Examples

### Blocking an Edit that touches protected range

**File: config.py**
```python
# <!-- conclaude-uneditable:start -->
DATABASE_URL = "postgresql://prod.db"
API_KEY = "super-secret-key"
# <!-- conclaude-uneditable:end -->

DEBUG_MODE = False
```

**Claude Code attempts:**
```python
# Edit tool targeting lines 2-3
DATABASE_URL = "postgresql://dev.db"  # ❌ BLOCKED
```

**Result:**
```
Blocked Edit operation: attempting to modify protected range (lines 1-3) in config.py.
Protected range is marked with 'conclaude-uneditable' markers.
```

### Allowing edits outside protected range

**Same file, different edit:**
```python
# Edit tool targeting line 5
DEBUG_MODE = True  # ✅ ALLOWED
```

## Design Considerations

1. **Marker format**: Use HTML-comment style for familiarity and universal recognizability
2. **Comment detection**: Map file extensions to known comment syntaxes (with fallback to generic patterns)
3. **Range boundaries**: Include marker lines themselves in the protected range
4. **Validation timing**: Check at PreToolUse hook before tool executes
5. **Error handling**: Clear, actionable error messages indicating which lines are protected

## Alternatives Considered

1. **Use language-specific markers** (e.g., `@conclaude-uneditable` in comments)
   - **Rejected**: Would require different markers per language, harder to remember and maintain

2. **Use pragma-style markers** (e.g., `#pragma conclaude uneditable`)
   - **Rejected**: Not all languages support pragma directives; less universal than comments

3. **Configure ranges in `.conclaude.yml`** (e.g., `file.py:10-20`)
   - **Rejected**: Separates protection from code, brittle to line number changes, harder to maintain

4. **Support only specific languages initially**
   - **Considered**: Could start with top 5 languages (Go, Python, JS, TS, Rust) and expand later

## Open Questions

Before finalizing the proposal, I need clarification on:

1. **Marker customization**: Should users be able to configure custom marker patterns in `.conclaude.yml`?
   - Default: `<!-- conclaude-uneditable:start/end -->`
   - Custom example: `@protected:start/@protected:end`

2. **Nested markers**: How should nested/overlapping markers be handled?
   - Option A: Report error and block all edits to file
   - Option B: Merge into single protected range
   - Option C: Support nested ranges with innermost taking precedence

3. **Language coverage**: Should we support all languages immediately or start with a core set?
   - Core set: Go, Python, JavaScript, TypeScript, Rust, Java, C/C++, Ruby, PHP, Shell
   - Full set: Include less common languages (Haskell, Scala, Kotlin, Swift, etc.)

4. **Partial line edits**: If an edit operation spans both protected and unprotected lines, should it:
   - Option A: Block entirely if any protected lines are touched
   - Option B: Allow if protected lines remain unchanged (more complex validation)

5. **Configuration opt-in**: Should this feature be:
   - Option A: Always enabled (if markers present, they're enforced)
   - Option B: Require explicit opt-in in `.conclaude.yml` (`rules.enableUneditableMarkers: true`)

## Success Criteria

- [ ] Markers correctly detected in all supported language file types
- [ ] `Edit` operations blocked when modifying protected ranges
- [ ] `Write` operations blocked when overwriting files with protected content
- [ ] Clear error messages indicate which lines are protected
- [ ] Improperly nested markers detected and reported
- [ ] All tests pass
- [ ] Documentation includes examples for major languages
- [ ] Performance impact is negligible (< 10ms overhead per file check)

## Files Affected

- `src/hooks.rs` - PreToolUse hook validation logic
- `src/markers.rs` (new) - Marker parsing and language detection
- `src/types.rs` - Protected range types and error responses
- `src/config.rs` - Optional configuration fields (if custom markers supported)
- `conclaude-schema.json` - Regenerated schema
- Documentation (README, examples)

## Timeline

This is a moderately complex feature requiring:
- Language detection logic (comment syntax mapping)
- Marker parsing and range extraction
- Integration with existing PreToolUse hook
- Comprehensive testing across multiple languages
- Clear error messaging and documentation

Estimated complexity: **Medium** (requires careful design for language support and edge cases)
