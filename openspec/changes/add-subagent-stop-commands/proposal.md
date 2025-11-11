# Proposal: Subagent-Specific Stop Hook Commands

**Change ID:** `add-subagent-stop-commands`
**Status:** Proposal
**Author:** Claude Code
**Date:** 2025-11-11

## Executive Summary

Enable users to configure stop hook commands that run when specific subagents (or all subagents) terminate. This extends the existing `SubagentStop` hook with configurable command execution, glob pattern matching for subagent names, and environment variable context passing.

**Key benefits:**
- Run cleanup, validation, or logging commands when specific subagents complete
- Use glob patterns to target groups of subagents (e.g., `test*`, `*coder`)
- Access subagent context (name, session ID) via environment variables in commands
- Full backward compatibility (SubagentStop notification behavior unchanged)

## Why

The existing `SubagentStop` hook currently only sends system notifications but cannot execute commands. Users need programmatic control to run cleanup scripts, validation checks, or logging when specific subagents complete their work.

## Motivation

### Current State
The `SubagentStop` hook is fired when Claude subagents terminate, but it only supports system notifications. There is no way to:

1. **Run commands** when subagents stop (like the `stop` hook does for main sessions)
2. **Target specific subagents** by name or pattern
3. **Access subagent context** (name, session ID) in custom commands
4. **Differentiate behavior** based on which subagent completed

### User Request
Users need the ability to run commands when subagents stop, with matching logic for specific subagent names and wildcard patterns, passing subagent context via environment variables.

## What Changes

1. **Configuration Schema** - Add new `subagentStop` section to YAML config with command execution and glob pattern matching
2. **Transcript Parsing** - Extract subagent name from transcript file when `SubagentStop` hook fires
3. **Environment Variables** - Pass subagent context (name, session ID, transcript path) to all hook commands
4. **Command Execution** - Support both specific subagent name matching and glob patterns (`*`, `test*`, `*coder`, `agent_[0-9]*`)
5. **Matching Logic** - Run both wildcard and specific commands when both match
6. **Type System** - Add `SubagentStopConfig` struct similar to existing `StopConfig`
7. **Documentation** - Configuration examples and pattern matching guide

## Scope

### What's Included

**Configuration:**
- New `subagentStop` section in YAML config (separate from `stop`)
- Support for exact match, wildcard (`*`), and full glob patterns
- Command configuration per subagent pattern (run, message, showStdout, showStderr, maxOutputLines)
- Environment variable passing for all commands

**Execution:**
- Parse transcript file to extract subagent name from metadata
- Match subagent name against configured patterns (glob-based)
- Execute matching commands (both wildcard and specific)
- Pass context via environment variables: `CONCLAUDE_SUBAGENT_NAME`, `CONCLAUDE_SESSION_ID`, `CONCLAUDE_TRANSCRIPT_PATH`, `CONCLAUDE_HOOK_EVENT`

**Pattern Matching:**
- Exact match: `coder` matches only `coder`
- Wildcard: `*` matches all subagents
- Prefix glob: `test*` matches `tester`, `test-runner`, etc.
- Suffix glob: `*coder` matches `coder`, `auto-coder`, etc.
- Complex glob: `agent_[0-9]*` matches `agent_1`, `agent_2x`, etc.

### What's NOT Included

- Changes to existing `stop` hook behavior
- Subagent lifecycle hooks beyond stop (start, pause, etc.)
- Subagent execution control (blocking, restarting)
- Docker container support for subagent hooks (future enhancement)
- Timeout configuration (uses existing command execution timeout logic)

## Questions & Decisions

### Q: How should commands access subagent context?
**Decision:** Pass via environment variables in all hook commands:
- `CONCLAUDE_SUBAGENT_NAME` - Name of the subagent that stopped
- `CONCLAUDE_SESSION_ID` - Session ID from payload
- `CONCLAUDE_TRANSCRIPT_PATH` - Transcript file path
- `CONCLAUDE_HOOK_EVENT` - Always "SubagentStop"
- `CONCLAUDE_CWD` - Current working directory

