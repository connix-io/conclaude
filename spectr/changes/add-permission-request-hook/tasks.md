# Implementation Tasks: PermissionRequest Hook Support

## Phase 1: Type System & Configuration (Tasks 1-6)

- [ ] 1. Add PermissionRequestPayload struct to src/types.rs
  - Define struct with base, tool_name, tool_input fields
  - Add serde annotations for JSON serialization
  - Validation function for PermissionRequestPayload

- [ ] 2. Extend HookPayload enum with PermissionRequest variant
  - Add PermissionRequest(PermissionRequestPayload) variant
  - Update #[serde(tag)] annotations

- [ ] 3. Extend HookPayload helper methods
  - Add PermissionRequest case to session_id() method
  - Add PermissionRequest case to transcript_path() method
  - Add PermissionRequest case to hook_event_name() method

- [ ] 4. Add PermissionRequestConfig struct to src/config.rs
  - Define struct with default, allow, deny, hooks fields
  - Add serde annotations for YAML deserialization
  - Validation function for config rules

- [ ] 5. Integrate PermissionRequestConfig into ConclaudeConfig
  - Add permission_request: Option<PermissionRequestConfig> field
  - Update serde naming to match "permissionRequest"
  - Ensure proper loading from YAML

- [ ] 6. Add schema generation for permission rules
  - Update src/schema.rs to generate JSON Schema for new config section
  - Validate schema includes default field as required string
  - Validate schema lists allow/deny as optional arrays of strings

## Phase 2: Hook Handler Implementation (Tasks 7-11)

- [ ] 7. Implement glob pattern matching utility
  - Create function to match tool names against patterns
  - Use glob::Pattern for compilation and matching
  - Support exact, wildcard, prefix, suffix, character class patterns
  - Handle pattern compilation errors gracefully

- [ ] 8. Implement pattern caching system
  - Create struct to hold compiled pattern cache
  - Compile patterns once at config load time
  - Provide access to compiled patterns during decision making
  - Ensure thread-safe cache if needed

- [ ] 9. Implement handle_permission_request() core logic
  - Extract tool_name from payload
  - Check deny patterns first (deny precedence)
  - Check allow patterns second
  - Return default decision if no match
  - Wrap result in HookResult

- [ ] 10. Implement external hook execution for permission decisions
  - Execute hook command with PermissionRequest payload
  - Set environment variables (CONCLAUDE_TOOL_NAME, etc.)
  - Parse hook response JSON
  - Fallback to rule-based decision on error

- [ ] 11. Implement environment variable setup
  - Create function to build CONCLAUDE_* env vars
  - Include CONCLAUDE_TOOL_NAME from payload
  - Include CONCLAUDE_PERMISSION_MODE from payload
  - Include all standard session context variables

## Phase 3: Integration & Configuration (Tasks 12-15)

- [ ] 12. Add hook dispatcher routing for PermissionRequest
  - Detect hook_event_name == "PermissionRequest"
  - Route to handle_permission_request()
  - Integrate with existing dispatcher pattern

- [ ] 13. Add PermissionRequest to default-config.yaml
  - Include commented example with whitelist approach
  - Include commented example with blacklist approach
  - Include commented example with external hook
  - Add documentation comments explaining patterns

- [ ] 14. Update configuration validation
  - Validate default field is "allow" or "deny"
  - Validate allow/deny patterns are non-empty strings
  - Log and skip invalid patterns
  - Provide helpful error messages

- [ ] 15. Add environment variable passing to hook execution
  - Update hook execution to include CONCLAUDE_TOOL_NAME
  - Ensure all CONCLAUDE_* vars passed to external hooks
  - Document available variables in code comments

## Phase 4: Testing (Tasks 16-24)

- [ ] 16. Unit test: Exact match pattern
  - Test that "Bash" matches "Bash" exactly
  - Test that "Bash" doesn't match "BashOutput"

