# Add Validate Subcommand

## Why
Users need a way to validate their conclaude configuration files without running hooks or triggering commands. Currently, configuration errors are only discovered when hooks execute, which can lead to unexpected failures during development. A dedicated validate subcommand provides immediate feedback on configuration issues, improves developer experience, and enables CI/CD validation workflows.

## What Changes
- Add new `validate` subcommand to the CLI that validates configuration without executing hooks
- Provide clear, actionable error messages for configuration issues
- Support optional path specification to validate non-default configuration locations
- Display configuration file location and validation status
- Return appropriate exit codes for scripting and CI/CD integration (0 for success, non-zero for failure)

## Impact
- **Affected specs**: cli (new capability)
- **Affected code**:
  - `src/main.rs` - Add new Commands::Validate variant and handler function
  - `src/config.rs` - Reuse existing `load_conclaude_config` function
  - `src/schema.rs` - Reuse existing `validate_config_against_schema` function
- **User experience**: Provides immediate feedback on configuration validity before running hooks
- **CI/CD integration**: Enables automated configuration validation in pipelines
