# CLI Specification Delta

## REMOVED Requirements

### Requirement: Generate Schema Subcommand
**Reason**: Schema generation is a build-time tool, not a user-facing feature. Moving to external script reduces CLI complexity and clarifies that schema generation is part of the release automation.

**Migration**: Users who need to generate schemas locally can use `cargo run --bin generate-schema` instead. The schema file remains available at the same GitHub release URL for IDE autocomplete.

The `conclaude generate-schema` subcommand SHALL be removed from the CLI, including:
- The `GenerateSchema` command enum variant
- The `handle_generate_schema` function
- All associated CLI argument parsing (output path, validate flag)
- Documentation and help text for the subcommand

**Previous Scenarios (now invalid)**:
- Generating schema with default output path
- Generating schema with custom output path
- Validating generated schema with --validate flag
- Schema generation error handling

## ADDED Requirements

### Requirement: External Schema Generation Script
The system SHALL provide a standalone Rust script for generating the JSON Schema during development and releases.

#### Scenario: Running schema generation script locally
- **WHEN** a developer runs `cargo run --bin generate-schema`
- **THEN** the script SHALL generate `conclaude-schema.json` in the workspace root
- **AND** the schema SHALL be identical to what the removed subcommand produced
- **AND** success SHALL be indicated with a clear message

#### Scenario: Schema generation during CI/CD
- **WHEN** the release workflow executes the schema generation step
- **THEN** `conclaude-schema.json` SHALL be created in the workspace root
- **AND** the file SHALL be included in release artifacts by cargo-dist
- **AND** the schema SHALL be uploaded to GitHub releases at the stable URL

#### Scenario: Schema generation failure
- **WHEN** schema generation fails for any reason
- **THEN** the script SHALL exit with a non-zero status code
- **AND** a clear error message SHALL be displayed
- **AND** the CI/CD workflow SHALL fail, preventing incomplete releases

### Requirement: Schema Module Library Exposure
The schema generation functions SHALL remain available as library functions for use by the external script and tests.

#### Scenario: External script uses schema library
- **WHEN** the `generate-schema` binary imports `conclaude::schema` module
- **THEN** all schema generation functions SHALL be available
- **AND** the functions SHALL work identically to the removed subcommand implementation
- **AND** no code duplication SHALL exist between script and library

#### Scenario: Schema validation in tests
- **WHEN** tests validate the schema structure
- **THEN** the same schema module functions SHALL be used
- **AND** test coverage SHALL remain at or above current levels
- **AND** schema changes SHALL be caught by existing tests

## MODIFIED Requirements

### Requirement: Help Text and Command Listing
The CLI help output SHALL NOT include `generate-schema` in the list of available commands.

#### Scenario: Running conclaude --help
- **WHEN** a user runs `conclaude --help`
- **THEN** the command list SHALL NOT include `generate-schema`
- **AND** all other commands SHALL be displayed as before
- **AND** the help text SHALL be accurate and complete

#### Scenario: Running conclaude generate-schema (error case)
- **WHEN** a user attempts to run `conclaude generate-schema`
- **THEN** an error SHALL be displayed indicating the command is not recognized
- **AND** the error message SHOULD suggest using the init command for configuration
- **OR** the error message SHOULD note that schema is auto-generated in releases

### Requirement: Documentation Accuracy
All project documentation SHALL accurately reflect the removal of the generate-schema subcommand.

#### Scenario: README command reference
- **WHEN** reading the README.md command line interface section
- **THEN** `generate-schema` SHALL NOT be listed as an available command
- **AND** no references to `conclaude generate-schema` usage SHALL exist
- **AND** the schema URL for IDE support SHALL still be documented

#### Scenario: Configuration guide schema reference
- **WHEN** reading the configuration documentation
- **THEN** the YAML language server header SHALL still be documented
- **AND** the schema URL SHALL be documented as auto-generated
- **AND** no instructions to run `generate-schema` manually SHALL exist

#### Scenario: Development documentation
- **WHEN** contributors read development setup documentation
- **THEN** schema generation SHALL be documented as `cargo run --bin generate-schema`
- **AND** the purpose and timing of schema generation SHALL be clear
- **AND** CI/CD automation of schema generation SHALL be documented
