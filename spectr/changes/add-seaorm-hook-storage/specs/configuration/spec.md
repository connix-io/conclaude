## ADDED Requirements

### Requirement: Database Configuration

The system SHALL support optional database configuration in the conclaude config file.

#### Scenario: Default database configuration
- **WHEN** no database configuration is specified
- **THEN** the database is enabled with default platform data directory

#### Scenario: Custom database path
- **WHEN** database.path is specified in configuration
- **THEN** the database is stored at the specified path

#### Scenario: Database disabled
- **WHEN** database.enabled is set to false
- **THEN** no database operations are performed
- **AND** hook execution logging is skipped

### Requirement: Database Environment Variable

The system SHALL support CONCLAUDE_DATA_DIR environment variable for data directory override.

#### Scenario: Environment variable takes precedence
- **WHEN** CONCLAUDE_DATA_DIR is set
- **AND** database.path is also configured
- **THEN** the environment variable takes precedence
