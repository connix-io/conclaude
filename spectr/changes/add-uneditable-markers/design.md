# Design: Uneditable Code Range Markers

## Context

This design document describes the technical approach for implementing in-code uneditable range markers. The feature allows developers to protect specific code ranges from modification by embedding markers in comments, working across all programming languages.

**Stakeholders:**
- Developers protecting critical code sections
- Claude Code (AI assistant respecting protection markers)
- conclaude (enforcing protection rules)

**Constraints:**
- Must work across all major programming languages
- Must not break existing file protection mechanisms
- Must have minimal performance impact (< 10ms per file check)
- Must provide clear, actionable error messages

## Goals / Non-Goals

### Goals
1. Support uneditable range markers in all major programming languages
2. Detect and validate markers at PreToolUse hook before tool execution
3. Block `Edit` and `Write` operations that would modify protected ranges
4. Provide clear error messages indicating which ranges are protected
5. Handle edge cases gracefully (nested markers, unclosed markers, etc.)

### Non-Goals
1. **NOT** providing automatic marker insertion or management tools (future work)
2. **NOT** supporting region-based markers (e.g., `#region`/`#endregion`) - only comment-based
3. **NOT** runtime enforcement (markers are static, checked before edits)
4. **NOT** supporting partial-line protection (entire lines are protected)

## Architecture

### Component Overview

```
┌─────────────────────────────────────────────────────┐
│           PreToolUse Hook (hooks.rs)                │
│  - Receives Edit/Write tool payloads                │
│  - Extracts file path and edit range                │
│  - Calls marker detection & validation              │
└────────────────────┬────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────┐
│        Marker Detection (markers.rs - NEW)          │
│  - Detect language from file extension              │
│  - Map language to comment syntax                   │
│  - Parse file for markers                           │
│  - Extract protected ranges                         │
└────────────────────┬────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────┐
│         Range Validation (markers.rs)               │
│  - Check if edit range overlaps protected range     │
│  - Validate marker pairing (start/end matched)      │
│  - Detect nested/malformed markers                  │
└────────────────────┬────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────┐
│         HookResult (types.rs)                       │
│  - Return blocked result if overlap detected        │
│  - Return allowed if no overlap                     │
└─────────────────────────────────────────────────────┘
```

### Data Flow

1. **Claude Code** attempts `Edit` or `Write` operation
2. **PreToolUse hook** receives payload with file path and tool parameters
3. **Marker detector** reads target file and scans for markers
4. **Language detector** determines comment syntax based on file extension
5. **Parser** extracts protected ranges (line numbers)
6. **Validator** checks if edit range overlaps with any protected range
7. **Hook** returns `HookResult::blocked()` if overlap, `HookResult::allowed()` otherwise

### Language Detection Strategy

**Approach:** Extension-based mapping with fallback to heuristics

```rust
// Simplified pseudocode
fn detect_language(file_path: &Path) -> Language {
    match file_path.extension().and_then(|e| e.to_str()) {
        Some("go") => Language::Go,
        Some("py") => Language::Python,
        Some("js") => Language::JavaScript,
        Some("ts") | Some("tsx") => Language::TypeScript,
        Some("rs") => Language::Rust,
        Some("java") => Language::Java,
        Some("c") | Some("h") => Language::C,
        Some("cpp") | Some("cc") | Some("hpp") => Language::Cpp,
        Some("rb") => Language::Ruby,
        Some("php") => Language::Php,
        Some("sh") | Some("bash") => Language::Shell,
        _ => Language::Unknown, // Fallback: try generic comment patterns
    }
}
```

### Comment Syntax Mapping

Each language maps to its comment syntax pattern(s):

```rust
pub enum CommentStyle {
    DoubleSlash,      // Go, Rust, JavaScript, TypeScript, C++, Java
    Hash,             // Python, Ruby, Shell
    DoubleSlashBlock, // C-style /* */ (future)
    Mixed,            // Languages with multiple comment styles
}

fn get_comment_pattern(lang: Language) -> Vec<&'static str> {
    match lang {
        Language::Go | Language::Rust | Language::JavaScript
        | Language::TypeScript | Language::Java | Language::Cpp => {
            vec!["//"]
        },
        Language::Python | Language::Ruby | Language::Shell => {
            vec!["#"]
        },
        Language::C => {
            vec!["//", "/*"] // Support both styles
        },
        _ => {
            vec!["//", "#", "/*"] // Try all common patterns
        }
    }
}
```

