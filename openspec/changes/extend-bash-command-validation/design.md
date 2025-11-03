# Design: Bash Command Validation Extension

## Overview

This design extends the existing `toolUsageValidation` mechanism to support pattern matching on Bash command strings, enabling users to block or allow specific commands based on configurable glob patterns with two matching modes: full command and prefix matching.

## Architecture

### Configuration Schema Extension

The existing `ToolUsageRule` struct in `src/config.rs` will be extended with two optional fields:

```rust
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ToolUsageRule {
    pub tool: String,
    pub pattern: String,        // Existing: for file path matching
    pub action: String,          // Existing: "block" or "allow"
    pub message: Option<String>, // Existing: custom error message

    #[serde(rename = "commandPattern")]
    pub command_pattern: Option<String>,  // NEW: for command string matching

    #[serde(rename = "matchMode")]
    pub match_mode: Option<String>,       // NEW: "full" or "prefix", defaults to "full"
}
```

**Design Rationale:**
- **Optional fields**: Maintains backward compatibility; existing rules continue to work without modification
- **Separate field names**: `commandPattern` is distinct from `pattern` to clarify intent (command vs. file path)
- **camelCase in YAML**: Uses `commandPattern` and `matchMode` to match existing conventions in the codebase
- **Default mode**: When `commandPattern` is present but `matchMode` is omitted, default to "full" for maximum safety

### Validation Logic Flow

The hook execution flow in `handle_pre_tool_use()` will be extended:

```
PreToolUse hook triggered
    ↓
Extract payload (tool_name, tool_input)
    ↓
Is tool_name == "Bash"?
    ├─ YES → Extract command from tool_input["command"]
    │        ↓
    │        Check if rule has commandPattern
    │        ├─ YES → Match command against commandPattern using matchMode
    │        │        ├─ FULL mode → glob pattern must match entire command
    │        │        └─ PREFIX mode → glob pattern must match command start
    │        └─ NO → Skip command validation
    │
    └─ NO → Use existing file path validation logic
         ↓
         Extract file_path from tool_input
         Match against pattern field (existing logic)
    ↓
Return HookResult (blocked or success)
```

### Matching Modes

#### Full Mode (`matchMode: "full"`)
The glob pattern must match the **entire command string**.

**Examples:**
```yaml
commandPattern: "rm -rf /*"
matchMode: "full"
```
- ✅ Matches: `rm -rf /`
- ✅ Matches: `rm -rf /tmp`
- ❌ Does NOT match: `rm -rf / && echo done` (extra content)
- ❌ Does NOT match: `sudo rm -rf /` (prefix added)

**Use case**: Blocking exact dangerous commands while allowing variations

#### Prefix Mode (`matchMode: "prefix"`)
The glob pattern must match the **beginning of the command string**.

**Examples:**
```yaml
commandPattern: "curl *"
matchMode: "prefix"
```
- ✅ Matches: `curl https://example.com`
- ✅ Matches: `curl -X POST https://api.example.com/data`
- ❌ Does NOT match: `wget https://example.com`
- ❌ Does NOT match: `echo test && curl https://example.com` (curl not at start)

**Use case**: Blocking entire command families regardless of arguments

### Implementation Components

#### 1. Command Extraction Helper

Add to `src/hooks.rs`:

```rust
/// Extracts the Bash command string from tool input payload
/// Returns None if the command is missing, empty, or contains only whitespace
pub fn extract_bash_command<S: std::hash::BuildHasher>(
    tool_input: &std::collections::HashMap<String, Value, S>,
) -> Option<String> {
    tool_input
        .get("command")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(std::string::ToString::to_string)
}
```

**Rationale**: Mirrors existing `extract_file_path()` pattern for consistency

#### 2. Pattern Matching Logic

Extend `check_tool_usage_rules()` in `src/hooks.rs`:

