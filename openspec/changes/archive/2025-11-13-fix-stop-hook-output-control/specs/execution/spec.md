# execution Spec Delta

## MODIFIED Requirements

### Requirement: Output Limiting with showStdout and showStderr
The system SHALL apply output limiting only when the respective output stream is being shown to the user or Claude. The system SHALL NOT output command stdout/stderr to the console (via eprintln or similar) when the respective show flags are disabled.

#### Scenario: maxOutputLines with showStdout disabled
- **WHEN** maxOutputLines is configured but showStdout is false
- **THEN** stdout SHALL NOT be shown to the user or Claude in the error message
- **AND** stdout SHALL NOT be printed to the console in diagnostic logging
- **AND** maxOutputLines SHALL have no effect on stdout

#### Scenario: maxOutputLines with showStderr disabled
- **WHEN** maxOutputLines is configured but showStderr is false
- **THEN** stderr SHALL NOT be shown to the user or Claude in the error message
- **AND** stderr SHALL NOT be printed to the console in diagnostic logging
- **AND** maxOutputLines SHALL have no effect on stderr

#### Scenario: maxOutputLines with both outputs enabled
- **WHEN** maxOutputLines is configured with showStdout true and showStderr true
- **THEN** both stdout and stderr SHALL be limited to maxOutputLines in the error message
- **AND** both stdout and stderr MAY be included in diagnostic logging to the console
- **AND** truncation indicators SHALL be shown for each stream if exceeded

#### Scenario: Diagnostic logging respects output flags
- **WHEN** a stop command fails and writes diagnostic information via eprintln
- **THEN** stdout SHALL only be included in the diagnostic output if showStdout is true
- **AND** stderr SHALL only be included in the diagnostic output if showStderr is true
- **AND** command name and exit code SHALL always be included regardless of flags
- **AND** placeholder text like "(output hidden by configuration)" MAY be shown when flags are false
