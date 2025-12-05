## ADDED Requirements

### Requirement: Command Working Directory

The system SHALL execute all configured commands with the current working directory set to the directory containing the configuration file, not the directory from which Claude Code was invoked.

#### Scenario: Stop command executes from config directory

- **WHEN** a stop command is configured in `.conclaude.yaml` located at `/home/user/project/.conclaude.yaml`
- **AND** Claude Code session runs from `/home/user/project/src/nested`
- **THEN** the stop command SHALL execute with cwd set to `/home/user/project/`
- **AND** relative paths in commands like `npm test` SHALL resolve from `/home/user/project/`

#### Scenario: Subagent stop command executes from config directory

- **WHEN** a subagent stop command is configured in `.conclaude.yaml` located at `/home/user/project/.conclaude.yaml`
- **AND** Claude Code session runs from `/home/user/project/deep/nested/dir`
- **THEN** the subagent stop command SHALL execute with cwd set to `/home/user/project/`
- **AND** the command SHALL have access to project files relative to the config location

#### Scenario: Config in current directory

- **WHEN** the configuration file is in the same directory as the Claude Code session
- **THEN** commands SHALL execute from that directory
- **AND** behavior SHALL be identical to previous implementation for this case

#### Scenario: Commands with absolute paths

- **WHEN** a command uses an absolute path (e.g., `/usr/bin/custom-linter`)
- **THEN** the command SHALL execute successfully regardless of working directory
- **AND** the working directory SHALL still be set to the config directory

### Requirement: Config Directory Environment Variable

The system SHALL expose the configuration file's parent directory as an environment variable to all executed commands.

#### Scenario: CONCLAUDE_CONFIG_DIR available to stop commands

- **WHEN** a stop command executes
- **THEN** the environment variable `CONCLAUDE_CONFIG_DIR` SHALL be set
- **AND** its value SHALL be the absolute path to the directory containing `.conclaude.yaml`

#### Scenario: CONCLAUDE_CONFIG_DIR available to subagent stop commands

- **WHEN** a subagent stop command executes
- **THEN** the environment variable `CONCLAUDE_CONFIG_DIR` SHALL be set
- **AND** its value SHALL be the absolute path to the directory containing `.conclaude.yaml`

#### Scenario: Script uses CONCLAUDE_CONFIG_DIR

- **WHEN** a command script references `$CONCLAUDE_CONFIG_DIR`
- **THEN** it SHALL resolve to the config file's parent directory
- **AND** the script MAY use it for explicit path construction (e.g., `$CONCLAUDE_CONFIG_DIR/scripts/lint.sh`)
