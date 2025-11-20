# Design: Uneditable Comment Ranges

## Context

Conclaude currently provides file-level protection via `uneditableFiles` patterns. This enhancement adds fine-grained, inline protection for specific code regions within files through comment-based markers.

**Key constraints:**
- Must work across multiple programming languages with different comment syntaxes
- Must integrate with existing `PreToolUse` hook without breaking current behavior
- Performance-critical: file scanning must be fast (runs on every Edit operation)
- Must support nested ranges and partial overlap detection

**Stakeholders:**
- **Users**: Need intuitive, language-agnostic markers that fit naturally in code
- **AI (Claude)**: Receives clear error messages when edits are blocked
- **Maintainers**: Require extensible design for adding new language support

## Goals / Non-Goals

### Goals
- ✅ Protect specific line ranges within files via inline markers
- ✅ Support major programming languages (Go, Python, JavaScript, Rust, Shell)
- ✅ Block Edit operations that overlap protected ranges (full or partial)
- ✅ Support nested uneditable ranges
- ✅ Provide custom error messages with range context
- ✅ Opt-in configuration (disabled by default)
- ✅ Extensible for future language additions

### Non-Goals
- ❌ AST-based protection (no semantic understanding of code)
- ❌ Write tool blocking (only Edit operations)
- ❌ Real-time file watching or caching (parse on-demand only)
- ❌ IDE integration or language server features
- ❌ Custom marker text (fixed format only)
- ❌ Auto-generation of markers (users manually add them)

## Decisions

### Decision 1: Marker Format

**Choice:** Use HTML-comment-style markers inside language-specific comments

```
<language-comment-prefix> <!-- conclaude-uneditable:start -->
<protected content>
<language-comment-prefix> <!-- conclaude-uneditable:end -->
```

**Rationale:**
- **Language-agnostic**: HTML-style `<!-- ... -->` works inside any comment syntax
- **Visual distinction**: Clearly stands out from regular comments
- **Familiar pattern**: Similar to template systems (Jinja, ERB, etc.)
- **Grep-friendly**: Easy to search across codebases (`rg "conclaude-uneditable"`)
- **Explicit intent**: Self-documenting, obvious purpose

**Alternatives considered:**
- `@conclaude-protected` annotations → Less visually distinct, harder to pair start/end
- Custom syntax like `==PROTECTED==` → Not standard, harder to remember
- Language-specific pragmas → Different per language, high complexity

### Decision 2: Language Detection

**Choice:** File extension mapping to comment prefix

