# Config Discovery Specification

## Purpose

TODO: Add purpose description

## Requirements

### Requirement: Upward Directory Search

The system SHALL search for configuration files by traversing upward from the current working directory (or specified start directory) through all parent directories until reaching the filesystem root or maximum search depth.

#### Scenario: Config found in parent directory
- **WHEN** the user runs conclaude from `/home/user/project/src`
- **AND** a config file exists at `/home/user/project/.conclaude.yaml`
- **THEN** the system SHALL find and load the configuration file

#### Scenario: Config found multiple levels up
- **WHEN** the user runs conclaude from `/home/user/project/nested/deep/dir`
- **AND** a config file exists at `/home/user/.conclaude.yaml`
- **THEN** the system SHALL find and load the configuration file

### Requirement: Multiple Config File Extensions

The system SHALL check for both `.conclaude.yaml` and `.conclaude.yml` extensions in each directory during the upward search.

#### Scenario: YAML extension found
- **WHEN** the system searches a directory containing `.conclaude.yaml`
- **THEN** the system SHALL include this file in the search paths

#### Scenario: YML extension found
- **WHEN** the system searches a directory containing `.conclaude.yml`
- **THEN** the system SHALL include this file in the search paths

### Requirement: Continue Past Package.json Files

The system SHALL continue searching parent directories even when encountering `package.json` files, without treating them as search boundaries or project root markers.

#### Scenario: Config above package.json
- **WHEN** the user runs conclaude from `/home/user/project/src`
- **AND** a `package.json` exists at `/home/user/project/package.json`
- **AND** a config file exists at `/home/user/.conclaude.yaml`
- **THEN** the system SHALL find the config at `/home/user/.conclaude.yaml`

#### Scenario: Nested package.json in monorepo
- **WHEN** the user runs conclaude from `/home/user/monorepo/packages/app/src`
- **AND** `package.json` files exist at multiple levels:
  - `/home/user/monorepo/package.json`
  - `/home/user/monorepo/packages/app/package.json`
- **AND** a config file exists at `/home/user/monorepo/.conclaude.yaml`
- **THEN** the system SHALL find the config at `/home/user/monorepo/.conclaude.yaml`

### Requirement: Maximum Search Depth Limit

The system SHALL stop searching after traversing 12 directory levels upward from the starting directory, even if the filesystem root has not been reached.

#### Scenario: Search within depth limit
- **WHEN** the user runs conclaude from a directory 10 levels deep
- **AND** a config file exists 10 levels up
- **THEN** the system SHALL find and load the configuration file

#### Scenario: Search exceeds depth limit
- **WHEN** the user runs conclaude from a directory 15 levels deep
- **AND** a config file exists only 15 levels up
- **THEN** the system SHALL NOT find the configuration file
- **AND** the search SHALL terminate after 12 levels

### Requirement: Filesystem Root Termination

The system SHALL stop searching when reaching the filesystem root directory, even if the maximum search depth has not been reached.

#### Scenario: Filesystem root reached
- **WHEN** the user runs conclaude from `/home/user/project`
- **AND** no config file exists in the search path
- **AND** the search reaches the filesystem root `/`
- **THEN** the system SHALL terminate the search
- **AND** no configuration file SHALL be loaded

