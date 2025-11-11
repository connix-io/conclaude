# subagent-hooks Specification Delta

This spec defines the requirements for executing commands when subagents terminate, including pattern matching and environment variable context.

## ADDED Requirements

### Requirement: Subagent Stop Command Configuration
The system SHALL provide a `subagentStop` configuration section that maps subagent name patterns to lists of commands to execute when matching subagents terminate.

#### Scenario: Wildcard pattern configuration
- **WHEN** config includes `subagentStop.commands["*"]` with a list of commands
- **THEN** those commands SHALL execute for every subagent that stops
- **AND** each command SHALL include run, message, showStdout, showStderr, maxOutputLines fields

#### Scenario: Exact match pattern configuration
- **WHEN** config includes `subagentStop.commands["coder"]` with a list of commands
- **THEN** those commands SHALL execute only when a subagent named "coder" stops
- **AND** commands SHALL NOT execute for subagents with different names

#### Scenario: Glob pattern configuration
- **WHEN** config includes `subagentStop.commands["test*"]` with commands
- **THEN** those commands SHALL execute for subagents matching the glob pattern
- **AND** pattern matching SHALL support `*`, `?`, `[...]` glob syntax

#### Scenario: Multiple pattern configuration
- **WHEN** config includes both wildcard and specific patterns (e.g., `*` and `coder`)
- **THEN** the configuration SHALL be valid
- **AND** both patterns SHALL be evaluated independently during matching

### Requirement: Transcript Parsing for Subagent Name
The system SHALL parse the transcript file to extract the subagent name when a SubagentStop hook fires.

#### Scenario: Transcript contains subagent invocation
- **WHEN** SubagentStop hook fires with transcript_path pointing to valid JSONL file
- **AND** transcript contains a "Task" tool invocation with "subagent_type" parameter
- **THEN** the system SHALL extract the subagent_type value as the subagent name
- **AND** use the most recent matching invocation if multiple exist

#### Scenario: Transcript does not contain subagent data
- **WHEN** transcript file cannot be read or parsed
- **OR** transcript does not contain any Task tool invocations with subagent_type
- **THEN** the system SHALL use "unknown" as the subagent name
- **AND** continue processing without failing the hook

#### Scenario: Malformed transcript file
- **WHEN** transcript file contains invalid JSONL
- **THEN** the system SHALL log a warning
- **AND** use "unknown" as the subagent name
- **AND** continue SubagentStop hook processing

### Requirement: Glob Pattern Matching
The system SHALL match subagent names against configured patterns using glob syntax, supporting wildcard, exact, and glob patterns.

#### Scenario: Wildcard matches all subagents
- **WHEN** pattern is `*`
- **AND** any subagent stops (e.g., "coder", "tester", "stuck")
- **THEN** the pattern SHALL match
- **AND** associated commands SHALL be queued for execution

#### Scenario: Exact match pattern
- **WHEN** pattern is `coder`
- **AND** subagent name is exactly "coder"
- **THEN** the pattern SHALL match
- **AND** pattern SHALL NOT match "auto-coder" or "coder-agent"

#### Scenario: Prefix glob pattern
- **WHEN** pattern is `test*`
- **AND** subagent name is "tester", "test-runner", or "testing"
- **THEN** the pattern SHALL match
- **AND** pattern SHALL NOT match "runner-test"

#### Scenario: Suffix glob pattern
- **WHEN** pattern is `*coder`
- **AND** subagent name is "coder", "auto-coder", or "smart-coder"
- **THEN** the pattern SHALL match
- **AND** pattern SHALL NOT match "coder-agent"

#### Scenario: Character class glob pattern
- **WHEN** pattern is `agent_[0-9]*`
- **AND** subagent name is "agent_1", "agent_2x", or "agent_99test"
- **THEN** the pattern SHALL match
- **AND** pattern SHALL NOT match "agent_x" or "agent"

#### Scenario: Multiple patterns match same subagent
- **WHEN** subagent name is "coder"
- **AND** config has patterns `*`, `coder`, and `*coder`
- **THEN** all three patterns SHALL match
- **AND** commands from all matching patterns SHALL be collected for execution

### Requirement: Command Execution Order
The system SHALL execute commands in a deterministic order when multiple patterns match the same subagent.

#### Scenario: Wildcard and specific pattern both match
- **WHEN** subagent "coder" stops
- **AND** config has both `*` and `coder` patterns with commands
- **THEN** wildcard (`*`) commands SHALL execute first
- **AND** specific (`coder`) commands SHALL execute second
- **AND** all commands SHALL complete before hook returns

#### Scenario: Multiple glob patterns match
- **WHEN** subagent "auto-coder" stops
- **AND** config has patterns `*coder`, `auto*`, and `*`
- **THEN** wildcard (`*`) commands SHALL execute first
- **AND** other matching patterns SHALL execute in stable order
- **AND** execution order SHALL be consistent across runs

#### Scenario: No patterns match
- **WHEN** subagent "unknown-agent" stops
- **AND** config only has specific patterns like `coder` and `tester`
- **THEN** no commands SHALL execute
- **AND** SubagentStop hook SHALL complete successfully
- **AND** notification (if configured) SHALL still be sent

