# Permission Rules Specification

## Purpose

Specifies the configuration and rule evaluation system for tool permission decisions, including glob pattern matching, allow/deny precedence, and external hook integration.

## Requirements

### Requirement: PermissionRequest Configuration Section
The system SHALL support a `permissionRequest` configuration section in `.claude/config.yaml`.

#### Scenario: Configuration with default and allow list
- **WHEN** a configuration file contains a `permissionRequest` section with `default: deny` and an `allow` list
- **THEN** the configuration SHALL load successfully
- **AND** the `allow` list SHALL contain tool names to explicitly permit

#### Scenario: Configuration with default and deny list
- **WHEN** a configuration file contains a `permissionRequest` section with `default: allow` and a `deny` list
- **THEN** the configuration SHALL load successfully
- **AND** the `deny` list SHALL contain tool names to explicitly block

#### Scenario: Configuration with both allow and deny lists
- **WHEN** a configuration file contains `allow` and `deny` rules
- **THEN** both lists SHALL be loaded
- **AND** deny rules SHALL take precedence over allow rules

#### Scenario: Configuration without permissionRequest section
- **WHEN** a configuration file does not contain a `permissionRequest` section
- **THEN** the configuration SHALL load successfully
- **AND** permission checking SHALL be disabled (permissive mode)

### Requirement: Default Decision Setting
The system SHALL require and validate a `default` field in permissionRequest configuration.

#### Scenario: Valid default value allow
- **WHEN** `default: allow` is specified
- **THEN** tools not matching any rule SHALL be allowed
- **AND** this serves as permissive fallback

#### Scenario: Valid default value deny
- **WHEN** `default: deny` is specified
- **THEN** tools not matching any rule SHALL be denied
- **AND** this serves as restrictive fallback

#### Scenario: Missing default field
- **WHEN** a `permissionRequest` section exists without a `default` field
- **THEN** configuration validation SHALL fail
- **AND** an error message SHALL indicate that `default` is required

#### Scenario: Invalid default value
- **WHEN** `default: maybe` or other invalid value is specified
- **THEN** configuration validation SHALL fail
- **AND** an error message SHALL list the valid options ("allow", "deny")

### Requirement: Glob Pattern Matching for Tools
The system SHALL support glob patterns in allow/deny rules for matching tool names.

#### Scenario: Exact match pattern
- **WHEN** a rule contains `"Bash"` (exact match)
- **THEN** it SHALL match only the tool named exactly "Bash"
- **AND** it SHALL NOT match "BashOutput" or other variations

#### Scenario: Wildcard pattern
- **WHEN** a rule contains `"*"` (matches all)
- **THEN** it SHALL match any tool name
- **AND** it SHALL apply universally to all tools

#### Scenario: Prefix match pattern
- **WHEN** a rule contains `"Edit*"` (prefix match)
- **THEN** it SHALL match "Edit", "EditFile", "EditPath", etc.
- **AND** it SHALL NOT match "ReadEdit" or non-matching prefixes

#### Scenario: Suffix match pattern
- **WHEN** a rule contains `"*Read"` (suffix match)
- **THEN** it SHALL match "Read", "FileRead", "BlobRead", etc.
- **AND** it SHALL NOT match "ReadFile" or non-matching suffixes

#### Scenario: Character class pattern
- **WHEN** a rule contains `"[BR]ash"` (character class)
- **THEN** it SHALL match "Bash" and "Rash"
- **AND** it SHALL NOT match "Kash" or "Trash"

### Requirement: Deny Precedence in Rule Evaluation
The system SHALL evaluate rules with deny taking precedence over allow.

#### Scenario: Deny rule blocks wildcard allow
- **WHEN** a tool matches both `allow: ["*"]` and `deny: ["Bash"]`
- **THEN** the tool SHALL be denied (deny takes precedence)
- **AND** the decision SHALL NOT be allow

#### Scenario: Specific deny overrides prefix allow
- **WHEN** a tool matches both `allow: ["Edit*"]` and `deny: ["Edit"]`
- **THEN** the tool "Edit" SHALL be denied
- **AND** tools like "EditFile" (if not in deny list) SHALL be allowed

#### Scenario: Allow only when not denied
- **WHEN** a tool matches `allow: ["Read"]` but does NOT match any deny rule
- **THEN** the tool SHALL be allowed
- **AND** the decision SHALL be definitive

#### Scenario: Evaluation order
- **WHEN** evaluating a tool against multiple rules
- **THEN** the system SHALL check deny rules first
- **AND** only check allow rules if tool is not denied
- **AND** use default only if no rules match

### Requirement: Default Configuration Example
The system SHALL include documented examples of permission rules in the default configuration.