```rust
async fn check_tool_usage_rules(payload: &PreToolUsePayload) -> Result<Option<HookResult>> {
    let (config, _config_path) = get_config().await?;

    for rule in &config.rules.tool_usage_validation {
        if rule.tool == payload.tool_name || rule.tool == "*" {
            // NEW: Bash command pattern matching
            if payload.tool_name == "Bash" && rule.command_pattern.is_some() {
                if let Some(command) = extract_bash_command(&payload.tool_input) {
                    let pattern = rule.command_pattern.as_ref().unwrap();
                    let mode = rule.match_mode.as_deref().unwrap_or("full");

                    let matches = match mode {
                        "full" => Pattern::new(pattern)?.matches(&command),
                        "prefix" => {
                            // For prefix mode, check if pattern matches any prefix of the command
                            let glob = Pattern::new(pattern)?;

                            // Test progressively longer word-delimited prefixes
                            let words: Vec<&str> = command.split_whitespace().collect();
                            (1..=words.len()).any(|i| {
                                let prefix = words[..i].join(" ");
                                glob.matches(&prefix)
                            })
                        }
                        _ => false, // Invalid mode, skip rule
                    };

                    if rule.action == "block" && matches {
                        let message = rule.message.clone().unwrap_or_else(|| {
                            format!("Bash command blocked by validation rule: {}", pattern)
                        });
                        return Ok(Some(HookResult::blocked(message)));
                    } else if rule.action == "allow" && matches {
                        let message = rule.message.clone().unwrap_or_else(|| {
                            format!("Bash command allowed by validation rule: {}", pattern)
                        });
                        return Ok(Some(HookResult::allowed(message)));
                    } else if rule.action == "allow" && !matches {
                        let message = rule.message.clone().unwrap_or_else(|| {
                            format!("Bash command blocked: does not match allow rule pattern: {}", pattern)
                        });
                        return Ok(Some(HookResult::blocked(message)));
                    }
                }
                continue; // Skip file path validation for Bash when commandPattern exists
            }

            // EXISTING: File path validation logic (unchanged)
            if let Some(file_path) = extract_file_path(&payload.tool_input) {
                let matches = Pattern::new(&rule.pattern)?.matches(&file_path);

                if (rule.action == "block" && matches) || (rule.action == "allow" && !matches) {
                    let message = rule.message.clone().unwrap_or_else(|| {
                        format!("Tool usage blocked by validation rule: {}", rule.pattern)
                    });
                    return Ok(Some(HookResult::blocked(message)));
                }
            }
        }
    }

    Ok(None)
}
```

### Glob Pattern Behavior

Using the `glob` crate's `Pattern::matches()`:
- `*` matches any sequence of characters except `/`
- `**` matches any sequence of characters including `/`
- `?` matches any single character
- `[abc]` matches any character in the set
- `[!abc]` matches any character NOT in the set

**Command matching examples:**
```yaml
# Block all rm commands recursively
commandPattern: "rm -rf*"
matchMode: "prefix"

# Block specific git operations
commandPattern: "git push --force*"
matchMode: "prefix"

# Block curl to any https endpoint
commandPattern: "curl *https://*"
matchMode: "prefix"

# Block exact docker command
commandPattern: "docker run --privileged*"
matchMode: "full"
```

## Trade-offs and Decisions

### Decision: Extend existing toolUsageValidation vs. create new section

**Chosen**: Extend existing `toolUsageValidation`

**Rationale:**
- Reuses existing infrastructure (schema validation, hook integration)
- Users already understand `toolUsageValidation` concept
- Avoids duplication of action/message handling logic
- Simpler configuration with all tool rules in one place

**Rejected alternative**: Create separate `bashCommandValidation` section
- Would require duplicate validation logic
- Increases configuration complexity
- Harder to manage rules across different sections

### Decision: Support both full and prefix matching

**Chosen**: Implement both modes with configuration option

**Rationale:**
- Full mode needed for exact blocking (e.g., `rm -rf /` but not `rm -rf /tmp`)
- Prefix mode needed for command families (e.g., any `curl` regardless of args)
- User feedback indicated need for both use cases
- Minimal implementation complexity (single conditional)

**Rejected alternative**: Only prefix matching
- Insufficient for exact dangerous command blocking
- Less flexible for users

### Decision: No pre-execution warning messages

**Chosen**: Only block with error message (no separate warning mode)

**Rationale:**
- User explicitly requested "block only" functionality
- Simpler implementation matches existing pattern
- Warning mode adds complexity (separate action type, dual message fields)
- Can be added in future proposal if needed

