# validation-rules Specification

## Purpose

Define the structure and behavior of the `uneditableFiles` validation rule to support context-aware error messaging.

## ADDED Requirements

### Requirement: Uneditable File Rule Structure

The system SHALL support two formats for specifying uneditable file rules: a simple string pattern and a detailed object with pattern and optional message.

#### Scenario: Simple string pattern format

- **GIVEN** a YAML configuration with `uneditableFiles: ["*.lock"]`
- **WHEN** the configuration is deserialized
- **THEN** the rule SHALL be interpreted as a file pattern with no custom message
- **AND** files matching `*.lock` SHALL be blocked with a generic error message

#### Scenario: Detailed rule format with pattern and message

- **GIVEN** a YAML configuration with `uneditableFiles: [{pattern: "*.lock", message: "Lock files are auto-generated"}]`
- **WHEN** the configuration is deserialized
- **THEN** the rule SHALL include both pattern and custom message
- **AND** files matching `*.lock` SHALL be blocked with the custom error message

#### Scenario: Backward compatibility with string array

- **GIVEN** a legacy configuration using only string patterns: `uneditableFiles: ["*.lock", ".env"]`
- **WHEN** the configuration is deserialized
- **THEN** deserialization SHALL succeed
- **AND** files matching either pattern SHALL be blocked with generic error messages
- **AND** no changes to the configuration are required for existing users

#### Scenario: Mixed format in single configuration

- **GIVEN** a configuration mixing both formats: `uneditableFiles: ["*.lock", {pattern: ".env", message: "Secrets..."}]`
- **WHEN** the configuration is deserialized
- **THEN** both rules SHALL be processed correctly
- **AND** `*.lock` files use generic message
- **AND** `.env` files use the custom message

### Requirement: Custom Message Error Reporting

The system SHALL display custom messages when available and fall back to generic messages when not provided.

#### Scenario: Custom message displayed when provided

- **GIVEN** an uneditable file rule with a custom message: `{pattern: "*.lock", message: "Auto-generated. Run 'npm install' to update."}`
- **WHEN** Claude attempts to write a file matching `*.lock`
- **THEN** conclaude SHALL block the operation
- **AND** the error message SHALL be the custom message: "Auto-generated. Run 'npm install' to update."
- **AND** the error SHALL be returned to Claude Code

#### Scenario: Generic message used when custom message absent

- **GIVEN** an uneditable file rule without a custom message: `*.lock` or `{pattern: "*.lock"}`
- **WHEN** Claude attempts to write a file matching `*.lock`
- **THEN** conclaude SHALL block the operation
- **AND** the error message SHALL be the default format: `"Blocked {operation} operation: file matches uneditable pattern '{pattern}'. File: {file}"`
- **AND** the error SHALL be returned to Claude Code

#### Scenario: Custom message for context-specific guidance

- **GIVEN** a configuration with multiple rules with custom messages:
  ```yaml
  uneditableFiles:
    - pattern: "*.lock"
      message: "Lock files are generated. Update via npm/yarn."
    - pattern: ".env*"
      message: "Environment files contain secrets. Use .env.example instead."
    - pattern: ".conclaude.yaml"
      message: "Config changes require PR approval per security policy."
  ```
- **WHEN** Claude attempts to edit any of these files
- **THEN** each attempt SHALL be blocked with the corresponding custom message
- **AND** the context-specific guidance SHALL help Claude understand the protection policy

### Requirement: Configuration Validation

The system SHALL validate that uneditable file rules are correctly formatted and contain required fields.

#### Scenario: Valid rule with pattern only

- **GIVEN** a rule: `"*.lock"`
- **WHEN** the configuration is validated
- **THEN** validation SHALL pass
- **AND** the rule SHALL be usable

#### Scenario: Valid rule with pattern and message

- **GIVEN** a rule: `{pattern: "*.log", message: "Log files are ephemeral"}`
- **WHEN** the configuration is validated
- **THEN** validation SHALL pass
- **AND** both pattern and message SHALL be accessible

#### Scenario: Invalid rule missing pattern field

- **GIVEN** a rule: `{message: "Missing pattern"}`
- **WHEN** the configuration is validated
- **THEN** validation SHALL fail
- **AND** an error message SHALL indicate the missing `pattern` field
- **AND** configuration processing SHALL stop

#### Scenario: Invalid rule with non-string pattern

- **GIVEN** a rule: `{pattern: 123, message: "Number not string"}`
- **WHEN** the configuration is deserialized
- **THEN** deserialization SHALL fail
- **AND** an error message SHALL indicate type mismatch
- **AND** configuration processing SHALL stop

### Requirement: Pattern Matching Behavior

The system SHALL correctly match files against uneditable file patterns using glob pattern semantics.

#### Scenario: Exact filename match

- **GIVEN** an uneditable pattern: `.env`
- **WHEN** checking files: `.env`, `.env.local`, `src/.env`
- **THEN** only `.env` file at any directory level SHALL match

#### Scenario: Wildcard pattern matching

- **GIVEN** an uneditable pattern: `*.lock`
- **WHEN** checking files: `package-lock.json`, `yarn.lock`, `src/package.json`
- **THEN** both `package-lock.json` and `yarn.lock` SHALL match
- **AND** `package.json` SHALL not match

#### Scenario: Directory wildcard matching

- **GIVEN** an uneditable pattern: `.git/*`
- **WHEN** checking files: `.git/config`, `.git/objects/abc`, `src/.git/config`
- **THEN** files under `.git/` SHALL match
- **AND** files outside `.git/` with similar names SHALL not match

### Requirement: Error Handling

The system SHALL handle edge cases gracefully and provide clear error information.

#### Scenario: Empty message field

- **GIVEN** a rule: `{pattern: "*.lock", message: ""}`
- **WHEN** Claude attempts to write a matching file
- **THEN** conclaude SHALL use the empty string message
- **AND** the blocked error SHALL contain no additional context text

#### Scenario: Whitespace-only message

- **GIVEN** a rule: `{pattern: "*.lock", message: "   "}`
- **WHEN** Claude attempts to write a matching file
- **THEN** conclaude SHALL use the whitespace message as-is
- **AND** the blocked error SHALL contain the whitespace message

#### Scenario: Multi-line message in YAML

- **GIVEN** a YAML configuration:
  ```yaml
  uneditableFiles:
    - pattern: "*.lock"
      message: |
        Lock files are auto-generated.
        Run 'npm install' to update them.
  ```
- **WHEN** the configuration is deserialized and the file is blocked
- **THEN** the multi-line message SHALL be preserved and displayed
- **AND** newlines SHALL be included in the error message

#### Scenario: Special characters in message

- **GIVEN** a rule with special characters: `{pattern: "*.lock", message: "Error: Can't edit <auto-generated> files!"}`
- **WHEN** Claude attempts to write a matching file
- **THEN** the message SHALL be displayed as-is
- **AND** special characters SHALL not be escaped or modified

## Open Questions

1. **Message length limits**: Should there be a maximum message length? (Current: no limit)
2. **Message encoding**: Are there characters that should not be allowed in messages? (Current: none)
3. **Template variables**: Should messages support substitution like `{pattern}` or `{file_path}`? (Current: no - defer to v2)
