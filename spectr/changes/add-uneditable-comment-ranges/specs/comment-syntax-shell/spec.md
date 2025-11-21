# comment-syntax-shell Specification

## Purpose

Define Shell/Bash-specific comment syntax detection for uneditable range markers, supporting line comments (`#`).

## ADDED Requirements

### Requirement: Line Comment Detection

The system SHALL recognize uneditable markers within Shell line comments (`#`).

#### Scenario: Single-line comment with marker

- **GIVEN** a Shell script with:
  ```bash
  # <!-- conclaude-uneditable:start -->
  ```
- **WHEN** the file is parsed for uneditable markers
- **THEN** the start marker SHALL be detected
- **AND** the marker line SHALL be recorded as the start of a protected range

#### Scenario: Indented comment with marker

- **GIVEN** a Shell script with:
  ```bash
  if [ -f file ]; then
      # <!-- conclaude-uneditable:start -->
      echo "Protected"
      # <!-- conclaude-uneditable:end -->
  fi
  ```
- **WHEN** the file is parsed
- **THEN** both markers SHALL be detected regardless of indentation
- **AND** a protected range from line 2 to line 4 SHALL be created

#### Scenario: Comment with leading whitespace

- **GIVEN** a Shell script with:
  ```bash
      # <!-- conclaude-uneditable:start -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be detected after trimming leading whitespace
- **AND** the range SHALL begin at this line

### Requirement: File Extension Mapping

The system SHALL detect Shell scripts by their file extension and apply Shell comment syntax rules.

#### Scenario: .sh file extension

- **GIVEN** a file named "deploy.sh"
- **WHEN** the file is processed for uneditable ranges
- **THEN** Shell comment syntax rules SHALL be applied
- **AND** markers within `#` comments SHALL be detected

#### Scenario: .bash file extension

- **GIVEN** a file named "script.bash"
- **WHEN** the file is processed for uneditable ranges
- **THEN** Shell comment syntax rules SHALL be applied
- **AND** markers SHALL be detected correctly

#### Scenario: .zsh file extension

- **GIVEN** a file named "setup.zsh"
- **WHEN** the file is processed for uneditable ranges
- **THEN** Shell comment syntax rules SHALL be applied (same as bash)
- **AND** markers SHALL be detected correctly

#### Scenario: No extension with shebang

- **GIVEN** a file named "install" with shebang `#!/bin/bash`
- **WHEN** the file is processed for uneditable ranges
- **THEN** Shell comment syntax rules SHOULD be applied if shebang is detected
- **OR** no detection occurs if file extension is the only method (acceptable fallback)

#### Scenario: .sh file with markers

- **GIVEN** a file "generated.sh" containing:
  ```bash
  # <!-- conclaude-uneditable:start -->
  # Auto-generated deployment script
  export API_KEY="generated"
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** a protected range from line 1 to line 4 SHALL be created
- **AND** edits overlapping this range SHALL be blocked

### Requirement: Marker Format Validation

The system SHALL only recognize correctly formatted markers within Shell comments.

#### Scenario: Marker in string literal (not detected)

- **GIVEN** a Shell script with:
  ```bash
  message="# <!-- conclaude-uneditable:start -->"
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL NOT be detected (inside string, not comment)
- **AND** no protected range SHALL be created

#### Scenario: Marker in heredoc (not detected)

- **GIVEN** a Shell script with:
  ```bash
  cat << EOF
  # <!-- conclaude-uneditable:start -->
  Some text
  EOF
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL NOT be detected (inside heredoc)
- **AND** no protected range SHALL be created

#### Scenario: Correctly formatted marker in comment

- **GIVEN** a Shell script with:
  ```bash
  # <!-- conclaude-uneditable:start -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be recognized
- **AND** a protected range SHALL start at this line

### Requirement: Nested Range Handling

The system SHALL handle nested ranges within Shell script structure.

#### Scenario: Nested function protection

- **GIVEN** a Shell script with:
  ```bash
  # <!-- conclaude-uneditable:start -->  # Line 1
  function deploy() {
      # <!-- conclaude-uneditable:start -->  # Line 3
      local env="production"
      # <!-- conclaude-uneditable:end -->  # Line 5
      echo "Deploying to $env"
  }
  # <!-- conclaude-uneditable:end -->  # Line 8
  ```
- **WHEN** the file is parsed
- **THEN** two ranges SHALL be created: (1-8) and (3-5)
- **AND** both ranges SHALL be marked as protected
- **AND** edits overlapping either range SHALL be blocked

### Requirement: Edge Case Handling

The system SHALL gracefully handle edge cases in Shell comment syntax.

#### Scenario: Inline comment after command

- **GIVEN** a Shell script with:
  ```bash
  echo "test"  # <!-- conclaude-uneditable:start -->
  rm -rf /tmp/data
  echo "done"  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** both markers SHALL be detected (inline comments are valid)
- **AND** a protected range from line 1 to line 3 SHALL be created

#### Scenario: Marker at script start (after shebang)

- **GIVEN** a Shell script with:
  ```bash
  #!/bin/bash
  # <!-- conclaude-uneditable:start -->
  set -euo pipefail
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the range SHALL start at line 2 (after shebang)
- **AND** the range SHALL end at line 4

#### Scenario: Marker at script end

- **GIVEN** a Shell script ending with:
  ```bash
  echo "End of script"
  # <!-- conclaude-uneditable:start -->
  # Generated footer
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the range SHALL end at the last line of the file
- **AND** the range SHALL be valid

#### Scenario: Empty lines between markers

- **GIVEN** a Shell script with:
  ```bash
  # <!-- conclaude-uneditable:start -->


  export VAR="value"


  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the range SHALL include all lines between markers (including empty lines)
- **AND** edits within the range SHALL be blocked

### Requirement: Shebang Compatibility

The system SHALL correctly handle Shell scripts with shebang lines.

#### Scenario: Bash shebang

- **GIVEN** a Shell script starting with:
  ```bash
  #!/bin/bash
  # <!-- conclaude-uneditable:start -->
  source config.sh
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 2 SHALL be detected (after shebang)
- **AND** a protected range from line 2 to line 4 SHALL be created

#### Scenario: Env shebang

- **GIVEN** a Shell script with:
  ```bash
  #!/usr/bin/env bash
  # <!-- conclaude-uneditable:start -->
  set -x
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 2 SHALL be detected
- **AND** a protected range from line 2 to line 4 SHALL be created

### Requirement: Performance Characteristics

The system SHALL efficiently parse Shell scripts for uneditable markers.

#### Scenario: Large Shell script with multiple markers

- **GIVEN** a Shell script with 3,000 lines and 8 protected ranges
- **WHEN** the file is parsed for markers
- **THEN** parsing SHALL complete in under 40ms
- **AND** all 8 ranges SHALL be correctly identified

#### Scenario: Shell script with no markers

- **GIVEN** a Shell script with 1,000 lines and no uneditable markers
- **WHEN** the file is parsed
- **THEN** parsing SHALL complete quickly (under 20ms)
- **AND** an empty range list SHALL be returned
- **AND** no false positive detections SHALL occur

#### Scenario: Script with many comments but no markers

- **GIVEN** a Shell script with 200 comment lines but no uneditable markers
- **WHEN** the file is parsed
- **THEN** parsing SHALL scan all comments efficiently
- **AND** no protected ranges SHALL be created
- **AND** performance SHALL remain acceptable (under 30ms)
