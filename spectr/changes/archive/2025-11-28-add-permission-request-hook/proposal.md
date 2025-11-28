# Proposal: PermissionRequest Hook Support

**Change ID:** `add-permission-request-hook`
**Status:** Proposal
**Author:** Claude Code
**Date:** 2025-11-18

## Executive Summary

Add support for the `PermissionRequest` hook to enable users to automatically approve or deny tool permission requests with custom logic. This hook is part of the Claude Agent SDK and allows guardrail policies to make permission decisions programmatically rather than requiring interactive prompts.

**Key benefits:**
- Automatically approve or deny tool usage based on custom logic
- Enforce security policies without user interaction
- Pass tool context (name, input) to decision handlers
- Support both approval and denial with custom messaging
- Streamline workflows in automated environments

## Why

The `PermissionRequest` hook is a new feature in the Claude Agent SDK that addresses the need for programmatic permission management. Users need to enforce tool usage policies without breaking interactive flows or requiring manual approval for each request. This capability enables:

1. **Security enforcement** - Deny dangerous tools based on project context
2. **Workflow automation** - Pre-approve safe tools to reduce friction
3. **Policy compliance** - Implement org-specific tool restrictions
4. **Graceful denial** - Reject requests with explanations instead of silent blocking

## Motivation

### Current State

Conclaude currently does not support the `PermissionRequest` hook. Users cannot:

1. Automatically approve or deny tool requests based on custom logic
2. Implement security policies for tool usage
3. Access tool request context (tool name, input parameters)
4. Provide custom denial messages to Claude

### User Request

Users need programmatic control over tool permissions using the new `PermissionRequest` hook from the Claude Agent SDK, allowing enforcement of security policies and automatic approval/denial decisions.

## Dependencies

**Independent feature** - No dependencies on other changes.

This change aligns with the Claude Agent SDK's new capabilities but doesn't depend on other conclaude features.

## What Changes

1. **Type System** - Add `PermissionRequestPayload` struct to represent hook event
2. **Configuration** - Support `permissionRequest` section in YAML config with rule-based tool approval/denial
3. **Hook Handler** - Implement `handle_permission_request()` to process requests
4. **Decision Logic** - Support exact tool names, glob patterns, and custom rules
5. **Response Format** - Return approval decision with optional custom messaging
6. **Documentation** - Configuration examples and policy patterns

## Scope

### What's Included

**Payload Structure:**
- Tool name being requested
- Input parameters (as JSON for inspection)
- Session context (session_id, cwd, permission_mode)
- Hook event metadata (timestamp, transcript path)

**Configuration:**
- New `permissionRequest` section in YAML config
- Rule-based approval/denial logic
- Support for exact tool matching and glob patterns
- Optional custom messages for denials
- Decision order (allow-list, block-list, default action)

**Execution:**
- Hook handler processes PermissionRequest events
- Matches tool name against configured rules
- Returns `allow` or `deny` decision
- Includes optional reasoning message
- Passes context via environment variables

**Decision Logic:**
- Exact match: `"Bash"` matches only the Bash tool
- Wildcard: `"*"` matches all tools
- Prefix glob: `"Edit*"` matches Edit, EditFile, etc.
- Suffix glob: `"*Read"` matches Read, FileRead, etc.
- Pattern-based: `"Tool[A-Z]*"` matches Tool names starting with capital letters

**Response Format:**
- `decision`: `"allow"` or `"deny"`
- `message`: Optional explanation (for UI feedback)
- Uses existing `HookResult` structure for consistency

### What's NOT Included

- Changes to existing permission modes (acceptEdits, bypassPermissions, plan)
- Interactive permission prompts (hooks cannot trigger UI)
- Tool-specific validation beyond name matching
- Role-based access control (future enhancement)
- Tool parameter validation (future enhancement)
- Audit logging (covered by existing logging infrastructure)
- Nested decision rules (keep simple: match or not)

## Questions & Decisions

### Q: How should the hook represent tool permission decisions?
**Decision:** Return `{"decision": "allow"|"deny", "message": "optional explanation"}` using the same response pattern as other control flow hooks.

This aligns with existing `PreToolUse` and blocks patterns in conclaude.

### Q: What information should the PermissionRequest payload contain?
**Decision:** Include:
- `tool_name` - Name of the tool being requested (e.g., "Bash", "Edit", "Read")
- `tool_input` - Input parameters as HashMap (tool-specific arguments)
- All base fields: session_id, transcript_path, hook_event_name, cwd, permission_mode

This provides tool context for decision logic while maintaining backward compatibility with existing payload patterns.

### Q: How should rules be structured in the config?
**Decision:** Use simple rule format with allow-list/block-list precedence.

```yaml
permissionRequest:
  default: deny  # or "allow" - what to do if no rules match
  allow:         # tools allowed by whitelist
    - "Read"
    - "Edit"
    - "Glob"
    - "Bash*"    # glob patterns allowed
  deny:          # tools to block (checked after allow)
    - "Bash"     # deny takes precedence for specificity
    - "*://upload*"
```

This is intuitive and matches common security patterns (whitelist > blocklist).

### Q: When both allow and deny rules match, which takes precedence?
**Decision:** Deny takes precedence for security (fail-safe closure).

Order of evaluation:
1. Check if tool matches `deny` rules → return deny
2. Check if tool matches `allow` rules → return allow
3. Return `default` decision

