# Tasks: Refine preventRootAdditions to Allow Root File Edits

## Implementation Order

### Phase 1: Test-Driven Development (RED)

#### Task 1.1: Write unit tests for refined preventRootAdditions behavior
- [x] **File:** `tests/hooks_tests.rs` (extend existing prevent_root_additions tests)
- [x] **Tests to add:**
  1. Block creation of new root file (no prior file exists)
  2. Allow Write to existing root file (file exists - overwrite)
  3. Allow Edit tool on root file (Edit not blocked by preventRootAdditions)
  4. Allow NotebookEdit on root file
  5. Block creation when sibling file exists (only target matters)
  6. Verify error message for blocked operation
  7. Verify no error for allowed modification
  8. Test with relative paths
  9. Test with canonical path resolution
  10. Interaction with uneditableFiles (both rules enforced)
- [x] **Validation:** `cargo test hooks_tests::prevent_root_additions -- --nocapture`

#### Task 1.2: Run tests to confirm failures
- [x] **Command:** `cargo test hooks_tests::prevent_root_additions --lib -- --nocapture`
- [x] **Validation:** Document test failures for clarity

### Phase 2: Implementation (GREEN)

#### Task 2.1: Add file existence check to check_file_validation_rules()
- [x] **File:** `src/hooks.rs`
- [x] **Function:** `check_file_validation_rules()` (lines 298-314)
- [x] **Change:** Add `&& !resolved_path.exists()` to the preventRootAdditions condition
- [x] **Code:**
  ```rust
  // Check preventRootAdditions rule - only applies to Write tool for NEW files
  // File existence check allows modifications to existing root files (e.g., package.json)
  // but prevents creation of new files at root
  if config.pre_tool_use.prevent_root_additions
      && payload.tool_name == "Write"
      && is_root_addition(&file_path, &relative_path, config_path)
      && !resolved_path.exists()  // NEW: Allow modifications to existing files
  {
      // ... block logic unchanged ...
  }
  ```
- [x] **Validation:** Syntax check, no compilation errors

#### Task 2.2: Run unit tests to verify implementation
- [x] **Command:** `cargo test hooks_tests::prevent_root_additions --lib -- --nocapture`
- [x] **Validation:** All tests PASS (green phase)

#### Task 2.3: Run full test suite to check for regressions
- [x] **Command:** `cargo test --lib`
- [x] **Validation:** All existing tests pass (zero regressions)

### Phase 3: Validation (REFACTOR)

#### Task 3.1: Manual testing with real configuration
- [x] **Setup:** Create test config with `preventRootAdditions: true`
- [x] **Validation:** Behavior matches spec

#### Task 3.2: Integration test with hook system
- [x] **Command:** `cargo test --test hooks_tests`
- [x] **Validation:** All integration tests pass (94 tests)

#### Task 3.3: Code review for clarity
- [x] **Review:** `src/hooks.rs` changes
- [x] **Checklist:**
  - [x] Change is minimal and focused
  - [x] Error messages unchanged (backward compatible)
  - [x] Comment explains the file existence check
  - [x] No unnecessary code added
  - [x] Follows project style guide
- [x] **Validation:** Code approved

#### Task 3.4: Run full project test suite
- [x] **Commands:**
  ```bash
  cargo test --lib
  cargo test --test "*"
  cargo clippy
  cargo fmt --check
  ```
- [x] **Validation:** All tests pass, no clippy warnings, proper formatting

### Phase 4: Documentation (RELEASE)

#### Task 4.1: Update README.md with refined semantics
- [x] **File:** `README.md`
- [x] **Sections updated:**
  - preventRootAdditions description (clarified "additions" = creation of new files)
  - Example showing blocking new files but allowing edits to existing files
- [x] **Validation:** Documentation reflects actual behavior

#### Task 4.2: Update CHANGELOG.md
- [x] **File:** `CHANGELOG.md`
- [x] **Entry added:**
  ```markdown
  ### Fixed
  - **preventRootAdditions now allows editing existing root files**: Refined semantics to distinguish between file creation (blocked) and modification (allowed). Users can now edit root-level configuration files like `Cargo.toml` and `package.json` while still preventing accidental new file creation at the repository root.
  ```
- [x] **Validation:** Entry is clear and in correct format

#### Task 4.3: Update default config example
- [x] **File:** `README.md` config reference section
- [x] **Updated comment in configuration examples**
- [x] **Validation:** Example is accurate and helpful

#### Task 4.4: Add code comment to implementation
- [x] **File:** `src/hooks.rs`
- [x] **Location:** preventRootAdditions check
- [x] **Comment added:**
  ```rust
  // Check preventRootAdditions rule - only applies to Write tool for NEW files
  // File existence check allows modifications to existing root files (e.g., package.json)
  // but prevents creation of new files at root
  ```
- [x] **Validation:** Comment is clear and helpful

### Phase 5: Verification

#### Task 5.1: Final regression test
- [x] **Command:** `cargo test --all`
- [x] **Validation:** All tests pass (351 total tests)

#### Task 5.2: Validate with spectr
- [x] **Command:** `spectr validate refine-prevent-root-additions --strict`
- [x] **Validation:** Validation passes

---

## Success Criteria

- [x] All unit tests pass (Phase 2.2)
- [x] No regressions in existing tests (Phase 2.3)
- [x] Manual testing validates behavior (Phase 3.1)
- [x] Integration tests pass (Phase 3.2)
- [x] Documentation updated (Phase 4)
- [x] Final regression test passes (Phase 5.1)
- [x] Spectr validation passes (Phase 5.2)
- [x] Error messages unchanged (backward compatible)
- [x] No performance degradation (single file exists check, minimal overhead)

---

**Implementation Complete:** All tasks verified and passing.
