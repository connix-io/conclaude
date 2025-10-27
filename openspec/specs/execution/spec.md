# execution Specification

## Purpose
TBD - created by archiving change add-output-limiting. Update Purpose after archive.
## Requirements
### Requirement: Command Output Line Limiting Configuration
The system SHALL provide an optional maxOutputLines field for individual stop commands in the configuration to limit stdout and stderr output.

#### Scenario: Command with maxOutputLines configured
- **WHEN** a stop command includes maxOutputLines field set to 10
- **THEN** stdout and stderr SHALL each be limited to 10 lines maximum
- **AND** output exceeding the limit SHALL be truncated with a clear indicator

#### Scenario: Command without maxOutputLines configured
- **WHEN** a stop command does not include a maxOutputLines field
- **THEN** the command output SHALL be displayed in full without truncation
- **AND** existing behavior SHALL be preserved for backward compatibility

### Requirement: Output Truncation Enforcement
The system SHALL truncate command output when it exceeds the configured line limit and provide clear indication of truncation.

#### Scenario: Stdout exceeds line limit
- **WHEN** a command produces stdout exceeding the maxOutputLines limit
- **THEN** only the first N lines SHALL be displayed
- **AND** a truncation indicator SHALL be appended (e.g., "... (123 lines omitted)")
- **AND** the omitted line count SHALL be accurate

#### Scenario: Stderr exceeds line limit
- **WHEN** a command produces stderr exceeding the maxOutputLines limit
- **THEN** only the first N lines SHALL be displayed
- **AND** a truncation indicator SHALL be appended (e.g., "... (45 lines omitted)")
- **AND** the omitted line count SHALL be accurate

#### Scenario: Output within line limit
- **WHEN** a command produces output within the maxOutputLines limit
- **THEN** all output lines SHALL be displayed
- **AND** no truncation indicator SHALL be shown

#### Scenario: Independent stdout and stderr limiting
- **WHEN** both stdout and stderr are produced by a command with maxOutputLines configured
- **THEN** stdout SHALL be limited to maxOutputLines independently
- **AND** stderr SHALL be limited to maxOutputLines independently
- **AND** truncation SHALL be applied separately to each stream

### Requirement: Configuration Validation
The system SHALL validate maxOutputLines values in the configuration to ensure they are properly formatted and reasonable.

#### Scenario: Valid maxOutputLines value
- **WHEN** maxOutputLines field contains a positive integer value
- **THEN** the configuration SHALL be accepted
- **AND** the limit SHALL be applied during command execution

#### Scenario: Invalid maxOutputLines value
- **WHEN** maxOutputLines field contains a non-numeric value, zero, or negative number
- **THEN** the configuration loading SHALL fail with a validation error
- **AND** the error message SHALL indicate the maxOutputLines value format issue

#### Scenario: Reasonable value range
- **WHEN** maxOutputLines field contains a value between 1 and 10000
- **THEN** the configuration SHALL be accepted
- **AND** values outside this range SHALL be rejected with a helpful error message

### Requirement: Output Limiting with showStdout and showStderr
The system SHALL apply output limiting only when the respective output stream is being shown to the user or Claude.

#### Scenario: maxOutputLines with showStdout disabled
- **WHEN** maxOutputLines is configured but showStdout is false
- **THEN** stdout SHALL NOT be shown to the user or Claude
- **AND** maxOutputLines SHALL have no effect on stdout

#### Scenario: maxOutputLines with showStderr disabled
- **WHEN** maxOutputLines is configured but showStderr is false
- **THEN** stderr SHALL NOT be shown to the user or Claude
- **AND** maxOutputLines SHALL have no effect on stderr

#### Scenario: maxOutputLines with both outputs enabled
- **WHEN** maxOutputLines is configured with showStdout true and showStderr true
- **THEN** both stdout and stderr SHALL be limited to maxOutputLines
- **AND** truncation indicators SHALL be shown for each stream if exceeded

