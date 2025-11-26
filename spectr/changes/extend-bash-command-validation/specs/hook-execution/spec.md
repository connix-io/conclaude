# Hook Execution Spec Deltas

## ADDED Requirements

### Requirement: Bash Command Pattern Validation

The system SHALL support pattern-based validation of Bash command strings in PreToolUse hooks, enabling users to block specific commands based on configurable glob patterns.

#### Scenario: Block exact dangerous command with full match mode

- **GIVEN** a configuration with a toolUsageValidation rule for tool "Bash"
- **AND** the rule has commandPattern set to "rm -rf /*" with matchMode "full"
- **AND** the rule action is "block"
- **WHEN** Claude Code attempts to execute Bash with command "rm -rf /"
- **THEN** the PreToolUse hook SHALL return a blocked result
- **AND** the custom message from the rule SHALL be displayed
- **AND** the command SHALL NOT be executed

#### Scenario: Allow command that does not match full pattern

- **GIVEN** a configuration with commandPattern "rm -rf /*" and matchMode "full"
- **WHEN** Claude Code attempts to execute Bash with command "rm -rf /tmp"
- **THEN** the PreToolUse hook SHALL return success
- **AND** the command SHALL be allowed to execute

#### Scenario: Block command family with prefix match mode

- **GIVEN** a configuration with commandPattern "curl *" and matchMode "prefix"
- **AND** the rule action is "block"
- **WHEN** Claude Code attempts to execute Bash with command "curl https://example.com"
- **THEN** the PreToolUse hook SHALL return a blocked result
- **AND** the command SHALL NOT be executed

#### Scenario: Prefix mode matches command start regardless of arguments

- **GIVEN** a configuration with commandPattern "git push --force*" and matchMode "prefix"
- **WHEN** Claude Code attempts to execute Bash with command "git push --force origin main"
- **THEN** the PreToolUse hook SHALL return a blocked result
- **AND** when Claude Code attempts "git push origin main" (without --force)
- **THEN** the hook SHALL return success

#### Scenario: Command pattern validation does not affect file-based tools

- **GIVEN** a configuration with a Bash commandPattern rule
- **WHEN** Claude Code uses a different tool like Write or Edit
- **THEN** the command pattern validation SHALL be skipped
- **AND** existing file path pattern validation SHALL be applied

### Requirement: Match Mode Configuration

The system SHALL support two distinct matching modes for Bash command patterns: full command matching and prefix matching.

#### Scenario: Full match mode requires exact pattern match

- **GIVEN** a commandPattern with matchMode "full"
- **WHEN** the Bash command contains additional content beyond the pattern
- **THEN** the pattern SHALL NOT match
- **AND** the command SHALL be allowed (if action is "block")

#### Scenario: Prefix match mode requires pattern at command start

- **GIVEN** a commandPattern with matchMode "prefix"
- **WHEN** the Bash command starts with content matching the pattern
- **THEN** the pattern SHALL match regardless of subsequent content
- **AND** the configured action SHALL be applied

#### Scenario: Default match mode when not specified

- **GIVEN** a commandPattern without a matchMode field
- **WHEN** validating a Bash command
- **THEN** the system SHALL default to "full" match mode
- **AND** SHALL require exact pattern matching

#### Scenario: Invalid match mode value

- **GIVEN** a commandPattern with matchMode set to an invalid value (not "full" or "prefix")
- **WHEN** the configuration is loaded
- **THEN** the system SHALL reject the configuration with a validation error
- **AND** SHALL provide a clear error message indicating valid matchMode values

### Requirement: Command Extraction from Bash Tool Payloads

The system SHALL extract the command string from Bash tool input payloads for pattern matching during PreToolUse hook execution.

#### Scenario: Extract command from valid Bash payload

- **GIVEN** a PreToolUse payload with tool_name "Bash"
- **AND** tool_input contains a "command" field with string value
- **WHEN** extracting the command for validation
- **THEN** the system SHALL return the command string
- **AND** SHALL use it for pattern matching against commandPattern rules

#### Scenario: Handle missing command field

- **GIVEN** a PreToolUse payload with tool_name "Bash"
- **AND** tool_input does not contain a "command" field
- **WHEN** extracting the command for validation
- **THEN** the system SHALL return None
- **AND** SHALL skip command pattern validation for that payload

#### Scenario: Handle empty command string

- **GIVEN** a PreToolUse payload with tool_name "Bash"
- **AND** tool_input contains a "command" field with empty string value
- **WHEN** extracting the command for validation
- **THEN** the system SHALL return the empty string
- **AND** SHALL evaluate it against patterns (typically resulting in no match)