This provides maximum flexibility and follows Unix conventions.

### Q: How to obtain subagent name from SubagentStop payload?
**Decision:** Parse transcript file to extract subagent name from metadata.

The `SubagentStopPayload` does not include subagent name directly. We'll parse the JSONL transcript file to find the most recent subagent invocation and extract its name/type.

### Q: When both wildcard and specific commands match, what should happen?
**Decision:** Run both wildcard and specific commands in order.

Execution order:
1. Wildcard (`*`) commands (if configured)
2. Specific matching commands (exact or glob pattern)

This allows global cleanup plus targeted actions.

### Q: Should subagent stop hooks support glob patterns beyond exact match and wildcard?
**Decision:** Yes, full glob support using the `glob` crate's `Pattern::matches`.

Supported patterns:
- `*` - matches all subagents
- `coder` - exact match
- `test*` - prefix match
- `*coder` - suffix match
- `agent_[0-9]*` - character class patterns

### Q: How should the configuration be structured in YAML?
**Decision:** New `subagentStop` section with pattern-based command mapping.

```yaml
subagentStop:
  commands:
    # Wildcard - runs for all subagents
    "*":
      - run: "echo 'Subagent completed'"
        message: "Logging subagent completion"

    # Exact match
    "coder":
      - run: "npm run lint"
        message: "Running linter after coder"

    # Glob patterns
    "test*":
      - run: "echo 'Test agent completed'"

    "*coder":
      - run: "git add ."
        message: "Staging changes from coder agents"
```

## Success Criteria

1. **Configuration validates** - Schema accepts `subagentStop` section with pattern-based commands
2. **Transcript parsing works** - Subagent name correctly extracted from transcript file
3. **Pattern matching works** - Glob patterns correctly match subagent names
4. **Environment variables available** - Commands receive subagent context via env vars
5. **Multiple commands execute** - Both wildcard and specific commands run when applicable
6. **Backward compatibility maintained** - Existing `SubagentStop` notification behavior unchanged

## Risks & Mitigations

| Risk | Impact | Likelihood | Mitigation |
|------|--------|------------|------------|
| Transcript format changes | High | Low | Robust parsing with fallback to "unknown" subagent name |
| Glob pattern performance | Medium | Low | Pattern compilation cached; matches are O(n) per hook |
| Environment variable conflicts | Low | Low | Use `CONCLAUDE_` prefix to avoid collisions |
| Command execution failures | Medium | Medium | Graceful error handling; don't block SubagentStop completion |

## Migration Path

**For users without subagentStop config:**
- No changes required
- Existing `SubagentStop` notification behavior continues

**For users wanting subagent commands:**
- Add `subagentStop` section to config
- Define commands per subagent pattern
- Test with `conclaude SubagentStop < test_payload.json`

**Breaking changes:**
- None - this is a purely additive feature

## Alternatives Considered

### Alternative 1: Reuse `stop` hook with subagent context
**Rejected:** Would blur the distinction between session stop and subagent stop events. Users need separate configuration for different lifecycle events.

### Alternative 2: Simple array of commands without pattern matching
**Rejected:** Too inflexible. Users need to target specific subagents or groups of subagents differently.

### Alternative 3: Pass subagent name as command line argument
**Rejected:** Environment variables are more idiomatic for Unix/shell commands and easier to access from various scripting languages.

## Related Work

- **enable-docker-stop-hooks** - Future enhancement could extend Docker support to subagent hooks
- **add-command-timeout** - Timeout logic will apply to subagent commands once implemented

## Implementation Notes

**Key files to modify:**
- `src/config.rs` - Add `SubagentStopConfig` struct and `subagentStop` field to `ConclaudeConfig`
- `src/hooks.rs` - Modify `handle_subagent_stop()` to parse transcript, match patterns, execute commands
- `src/types.rs` - No changes needed (SubagentStopPayload sufficient)
- Schema will auto-generate from new config structs

**Testing approach:**
- Unit tests for transcript parsing logic
- Unit tests for glob pattern matching
- Integration tests with mock transcript files
- End-to-end tests with actual subagent stop events
