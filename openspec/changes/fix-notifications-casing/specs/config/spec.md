## MODIFIED Requirements
### Requirement: Notification Configuration Serialization
The NotificationsConfig struct SHALL serialize field names using camelCase convention to maintain consistency with the project's configuration standards.

#### Scenario: Configuration file serialization
- **WHEN** NotificationsConfig is serialized to JSON/YAML
- **THEN** field names SHALL be: showErrors, showSuccess, showSystemEvents

#### Scenario: Configuration file deserialization
- **WHEN** JSON/YAML config contains camelCase field names
- **THEN** system SHALL correctly parse showErrors, showSuccess, showSystemEvents fields

#### Scenario: Error message consistency
- **WHEN** configuration validation fails
- **THEN** error messages SHALL reference camelCase field names: showErrors, showSuccess, showSystemEvents

#### Scenario: Backward compatibility
- **WHEN** existing snake_case field names are encountered
- **THEN** system SHALL fail gracefully with clear error message indicating camelCase requirement