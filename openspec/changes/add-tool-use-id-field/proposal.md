# Proposal: Add tool_use_id Field to Tool Use Hook Payloads

## Why

Claude Code now provides a unique identifier (`tool_use_id`) for each tool invocation in its hook payloads. Conclaude must support this field to maintain protocol compatibility and enable correlation of PreToolUse and PostToolUse events for the same tool execution.

## Problem
Claude Code has added a new `tool_use_id` field to `PreToolUseHookInput` and `PostToolUseHookInput` types. Conclaude must support this field to maintain compatibility with the latest Claude Code lifecycle hook protocol.

## Context
Claude Code now provides a unique identifier (`tool_use_id`) for each tool invocation. This allows hook handlers to correlate PreToolUse and PostToolUse events for the same tool execution, enabling:
- Audit trails linking tool requests to their results
- Performance tracking for specific tool invocations
- Debug workflows that trace a tool call from request through completion
- State management across the tool lifecycle

Currently, conclaude's `PreToolUsePayload` and `PostToolUsePayload` types do not include this field, meaning:
- Hook handlers cannot access the tool_use_id from Claude Code
- JSON deserialization may fail or ignore the field (depending on serde settings)
- We're not fully aligned with the upstream Claude Code protocol

## Proposed Solution
Add a required `tool_use_id: String` field to both `PreToolUsePayload` and `PostToolUsePayload` structs in `src/types.rs`.

### Why Required?
- **Protocol alignment**: Claude Code now always sends this field
- **Simpler implementation**: No need to handle Option unwrapping
- **Type safety**: Guarantees the field is always present for correlation logic

## Impact
- **Low risk**: Adding a required field that Claude Code now provides
- **No configuration changes**: No user-facing config updates needed
- **Documentation**: Update payload documentation to describe the field
- **Testing**: Add test cases for payloads with tool_use_id

## Files Affected
- `src/types.rs` - Add field to PreToolUsePayload and PostToolUsePayload
- `tests/types_tests.rs` - Add serialization/deserialization tests
- Documentation (if applicable)

## Dependencies
None - this is a self-contained type system update

## Alternatives Considered
1. **Make it optional** - Rejected: unnecessary complexity since Claude Code always provides it
2. **Add to BasePayload** - Rejected: only tool-related hooks receive this field, not all hooks
3. **Wait for more fields** - Rejected: better to track upstream changes incrementally

## Success Criteria
- [x] `tool_use_id` field added to PreToolUsePayload as required String
- [x] `tool_use_id` field added to PostToolUsePayload as required String
- [x] Tests verify serialization/deserialization with tool_use_id
- [x] `openspec validate --strict` passes
