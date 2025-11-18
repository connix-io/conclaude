# Tasks: Refine preventRootAdditions to Allow Root File Edits

## Implementation Order

### Phase 1: Test-Driven Development (RED)

#### Task 1.1: Write unit tests for refined preventRootAdditions behavior
- **File:** `tests/hooks_tests.rs` (extend existing prevent_root_additions tests)
- **Tests to add:**
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
- **Expected:** All tests FAIL (red phase)
- **Validation:** `cargo test hooks_tests::prevent_root_additions -- --nocapture`

#### Task 1.2: Run tests to confirm failures
- **Command:** `cargo test hooks_tests::prevent_root_additions --lib -- --nocapture`
- **Expected:** All new tests fail with assertion errors
- **Validation:** Document test failures for clarity

### Phase 2: Implementation (GREEN)

#### Task 2.1: Add file existence check to check_file_validation_rules()
- **File:** `src/hooks.rs`
- **Function:** `check_file_validation_rules()` (lines 298-314)
- **Change:** Add `&& !resolved_path.exists()` to the preventRootAdditions condition
- **Code:**
  ```rust
  // Check preventRootAdditions rule - only applies to Write tool for NEW files
  if config.rules.prevent_root_additions
      && payload.tool_name == "Write"
      && is_root_addition(&file_path, &relative_path, config_path)
      && !resolved_path.exists()  // NEW: Allow modifications to existing files
  {
      // ... block logic unchanged ...
  }
  ```
- **Validation:** Syntax check, no compilation errors

#### Task 2.2: Run unit tests to verify implementation
- **Command:** `cargo test hooks_tests::prevent_root_additions --lib -- --nocapture`
- **Expected:** All tests PASS (green phase)
- **Validation:** Zero test failures

#### Task 2.3: Run full test suite to check for regressions
- **Command:** `cargo test --lib`
- **Expected:** All existing tests pass
- **Validation:** No regressions in unrelated tests

### Phase 3: Validation (REFACTOR)

#### Task 3.1: Manual testing with real configuration
- **Setup:** Create test config with `preventRootAdditions: true`
- **Test 1:** Attempt to create new root file → BLOCKED ✓
- **Test 2:** Attempt to edit existing root file → ALLOWED ✓
- **Test 3:** Attempt to create non-root file → ALLOWED ✓
- **Test 4:** Verify error messages are clear
- **Command:** Use project's CLI or hook invocation
- **Validation:** Behavior matches spec

#### Task 3.2: Integration test with hook system
- **File:** `tests/integration_tests.rs`
- **Scope:** Full hook cycle with real config loading
- **Test scenarios:**
  - Load config with preventRootAdditions
  - Invoke PreToolUse hook for Write to new root file
  - Invoke PreToolUse hook for Write to existing root file
  - Verify blocking and allowing behavior
- **Command:** `cargo test --test integration_tests`
- **Validation:** All integration tests pass

#### Task 3.3: Code review for clarity
- **Review:** `src/hooks.rs` changes
- **Checklist:**
  - [ ] Change is minimal and focused
  - [ ] Error messages unchanged (backward compatible)
  - [ ] Comment explains the file existence check
  - [ ] No unnecessary code added
  - [ ] Follows project style guide
- **Validation:** Code approved

#### Task 3.4: Run full project test suite
- **Commands:**
  ```bash
  cargo test --lib
  cargo test --test "*"
  cargo clippy
  cargo fmt --check
  ```
- **Expected:** All tests pass, no clippy warnings, proper formatting
- **Validation:** Zero issues

### Phase 4: Documentation (RELEASE)

#### Task 4.1: Update README.md with refined semantics
- **File:** `README.md`
- **Sections to update:**
  - preventRootAdditions description (clarify "additions" = creation)
  - Example showing blocking new files but allowing edits
  - Comparison with uneditableFiles rule
- **Validation:** Documentation reflects actual behavior

