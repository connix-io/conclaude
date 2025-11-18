# Design: PermissionRequest Hook Support

## Architecture Overview

The PermissionRequest hook integrates with conclaude's existing hook system to provide programmatic tool permission decisions. The design follows the established patterns from PreToolUse and Stop hooks while introducing pattern-based rule matching for tool names.

## System Flow

```
Claude Agent
    ↓
[Tool Permission Request]
    ↓
PermissionRequest Payload
    ↓
Conclaude Hook Handler
    ├─ Deserialize payload
    ├─ Extract tool_name
    ├─ Match against allow/deny rules
    ├─ Determine decision
    └─ Return allow/deny response
    ↓
Claude Agent continues
(tool allowed/denied based on decision)
```

## Type System Design

### PermissionRequestPayload

```rust
pub struct PermissionRequestPayload {
    #[serde(flatten)]
    pub base: BasePayload,
    /// Name of the tool being requested (e.g., "Bash", "Edit", "Read")
    pub tool_name: String,
    /// Input parameters that will be passed to the tool
    pub tool_input: HashMap<String, serde_json::Value>,
}
```

**Rationale:**
- Extends `BasePayload` for consistency with other hooks (session_id, cwd, etc.)
- `tool_name` enables pattern-based matching
- `tool_input` allows inspection of what the tool will do
- JSON structure matches existing PostToolUse payload design

### HookPayload Integration

Add new variant:
```rust
pub enum HookPayload {
    // ... existing variants ...
    #[serde(rename = "PermissionRequest")]
    PermissionRequest(PermissionRequestPayload),
}
```

Extend `session_id()`, `transcript_path()`, `hook_event_name()` methods to handle new variant.

### Response Format

Use existing `HookResult` structure:
```rust
pub struct HookResult {
    pub message: Option<String>,    // Optional explanation
    pub blocked: Option<bool>,      // false = allow, true = deny
}
```

**Interpretation:**
- `blocked: false` → Tool permission approved
- `blocked: true` → Tool permission denied
- `message` → Optional explanation (passed to Claude or logged)

## Configuration Design

### PermissionRequestConfig Structure

```rust
pub struct PermissionRequestConfig {
    /// Default action if no rules match: "allow" or "deny"
    pub default: String,
    /// Tools to explicitly allow (supports glob patterns)
    pub allow: Option<Vec<String>>,
    /// Tools to explicitly deny (supports glob patterns)
    pub deny: Option<Vec<String>>,
    /// External hook commands for custom logic
    pub hooks: Option<Vec<ToolConfig>>,
}
```

**Configuration Example:**
```yaml
permissionRequest:
  default: deny
  allow:
    - "Read"
    - "Glob"
    - "Edit"
    - "Bash"
    - "Task"
  deny:
    - "BashOutput"
    - "KillShell"
```

### Rule Evaluation Algorithm

```
1. Load configuration
2. On PermissionRequest event:
   a. Extract tool_name from payload
   b. Compile deny rules to glob patterns (once, cached)
   c. Compile allow rules to glob patterns (once, cached)
   d. Check: does tool_name match any deny pattern?
      → YES: return blocked=true (deny)
      → NO: go to step e
   e. Check: does tool_name match any allow pattern?
      → YES: return blocked=false (allow)
      → NO: return decision based on `default` setting
```

**Design decisions:**
- **Deny precedence:** Security principle - if unsure, block
- **Pattern compilation:** Cache compiled patterns to avoid recompilation per request
- **Short-circuit evaluation:** Check deny first, stop if matched
- **Default fallback:** Configurable safety net when rules don't match

## Pattern Matching Implementation

Use `glob` crate's `Pattern` for matching:

```rust
use glob::Pattern;

fn matches_pattern(tool_name: &str, pattern: &str) -> Result<bool, PatternError> {
    Pattern::new(pattern)?.matches(tool_name)
}
```

**Supported patterns:**
- `"Bash"` - Exact match
- `"*"` - Wildcard (all tools)
- `"Edit*"` - Prefix match (Edit, EditFile, etc.)
- `"*Read"` - Suffix match (Read, FileRead, etc.)
- `"Tool[A-Z]*"` - Character class patterns
- `"[[:digit:]]*"` - POSIX character classes

**Pattern caching strategy:**
```rust
struct PermissionRulesCache {
    allow_patterns: Vec<Pattern<'static>>,
    deny_patterns: Vec<Pattern<'static>>,
}

// Compile patterns once at config load time, reuse for all requests
```

## Hook Handler Implementation

### Function Signature

```rust
pub fn handle_permission_request(
    config: &PermissionRequestConfig,
    payload: &PermissionRequestPayload,
    env_vars: &HashMap<String, String>,
) -> HookResult
```

### Algorithm

