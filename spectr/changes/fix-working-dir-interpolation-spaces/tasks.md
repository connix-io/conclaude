## 1. Specification
- [x] 1.1 Add execution spec coverage that workingDir interpolation preserves whitespace in paths

## 2. Implementation
- [x] 2.1 Update workingDir interpolation/quoting to keep spaces intact while still performing bash expansions
- [x] 2.2 Add unit tests for workingDir values containing spaces across stop and subagent stop handling

## 3. Validation
- [x] 3.1 Run targeted tests for hooks interpolation logic (e.g., `cargo test hooks::tests::test_interpolate_working_dir_*`)
