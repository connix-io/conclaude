# Implementation Tasks: Uneditable Code Range Markers

## 1. Core Module Setup

- [ ] 1.1 Create new module `src/markers.rs`
- [ ] 1.2 Add module declaration to `src/lib.rs`
- [ ] 1.3 Define `ProtectedRange` struct with start_line, end_line, and optional reason
- [ ] 1.4 Define `Language` enum for supported languages
- [ ] 1.5 Define `CommentPattern` enum for comment syntax types

## 2. Language Detection Implementation

- [ ] 2.1 Implement `detect_language(file_path: &Path) -> Language` function
- [ ] 2.2 Add extension mapping for core languages (Go, Python, JS, TS, Rust, Java)
- [ ] 2.3 Add extension mapping for additional languages (C, C++, Ruby, PHP, Shell)
- [ ] 2.4 Add extension mapping for special file types (HTML, Markdown, YAML)
- [ ] 2.5 Implement fallback logic for unknown extensions
- [ ] 2.6 Write unit tests for language detection (all supported extensions)

## 3. Comment Pattern Mapping

- [ ] 3.1 Implement `get_comment_patterns(language: Language) -> Vec<&'static str>` function
- [ ] 3.2 Map double-slash languages (`//`: Go, Rust, JS, TS, Java, C++, C)
- [ ] 3.3 Map hash languages (`#`: Python, Ruby, Shell, YAML)
- [ ] 3.4 Map HTML-comment languages (`<!--`: HTML, Markdown, XML)
- [ ] 3.5 Map multi-pattern languages (C: both `//` and `/*`, PHP: both `//` and `#`)
- [ ] 3.6 Write unit tests for comment pattern mapping

## 4. Marker Detection Implementation

- [ ] 4.1 Implement `extract_protected_ranges(content: &str, patterns: &[&str]) -> Result<Vec<ProtectedRange>>` function
- [ ] 4.2 Implement line-by-line scanning with comment pattern matching
- [ ] 4.3 Detect `<!-- conclaude-uneditable:start -->` markers
- [ ] 4.4 Detect `<!-- conclaude-uneditable:end -->` markers
- [ ] 4.5 Build protected range from matched start/end pairs (inclusive of marker lines)
- [ ] 4.6 Implement stack-based tracking for marker pairing
- [ ] 4.7 Detect and error on unclosed start markers
- [ ] 4.8 Detect and error on unmatched end markers
- [ ] 4.9 Detect and error on nested markers (start-start-end-end)
- [ ] 4.10 Write unit tests for marker detection (valid, invalid, edge cases)

## 5. Range Validation Logic

- [ ] 5.1 Implement `has_overlap(edit_start: usize, edit_end: usize, protected: &ProtectedRange) -> bool` function
- [ ] 5.2 Handle overlap when edit starts inside range
- [ ] 5.3 Handle overlap when edit ends inside range
- [ ] 5.4 Handle overlap when edit fully contains range
- [ ] 5.5 Handle overlap when edit is fully contained by range
- [ ] 5.6 Handle no overlap cases (edit before or after range)
- [ ] 5.7 Write unit tests for overlap detection (all scenarios)

## 6. Edit Range Extraction from Payloads

- [ ] 6.1 Implement `extract_file_path(payload: &PreToolUsePayload) -> Result<PathBuf>` function
- [ ] 6.2 Handle `Edit` tool payload file path extraction
- [ ] 6.3 Handle `Write` tool payload file path extraction
- [ ] 6.4 Implement `extract_edit_range(payload: &PreToolUsePayload) -> Result<(usize, usize)>` function
- [ ] 6.5 Parse `Edit` tool parameters for start/end line numbers
- [ ] 6.6 Handle `Write` tool as full-file edit (lines 1 to EOF)
- [ ] 6.7 Write unit tests for payload parsing

## 7. PreToolUse Hook Integration

- [ ] 7.1 Add `check_uneditable_markers(payload, config_dir) -> Result<Option<String>>` function to `hooks.rs`
- [ ] 7.2 Call marker check in `handle_pre_tool_use` for `Edit` and `Write` tools only
- [ ] 7.3 Skip marker check for other tools (Read, Bash, etc.)
- [ ] 7.4 Read target file content
- [ ] 7.5 Detect language from file path
- [ ] 7.6 Get comment patterns for language
- [ ] 7.7 Extract protected ranges from file
- [ ] 7.8 Extract edit range from payload
- [ ] 7.9 Check for overlap between edit and protected ranges
- [ ] 7.10 Return blocked result with clear error message if overlap detected
- [ ] 7.11 Return allowed result if no overlap
- [ ] 7.12 Handle errors (file read failures, malformed markers, etc.)

## 8. Error Message Implementation

- [ ] 8.1 Implement clear error message for blocked edits (include line numbers, file path)
- [ ] 8.2 Implement error message for unclosed markers (include line number)
- [ ] 8.3 Implement error message for unmatched end markers (include line number)
- [ ] 8.4 Implement error message for nested markers (include line numbers)
- [ ] 8.5 Ensure error messages are actionable and guide users to fix issues