### Marker Detection Algorithm

**Input:** File content (String), comment patterns (Vec<&str>)
**Output:** Vec<ProtectedRange> with line numbers

```rust
pub struct ProtectedRange {
    pub start_line: usize,  // Inclusive (1-indexed)
    pub end_line: usize,    // Inclusive (1-indexed)
    pub reason: Option<String>, // Optional custom message
}

fn extract_protected_ranges(
    content: &str,
    comment_patterns: &[&str]
) -> Result<Vec<ProtectedRange>> {
    let mut ranges = Vec::new();
    let mut stack = Vec::new(); // For tracking nested markers

    for (line_num, line) in content.lines().enumerate() {
        let line_num = line_num + 1; // Convert to 1-indexed

        for pattern in comment_patterns {
            let comment_start = line.find(pattern);
            if let Some(pos) = comment_start {
                let comment_text = &line[pos..];

                if comment_text.contains("<!-- conclaude-uneditable:start -->") {
                    stack.push(line_num);
                } else if comment_text.contains("<!-- conclaude-uneditable:end -->") {
                    if let Some(start) = stack.pop() {
                        ranges.push(ProtectedRange {
                            start_line: start,
                            end_line: line_num,
                            reason: None,
                        });
                    } else {
                        return Err(anyhow!("Unmatched end marker at line {}", line_num));
                    }
                }
            }
        }
    }

    if !stack.is_empty() {
        return Err(anyhow!("Unclosed protected range starting at line {}", stack[0]));
    }

    Ok(ranges)
}
```

### Edit Range Overlap Detection

**Input:** Edit range (start, end), Protected ranges
**Output:** bool (overlap detected)

```rust
fn has_overlap(
    edit_start: usize,
    edit_end: usize,
    protected: &ProtectedRange
) -> bool {
    // Check if ranges overlap
    !(edit_end < protected.start_line || edit_start > protected.end_line)
}

fn check_edit_allowed(
    edit_start: usize,
    edit_end: usize,
    protected_ranges: &[ProtectedRange]
) -> Option<&ProtectedRange> {
    protected_ranges.iter()
        .find(|range| has_overlap(edit_start, edit_end, range))
}
```

### Integration with PreToolUse Hook

**Modified `handle_pre_tool_use` in `hooks.rs`:**

```rust
async fn handle_pre_tool_use(
    payload: &PreToolUsePayload,
    config: &ConclaudeConfig,
    config_dir: &Path,
) -> Result<Option<HookResult>> {
    // ... existing validation logic (uneditableFiles, etc.) ...

    // NEW: Check for uneditable markers if Edit or Write tool
    if payload.tool_name == "Edit" || payload.tool_name == "Write" {
        if let Some(blocked_reason) = check_uneditable_markers(payload, config_dir)? {
            return Ok(Some(HookResult::blocked(blocked_reason)));
        }
    }

    // ... rest of validation ...
}

fn check_uneditable_markers(
    payload: &PreToolUsePayload,
    config_dir: &Path,
) -> Result<Option<String>> {
    // Extract file path from payload
    let file_path = extract_file_path(payload)?;

    // Read file content
    let content = fs::read_to_string(&file_path)?;

    // Detect language and get comment patterns
    let language = detect_language(&file_path);
    let patterns = get_comment_patterns(language);

    // Extract protected ranges
    let protected_ranges = extract_protected_ranges(&content, &patterns)?;

    if protected_ranges.is_empty() {
        return Ok(None); // No markers, allow operation
    }

    // Extract edit range from payload
    let (edit_start, edit_end) = extract_edit_range(payload)?;

    // Check for overlap
    if let Some(blocked_range) = check_edit_allowed(edit_start, edit_end, &protected_ranges) {
        let message = format!(
            "Blocked {} operation: attempting to modify protected range (lines {}-{}) in {}.\n\
             Protected range is marked with 'conclaude-uneditable' markers.",
            payload.tool_name,
            blocked_range.start_line,
            blocked_range.end_line,
            file_path.display()
        );
        return Ok(Some(message));
    }

    Ok(None) // No overlap, allow operation
}
```