#### Task 4.2: Update CHANGELOG.md
- **File:** `CHANGELOG.md`
- **Entry:**
  ```markdown
  ### Fixed
  - **preventRootAdditions now allows editing existing root files**
    - Refined semantics to distinguish between file creation (blocked) and modification (allowed)
    - Users can now edit root-level configuration files like `.env` and `package.json`
    - Maintains protection against accidental root file creation
  ```
- **Validation:** Entry is clear and in correct format

#### Task 4.3: Update default config example
- **File:** `src/default-config.yaml`
- **Section:** Rules section with preventRootAdditions
- **Add comment:**
  ```yaml
  rules:
    # Prevent creating new files at repository root
    # Allows editing existing root-level configuration files
    preventRootAdditions: true
  ```
- **Validation:** Example is accurate and helpful

#### Task 4.4: Add code comment to implementation
- **File:** `src/hooks.rs`
- **Location:** preventRootAdditions check
- **Comment:** Explain why file existence check is important
  ```rust
  // Check preventRootAdditions rule - only applies to Write tool for NEW files
  // File existence check allows modifications to existing root files (e.g., package.json)
  // but prevents creation of new files at root
  ```
- **Validation:** Comment is clear and helpful

### Phase 5: Verification

#### Task 5.1: Final regression test
- **Command:** `cargo test --all`
- **Expected:** All tests pass
- **Validation:** Zero failures

#### Task 5.2: Validate with openspec
- **Command:** `openspec validate refine-prevent-root-additions --strict`
- **Expected:** Validation passes
- **Validation:** All spec requirements verified

#### Task 5.3: Create test scenario document
- **File:** `openspec/changes/refine-prevent-root-additions/TEST_SCENARIOS.md` (optional)
- **Content:** Step-by-step manual test scenarios for QA
- **Validation:** Clear, reproducible scenarios

---

## Task Dependencies

```
Phase 1 (Tests)
  ├─ 1.1: Write tests
  └─ 1.2: Confirm test failures
        ↓
Phase 2 (Implementation)
  ├─ 2.1: Add file existence check
  ├─ 2.2: Confirm tests pass
  └─ 2.3: Run full test suite
        ↓
Phase 3 (Validation)
  ├─ 3.1: Manual testing
  ├─ 3.2: Integration tests
  ├─ 3.3: Code review
  └─ 3.4: Full suite validation
        ↓
Phase 4 (Documentation)
  ├─ 4.1: Update README
  ├─ 4.2: Update CHANGELOG
  ├─ 4.3: Update default config
  └─ 4.4: Add code comments
        ↓
Phase 5 (Final Verification)
  ├─ 5.1: Final regression test
  ├─ 5.2: OpenSpec validation
  └─ 5.3: Create test scenarios (optional)
```

## Parallel Work

The following tasks can run in parallel:
- 4.1, 4.2, 4.3, 4.4 (documentation tasks)
- 3.1 and 3.2 (testing tasks can overlap)

## Success Criteria

- ✅ All unit tests pass (Phase 2.2)
- ✅ No regressions in existing tests (Phase 2.3)
- ✅ Manual testing validates behavior (Phase 3.1)
- ✅ Integration tests pass (Phase 3.2)
- ✅ Documentation updated (Phase 4)
- ✅ Final regression test passes (Phase 5.1)
- ✅ OpenSpec validation passes (Phase 5.2)
- ✅ Error messages unchanged (backward compatible)
- ✅ No performance degradation

## Effort Estimate

| Phase | Tasks | Effort | Duration |
|-------|-------|--------|----------|
| 1 | 1.1-1.2 | ~1 hour | Parallel |
| 2 | 2.1-2.3 | ~30 min | Sequential |
| 3 | 3.1-3.4 | ~1 hour | Sequential |
| 4 | 4.1-4.4 | ~30 min | Parallel |
| 5 | 5.1-5.3 | ~30 min | Sequential |
| **Total** | **15** | **~3.5 hours** | **~2-3 hours** |

---

**Note:** Effort estimates assume developer familiarity with codebase. Actual time may vary.