```
1. Extract tool_name and tool_input from payload
2. Build environment variables:
   - Set CONCLAUDE_TOOL_NAME = tool_name
   - Set CONCLAUDE_PERMISSION_MODE = permission_mode
   - Pass through existing env vars
3. If external hooks configured:
   a. Execute hook command
   b. Parse hook response for decision
   c. Return decision (can use hook custom message)
4. If no external hooks:
   a. Check deny patterns → return deny
   b. Check allow patterns → return allow
   c. Return default decision
5. Wrap decision in HookResult with optional message
```

## Integration Points

### Config Loading
- Parse `permissionRequest` section from YAML
- Validate rules are non-empty strings
- Compile glob patterns at config load time
- Cache compiled patterns for entire session

### Hook Dispatch
- Main hook handler receives PermissionRequest event
- Matches event type: `hook_event_name == "PermissionRequest"`
- Deserializes to `PermissionRequestPayload`
- Calls `handle_permission_request()`
- Returns decision to Claude Agent

### Error Handling
- Invalid glob pattern → Log warning, skip rule
- Malformed tool_input JSON → Log debug info, continue with matching
- Hook execution failure → Log error, use fallback decision (default setting)
- Missing config section → No-op (don't enforce permissions)

## Environment Variables

Pass to all hook commands:
```
CONCLAUDE_HOOK_EVENT=PermissionRequest
CONCLAUDE_SESSION_ID=<session_id>
CONCLAUDE_TRANSCRIPT_PATH=<path>
CONCLAUDE_CWD=<cwd>
CONCLAUDE_TOOL_NAME=<tool_name>
CONCLAUDE_PERMISSION_MODE=<permission_mode>
```

External scripts access these for custom decision logic:
```bash
#!/bin/bash
if [ "$CONCLAUDE_TOOL_NAME" = "Bash" ]; then
    # Custom Bash permission logic
    exit 0  # approve
else
    exit 1  # deny
fi
```

## Testing Strategy

### Unit Tests

1. **Pattern Matching**
   - Exact match: `matches_pattern("Bash", "Bash")` → true
   - Exact non-match: `matches_pattern("Read", "Bash")` → false
   - Wildcard: `matches_pattern("Bash", "*")` → true
   - Prefix: `matches_pattern("EditFile", "Edit*")` → true
   - Suffix: `matches_pattern("FileRead", "*Read")` → true
   - Character class: `matches_pattern("Tool123", "Tool[0-9]*")` → true

2. **Decision Logic**
   - Deny precedence: deny rule blocks allow rule
   - Default fallback: unknown tool uses default setting
   - All allow: allow-only config approves all matching
   - All deny: deny-only config blocks all matching
   - Empty rules: empty config uses only default

3. **Configuration**
   - Valid config loads without errors
   - Invalid patterns logged (continue with others)
   - Missing section handled gracefully

### Integration Tests

1. **Payload Deserialization**
   - Valid PermissionRequest payload deserializes correctly
   - tool_name extracted properly
   - tool_input preserved as JSON

2. **Hook Response**
   - Valid response returns HookResult
   - blocked=true represents denial
   - blocked=false represents approval
   - Message included in response

3. **Environment Variables**
   - External scripts receive CONCLAUDE_TOOL_NAME
   - External scripts receive CONCLAUDE_PERMISSION_MODE
   - Other standard variables also passed

### End-to-End Tests

1. Tool request with allow rule → approval decision
2. Tool request with deny rule → denial decision
3. Tool request with wildcard allow → approval for any tool
4. Tool request not matching any rule → uses default decision
5. External hook script influences decision

## Performance Considerations

**Pattern Compilation:**
- Patterns compiled once at config load time
- Cached for entire session duration
- Per-request matching is O(n) where n = number of rules
- Typical configs have <20 rules → negligible overhead

**Memory:**
- One PermissionRequestPayload per request (small JSON objects)
- Cached patterns consume minimal memory
- No unbounded allocations

**Latency:**
- Pattern matching: microseconds per request
- Hook execution: depends on external script (can timeout)
- Set reasonable timeout (default 5s from existing config)

## Security Considerations

**Principle of Least Privilege:**
- Recommend `default: deny` to start
- Explicitly allow only necessary tools
- Regular audit of allow/deny lists

**Input Validation:**
- tool_name is string from trusted source (Claude Agent)
- tool_input is JSON from trusted source
- No execution/evaluation of user input

**Decision Integrity:**
- Decisions cannot be overridden by tool parameters
- tool_input is for inspection only, not decision logic
- Deny precedence prevents bypass via complexity

**Logging:**
- All permission decisions logged (allow + deny)
- Tool name and decision recorded for audit trail
- Optional custom message for context

## Future Enhancements

1. **Role-based access control** - User roles determine allowed tools
2. **Tool parameter validation** - Inspect tool_input and make decisions based on arguments
3. **Dynamic rule loading** - Load rules from external source
4. **Audit logging** - Detailed permission decision history
5. **Integration with external auth** - LDAP, OAuth scopes for permissions