- [ ] 17. Unit test: Wildcard pattern
  - Test that "*" matches any tool name
  - Test with various tool names (Bash, Read, Edit, etc.)

- [ ] 18. Unit test: Prefix and suffix patterns
  - Test "Edit*" matches "Edit", "EditFile", "EditPath"
  - Test "*Read" matches "Read", "FileRead", "BlobRead"
  - Test non-matches are rejected

- [ ] 19. Unit test: Character class patterns
  - Test "[BE]*" matches "Bash", "Edit"
  - Test "[0-9]*" patterns

- [ ] 20. Unit test: Deny precedence logic
  - Test deny rule blocks allow rule
  - Test specific deny blocks wildcard allow
  - Create multiple scenarios showing precedence

- [ ] 21. Unit test: Default decision fallback
  - Test default: deny when no rules match
  - Test default: allow when no rules match
  - Test with empty allow/deny lists

- [ ] 22. Unit test: Configuration validation
  - Test valid default values ("allow", "deny")
  - Test invalid default value rejected
  - Test missing default field detected
  - Test invalid patterns handled gracefully

- [ ] 23. Integration test: Payload deserialization
  - Create sample PermissionRequest JSON
  - Deserialize to PermissionRequestPayload
  - Verify all fields correctly parsed
  - Test with various tool_input structures

- [ ] 24. Integration test: Hook response format
  - Create hook response JSON with decision
  - Parse and validate response structure
  - Test blocked=true and blocked=false cases
  - Test optional message field

## Phase 5: Documentation & Examples (Tasks 25-28)

- [ ] 25. Add inline code documentation
  - Document PermissionRequestPayload purpose and fields
  - Document PermissionRequestConfig purpose and fields
  - Document handle_permission_request() algorithm
  - Add examples in doc comments

- [ ] 26. Create configuration documentation
  - Document whitelist vs blacklist approaches
  - Document glob pattern syntax
  - Document environment variable names and values
  - Document external hook response format

- [ ] 27. Document security considerations
  - Explain deny precedence as fail-safe
  - Recommend whitelist default for security
  - Document that decisions are logged for audit
  - Warn about regex complexity limits

- [ ] 28. Create examples for common use cases
  - Example: Allow only read-only tools
  - Example: Block dangerous tools
  - Example: Custom logic via external hook
  - Example: Combination of rules and default

## Phase 6: Validation & Polish (Tasks 29-30)

- [ ] 29. Run strict validation
  - Execute `openspec validate add-permission-request-hook --strict`
  - Fix any validation errors
  - Ensure all spec files are valid

- [ ] 30. Final review and cleanup
  - Review all source code changes for style consistency
  - Ensure error messages are user-friendly
  - Verify logging is at appropriate levels
  - Check for TODOs or FIXMEs to address

## Dependencies

- Phase 2 depends on Phase 1 (types must exist before handler implementation)
- Phase 3 depends on Phase 1 and Phase 2
- Phase 4 tests both Phase 2 and Phase 3
- Phase 5 and Phase 6 depend on all previous phases

## Validation Checkpoints

**After Phase 1:**
- [ ] All types compile without errors
- [ ] Schema generation produces valid JSON schema
- [ ] Config loads from YAML correctly

**After Phase 2:**
- [ ] Pattern matching works for all pattern types
- [ ] Deny precedence logic is correct
- [ ] External hooks execute and parse responses

**After Phase 3:**
- [ ] Hook dispatcher routes PermissionRequest events correctly
- [ ] Default config loads without errors
- [ ] Environment variables passed to hooks

**After Phase 4:**
- [ ] All unit tests pass
- [ ] All integration tests pass
- [ ] Code coverage for handler logic >90%

**After Phase 5:**
- [ ] Documentation complete and accurate
- [ ] Examples are functional and clear

**Before Submission:**
- [ ] `openspec validate add-permission-request-hook --strict` passes
- [ ] No compiler warnings
- [ ] All tests pass
