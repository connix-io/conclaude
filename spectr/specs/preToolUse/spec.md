# PreToolUse Specification

## Purpose

TODO: Add purpose description

## Requirements

### Requirement: Root-Level File Addition Prevention
The system SHALL prevent Claude from creating or modifying files at the repository root when `preventRootAdditions` is enabled.

#### Scenario: Prevent root additions enabled
- **WHEN** `preToolUse.preventRootAdditions` is set to `true`
- **THEN** Claude SHALL NOT be allowed to create or modify files with paths that have no directory separator (e.g., `README.md`, `.env`)
- **AND** any attempt to create/modify such files SHALL result in an error message explaining the restriction

#### Scenario: Prevent root additions disabled
- **WHEN** `preToolUse.preventRootAdditions` is set to `false`
- **THEN** Claude SHALL be allowed to create or modify files at the repository root
- **AND** all file operations in subdirectories remain subject to other restrictions

#### Scenario: Default behavior
- **WHEN** `preToolUse.preventRootAdditions` is not specified in configuration
- **THEN** the system SHALL default to `preventRootAdditions: true`
- **AND** root-level additions SHALL be prevented by default

### Requirement: File Protection via Glob Patterns
The system SHALL prevent Claude from editing specified files using glob patterns in the `uneditableFiles` configuration.

#### Scenario: Exact file match
- **WHEN** `preToolUse.uneditableFiles` contains `"package.json"`
- **THEN** Claude SHALL NOT be allowed to edit `package.json` at any directory level
- **AND** any attempt to modify this file SHALL result in an error message

#### Scenario: Extension-based patterns
- **WHEN** `preToolUse.uneditableFiles` contains `"*.md"`
- **THEN** Claude SHALL NOT be allowed to edit any `.md` files in the repository
- **AND** this includes nested paths like `docs/README.md`

#### Scenario: Directory and nested patterns
- **WHEN** `preToolUse.uneditableFiles` contains `"src/**/*.ts"`
- **THEN** Claude SHALL NOT be allowed to edit any `.ts` files under the `src/` directory
- **AND** all nested TypeScript files are protected

#### Scenario: Entire directory protection
- **WHEN** `preToolUse.uneditableFiles` contains `"node_modules/**"`
- **THEN** Claude SHALL NOT be allowed to modify any files in `node_modules/` or its subdirectories
- **AND** attempts to modify nested files SHALL be rejected

#### Scenario: Multi-pattern definitions
- **WHEN** `preToolUse.uneditableFiles` contains multiple patterns like `["package.json", "*.lock", "src/**/*.test.ts"]`
- **THEN** ALL files matching any pattern SHALL be protected
- **AND** all other files remain editable unless blocked by other restrictions

#### Scenario: Default behavior
- **WHEN** `preToolUse.uneditableFiles` is not specified in configuration
- **THEN** the system SHALL default to an empty array
- **AND** no files are protected by this field by default

#### Scenario: Empty configuration
- **WHEN** `preToolUse.uneditableFiles` is set to an empty array `[]`
- **THEN** no files are protected by this specific field
- **AND** file editing is only restricted by other validation rules

### Requirement: Configuration Validation
The system SHALL validate `preventRootAdditions` and `uneditableFiles` configuration values.

#### Scenario: Valid preventRootAdditions value
- **WHEN** `preToolUse.preventRootAdditions` contains a boolean value (`true` or `false`)
- **THEN** the configuration SHALL be accepted
- **AND** the setting SHALL be applied during pre-tool-use validation

#### Scenario: Invalid preventRootAdditions value
- **WHEN** `preToolUse.preventRootAdditions` contains a non-boolean value
- **THEN** the configuration loading SHALL fail with a validation error
- **AND** the error message SHALL clearly indicate the type mismatch

#### Scenario: Valid uneditableFiles array
- **WHEN** `preToolUse.uneditableFiles` contains an array of string glob patterns
- **THEN** the configuration SHALL be accepted
- **AND** each pattern SHALL be evaluated against file paths