### Decision: Default matchMode to "full"

**Chosen**: When `matchMode` is omitted, default to "full"

**Rationale:**
- Safer default (more restrictive, harder to accidentally block too much)
- Explicit opt-in required for broader prefix matching
- Follows principle of least surprise

## Validation Rules

### Configuration Validation

The schema will enforce:
1. `commandPattern` is a valid glob pattern string
2. `matchMode` is either "full", "prefix", or omitted (defaults to "full")
3. If `commandPattern` is present, `tool` must be "Bash" or "*"
4. Cannot have both `pattern` and `commandPattern` in the same rule (mutually exclusive for clarity)

### Runtime Validation

Edge cases handled:
- **Empty command**: If `tool_input["command"]` is empty, whitespace-only, or missing, skip the rule
- **Invalid glob pattern**: Return error via `Pattern::new()` failure
- **Invalid matchMode**: Treat as non-matching (skip rule silently, log warning)
- **No rules match**: Allow command execution (fail-open for safety)

## Testing Strategy

### Unit Tests
1. `extract_bash_command()` correctly parses command from tool_input
2. Pattern matching works for both full and prefix modes
3. Glob patterns with wildcards match correctly
4. Edge cases: empty commands, missing fields, invalid patterns

### Integration Tests
1. Full command matching blocks exact matches
2. Prefix matching blocks command families
3. Non-matching patterns allow execution
4. Custom messages appear in blocked results
5. Backward compatibility: existing file-path rules still work
6. Multiple rules evaluated in order
7. tool="*" applies to Bash commands

### Example Test Cases

```rust
#[test]
fn test_bash_command_full_match() {
    // commandPattern: "rm -rf /*", matchMode: "full"
    assert!(matches_command("rm -rf /", "rm -rf /*", "full"));
    assert!(!matches_command("sudo rm -rf /", "rm -rf /*", "full"));
}

#[test]
fn test_bash_command_prefix_match() {
    // commandPattern: "curl *", matchMode: "prefix"
    assert!(matches_command("curl https://example.com", "curl *", "prefix"));
    assert!(matches_command("curl -X POST https://api.com", "curl *", "prefix"));
    assert!(!matches_command("wget https://example.com", "curl *", "prefix"));
}

#[test]
fn test_backward_compatibility() {
    // Existing file-path rules should continue working
    assert!(matches_file_path("/path/to/file.env", "**/*.env", "block"));
}
```

## Security Considerations

### Command Injection Risks
- **Not applicable**: No command execution in validation logic; we only parse and match strings
- Commands are matched against patterns, never evaluated or executed
- glob crate handles pattern matching safely

### Bypass Scenarios
Users could potentially bypass rules with:
- **Shell aliases**: `alias dangerous='rm -rf /'` → run `dangerous`
  - Mitigation: Document limitation; users should block both command and common aliases
- **Subshells**: `bash -c "rm -rf /"`
  - Mitigation: Add rule to block `bash -c *` if needed
- **Environment variable expansion**: `CMD="rm -rf /"; $CMD`
  - Mitigation: Pattern matching happens on final command string after variable expansion

### Fail-Open vs. Fail-Closed
- **Chosen**: Fail-open (allow execution if validation has errors)
- **Rationale**: Avoids blocking legitimate usage due to misconfiguration
- **Trade-off**: Potential security gap if validation silently fails
- **Mitigation**: Log validation errors clearly; users can test with `validate` subcommand

## Documentation Requirements

Update the following:
1. README.md - Add Bash command validation examples
2. schema.json - Document new fields with descriptions and examples
3. Configuration guide - Explain matchMode differences with examples
4. Migration guide - Show how to add command rules to existing configs

## Future Enhancements (Out of Scope)

These could be separate proposals:
1. **Warning mode**: Show message but allow execution (requires new action type)
2. **Regex patterns**: Support regex in addition to glob patterns
3. **Argument extraction**: Match specific command arguments (e.g., block `--force` flag)
4. **Environment-aware rules**: Different rules for dev vs. production
5. **Audit logging**: Log all blocked/allowed commands to file
