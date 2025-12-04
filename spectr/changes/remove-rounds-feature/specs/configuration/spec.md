# Configuration Spec Deltas

## ADDED Requirements

### Requirement: Configuration Schema Excludes Unspecified Fields

The system SHALL reject configuration fields that exist in the code but are not formally specified, preventing users from relying on unsupported or deprecated functionality.

#### Scenario: Rejected configuration with rounds field

- **WHEN** a user's configuration includes the `rounds` field in the `stop` section
- **THEN** configuration loading SHALL fail with a validation error
- **AND** the error message SHALL indicate that `rounds` is not a recognized field
- **AND** suggestions SHALL point to using `infinite` mode instead