#### Scenario: Invalid uneditableFiles value
- **WHEN** `preToolUse.uneditableFiles` is not an array (e.g., a string or object)
- **THEN** the configuration loading SHALL fail with a validation error
- **AND** the error message SHALL indicate that an array is expected

### Requirement: Combined File Protection Policies
The system SHALL enforce both `preventRootAdditions` and `uneditableFiles` restrictions together as a unified file protection policy.

#### Scenario: Root addition prevention with glob patterns
- **WHEN** both `preventRootAdditions: true` and `uneditableFiles: ["Cargo.toml"]` are configured
- **THEN** Claude SHALL NOT create files in the root directory
- **AND** Claude SHALL NOT edit `Cargo.toml` regardless of directory level
- **AND** both restrictions are enforced independently

#### Scenario: Overlapping protections
- **WHEN** `preventRootAdditions: true` and `uneditableFiles: ["*"]` are configured
- **THEN** all files are protected from modification
- **AND** the system evaluates both rules and applies the most restrictive result

#### Scenario: Nested files with root prevention
- **WHEN** `preventRootAdditions: true` is set
- **THEN** files in subdirectories like `src/app.ts` remain editable (unless blocked by other rules)
- **AND** only root-level files are blocked by this specific restriction

### Requirement: Configuration Default Values and Backward Compatibility
The system SHALL provide appropriate defaults and signal deprecation for the removed `rules` section.

#### Scenario: New configuration format
- **WHEN** a user provides a configuration with `preToolUse` containing `preventRootAdditions` and `uneditableFiles`
- **THEN** the configuration SHALL be accepted as valid
- **AND** the values SHALL be used as specified

#### Scenario: Detection of old configuration format
- **WHEN** configuration contains the old `rules` section
- **THEN** the system SHALL fail configuration loading with an error
- **AND** the error message SHALL clearly indicate that the `rules` section is no longer supported
- **AND** the error message SHALL provide specific migration instructions for moving fields to `preToolUse`

#### Scenario: Migration example in documentation
- **WHEN** user documentation is generated
- **THEN** migration examples SHALL clearly show before/after configurations
- **AND** the rationale for consolidation SHALL be documented

### Requirement: Tool Usage Validation Rules
The system SHALL enforce per-tool restrictions defined in `toolUsageValidation` to control which tools can operate on which files.

#### Scenario: Block tool on file pattern
- **WHEN** `preToolUse.toolUsageValidation` contains a rule blocking "bash" on "*.md" files
- **THEN** Claude SHALL NOT be allowed to execute bash commands on markdown files
- **AND** any attempt SHALL result in an error message referencing the tool usage rule

#### Scenario: Allow tool on specific pattern
- **WHEN** `preToolUse.toolUsageValidation` contains a rule allowing "Write" only on "src/**/*.ts"
- **THEN** Claude SHALL NOT be allowed to use Write tool on files outside the `src/` TypeScript directory
- **AND** the permission boundary SHALL be enforced

#### Scenario: Command pattern matching
- **WHEN** a tool usage rule includes a `commandPattern` (e.g., regex)
- **THEN** the rule SHALL match against the specific command/parameters passed to the tool
- **AND** match mode (exact, regex, glob) SHALL determine matching behavior

#### Scenario: Multiple validation rules
- **WHEN** multiple `toolUsageValidation` rules are configured
- **THEN** all applicable rules SHALL be evaluated
- **AND** the first matched rule SHALL determine the action (block/allow)

#### Scenario: Rule precedence with match modes
- **WHEN** multiple rules could apply to the same tool and file pattern
- **THEN** rules SHALL be evaluated in order
- **AND** the first matching rule SHALL take precedence

#### Scenario: Validation error messages
- **WHEN** a tool usage rule blocks an operation
- **THEN** the error message SHALL include the tool name, file pattern, and custom message if provided
- **AND** the user SHALL understand why the operation was blocked

