## ADDED Requirements
### Requirement: Working Directory Interpolation Preserves Spaces
Bash interpolation for `workingDir` values SHALL retain whitespace characters so commands execute from the intended path.

#### Scenario: Absolute workingDir containing spaces
- **WHEN** a stop command specifies `workingDir` set to "/tmp/my project"
- **AND** that directory exists
- **THEN** interpolation SHALL produce "/tmp/my project" without stripping spaces
- **AND** the command SHALL execute from "/tmp/my project" as its working directory

#### Scenario: Interpolation with expansions producing spaces
- **WHEN** `workingDir` is set to "$HOME/My Projects/tool"
- **AND** HOME is "/home/user"
- **THEN** interpolation SHALL produce "/home/user/My Projects/tool" with spaces intact
- **AND** command resolution SHALL treat the entire path as the working directory