```rust
pub struct CommentSyntax {
    pub line_prefix: String,           // e.g., "//" or "#"
    pub block_start: Option<String>,   // e.g., "/*"
    pub block_end: Option<String>,     // e.g., "*/"
}

// Mapping from file extension → CommentSyntax
lazy_static! {
    static ref LANGUAGE_MAP: HashMap<&'static str, CommentSyntax> = {
        let mut m = HashMap::new();
        // C/C++ family
        m.insert("c", CommentSyntax { line_prefix: "//", block_start: Some("/*"), block_end: Some("*/") });
        m.insert("h", CommentSyntax { line_prefix: "//", block_start: Some("/*"), block_end: Some("*/") });
        m.insert("cpp", CommentSyntax { line_prefix: "//", block_start: Some("/*"), block_end: Some("*/") });
        m.insert("cxx", CommentSyntax { line_prefix: "//", block_start: Some("/*"), block_end: Some("*/") });
        m.insert("cc", CommentSyntax { line_prefix: "//", block_start: Some("/*"), block_end: Some("*/") });
        m.insert("hpp", CommentSyntax { line_prefix: "//", block_start: Some("/*"), block_end: Some("*/") });
        m.insert("hxx", CommentSyntax { line_prefix: "//", block_start: Some("/*"), block_end: Some("*/") });
        // C#
        m.insert("cs", CommentSyntax { line_prefix: "//", block_start: Some("/*"), block_end: Some("*/") });
        m.insert("csx", CommentSyntax { line_prefix: "//", block_start: Some("/*"), block_end: Some("*/") });
        // Go
        m.insert("go", CommentSyntax { line_prefix: "//", block_start: Some("/*"), block_end: Some("*/") });
        // Java
        m.insert("java", CommentSyntax { line_prefix: "//", block_start: Some("/*"), block_end: Some("*/") });
        // JSONC
        m.insert("jsonc", CommentSyntax { line_prefix: "//", block_start: Some("/*"), block_end: Some("*/") });
        // Nix
        m.insert("nix", CommentSyntax { line_prefix: "#", block_start: Some("/*"), block_end: Some("*/") });
        // Python
        m.insert("py", CommentSyntax { line_prefix: "#", block_start: None, block_end: None });
        m.insert("pyw", CommentSyntax { line_prefix: "#", block_start: None, block_end: None });
        // R
        m.insert("r", CommentSyntax { line_prefix: "#", block_start: None, block_end: None });
        m.insert("R", CommentSyntax { line_prefix: "#", block_start: None, block_end: None });
        m.insert("Rmd", CommentSyntax { line_prefix: "#", block_start: None, block_end: None });
        // JavaScript/TypeScript
        m.insert("js", CommentSyntax { line_prefix: "//", block_start: Some("/*"), block_end: Some("*/") });
        m.insert("jsx", CommentSyntax { line_prefix: "//", block_start: Some("/*"), block_end: Some("*/") });
        m.insert("ts", CommentSyntax { line_prefix: "//", block_start: Some("/*"), block_end: Some("*/") });
        m.insert("tsx", CommentSyntax { line_prefix: "//", block_start: Some("/*"), block_end: Some("*/") });
        m.insert("mjs", CommentSyntax { line_prefix: "//", block_start: Some("/*"), block_end: Some("*/") });
        m.insert("cjs", CommentSyntax { line_prefix: "//", block_start: Some("/*"), block_end: Some("*/") });
        // Rust
        m.insert("rs", CommentSyntax { line_prefix: "//", block_start: Some("/*"), block_end: Some("*/") });
        // Ruby
        m.insert("rb", CommentSyntax { line_prefix: "#", block_start: Some("=begin"), block_end: Some("=end") });
        m.insert("rake", CommentSyntax { line_prefix: "#", block_start: Some("=begin"), block_end: Some("=end") });
        m.insert("gemspec", CommentSyntax { line_prefix: "#", block_start: Some("=begin"), block_end: Some("=end") });
        // Shell
        m.insert("sh", CommentSyntax { line_prefix: "#", block_start: None, block_end: None });
        m.insert("bash", CommentSyntax { line_prefix: "#", block_start: None, block_end: None });
        m.insert("zsh", CommentSyntax { line_prefix: "#", block_start: None, block_end: None });
        // TOML
        m.insert("toml", CommentSyntax { line_prefix: "#", block_start: None, block_end: None });
        // YAML
        m.insert("yaml", CommentSyntax { line_prefix: "#", block_start: None, block_end: None });
        m.insert("yml", CommentSyntax { line_prefix: "#", block_start: None, block_end: None });
        // Zig
        m.insert("zig", CommentSyntax { line_prefix: "//", block_start: None, block_end: None });
        // Nim
        m.insert("nim", CommentSyntax { line_prefix: "#", block_start: Some("#["), block_end: Some("]#") });
        m.insert("nims", CommentSyntax { line_prefix: "#", block_start: Some("#["), block_end: Some("]#") });
        m.insert("nimble", CommentSyntax { line_prefix: "#", block_start: Some("#["), block_end: Some("]#") });
        // Web components
        m.insert("svelte", CommentSyntax { line_prefix: "//", block_start: Some("<!--"), block_end: Some("-->") });
        m.insert("astro", CommentSyntax { line_prefix: "//", block_start: Some("<!--"), block_end: Some("-->") });
        // HTML/Markdown
        m.insert("html", CommentSyntax { line_prefix: "", block_start: Some("<!--"), block_end: Some("-->") });
        m.insert("htm", CommentSyntax { line_prefix: "", block_start: Some("<!--"), block_end: Some("-->") });
        m.insert("md", CommentSyntax { line_prefix: "", block_start: Some("<!--"), block_end: Some("-->") });
        m.insert("markdown", CommentSyntax { line_prefix: "", block_start: Some("<!--"), block_end: Some("-->") });
        m.insert("mdx", CommentSyntax { line_prefix: "", block_start: Some("<!--"), block_end: Some("-->") });
        // RST (reStructuredText)
        m.insert("rst", CommentSyntax { line_prefix: "..", block_start: None, block_end: None });
        m.insert("rest", CommentSyntax { line_prefix: "..", block_start: None, block_end: None });
        m
    };
}
```

**Rationale:**
- **Simple**: No complex language detection logic needed
- **Fast**: O(1) lookup by file extension
- **Maintainable**: Adding new languages is trivial (one-line map entry)
- **Sufficient**: File extensions reliably indicate language for 99% of cases

**Alternatives considered:**
- **Content-based detection** (shebang, heuristics) → Overcomplicated, slow
- **User-configured patterns** → More flexible but harder to use, prone to misconfiguration
- **AST parsing** → Overkill, requires language-specific parsers

### Decision 3: Range Storage and Overlap Detection

**Choice:** Parse file into `Vec<UneditableRange>`, check overlaps via line numbers