## Decisions

### Decision 1: Always-On Feature (No Configuration Required)

**Decision:** Enable marker detection automatically whenever markers are present in files.

**Rationale:**
- Simplicity: No configuration needed to use the feature
- Explicit intent: Developers who add markers clearly want protection
- Backward compatible: Files without markers are unaffected
- Discoverability: No hidden configuration to discover

**Alternatives considered:**
- Require opt-in via `rules.enableUneditableMarkers: true`
  - **Rejected**: Adds friction, makes feature less discoverable
- Make it opt-out via `rules.disableUneditableMarkers: true`
  - **Rejected**: Default-on is simpler, opt-out rarely needed

### Decision 2: Block Entire Edit if Any Protected Line Touched

**Decision:** Block the entire `Edit` or `Write` operation if it overlaps with any protected range, even partially.

**Rationale:**
- **Safety first**: Prevents accidental modification of protected code
- **Simplicity**: Clear, unambiguous rule
- **Predictability**: Easy to understand and explain to users
- **Implementation**: Simpler validation logic

**Alternatives considered:**
- **Allow edit if protected lines remain unchanged**
  - **Rejected**: Would require diff-based validation, complex edge cases
  - Example complexity: What if edit changes indentation of protected line?
- **Split edit into allowed/blocked portions**
  - **Rejected**: Claude Code API doesn't support partial edit approval

### Decision 3: Include Marker Lines in Protected Range

**Decision:** The lines containing `start` and `end` markers are themselves part of the protected range.

**Rationale:**
- Prevents marker removal (deleting markers would disable protection)
- Prevents marker modification (changing marker format could break detection)
- Intuitive: "Everything between and including these markers is protected"

**Example:**
```python
# Line 10
# <!-- conclaude-uneditable:start -->   <- Protected (line 11)
SECRET_KEY = "do-not-modify"             <- Protected (line 12)
# <!-- conclaude-uneditable:end -->     <- Protected (line 13)
# Line 14
```

Protected range: lines 11-13 (inclusive)

### Decision 4: Report Error on Nested/Malformed Markers

**Decision:** Detect and report errors for nested, overlapping, or unclosed markers.

**Rationale:**
- **Data integrity**: Malformed markers likely indicate user error
- **Clear feedback**: Better to fail fast with clear error than silently ignore
- **Maintainability**: Prevents confusing protection states

**Error cases:**
1. **Unclosed start marker**: Start without matching end
2. **Unmatched end marker**: End without matching start
3. **Nested markers**: Start-start-end-end sequence (currently disallowed)

**Error response:**
- Block ALL edits to file containing malformed markers
- Return clear error message indicating the issue and line numbers

**Future consideration:** Could support nested markers if use case emerges

### Decision 5: Language Detection by Extension Only (V1)

**Decision:** Use file extension mapping for language detection; fallback to trying all common comment patterns if extension unknown.

**Rationale:**
- **Simplicity**: Extension mapping is fast and deterministic
- **Coverage**: Covers 95%+ of real-world use cases
- **Fallback**: Unknown extensions still work via pattern matching all styles
- **Future-proof**: Can add shebang detection or content sniffing later if needed