## 9. Unit Tests

- [ ] 9.1 Test marker detection in Go files (`//` comments)
- [ ] 9.2 Test marker detection in Python files (`#` comments)
- [ ] 9.3 Test marker detection in JavaScript/TypeScript files (`//` comments)
- [ ] 9.4 Test marker detection in Rust files (`//` comments)
- [ ] 9.5 Test marker detection in Java files (`//` comments)
- [ ] 9.6 Test marker detection in C/C++ files (`//` comments)
- [ ] 9.7 Test marker detection in Ruby files (`#` comments)
- [ ] 9.8 Test marker detection in Shell files (`#` comments)
- [ ] 9.9 Test marker detection in HTML/Markdown files (native `<!--` comments)
- [ ] 9.10 Test marker detection in YAML files (`#` comments)
- [ ] 9.11 Test multiple protected ranges in same file
- [ ] 9.12 Test unclosed marker error
- [ ] 9.13 Test unmatched end marker error
- [ ] 9.14 Test nested marker error
- [ ] 9.15 Test empty file (no markers)
- [ ] 9.16 Test file with no markers
- [ ] 9.17 Test overlap detection (all scenarios)
- [ ] 9.18 Test edit allowed (no overlap)
- [ ] 9.19 Test unknown file extension fallback

## 10. Integration Tests

- [ ] 10.1 Test PreToolUse hook blocks Edit operation touching protected range
- [ ] 10.2 Test PreToolUse hook allows Edit operation outside protected range
- [ ] 10.3 Test PreToolUse hook blocks Write operation overwriting protected file
- [ ] 10.4 Test PreToolUse hook allows Write operation for new files (no markers)
- [ ] 10.5 Test error message clarity for blocked operations
- [ ] 10.6 Test error message clarity for malformed markers
- [ ] 10.7 Test interaction with existing `uneditableFiles` glob patterns (both enforced independently)
- [ ] 10.8 Test performance on small files (< 1000 lines, < 5ms)
- [ ] 10.9 Test performance on medium files (1000-5000 lines, < 10ms)
- [ ] 10.10 Test performance on large files (5000-10000 lines, < 50ms)

## 11. Edge Case Handling

- [ ] 11.1 Handle binary files gracefully (skip marker detection)
- [ ] 11.2 Handle empty files (return empty protected ranges)
- [ ] 11.3 Handle files with only markers (no other content)
- [ ] 11.4 Handle markers in string literals (should not be detected)
- [ ] 11.5 Handle markers with extra whitespace (should be detected)
- [ ] 11.6 Handle markers with tabs (should be detected)
- [ ] 11.7 Handle deeply indented markers (should be detected)
- [ ] 11.8 Handle file read errors (propagate error clearly)
- [ ] 11.9 Handle JSON files (no comment syntax, skip marker detection)

## 12. Documentation

- [ ] 12.1 Add documentation to `markers.rs` module
- [ ] 12.2 Add function-level documentation for all public functions
- [ ] 12.3 Update `README.md` with marker feature explanation
- [ ] 12.4 Add examples for major languages (Go, Python, JS, Rust, etc.)
- [ ] 12.5 Document marker format and usage guidelines
- [ ] 12.6 Document error messages and how to resolve them
- [ ] 12.7 Add FAQ section for common questions (nested markers, custom formats, etc.)

## 13. Performance Optimization (if needed)

- [ ] 13.1 Benchmark marker detection on typical codebases
- [ ] 13.2 Profile performance on large files
- [ ] 13.3 Optimize if benchmarks show > 10ms average overhead
- [ ] 13.4 Consider early-exit optimizations (stop after finding relevant range)
- [ ] 13.5 Consider caching marker positions (future enhancement)

## 14. Final Validation

- [ ] 14.1 Run all unit tests (`cargo test`)
- [ ] 14.2 Run all integration tests
- [ ] 14.3 Run clippy lints (`cargo clippy`)
- [ ] 14.4 Run rustfmt (`cargo fmt`)
- [ ] 14.5 Test manually with real codebases (Go, Python, JS, Rust)
- [ ] 14.6 Verify error messages are clear and actionable
- [ ] 14.7 Verify performance is acceptable (< 10ms on typical files)
- [ ] 14.8 Update schema if configuration added (`cargo run --bin generate-schema`)
- [ ] 14.9 Run spectr validation if available (`spectr validate --strict`)

## Dependencies

- No external dependencies required (uses existing `glob`, `serde`, `anyhow`)
- Tasks 1-5 can be developed in parallel with tasks 6-7
- Tasks 8-11 depend on tasks 1-7 being complete
- Tasks 12-14 are final polish and validation

## Estimated Complexity

- **Core implementation (tasks 1-7):** Medium complexity, ~300-400 LOC
- **Testing (tasks 9-11):** High coverage needed, ~500-600 LOC
- **Documentation (task 12):** Moderate effort, clear examples needed
- **Total estimated effort:** Medium-large feature (2-3 days of focused work)
