# Implementation Tasks

## 1. Configuration Schema

- [ ] 1.1 Define `UneditableRangesConfig` struct in `src/config.rs`
  - [ ] 1.1.1 Add `enabled: bool` field
  - [ ] 1.1.2 Add `message: Option<String>` field for custom error messages
  - [ ] 1.1.3 Implement serde `Deserialize` and `Serialize` traits
  - [ ] 1.1.4 Add JSON schema annotations
- [ ] 1.2 Add `uneditable_ranges` field to `RulesConfig` in `src/config.rs`
- [ ] 1.3 Update default configuration in `src/default-config.yaml`
- [ ] 1.4 Regenerate JSON schema (`conclaude-schema.json`)
- [ ] 1.5 Add configuration validation tests in `tests/config_tests.rs`

## 2. Language Detection and Comment Syntax

- [ ] 2.1 Define `CommentSyntax` struct in `src/types.rs`
  - [ ] 2.1.1 Add `line_prefix: String` field (e.g., `//`, `#`)
  - [ ] 2.1.2 Add `block_start: Option<String>` field (e.g., `/*`)
  - [ ] 2.1.3 Add `block_end: Option<String>` field (e.g., `*/`)
- [ ] 2.2 Create language mapping `HashMap<&str, CommentSyntax>` (use `lazy_static` or `OnceLock`)
  - [ ] 2.2.1 Add C/C++ mapping (`.c`, `.h`, `.cpp`, `.cxx`, `.cc`, `.hpp`, `.hxx` → `//` and `/* */`)
  - [ ] 2.2.2 Add C# mapping (`.cs`, `.csx` → `//` and `/* */`)
  - [ ] 2.2.3 Add Go mapping (`.go` → `//` and `/* */`)
  - [ ] 2.2.4 Add Java mapping (`.java` → `//` and `/* */`)
  - [ ] 2.2.5 Add Python mapping (`.py`, `.pyw` → `#`)
  - [ ] 2.2.6 Add JavaScript/TypeScript mapping (`.js`, `.jsx`, `.ts`, `.tsx`, `.mjs`, `.cjs` → `//` and `/* */`)
  - [ ] 2.2.7 Add Rust mapping (`.rs` → `//` and `/* */`)
  - [ ] 2.2.8 Add Ruby mapping (`.rb`, `.rake`, `.gemspec` → `#` and `=begin`/`=end`)
  - [ ] 2.2.9 Add Shell mapping (`.sh`, `.bash`, `.zsh` → `#`)
  - [ ] 2.2.10 Add Zig mapping (`.zig` → `//`)
  - [ ] 2.2.11 Add Nim mapping (`.nim`, `.nims`, `.nimble` → `#` and `#[` `]#`)
  - [ ] 2.2.12 Add TSX mapping (`.tsx` → `//` and `/* */` and `{/* */}`)
  - [ ] 2.2.13 Add Svelte mapping (`.svelte` → `//`, `/* */`, and `<!-- -->`)
  - [ ] 2.2.14 Add Astro mapping (`.astro` → `//`, `/* */`, and `<!-- -->`)
  - [ ] 2.2.15 Add HTML mapping (`.html`, `.htm` → `<!-- -->`)
  - [ ] 2.2.16 Add Markdown mapping (`.md`, `.markdown`, `.mdx` → `<!-- -->`)
- [ ] 2.3 Implement `get_comment_syntax(file_path: &str) -> Option<CommentSyntax>` function
- [ ] 2.4 Add unit tests for language detection in `tests/types_tests.rs`

## 3. Uneditable Range Parsing

- [ ] 3.1 Define `UneditableRange` struct in `src/types.rs`
  - [ ] 3.1.1 Add `start_line: usize` field
  - [ ] 3.1.2 Add `end_line: usize` field
  - [ ] 3.1.3 Add `nesting_level: usize` field
  - [ ] 3.1.4 Implement `Debug` and `Clone` traits