**Supported extensions (V1):**
- `.go` → Go (// comments)
- `.py` → Python (# comments)
- `.js`, `.jsx` → JavaScript (// comments)
- `.ts`, `.tsx` → TypeScript (// comments)
- `.rs` → Rust (// comments)
- `.java` → Java (// comments)
- `.c`, `.h` → C (// or /* */ comments)
- `.cpp`, `.cc`, `.hpp` → C++ (// or /* */ comments)
- `.rb` → Ruby (# comments)
- `.php` → PHP (// or # comments)
- `.sh`, `.bash` → Shell (# comments)

**Unknown extensions:** Try all patterns (`//`, `#`, `/*`)

## Risks / Trade-offs

### Risk 1: Performance Impact on Large Files

**Risk:** Scanning large files (> 10K lines) for markers could slow down PreToolUse hook.

**Mitigation:**
- Early exit: Stop scanning after finding relevant ranges for edit
- Caching: Cache marker positions per file (future optimization)
- Lazy loading: Only scan if file is being edited, not on every PreToolUse
- Benchmarking: Set performance budget (< 10ms per file)

**Acceptance:** For V1, accept small performance impact; optimize if profiling shows issues

### Risk 2: Marker Format Collisions

**Risk:** Code might legitimately contain `<!-- conclaude-uneditable:start -->` in strings or documentation.

**Mitigation:**
- Require marker to appear in comment (not in string literals)
- Use unique marker format unlikely to collide (`conclaude-uneditable` namespace)
- Document proper usage in README

**Acceptance:** Low probability; explicit format makes collisions unlikely

### Risk 3: Partial File Edits Could Be Confusing

**Risk:** If Claude Code wants to edit multiple ranges, some allowed and some blocked, the error message might be unclear.

**Mitigation:**
- Clear error messages indicating specific line numbers protected
- Consider future enhancement: allow multiple edits, block only conflicting ones

**Acceptance:** V1 blocks entire operation; can enhance UX in future iterations

### Risk 4: Language Coverage Gaps

**Risk:** New or obscure languages might not have comment syntax mapped.

**Mitigation:**
- Fallback to trying all common patterns
- Allow future configuration for custom comment syntax (if needed)
- Community can request language additions via issues

**Acceptance:** Fallback handles most cases; can expand language list incrementally

## Migration Plan

**No migration needed** - this is a new, additive feature.

**Adoption path:**
1. Users add markers to files where they want protection
2. conclaude automatically detects and enforces markers
3. No configuration changes required

**Backward compatibility:**
- Files without markers: No change in behavior
- Existing `uneditableFiles` rules: Continue to work independently
- Configuration: No breaking changes

## Testing Strategy

### Unit Tests

1. **Marker detection:**
   - Correctly parse start/end markers
   - Handle multiple protected ranges in same file
   - Detect unclosed markers
   - Detect unmatched end markers
   - Handle empty files
   - Handle files with no markers

2. **Language detection:**
   - Map extensions to correct languages
   - Handle unknown extensions (fallback)
   - Support multiple comment patterns per language

3. **Overlap detection:**
   - Detect overlap when edit starts inside range
   - Detect overlap when edit ends inside range
   - Detect overlap when edit fully contains range
   - Detect overlap when edit is fully contained by range
   - Allow edit when no overlap

### Integration Tests

1. **PreToolUse hook integration:**
   - Block Edit operation touching protected range
   - Allow Edit operation outside protected range
   - Block Write operation overwriting protected file
   - Return clear error messages

2. **Multi-language tests:**
   - Verify Go files with `//` comments
   - Verify Python files with `#` comments
   - Verify JavaScript files with `//` comments
   - Verify Rust files with `//` comments

3. **Edge cases:**
   - Nested markers (should error)
   - Markers in string literals (should ignore if not in comment)
   - Empty ranges (start and end on consecutive lines)
   - Very large files (performance test)

## Open Questions

1. **Custom marker formats:** Should users be able to configure alternative marker patterns in `.conclaude.yml`?
   - **Deferred to V2** - start with fixed format, add configuration if requested

2. **Marker metadata:** Should markers support optional metadata (e.g., reason, owner)?
   ```python
   # <!-- conclaude-uneditable:start reason="Generated by Prisma" -->
   ```
   - **Deferred to V2** - start simple, add metadata if use cases emerge

3. **Performance optimization:** Should we cache marker positions to avoid re-parsing?
   - **Deferred until profiling** - implement V1, measure, optimize if needed

4. **IDE integration:** Should we provide VS Code extension to highlight protected ranges?
   - **Out of scope** - nice-to-have, separate project

## Success Metrics

- Feature successfully blocks edits to protected ranges
- Performance impact < 10ms per file on typical codebases (< 5K lines)
- Zero false positives (legitimate edits not blocked)
- Zero false negatives (protected ranges not missed)
- Clear error messages (user understands why blocked)

## Implementation Phases

### Phase 1: Core Functionality (MVP)
- Marker detection for core languages (Go, Python, JS, TS, Rust)
- Basic overlap detection
- PreToolUse integration
- Unit tests

### Phase 2: Language Expansion
- Add support for additional languages (Java, C/C++, Ruby, PHP, Shell)
- Fallback handling for unknown extensions
- Integration tests across all languages

### Phase 3: Polish & Edge Cases
- Nested marker detection
- Improved error messages
- Performance optimization (if needed)
- Documentation and examples