```rust
#[derive(Debug, Clone)]
pub struct UneditableRange {
    pub start_line: usize,
    pub end_line: usize,
    pub nesting_level: usize,
}

// Check if edit overlaps any protected range
fn overlaps_protected_range(
    old_string: &str,
    file_content: &str,
    ranges: &[UneditableRange],
) -> Option<UneditableRange> {
    let edit_lines = find_edit_line_range(old_string, file_content)?;

    for range in ranges {
        if edit_lines.start <= range.end_line && edit_lines.end >= range.start_line {
            return Some(range.clone());
        }
    }
    None
}
```

**Rationale:**
- **Precise**: Line-based comparison is accurate and understandable
- **Efficient**: Simple numeric comparison, no complex string matching
- **Debuggable**: Easy to log and display to users ("protected lines 42-58")
- **Handles nesting**: Nested ranges are flattened into separate entries

**Alternatives considered:**
- **Byte offset ranges** → Harder to display to users, error-prone with Unicode
- **String-based containment** → Fragile, doesn't handle partial overlaps well
- **Interval tree** → Overkill for typical files (< 10 protected ranges)

### Decision 4: Nested Range Handling

**Choice:** Allow nesting, treat as union of ranges (all lines protected)

```python
# <!-- conclaude-uneditable:start -->  # Lines 1-10 protected
def outer():
    # <!-- conclaude-uneditable:start -->  # Lines 3-7 protected
    def inner():
        pass
    # <!-- conclaude-uneditable:end -->
    pass
# <!-- conclaude-uneditable:end -->
```

**Behavior:**
- Parse nested markers correctly (track depth)
- Store each range separately with nesting level
- Block edit if it overlaps **any** range (union semantics)

**Rationale:**
- **Intuitive**: Users expect nested regions to all be protected
- **Safe**: No ambiguity about what's protected (everything inside outer marker)
- **Flexible**: Allows hierarchical organization of protected code

**Alternatives considered:**
- **Reject nesting** (error if markers nest) → Too restrictive, breaks valid use cases
- **Intersection semantics** (only innermost range protected) → Confusing, unexpected

### Decision 5: Partial Overlap Behavior

**Choice:** Block edit if it **partially** or **fully** overlaps a protected range

**Example:**
```
Protected range: lines 10-20
Edit targets: lines 15-25 (old_string spans these lines)
Result: BLOCKED (partial overlap)
```

**Rationale:**
- **Safe by default**: Prevents accidental partial modifications
- **Clear intent**: If any part of edit touches protected code, reject it
- **Simpler logic**: No need to extract/rewrite partial edits

**Alternatives considered:**
- **Allow edits outside ranges** (split edit into safe parts) → Too complex, error-prone
- **Only block fully-contained edits** → Unsafe, allows partial corruption

### Decision 6: Edit Operation Detection

**Choice:** Only block `Edit` tool, not `Write` tool

**Rationale:**
- **Edit** modifies existing content → High risk of partial corruption
- **Write** overwrites entire file → User intent is explicit (full replacement)
- If user Writes entire file, they likely know what they're doing
- Markers in file will be replaced anyway (explicit override)

**Behavior:**
```yaml
Edit tool + overlap → BLOCKED
Write tool + overlap → ALLOWED (entire file overwritten anyway)
```

**Alternatives considered:**
- **Block both Edit and Write** → Too restrictive, prevents intentional full rewrites
- **Block neither** → Defeats purpose of protection

### Decision 7: Error Messages

**Choice:** Structured error with range info + custom message

```rust
pub struct UneditableRangeError {
    pub file_path: String,
    pub protected_lines: String,  // "42-58"
    pub custom_message: Option<String>,
}

// Formatted output:
// "Blocked Edit operation: file contains protected range (lines 42-58). File: src/auth.rs
//  Custom message: Critical authentication logic - do not modify without security review."
```

**Rationale:**
- **Informative**: Shows exactly which lines are protected
- **Actionable**: User can inspect file and understand why edit was blocked
- **Customizable**: Optional message provides context-specific guidance

## Risks / Trade-offs

### Risk: False Positives (Blocking Valid Edits)

**Scenario:** User wants to edit code near (but not inside) protected range, but `old_string` accidentally includes marker line.

**Mitigation:**
- Document marker format clearly
- Provide detailed error messages with line numbers
- Suggest users check exact line ranges in error output

### Risk: Mismatched Markers

**Scenario:** User adds start marker but forgets end marker, or vice versa.

**Mitigation:**
- Validate marker pairing during parsing
- Return clear error: "Unmatched uneditable marker in file (start at line 42, no matching end)"
- Block entire file if markers are mismatched (fail-safe)

### Risk: Performance on Large Files

**Scenario:** Parsing 10,000-line file on every Edit operation could be slow.