- [ ] 3.2 Implement `parse_uneditable_ranges(file_path: &str, content: &str) -> Result<Vec<UneditableRange>>` in `src/hooks.rs`
  - [ ] 3.2.1 Extract file extension from file_path
  - [ ] 3.2.2 Lookup comment syntax from language mapping
  - [ ] 3.2.3 Scan file line-by-line for markers
  - [ ] 3.2.4 Detect start markers (`<!-- conclaude-uneditable:start -->`) within comments
  - [ ] 3.2.5 Detect end markers (`<!-- conclaude-uneditable:end -->`) within comments
  - [ ] 3.2.6 Track nesting depth using a stack
  - [ ] 3.2.7 Validate marker pairing (return error if unmatched)
  - [ ] 3.2.8 Return `Vec<UneditableRange>` with all detected ranges
- [ ] 3.3 Implement helper function `is_start_marker(line: &str, syntax: &CommentSyntax) -> bool`
- [ ] 3.4 Implement helper function `is_end_marker(line: &str, syntax: &CommentSyntax) -> bool`
- [ ] 3.5 Add comprehensive unit tests for marker parsing in `tests/hooks_tests.rs`
  - [ ] 3.5.1 Test valid single range
  - [ ] 3.5.2 Test multiple non-overlapping ranges
  - [ ] 3.5.3 Test nested ranges
  - [ ] 3.5.4 Test mismatched markers (start without end, end without start)
  - [ ] 3.5.5 Test edge cases (empty lines, markers at file start/end)

## 4. Edit Operation Validation

- [ ] 4.1 Update `handle_pre_tool_use` in `src/hooks.rs` to check for uneditable ranges
  - [ ] 4.1.1 Check if `config.rules.uneditable_ranges.enabled` is true
  - [ ] 4.1.2 Check if `payload.tool_name == "Edit"` (only block Edit, not Write)
  - [ ] 4.1.3 Extract `file_path` from `payload.tool_input`
  - [ ] 4.1.4 Read file content using `fs::read_to_string`
  - [ ] 4.1.5 Call `parse_uneditable_ranges` to get protected ranges
  - [ ] 4.1.6 Extract `old_string` from `payload.tool_input`
  - [ ] 4.1.7 Call `overlaps_protected_range` to check for overlap
  - [ ] 4.1.8 If overlap detected, return `HookResult::blocked` with detailed error message
- [ ] 4.2 Implement `overlaps_protected_range(old_string: &str, file_content: &str, ranges: &[UneditableRange]) -> Option<UneditableRange>` in `src/hooks.rs`
  - [ ] 4.2.1 Determine line range of `old_string` within `file_content`
  - [ ] 4.2.2 Iterate through protected ranges
  - [ ] 4.2.3 Check for overlap: `edit_lines.start <= range.end_line && edit_lines.end >= range.start_line`
  - [ ] 4.2.4 Return first overlapping range (if any)
- [ ] 4.3 Implement `find_edit_line_range(old_string: &str, file_content: &str) -> Option<(usize, usize)>` helper function
- [ ] 4.4 Add unit tests for overlap detection in `tests/hooks_tests.rs`
  - [ ] 4.4.1 Test edit fully inside protected range (blocked)
  - [ ] 4.4.2 Test edit partially overlapping protected range (blocked)
  - [ ] 4.4.3 Test edit outside protected range (allowed)
  - [ ] 4.4.4 Test edit touching marker lines (blocked)

## 5. Error Message Formatting

- [ ] 5.1 Update error message in `handle_pre_tool_use` to include:
  - [ ] 5.1.1 "Blocked Edit operation" prefix
  - [ ] 5.1.2 Protected line range (e.g., "lines 42-58")
  - [ ] 5.1.3 File path
  - [ ] 5.1.4 Custom message from `config.rules.uneditable_ranges.message` (if set)
- [ ] 5.2 Ensure error message is clear and actionable
- [ ] 5.3 Add tests for error message formatting in `tests/hooks_tests.rs`

## 6. Integration Tests