This ensures that if a tool is explicitly blocked, it can't be allowed by wildcard rules.

### Q: Should environment variables be passed to hook handlers?
**Decision:** Yes, pass standard `CONCLAUDE_*` environment variables for consistency:
- `CONCLAUDE_SESSION_ID` - Session identifier
- `CONCLAUDE_CWD` - Current working directory
- `CONCLAUDE_HOOK_EVENT` - Always "PermissionRequest"
- `CONCLAUDE_TOOL_NAME` - Name of tool being requested
- `CONCLAUDE_PERMISSION_MODE` - Current permission mode

### Q: Should PermissionRequest support external scripts like other hooks?
**Decision:** Yes, follow the same pattern as PreToolUse and Stop hooks:
- Support both inline decisions and external script handlers
- Script receives payload as JSON via stdin
- Returns decision via JSON response

Configuration example:
```yaml
permissionRequest:
  hooks:
    - run: "./scripts/check-permission.sh"
      message: "Checking tool permissions"
```

### Q: How should the hook be mapped to HookPayload enum?
**Decision:** Add new `PermissionRequest` variant to `HookPayload` union type:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "hook_event_name")]
pub enum HookPayload {
    // ... existing variants ...
    #[serde(rename = "PermissionRequest")]
    PermissionRequest(PermissionRequestPayload),
}
```

## Success Criteria

1. **Type system complete** - PermissionRequestPayload properly defined and integrated
2. **Config validates** - Schema accepts `permissionRequest` section with rules
3. **Hook handler implemented** - Process PermissionRequest events correctly
4. **Tool matching works** - Exact, wildcard, and glob patterns all match correctly
5. **Decision logic correct** - Deny precedence, allow-list/deny-list evaluation
6. **Environment variables** - Commands receive CONCLAUDE_TOOL_NAME and other context
7. **Response format** - Returns proper allow/deny decision with optional message
8. **Backward compatibility** - Existing hooks unaffected; feature is additive only
9. **Tests passing** - Unit tests for pattern matching and decision logic
10. **Documentation** - Clear examples of common permission policies

## Risks & Mitigations

| Risk | Impact | Likelihood | Mitigation |
|------|--------|------------|------------|
| Too permissive default | High | Medium | Recommend whitelist model; default to deny in examples |
| Performance (pattern matching) | Low | Low | Pre-compile glob patterns; cache compiled patterns |
| Malformed tool_input JSON | Medium | Low | Validate JSON before passing; graceful fallback |
| Hook handler blocking workflow | Medium | Low | Set timeout on hook execution; fail-open if timeout |
| Confusion with PreToolUse | Low | Low | Clear documentation distinguishing PermissionRequest (higher-level policy) from PreToolUse (execution control) |

## Migration Path

**For users without permissionRequest config:**
- No changes required
- Tool permissions follow default Claude Code behavior
- Existing PreToolUse hooks continue working

**For users wanting permission policies:**
- Add `permissionRequest` section to `.claude/config.yaml`
- Define allow/deny rules for their tools
- Test with actual tool requests to verify behavior

**Breaking changes:**
- None - this is a purely additive feature

## Alternatives Considered

### Alternative 1: Extend PreToolUse hook for permission decisions
**Rejected:** PreToolUse already handles execution control (block/allow). PermissionRequest is a separate concern at the permission level. Keeping them separate provides clearer semantics and distinct configuration.

### Alternative 2: Simple yes/no rule per tool
**Rejected:** Too inflexible. Users need patterns (allow Read*, deny Bash) and default behaviors. Current approach with allow/deny lists is more powerful.

### Alternative 3: Complex rule engine (AND/OR logic)
**Rejected:** Adds complexity without clear value. Start with simple rules; enhance later if users need advanced logic.

### Alternative 4: Pass tool_input as string instead of parsed JSON
**Rejected:** Parsed JSON is more useful for inspection. If needed, raw strings can be added in future enhancements.

## Related Work

- **PreToolUse hook** - Different level of control (execution vs permission)
- **PreCompact hook** - Similar pattern of early event interception
- Role-based access control (future enhancement)
- Tool parameter validation (future enhancement)

## Implementation Notes

**Key files to modify:**
- `src/types.rs` - Add `PermissionRequestPayload` struct and update `HookPayload` enum
- `src/hooks.rs` - Implement `handle_permission_request()` function with pattern matching and decision logic
- `src/config.rs` - Add `PermissionRequestConfig` struct with allow/deny rules
- `src/schema.rs` - Update schema generation to include new config section
- `src/default-config.yaml` - Add example `permissionRequest` section with comments

**Testing approach:**
- Unit tests for glob pattern matching (exact, wildcard, prefix, suffix)
- Unit tests for allow/deny precedence logic
- Unit tests for default decision fallback
- Integration tests with mock PermissionRequestPayload
- Verify environment variables passed correctly
- Test with various tool names and patterns

**Documentation:**
- Add permission policy examples to default config
- Document allow-list vs block-list strategies
- Show common use cases (security policies, workflow optimization)
- Explain glob pattern syntax and matching behavior
- Provide troubleshooting guide for common configuration issues

**Backward compatibility:**
- No changes to existing hook types
- No changes to existing config sections
- New section is entirely optional
- Existing PreToolUse hooks unaffected