**Mitigation:**
- Defer optimization until proven necessary (benchmark first)
- Future: cache parsed ranges per file (keyed by file hash)
- Most protected sections are small (< 100 lines), scanning is fast

### Risk: Language Support Gaps

**Scenario:** User wants to protect ranges in unsupported language (e.g., Haskell, Kotlin).

**Mitigation:**
- Design is extensible (easy to add new languages)
- Document how to request new language support
- Fall back gracefully: if language unknown, skip range detection (no protection)

### Trade-off: Marker Verbosity

**Con:** Markers are somewhat verbose (`<!-- conclaude-uneditable:start -->`)

**Pro:**
- Clear and unambiguous
- Self-documenting
- Easy to search/grep

**Decision:** Verbosity is acceptable trade-off for clarity and robustness.

## Migration Plan

### Phase 1: Implementation
1. Add configuration schema for `rules.uneditableRanges`
2. Implement language detection and marker parsing
3. Add Edit operation validation in `PreToolUse` hook
4. Write comprehensive tests

### Phase 2: Rollout
- Feature is opt-in (disabled by default)
- No breaking changes to existing configurations
- Users explicitly enable via `uneditableRanges.enabled: true`

### Phase 3: Adoption
- Document marker usage with examples
- Provide templates for common use cases
- Collect feedback on language support gaps

### Rollback Plan
- If critical bugs found, users can disable via config
- Feature is isolated to `PreToolUse` hook (minimal blast radius)
- No data loss risk (only blocks operations, doesn't modify files)

## Open Questions

### Q1: Should we support inline custom messages per range?

**Example:**
```python
# <!-- conclaude-uneditable:start message="Auto-generated - see codegen.py" -->
def generated_method():
    pass
# <!-- conclaude-uneditable:end -->
```

**Status:** Defer to v2. Keep v1 simple with global message.

### Q2: Should we support regex-based marker detection?

**Use case:** Allow users to customize marker text.

**Status:** No. Fixed marker format reduces complexity and user error.

### Q3: Should ranges be reportable via a command?

**Example:** `conclaude show-protected-ranges src/auth.rs`

**Status:** Nice-to-have. Add if user feedback requests it.

### Q4: Should we support disabling ranges per-file via config?

**Example:**
```yaml
uneditableRanges:
  enabled: true
  excludeFiles:
    - "tests/**"
```

**Status:** Defer to v2. Users can add/remove markers manually for now.

## Implementation Notes

### File Scanning Strategy

```rust
// Pseudocode
fn parse_uneditable_ranges(file_path: &str, content: &str) -> Result<Vec<UneditableRange>> {
    let ext = get_file_extension(file_path)?;
    let syntax = LANGUAGE_MAP.get(ext)?;

    let mut ranges = Vec::new();
    let mut stack = Vec::new();  // Track nesting

    for (line_num, line) in content.lines().enumerate() {
        if is_start_marker(line, syntax) {
            stack.push(line_num);
        } else if is_end_marker(line, syntax) {
            if let Some(start) = stack.pop() {
                ranges.push(UneditableRange {
                    start_line: start,
                    end_line: line_num,
                    nesting_level: stack.len(),
                });
            } else {
                return Err("Unmatched end marker");
            }
        }
    }

    if !stack.is_empty() {
        return Err("Unmatched start marker");
    }

    Ok(ranges)
}
```

### Edit Validation Strategy

```rust
// In handle_pre_tool_use
if payload.tool_name == "Edit" && config.rules.uneditable_ranges.enabled {
    let file_path = extract_file_path(&payload.tool_input)?;
    let content = fs::read_to_string(&file_path)?;
    let ranges = parse_uneditable_ranges(&file_path, &content)?;

    let old_string = payload.tool_input.get("old_string")?;

    if let Some(blocked_range) = overlaps_protected_range(old_string, &content, &ranges) {
        return HookResult::blocked(
            format!(
                "Blocked Edit operation: file contains protected range (lines {}-{}). File: {}{}",
                blocked_range.start_line,
                blocked_range.end_line,
                file_path,
                config.rules.uneditable_ranges.message
                    .map(|m| format!("\n{}", m))
                    .unwrap_or_default()
            )
        );
    }
}
```

## Testing Strategy

### Unit Tests
- Marker parsing: valid, invalid, nested, mismatched
- Language detection: all supported extensions
- Range overlap detection: no overlap, partial, full, multiple ranges

### Integration Tests
- End-to-end Edit blocking with real files
- Configuration loading and validation
- Error message formatting

### Edge Cases
- Empty files
- Files with only start or only end markers
- Deeply nested ranges (10+ levels)
- Large files (10,000+ lines)
- Unicode in markers (should work, markers are ASCII)