- [ ] 6.1 Create end-to-end test files for each supported language in `tests/fixtures/`
  - [ ] 6.1.1 Create `test_c_uneditable.c` with protected ranges
  - [ ] 6.1.2 Create `test_csharp_uneditable.cs` with protected ranges
  - [ ] 6.1.3 Create `test_go_uneditable.go` with protected ranges
  - [ ] 6.1.4 Create `test_java_uneditable.java` with protected ranges
  - [ ] 6.1.5 Create `test_python_uneditable.py` with protected ranges
  - [ ] 6.1.6 Create `test_javascript_uneditable.js` with protected ranges
  - [ ] 6.1.7 Create `test_rust_uneditable.rs` with protected ranges
  - [ ] 6.1.8 Create `test_ruby_uneditable.rb` with protected ranges
  - [ ] 6.1.9 Create `test_shell_uneditable.sh` with protected ranges
  - [ ] 6.1.10 Create `test_zig_uneditable.zig` with protected ranges
  - [ ] 6.1.11 Create `test_nim_uneditable.nim` with protected ranges
  - [ ] 6.1.12 Create `test_tsx_uneditable.tsx` with protected ranges
  - [ ] 6.1.13 Create `test_svelte_uneditable.svelte` with protected ranges
  - [ ] 6.1.14 Create `test_astro_uneditable.astro` with protected ranges
  - [ ] 6.1.15 Create `test_html_uneditable.html` with protected ranges
  - [ ] 6.1.16 Create `test_md_uneditable.md` with protected ranges
- [ ] 6.2 Write integration tests in `tests/integration_tests.rs`
  - [ ] 6.2.1 Test Edit operation blocked when overlapping protected range
  - [ ] 6.2.2 Test Edit operation allowed when outside protected range
  - [ ] 6.2.3 Test Write operation allowed even with protected ranges
  - [ ] 6.2.4 Test configuration disabled (no blocking occurs)
  - [ ] 6.2.5 Test custom error message displayed correctly
  - [ ] 6.2.6 Test mismatched markers return error
- [ ] 6.3 Run all integration tests and verify they pass

## 7. Documentation

- [ ] 7.1 Update `README.md` with uneditable ranges feature
  - [ ] 7.1.1 Add example configuration with `uneditableRanges`
  - [ ] 7.1.2 Show marker format for each supported language
  - [ ] 7.1.3 Explain use cases (auto-generated code, critical sections)
- [ ] 7.2 Update configuration examples in `src/schema.rs`
- [ ] 7.3 Add inline code comments documenting key functions
- [ ] 7.4 Update CHANGELOG.md with new feature

## 8. Performance Testing

- [ ] 8.1 Create benchmark test for parsing large files (10,000+ lines)
- [ ] 8.2 Verify parsing completes in under 100ms for large files
- [ ] 8.3 Verify no performance degradation for files without markers
- [ ] 8.4 Profile `handle_pre_tool_use` to ensure minimal overhead

## 9. Edge Case Handling

- [ ] 9.1 Test and handle unsupported file extensions (graceful fallback)
- [ ] 9.2 Test and handle files without extensions
- [ ] 9.3 Test and handle empty files
- [ ] 9.4 Test and handle files with only start or only end markers
- [ ] 9.5 Test and handle Unicode in file content (markers are ASCII)
- [ ] 9.6 Test and handle very large files (memory efficiency)
- [ ] 9.7 Test and handle deeply nested ranges (10+ levels)

## 10. Final Validation

- [ ] 10.1 Run full test suite: `cargo test`
- [ ] 10.2 Run clippy with no warnings: `cargo clippy -- -D warnings`
- [ ] 10.3 Run rustfmt to ensure formatting: `cargo fmt --check`
- [ ] 10.4 Validate with Spectr: `spectr validate add-uneditable-comment-ranges --strict`
- [ ] 10.5 Manual testing with real files and Claude Code sessions
- [ ] 10.6 Verify no regressions in existing functionality
- [ ] 10.7 Verify configuration schema validates correctly
- [ ] 10.8 Verify all specs pass validation

## 11. Deployment Preparation

- [ ] 11.1 Update version in `Cargo.toml` (if applicable)
- [ ] 11.2 Update CHANGELOG.md with release notes
- [ ] 11.3 Create pull request with detailed description
- [ ] 11.4 Request code review from maintainers
- [ ] 11.5 Address review feedback and iterate
- [ ] 11.6 Merge to main after approval