#### Scenario: Example with whitelist approach
- **WHEN** a user examines the default configuration file
- **THEN** they SHALL find a commented example with `default: deny` and an `allow` list
- **AND** the example SHALL include common safe tools (Read, Glob, Edit, Task)

#### Scenario: Example with blacklist approach
- **WHEN** a user examines the default configuration file
- **THEN** they SHALL find a commented example with `default: allow` and a `deny` list
- **AND** the example SHALL include dangerous tools (BashOutput, KillShell)

#### Scenario: Example with external hook
- **WHEN** a user examines the default configuration file
- **THEN** they SHALL find a commented example showing hook-based decision making
- **AND** the example SHALL include proper hook command syntax

### Requirement: Pattern Validation During Config Load
The system SHALL validate glob patterns in configuration files.

#### Scenario: Valid pattern accepted
- **WHEN** a configuration contains patterns like `"Bash"`, `"Edit*"`, `"*Read"`, `"[BE]*"`
- **THEN** all patterns SHALL be accepted as valid
- **AND** configuration loading SHALL succeed

#### Scenario: Invalid pattern detected
- **WHEN** a configuration contains a malformed pattern like `"[invalid"`
- **THEN** the pattern SHALL be detected as invalid
- **AND** a clear error message SHALL be logged

#### Scenario: Partial pattern list with invalid entries
- **WHEN** a configuration contains a mix of valid and invalid patterns
- **THEN** valid patterns SHALL be used
- **AND** invalid patterns SHALL be skipped with a warning
- **AND** configuration loading SHALL NOT fail completely

### Requirement: Tool Permission Decision Execution
The system SHALL execute tool permission decisions based on configured rules.

#### Scenario: Tool allowed by exact match
- **WHEN** a tool "Read" is requested and config has `allow: ["Read"]` with `default: deny`
- **THEN** the tool SHALL be allowed
- **AND** execution SHALL proceed

#### Scenario: Tool denied by rule
- **WHEN** a tool "Bash" is requested and config has `deny: ["Bash"]`
- **THEN** the tool SHALL be denied
- **AND** execution SHALL be blocked with a clear message

#### Scenario: Tool uses default decision
- **WHEN** a tool "Unknown" is requested that matches no rules
- **THEN** the decision SHALL use the `default` setting
- **AND** if `default: deny`, the tool SHALL be blocked
- **AND** if `default: allow`, the tool SHALL be permitted

#### Scenario: Deny precedence in execution
- **WHEN** a tool matches both allow and deny rules
- **THEN** deny SHALL take effect
- **AND** the tool SHALL be blocked regardless of allow matches

### Requirement: External Hook Integration for Permission Decisions
The system SHALL support external hook commands for custom permission logic.

#### Scenario: Hook returns allow decision
- **WHEN** an external hook script executes and returns `{"blocked": false}`
- **THEN** the tool SHALL be allowed
- **AND** the hook decision SHALL be used

#### Scenario: Hook returns deny decision
- **WHEN** an external hook script executes and returns `{"blocked": true, "message": "Tool not approved"}`
- **THEN** the tool SHALL be denied
- **AND** the custom message SHALL be included in the response

#### Scenario: Hook timeout or error
- **WHEN** an external hook fails to execute or times out
- **THEN** the system SHALL fall back to rule-based decision
- **AND** an error SHALL be logged but not fail the entire permission check

#### Scenario: Hook receives environment context
- **WHEN** an external hook executes
- **THEN** the hook SHALL have access to environment variables like:
  - `CONCLAUDE_TOOL_NAME` - the name of the tool being requested
  - `CONCLAUDE_PERMISSION_MODE` - current permission mode
  - `CONCLAUDE_SESSION_ID` - session identifier

### Requirement: Environment Variable Context Passing
The system SHALL pass tool and session context via environment variables to hook handlers.

#### Scenario: Tool name environment variable
- **WHEN** a permission request is processed
- **THEN** the `CONCLAUDE_TOOL_NAME` environment variable SHALL be set to the tool name
- **AND** hook handlers SHALL have access to this variable

#### Scenario: Permission mode variable
- **WHEN** a permission request is processed
- **THEN** the `CONCLAUDE_PERMISSION_MODE` environment variable SHALL be set
- **AND** external hooks can use this to make context-aware decisions

#### Scenario: Standard session variables
- **WHEN** a permission request is processed
- **THEN** standard variables SHALL be set: `CONCLAUDE_SESSION_ID`, `CONCLAUDE_CWD`, `CONCLAUDE_HOOK_EVENT`
- **AND** all variables SHALL have correct values from the payload

