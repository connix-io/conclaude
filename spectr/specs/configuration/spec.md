# Configuration Specification

## Purpose

Define automatic field list generation and error suggestion behavior for YAML configuration parsing, ensuring compile-time synchronization between struct definitions and user-facing field name suggestions.

## Requirements

### Requirement: Auto-Generated Field Lists for Error Suggestions

The system SHALL automatically derive configuration field lists from struct definitions to ensure error suggestions remain synchronized with the actual schema.

#### Scenario: Field list generation at compile time

- **WHEN** the project is built
- **THEN** field lists SHALL be automatically generated from the struct definitions (`StopConfig`, `RulesConfig`, `PreToolUseConfig`, `NotificationsConfig`, `StopCommand`)
- **AND** the generated field lists SHALL include all public fields with their `#[serde(rename)]` names where applicable
- **AND** the generation SHALL occur via procedural macro or build script without requiring manual updates

#### Scenario: Error suggestion with auto-generated fields

- **WHEN** a user provides an unknown field name in their configuration (e.g., `infiniteMessages` instead of `infiniteMessage`)
- **THEN** the system SHALL suggest similar field names using the auto-generated field list
- **AND** the suggestions SHALL be based on Levenshtein distance (distance ≤ 3)
- **AND** the suggestions SHALL return up to 3 closest matches

#### Scenario: Struct field added

- **WHEN** a developer adds a new field to a configuration struct (e.g., adding `timeout` to `StopConfig`)
- **THEN** the next build SHALL automatically include the new field in the generated field list
- **AND** error suggestions SHALL immediately reflect the new field without manual intervention

#### Scenario: Struct field renamed

- **WHEN** a developer renames a field in a configuration struct or changes its `#[serde(rename)]` attribute
- **THEN** the next build SHALL automatically update the generated field list with the new name
- **AND** error suggestions SHALL use the updated field name
- **AND** old field names SHALL no longer appear in suggestions

#### Scenario: Struct field removed

- **WHEN** a developer removes a field from a configuration struct
- **THEN** the next build SHALL automatically remove the field from the generated field list
- **AND** error suggestions SHALL no longer include the removed field

### Requirement: Field List Mapping by Section

The system SHALL maintain a mapping of configuration section names to their respective field lists to provide context-aware error suggestions.

#### Scenario: Section-specific field suggestions

- **WHEN** an unknown field error occurs in the `stop` section
- **THEN** the system SHALL only suggest fields valid for `StopConfig`
- **AND** SHALL NOT suggest fields from other sections (e.g., `rules`, `notifications`)

#### Scenario: Nested structure field suggestions

- **WHEN** an unknown field error occurs in a `commands` array entry
- **THEN** the system SHALL suggest fields valid for `StopCommand`
- **AND** SHALL correctly map the nested structure to its section name

#### Scenario: Unknown section

- **WHEN** an unknown field error occurs in a section not in the mapping
- **THEN** the system SHALL return an empty suggestion list
- **AND** SHALL NOT crash or panic

### Requirement: Zero Runtime Overhead for Field Lists

The system SHALL generate field lists at compile time with no runtime performance impact on configuration validation.

#### Scenario: Generated code is static

- **WHEN** field lists are generated
- **THEN** they SHALL be represented as `const` or `static` data structures
- **AND** SHALL NOT require heap allocations during lookup
- **AND** SHALL NOT require runtime reflection or parsing

#### Scenario: Suggestion performance maintained

- **WHEN** generating error suggestions with auto-generated field lists
- **THEN** the performance SHALL be equivalent to or better than the previous hardcoded approach
- **AND** Levenshtein distance calculations SHALL remain unchanged

### Requirement: Serde Attribute Awareness

The system SHALL respect `#[serde(rename)]` attributes when generating field lists to ensure suggestions match the actual YAML/JSON field names.

#### Scenario: Field with serde rename

- **WHEN** a struct field has `#[serde(rename = "camelCase")]` attribute (e.g., `infinite_message` → `infiniteMessage`)
- **THEN** the generated field list SHALL use the renamed value (`infiniteMessage`)
- **AND** error suggestions SHALL use the camelCase name
- **AND** the Rust snake_case field name SHALL NOT appear in suggestions

#### Scenario: Field without serde rename

- **WHEN** a struct field does not have a `#[serde(rename)]` attribute
- **THEN** the generated field list SHALL use the Rust field name as-is
- **AND** error suggestions SHALL match the struct field name

#### Scenario: Multiple rename formats

- **WHEN** different fields use different `#[serde(rename)]` conventions (e.g., `showStdout`, `prevent_additions`)
- **THEN** each field SHALL appear in the generated list with its respective rename value
- **AND** suggestions SHALL respect the mixed naming conventions

### Requirement: Configuration Schema Excludes Unspecified Fields

The system SHALL reject configuration fields that exist in the code but are not formally specified, preventing users from relying on unsupported or deprecated functionality.

#### Scenario: Rejected configuration with rounds field

- **WHEN** a user's configuration includes the `rounds` field in the `stop` section
- **THEN** configuration loading SHALL fail with a validation error
- **AND** the error message SHALL indicate that `rounds` is not a recognized field
- **AND** suggestions SHALL point to using `infinite` mode instead
