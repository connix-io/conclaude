# Configuration Defaults Specification

## Purpose

Define default configuration values and security protections applied when initializing conclaude, including automatic protection of config files from AI modification to preserve guardrail settings.

## Requirements

### Requirement: Default Configuration File Protection

The system SHALL include conclaude configuration files in the default `uneditableFiles` list to prevent AI from modifying guardrail settings.

#### Scenario: Default configuration includes config file protection

- **WHEN** generating a default configuration with `conclaude init`
- **THEN** the `rules.uneditableFiles` array SHALL include `.conclaude.yml`
- **AND** the `rules.uneditableFiles` array SHALL include `.conclaude.yaml`
- **AND** the configuration SHALL include a comment explaining this security protection

#### Scenario: Config file protection prevents edits

- **WHEN** Claude Code attempts to edit `.conclaude.yml` and it is listed in `uneditableFiles`
- **THEN** the PreToolUse hook SHALL block the edit operation
- **AND** SHALL return an error message indicating the file cannot be modified

#### Scenario: Users can override default protection

- **WHEN** a user removes `.conclaude.yml` from their `uneditableFiles` list
- **THEN** the system SHALL allow edits to the configuration file
- **AND** SHALL respect the user's explicit configuration choice

### Requirement: Default Configuration Security Documentation

The system SHALL document the security rationale for protecting configuration files in the default configuration template.

#### Scenario: Default config includes security explanation

- **WHEN** viewing the generated default configuration file
- **THEN** the `uneditableFiles` section SHALL include a comment explaining that config files should be protected
- **AND** SHALL provide rationale about preventing AI from disabling guardrails
- **AND** SHALL inform users they can remove this protection if desired

