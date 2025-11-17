# Proposal: Add agent_id and agent_transcript_path to SubagentStop Payload

**Change ID:** `add-subagent-stop-payload-fields`
**Status:** Draft
**Created:** 2025-11-16
**Author:** AI Assistant

## Overview

Add `agent_id` and `agent_transcript_path` fields to the `SubagentStopPayload` structure to support Claude Code's new agent tracking capabilities. These fields enable conclaude to identify which specific subagent completed and access its transcript without parsing file paths.

## Motivation

Claude Code now provides explicit agent identification when SubagentStop hooks fire:
- `agent_id`: Unique identifier for the subagent instance (e.g., "coder", "tester", "stuck")
- `agent_transcript_path`: Path to the subagent's specific transcript file

**Current State:**
- SubagentStopPayload only contains base fields (session_id, transcript_path, etc.) and `stop_hook_active`
- No way to identify which subagent completed without parsing transcript files
- Existing `add-subagent-stop-commands` proposal planned to extract subagent names via parsing

**With This Change:**
- Direct access to agent identification from Claude Code
- Cleaner, more reliable than file parsing
- Enables the `add-subagent-stop-commands` proposal to use structured data
- Allows future pattern matching and conditional hook execution

## Scope

### In Scope
- Add `agent_id` (required String) to SubagentStopPayload
- Add `agent_transcript_path` (required String) to SubagentStopPayload
- Export fields as environment variables: `CONCLAUDE_AGENT_ID` and `CONCLAUDE_AGENT_TRANSCRIPT_PATH`
- Update payload validation to require these fields
- Add test coverage for new fields
- Auto-update schema.json via schema generation

### Out of Scope
- Pattern matching or conditional execution (handled by `add-subagent-stop-commands`)
- Command execution configuration (handled by `add-subagent-stop-commands`)
- Backward compatibility with old Claude Code versions (fields are required)

## Dependencies

### Depends On
- None (foundational change)

### Enables
- `add-subagent-stop-commands` - Can use agent_id instead of parsing transcript paths
- Future agent-specific hook configurations
- Future analytics/logging features that track subagent activity

## Affected Components

- `src/types.rs` - SubagentStopPayload struct definition
- `src/hooks.rs` - SubagentStop handler to validate and expose fields
- `tests/types_tests.rs` - Payload validation tests
- `tests/hooks_tests.rs` - Hook integration tests
- `schema.json` - Auto-generated schema update

## Risks and Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| Breaking change for old Claude Code versions | High | Document minimum Claude Code version requirement |
| Schema validation failures | Medium | Validate fields are present before processing |
| Test coverage gaps | Low | Add comprehensive tests for new fields |

## Success Criteria

- [ ] SubagentStopPayload includes agent_id and agent_transcript_path
- [ ] Fields are validated as required (non-empty strings)
- [ ] Environment variables CONCLAUDE_AGENT_ID and CONCLAUDE_AGENT_TRANSCRIPT_PATH are set
- [ ] Tests verify payload parsing with new fields
- [ ] Tests verify validation rejects missing fields
- [ ] `openspec validate add-subagent-stop-payload-fields --strict` passes
- [ ] schema.json includes new fields in SubagentStopPayload definition

## Implementation Notes

The existing `add-subagent-stop-commands` proposal should be updated to:
1. Depend on this proposal (add to dependencies section)
2. Remove transcript parsing logic for extracting subagent names
3. Use `agent_id` field directly instead

This creates a clean separation:
- **This proposal:** Receive and validate payload fields from Claude Code
- **add-subagent-stop-commands:** Use those fields for conditional command execution
