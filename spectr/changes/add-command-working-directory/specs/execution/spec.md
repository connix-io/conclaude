## ADDED Requirements
### Requirement: Command Working Directory Configuration
The system SHALL provide an optional `workingDir` field for individual stop commands and subagent stop commands to specify the directory from which the command should execute.

#### Scenario: Command with workingDir configured as absolute path
- **WHEN** a stop command includes `workingDir` field set to "/home/user/project/backend"
- **THEN** the command SHALL execute from the "/home/user/project/backend" directory
- **AND** environment variables like `PWD` SHALL reflect the working directory

#### Scenario: Command with workingDir configured as relative path
- **WHEN** a stop command includes `workingDir` field set to "packages/frontend"
- **AND** the config file is located at "/home/user/project/.conclaude.yaml"
- **THEN** the command SHALL execute from "/home/user/project/packages/frontend"
- **AND** the relative path SHALL be resolved relative to the config file directory

#### Scenario: Command without workingDir configured
- **WHEN** a stop command does not include a `workingDir` field
- **THEN** the command SHALL execute from the current working directory (cwd from payload)
- **AND** existing behavior SHALL be preserved for backward compatibility

#### Scenario: SubagentStopCommand with workingDir configured
- **WHEN** a subagent stop command includes `workingDir` field
- **THEN** the command SHALL execute from the specified directory
- **AND** environment variables passed to the command SHALL still include CONCLAUDE_* variables

### Requirement: Working Directory Validation
The system SHALL validate that the specified working directory exists before executing the command.

#### Scenario: Working directory exists
- **WHEN** `workingDir` points to an existing directory
- **THEN** the command SHALL execute from that directory
- **AND** no error SHALL be reported

#### Scenario: Working directory does not exist
- **WHEN** `workingDir` points to a non-existent directory
- **THEN** the command execution SHALL fail with a clear error message
- **AND** the error message SHALL indicate the directory path that was not found
- **AND** subsequent commands SHALL still be attempted (graceful failure)

#### Scenario: Working directory path is a file
- **WHEN** `workingDir` points to a file instead of a directory
- **THEN** the command execution SHALL fail with a clear error message
- **AND** the error message SHALL indicate the path is not a directory

### Requirement: Path Resolution Relative to Config File
The system SHALL resolve relative `workingDir` paths relative to the configuration file's directory, not the current working directory.

#### Scenario: Relative path from nested config
- **WHEN** config file is at "/repo/.conclaude.yaml"
- **AND** workingDir is set to "src/app"
- **THEN** the resolved path SHALL be "/repo/src/app"

#### Scenario: Relative path with parent directory reference
- **WHEN** config file is at "/repo/packages/backend/.conclaude.yaml"
- **AND** workingDir is set to "../frontend"
- **THEN** the resolved path SHALL be "/repo/packages/frontend"

#### Scenario: Absolute path ignores config location
- **WHEN** config file is at "/repo/.conclaude.yaml"
- **AND** workingDir is set to "/opt/tools"
- **THEN** the resolved path SHALL be "/opt/tools"
- **AND** the config file location SHALL NOT affect the path

### Requirement: Bash Interpolation Support
The system SHALL support full bash interpolation in `workingDir` values, including environment variables, command substitution, and tilde expansion. Interpolation SHALL occur at command execution time.

#### Scenario: Environment variable expansion
- **WHEN** workingDir is set to "$HOME/projects/backend"
- **AND** HOME environment variable is "/home/user"
- **THEN** the resolved path SHALL be "/home/user/projects/backend"

#### Scenario: CONCLAUDE environment variable expansion
- **WHEN** workingDir is set to "/tmp/$CONCLAUDE_SESSION_ID"
- **AND** CONCLAUDE_SESSION_ID is "abc123"
- **THEN** the resolved path SHALL be "/tmp/abc123"

#### Scenario: Command substitution for git repository root
- **WHEN** workingDir is set to "$(git rev-parse --show-toplevel)"
- **AND** the command is executed from within a git repository at "/repo"
- **THEN** the resolved path SHALL be "/repo"

#### Scenario: Command substitution for dynamic paths
- **WHEN** workingDir is set to "$(pwd)/build"
- **AND** the current directory is "/home/user/project"
- **THEN** the resolved path SHALL be "/home/user/project/build"

#### Scenario: Tilde expansion for home directory
- **WHEN** workingDir is set to "~/workspace/backend"
- **AND** the user's home directory is "/home/user"
- **THEN** the resolved path SHALL be "/home/user/workspace/backend"

#### Scenario: Combined interpolation with relative path
- **WHEN** workingDir is set to "$PROJECT_DIR/src"
- **AND** PROJECT_DIR environment variable is "myproject"
- **AND** config file is at "/repo/.conclaude.yaml"
- **THEN** bash expansion SHALL resolve "$PROJECT_DIR" to "myproject" first
- **AND** the relative path "myproject/src" SHALL be resolved relative to "/repo"
- **AND** the final resolved path SHALL be "/repo/myproject/src"

#### Scenario: Interpolation executed per command
- **WHEN** multiple stop commands use workingDir with "$(date +%s)"
- **THEN** each command SHALL execute bash interpolation independently
- **AND** each command MAY receive a different timestamp value
- **AND** the interpolation SHALL NOT be cached between commands

### Requirement: Bash Interpolation Error Handling
The system SHALL fail command execution with clear error messages when bash interpolation fails or produces invalid results.

#### Scenario: Command substitution fails
- **WHEN** workingDir is set to "$(git rev-parse --show-toplevel)"
- **AND** the command is executed outside a git repository
- **THEN** command execution SHALL fail with a clear error message
- **AND** the error message SHALL include "workingDir interpolation failed"
- **AND** the error message SHALL include the bash error output
- **AND** subsequent commands SHALL still be attempted (graceful failure)

#### Scenario: Interpolation results in empty string
- **WHEN** workingDir is set to "$NONEXISTENT_VAR"
- **AND** NONEXISTENT_VAR environment variable is not set or empty
- **THEN** command execution SHALL fail with a clear error message
- **AND** the error message SHALL indicate "workingDir interpolation resulted in empty path"

#### Scenario: Interpolated path does not exist
- **WHEN** workingDir is set to "$HOME/nonexistent"
- **AND** the interpolated path "/home/user/nonexistent" does not exist
- **THEN** command execution SHALL fail with a clear error message
- **AND** the error message SHALL indicate "workingDir does not exist: /home/user/nonexistent"

#### Scenario: Interpolated path is not a directory
- **WHEN** workingDir is set to "~/somefile.txt"
- **AND** the interpolated path "/home/user/somefile.txt" is a file
- **THEN** command execution SHALL fail with a clear error message
- **AND** the error message SHALL indicate "workingDir is not a directory: /home/user/somefile.txt"
