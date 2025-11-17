# Implementation Tasks: Add SubagentStop Payload Fields

**Change ID:** `add-subagent-stop-payload-fields`

## Task Breakdown

### 1. Update SubagentStopPayload struct definition
**File:** `src/types.rs`
**Dependencies:** None
**Validation:** Compiles without errors

**Actions:**
- Add `agent_id: String` field to SubagentStopPayload (around line 99)
- Add `agent_transcript_path: String` field to SubagentStopPayload
- Add rustdoc comments for both fields
- Ensure fields come after `stop_hook_active` to maintain logical order

**Acceptance:**
- SubagentStopPayload struct includes both new fields as required (non-Option) Strings
- Fields have clear documentation comments
- Code compiles successfully

---

### 2. Add payload validation for new fields
**File:** `src/types.rs`
**Dependencies:** Task 1
**Validation:** Unit tests pass

**Actions:**
- Create `validate_subagent_stop_payload()` function or extend existing validation
- Check that `agent_id` is not empty (trim and validate)
- Check that `agent_transcript_path` is not empty (trim and validate)
- Return descriptive error messages for validation failures

**Acceptance:**
- Validation rejects empty `agent_id`
- Validation rejects empty `agent_transcript_path`
- Error messages clearly indicate which field failed validation

---

### 3. Update SubagentStop hook handler
**File:** `src/hooks.rs`
**Dependencies:** Tasks 1, 2
**Validation:** Manual testing with sample payload

**Actions:**
- Update `handle_subagent_stop()` function (around line 832)
- Call validation for new fields (use new validation function from Task 2)
- Set environment variable `CONCLAUDE_AGENT_ID` from `payload.agent_id`
- Set environment variable `CONCLAUDE_AGENT_TRANSCRIPT_PATH` from `payload.agent_transcript_path`
- Update notification/logging to include agent_id

**Acceptance:**
- Environment variables are set before any command execution
- Validation is called and errors are properly handled
- Logs/notifications show which agent completed (e.g., "Subagent 'coder' completed")

---

### 4. Add unit tests for payload parsing
**File:** `tests/types_tests.rs`
**Dependencies:** Tasks 1, 2
**Validation:** `cargo test types_tests` passes

**Actions:**
- Test: Parse valid SubagentStopPayload with all required fields
- Test: Reject payload missing `agent_id` field
- Test: Reject payload missing `agent_transcript_path` field
- Test: Reject payload with empty string `agent_id`
- Test: Reject payload with empty string `agent_transcript_path`
- Test: Validate deserialization from JSON works correctly

**Acceptance:**
- All new tests pass
- Test coverage includes success and failure cases
- Tests verify error messages are descriptive

---

### 5. Add integration tests for SubagentStop hook
**File:** `tests/hooks_tests.rs`
**Dependencies:** Tasks 1, 2, 3
**Validation:** `cargo test hooks_tests` passes

**Actions:**
- Test: SubagentStop hook successfully processes payload with new fields
- Test: Environment variables CONCLAUDE_AGENT_ID and CONCLAUDE_AGENT_TRANSCRIPT_PATH are set
- Test: Hook execution fails gracefully when fields are missing
- Mock/simulate SubagentStop hook invocation with test payloads

**Acceptance:**
- Integration tests verify end-to-end hook processing
- Environment variable exports are tested
- Tests confirm validation failures prevent hook execution

---

### 6. Regenerate schema.json
**File:** `schema.json`
**Dependencies:** Task 1
**Validation:** Schema includes new fields

**Actions:**
- Run schema generation command (likely via build script or `cargo run --bin generate-schema` or similar)
- Verify `SubagentStopPayload` in schema includes `agent_id` and `agent_transcript_path`
- Verify fields are marked as required (not optional) in JSON schema
- Commit regenerated schema.json

**Acceptance:**
- schema.json includes both new fields in SubagentStopPayload definition
- Fields are required in the JSON schema
- Schema validates test payloads correctly

---

### 7. Update documentation and examples
**File:** `README.md`, docs, or inline examples
**Dependencies:** Tasks 1-6 complete
**Validation:** Documentation is clear and accurate

**Actions:**
- Document the new payload fields in relevant documentation
- Add example JSON payload showing agent_id and agent_transcript_path
- Document environment variables CONCLAUDE_AGENT_ID and CONCLAUDE_AGENT_TRANSCRIPT_PATH
- Note minimum Claude Code version requirement (once known)

**Acceptance:**
- Users understand the new fields and how to use them
- Example payloads are valid and complete
- Environment variable documentation is accessible

---

### 8. Run full test suite and validation
**File:** N/A (testing)
**Dependencies:** Tasks 1-7 complete
**Validation:** All tests pass, openspec validation succeeds

**Actions:**
- Run `cargo test` - all tests must pass
- Run `cargo clippy` - no warnings or errors
- Run `cargo build --release` - clean build
- Run `openspec validate add-subagent-stop-payload-fields --strict` - must pass
- Manually test with sample SubagentStop hook invocation

**Acceptance:**
- Zero test failures
- Zero clippy warnings
- Clean release build
- OpenSpec validation passes with --strict
- Manual testing confirms functionality works as specified

---

## Task Sequencing

**Sequential (must be done in order):**
1. Task 1 → Task 2 → Task 3 (core implementation chain)
2. Task 6 depends on Task 1 (schema generation needs struct changes)
3. Task 7 depends on Tasks 1-6 (document after implementation complete)
4. Task 8 depends on all others (final validation)

**Parallel (can be done simultaneously):**
- Task 4 and Task 5 can be developed in parallel after Tasks 1-2 complete
- Task 6 can run in parallel with Tasks 4-5 after Task 1 completes

**Critical Path:** 1 → 2 → 3 → 8 (core implementation and validation)

---

## Estimated Effort

- **Task 1:** 15 minutes (straightforward struct field additions)
- **Task 2:** 20 minutes (validation logic and error handling)
- **Task 3:** 25 minutes (hook handler updates and env var setting)
- **Task 4:** 30 minutes (comprehensive unit tests)
- **Task 5:** 30 minutes (integration test setup and execution)
- **Task 6:** 10 minutes (schema regeneration, mostly automated)
- **Task 7:** 20 minutes (documentation and examples)
- **Task 8:** 30 minutes (full validation and manual testing)

**Total:** ~3 hours of focused development and testing

---

## Success Criteria Checklist

Before marking this proposal as complete, verify:

- [x] SubagentStopPayload struct has agent_id and agent_transcript_path fields
- [x] Both fields are required (non-Optional) Strings
- [x] Validation rejects missing or empty field values
- [x] Environment variables CONCLAUDE_AGENT_ID and CONCLAUDE_AGENT_TRANSCRIPT_PATH are exported
- [x] Unit tests cover success and failure scenarios
- [x] Integration tests verify end-to-end hook processing
- [x] schema.json includes new fields as required
- [x] All tests pass (`cargo test`)
- [x] No clippy warnings (`cargo clippy`)
- [x] OpenSpec validation passes (`openspec validate add-subagent-stop-payload-fields --strict`)
- [x] Documentation updated with new fields and environment variables
- [x] Manual testing confirms hooks receive and process new fields correctly
