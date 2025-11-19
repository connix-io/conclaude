# Extend Bash Command Validation

## Why

Users need fine-grained control over which Bash commands Claude Code can execute during sessions. Currently, `toolUsageValidation` only supports pattern matching on file paths (for tools like Write, Edit, Read), but provides no mechanism to block or allow specific Bash commands based on their content. This gap means users cannot prevent dangerous operations like `rm -rf /`, `curl` to external endpoints, or other security-sensitive commands without completely blocking the Bash tool.

The absence of command-level validation creates security risks in automated environments, limits the ability to enforce organizational policies, and prevents safe delegation of Claude Code sessions in restricted contexts. Users have requested the ability to define guardrails for Bash commands similar to how they can protect specific files from modification.

## What Changes

- Extend existing `toolUsageValidation` configuration schema to support command pattern matching for Bash tools
- Add optional `commandPattern` field to `ToolUsageRule` for matching against Bash command strings
- Add optional `matchMode` field (`full` or `prefix`) to control how patterns are evaluated against commands
- Implement command extraction from Bash tool payloads in PreToolUse hooks
- Add pattern matching logic supporting both full command and prefix matching modes
- Maintain backward compatibility with existing file-path-based validation rules

**Configuration Example:**
```yaml
rules:
  toolUsageValidation:
    # Existing file-based validation continues to work
    - tool: "Write"
      pattern: "**/*.env"
      action: "block"
      message: "Cannot modify environment files"

    # NEW: Block dangerous recursive deletes
    - tool: "Bash"
      commandPattern: "rm -rf /*"
      matchMode: "full"
      action: "block"
      message: "Recursive delete of root directory blocked for safety"

    # NEW: Block any curl commands (prefix matching)
    - tool: "Bash"
      commandPattern: "curl *"
      matchMode: "prefix"
      action: "block"
      message: "External network access via curl is not permitted"
```

## Impact

- **Affected specs**: execution (new capability for hook execution)
- **Affected code**:
  - `src/config.rs` - Add `command_pattern` and `match_mode` fields to `ToolUsageRule` struct
  - `src/hooks.rs` - Add `extract_bash_command()` helper function
  - `src/hooks.rs` - Extend `check_tool_usage_rules()` to handle command pattern matching
  - `schema.json` - Update JSON schema to include new optional fields
  - `tests/integration_tests.rs` - Add integration tests for Bash command validation
- **User experience**: Users can enforce command-level policies in conclaude.yml without blocking all Bash usage
- **Security**: Enables fine-grained security controls for Bash command execution in Claude Code sessions
- **Backward compatibility**: Existing configurations continue to work unchanged; new fields are optional