### Requirement: Environment Variable Context
The system SHALL pass subagent context to command execution via environment variables.

#### Scenario: Environment variables available in commands
- **WHEN** a subagent stop command executes
- **THEN** the following environment variables SHALL be available:
  - `CONCLAUDE_SUBAGENT_NAME` - Name of the stopped subagent
  - `CONCLAUDE_SESSION_ID` - Session ID from payload
  - `CONCLAUDE_TRANSCRIPT_PATH` - Path to transcript file
  - `CONCLAUDE_HOOK_EVENT` - Always "SubagentStop"
  - `CONCLAUDE_CWD` - Current working directory

#### Scenario: Subagent name in environment variable
- **WHEN** subagent "coder" stops and command executes
- **THEN** `CONCLAUDE_SUBAGENT_NAME` environment variable SHALL equal "coder"
- **AND** command can access this via `$CONCLAUDE_SUBAGENT_NAME` in bash

#### Scenario: Unknown subagent name in environment variable
- **WHEN** subagent name cannot be determined from transcript
- **THEN** `CONCLAUDE_SUBAGENT_NAME` SHALL equal "unknown"
- **AND** all other environment variables SHALL still be populated

#### Scenario: Environment variables do not conflict with system
- **WHEN** commands execute with environment variables
- **THEN** all conclaude variables SHALL use `CONCLAUDE_` prefix
- **AND** SHALL NOT override standard environment variables (PATH, HOME, etc.)

### Requirement: Command Output Handling
The system SHALL respect showStdout, showStderr, and maxOutputLines settings for subagent stop commands.

#### Scenario: Command with showStdout enabled
- **WHEN** subagent stop command has `showStdout: true`
- **AND** command produces stdout
- **THEN** stdout SHALL be displayed to user/Claude
- **AND** maxOutputLines limit SHALL apply if configured

#### Scenario: Command with showStderr enabled
- **WHEN** subagent stop command has `showStderr: true`
- **AND** command produces stderr
- **THEN** stderr SHALL be displayed to user/Claude
- **AND** maxOutputLines limit SHALL apply if configured

#### Scenario: Command with output disabled
- **WHEN** subagent stop command has `showStdout: false` and `showStderr: false`
- **THEN** no command output SHALL be shown to user/Claude
- **AND** command SHALL still execute fully

### Requirement: Graceful Command Failure Handling
The system SHALL handle command failures without blocking SubagentStop hook completion.

#### Scenario: Command execution fails
- **WHEN** a subagent stop command fails (non-zero exit code)
- **THEN** the failure SHALL be logged
- **AND** SubagentStop hook SHALL continue processing
- **AND** remaining commands SHALL still execute

#### Scenario: Command spawning fails
- **WHEN** a subagent stop command cannot be spawned (command not found, permission denied)
- **THEN** the error SHALL be logged
- **AND** SubagentStop hook SHALL continue processing
- **AND** SubagentStop notification SHALL still be sent (if configured)

#### Scenario: All commands complete despite individual failures
- **WHEN** multiple subagent stop commands are configured
- **AND** some commands fail during execution
- **THEN** all commands SHALL be attempted
- **AND** SubagentStop hook SHALL complete with success status

### Requirement: Backward Compatibility
The system SHALL maintain existing SubagentStop notification behavior when no subagentStop config is present.

#### Scenario: Config without subagentStop section
- **WHEN** configuration does not include `subagentStop` section
- **AND** SubagentStop hook fires
- **THEN** existing notification behavior SHALL work unchanged
- **AND** no commands SHALL attempt to execute
- **AND** hook SHALL complete successfully

#### Scenario: Empty subagentStop configuration
- **WHEN** configuration includes `subagentStop: {}` with no commands
- **AND** SubagentStop hook fires
- **THEN** no commands SHALL execute
- **AND** existing notification behavior SHALL work unchanged

#### Scenario: subagentStop config does not affect other hooks
- **WHEN** subagentStop configuration is present
- **THEN** Stop, SessionEnd, and other hooks SHALL be unaffected
- **AND** only SubagentStop hook SHALL use subagentStop config

### Requirement: Configuration Validation
The system SHALL validate subagentStop configuration at load time to catch errors early.

#### Scenario: Valid subagentStop configuration
- **WHEN** config includes valid subagentStop section with patterns and commands
- **THEN** configuration SHALL load successfully
- **AND** schema validation SHALL pass

#### Scenario: Invalid command structure
- **WHEN** subagentStop command is missing required `run` field
- **THEN** configuration loading SHALL fail with validation error
- **AND** error message SHALL indicate missing field

#### Scenario: Invalid maxOutputLines value
- **WHEN** subagentStop command has `maxOutputLines: 0` or negative value
- **THEN** configuration loading SHALL fail with validation error
- **AND** error message SHALL indicate invalid value range

#### Scenario: Pattern is empty string
- **WHEN** config includes pattern key as empty string ("")
- **THEN** configuration loading SHALL fail with validation error
- **AND** error message SHALL indicate invalid pattern