### Requirement: Glob Pattern Support for Command Matching

The system SHALL support glob patterns in commandPattern fields using standard glob syntax for matching Bash command strings.

#### Scenario: Wildcard matches any sequence of characters

- **GIVEN** a commandPattern containing asterisk wildcard (*)
- **WHEN** matching against a Bash command
- **THEN** the asterisk SHALL match zero or more characters in the command string
- **AND** SHALL follow standard glob matching behavior

#### Scenario: Multiple wildcards in pattern

- **GIVEN** a commandPattern "docker run * --privileged *"
- **WHEN** matching against "docker run ubuntu --privileged -v /:/host"
- **THEN** the pattern SHALL match successfully
- **AND** both wildcards SHALL match their respective portions

#### Scenario: Pattern with no wildcards requires exact match

- **GIVEN** a commandPattern "reboot" with no wildcards
- **WHEN** matching against command "reboot"
- **THEN** the pattern SHALL match
- **AND** when matching against "sudo reboot"
- **THEN** the pattern SHALL NOT match (in full mode)

### Requirement: Backward Compatibility with File Path Validation

The system SHALL maintain full backward compatibility with existing file path-based toolUsageValidation rules when Bash command validation is added.

#### Scenario: Existing file path rules continue to work

- **GIVEN** an existing configuration with toolUsageValidation rules using pattern field
- **AND** the rules target tools like Write, Edit, Read
- **WHEN** the Bash command validation feature is added
- **THEN** all existing file path validation rules SHALL continue to function unchanged
- **AND** SHALL use the pattern field for file path matching

#### Scenario: Mixed rule configuration with both validation types

- **GIVEN** a configuration containing both file path rules and command pattern rules
- **WHEN** Claude Code executes various tools
- **THEN** file-based tools SHALL be validated against pattern field
- **AND** Bash tool SHALL be validated against commandPattern field
- **AND** both validation types SHALL work independently and correctly

#### Scenario: Rule with pattern but no commandPattern for Bash

- **GIVEN** a toolUsageValidation rule with tool "Bash" and pattern field but no commandPattern
- **WHEN** validating a Bash command execution
- **THEN** the system SHALL skip pattern validation for that rule
- **AND** SHALL not attempt to match the command against the file path pattern

### Requirement: Custom Error Messages for Blocked Commands

The system SHALL display user-configured custom error messages when blocking Bash commands via commandPattern rules.

#### Scenario: Custom message displayed on block

- **GIVEN** a commandPattern rule with action "block" and a custom message field
- **WHEN** a Bash command matches the pattern and is blocked
- **THEN** the custom message SHALL be returned in the HookResult
- **AND** SHALL be displayed to the user via stderr

#### Scenario: Default message when custom message not provided

- **GIVEN** a commandPattern rule with action "block" but no message field
- **WHEN** a Bash command matches the pattern and is blocked
- **THEN** the system SHALL generate a default error message
- **AND** the message SHALL include the pattern that caused the block
- **AND** the format SHALL be "Bash command blocked by validation rule: {pattern}"

#### Scenario: Message includes pattern for debugging

- **GIVEN** any blocked Bash command via commandPattern
- **WHEN** the block message is displayed
- **THEN** the message SHALL provide sufficient context for the user to understand why the block occurred
- **AND** SHALL reference the pattern or rule that triggered the block

### Requirement: Configuration Schema Validation for Command Patterns

The system SHALL validate commandPattern and matchMode fields in the configuration schema to ensure correct format and valid values.

#### Scenario: Valid commandPattern accepted

- **GIVEN** a toolUsageValidation rule with commandPattern as a non-empty string
- **AND** matchMode is either "full", "prefix", or omitted
- **WHEN** loading the configuration
- **THEN** the configuration SHALL be accepted as valid
- **AND** SHALL be ready for use in hook execution

#### Scenario: CommandPattern with invalid tool

- **GIVEN** a toolUsageValidation rule with commandPattern field
- **AND** tool field is set to a non-Bash tool (e.g., "Write")
- **WHEN** loading the configuration
- **THEN** the system SHALL accept the configuration and MAY log a warning (no strict enforcement)
- **AND** the commandPattern SHALL be ignored for non-Bash tools
- **AND** MAY log a warning about ineffective rule

#### Scenario: Invalid glob pattern in commandPattern

- **GIVEN** a commandPattern with invalid glob syntax
- **WHEN** attempting to compile the pattern during validation
- **THEN** the system SHALL return an error
- **AND** SHALL indicate which rule contains the invalid pattern
- **AND** SHALL prevent the configuration from being used

