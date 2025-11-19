# uneditable-ranges Specification

## Purpose

Define the core behavior for protecting specific line ranges within files through inline comment-based markers, enabling fine-grained control over which code sections can be modified by AI during Claude Code sessions.

## ADDED Requirements

### Requirement: Configuration Structure

The system SHALL support opt-in configuration for uneditable comment ranges with customizable settings.

#### Scenario: Configuration enabled with default settings

- **GIVEN** a YAML configuration with `rules.uneditableRanges.enabled: true`
- **WHEN** the configuration is loaded
- **THEN** uneditable range detection SHALL be active for all Edit operations
- **AND** default language mappings SHALL be used
- **AND** a generic error message SHALL be displayed when edits are blocked

#### Scenario: Configuration disabled

- **GIVEN** a YAML configuration with `rules.uneditableRanges.enabled: false` or no uneditableRanges section
- **WHEN** the configuration is loaded
- **THEN** uneditable range detection SHALL be inactive
- **AND** all Edit operations SHALL proceed without range validation

#### Scenario: Configuration with custom message

- **GIVEN** a configuration with:
  ```yaml
  rules:
    uneditableRanges:
      enabled: true
      message: "This range is protected. Contact security team before modifying."
  ```
- **WHEN** an Edit operation is blocked
- **THEN** the custom message SHALL be included in the error response
- **AND** the error SHALL include both the range info and the custom message

### Requirement: Marker Format

The system SHALL recognize uneditable markers in the format `<!-- conclaude-uneditable:start -->` and `<!-- conclaude-uneditable:end -->` embedded within language-specific comments.

#### Scenario: Go marker detection

- **GIVEN** a Go file containing:
  ```go
  // <!-- conclaude-uneditable:start -->
  func ProtectedFunction() {}
  // <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed for uneditable ranges
- **THEN** the markers SHALL be detected within `//` comments
- **AND** a range SHALL be created for the lines between markers

#### Scenario: Python marker detection

- **GIVEN** a Python file containing:
  ```python
  # <!-- conclaude-uneditable:start -->
  def protected_function():
      pass
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed for uneditable ranges
- **THEN** the markers SHALL be detected within `#` comments
- **AND** a range SHALL be created for the lines between markers

#### Scenario: Marker case sensitivity

- **GIVEN** a file with markers using different cases: `<!-- CONCLAUDE-UNEDITABLE:START -->`
- **WHEN** the file is parsed
- **THEN** the markers SHALL NOT be recognized (case-sensitive matching)
- **AND** no ranges SHALL be created

#### Scenario: Whitespace tolerance

- **GIVEN** markers with varying whitespace:
  ```python
  #<!-- conclaude-uneditable:start -->
  #  <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** both markers SHALL be recognized after trimming whitespace
- **AND** the range SHALL be correctly identified

### Requirement: Range Parsing

The system SHALL parse files to extract uneditable line ranges based on detected markers and language-specific comment syntax.

#### Scenario: Single uneditable range

- **GIVEN** a file with one pair of markers at lines 10 and 20
- **WHEN** the file is parsed
- **THEN** a single UneditableRange SHALL be created with start_line=10 and end_line=20
- **AND** edits overlapping lines 10-20 SHALL be blocked

#### Scenario: Multiple non-overlapping ranges

- **GIVEN** a file with markers at lines 5-10 and 30-40
- **WHEN** the file is parsed
- **THEN** two UneditableRange objects SHALL be created
- **AND** edits overlapping either range SHALL be blocked
- **AND** edits between ranges (lines 11-29) SHALL be allowed

#### Scenario: Nested ranges

- **GIVEN** a file with nested markers:
  ```python
  # <!-- conclaude-uneditable:start -->  # Line 5
  def outer():
      # <!-- conclaude-uneditable:start -->  # Line 7
      def inner():
          pass
      # <!-- conclaude-uneditable:end -->  # Line 10
      pass
  # <!-- conclaude-uneditable:end -->  # Line 12
  ```
- **WHEN** the file is parsed
- **THEN** two ranges SHALL be created: (5-12) and (7-10)
- **AND** both ranges SHALL be stored with appropriate nesting levels
- **AND** edits overlapping any nested range SHALL be blocked

#### Scenario: Mismatched markers (start without end)

- **GIVEN** a file with:
  ```python
  # <!-- conclaude-uneditable:start -->  # Line 10
  def function():
      pass
  # (missing end marker)
  ```
- **WHEN** the file is parsed
- **THEN** parsing SHALL fail with an error
- **AND** the error message SHALL indicate "Unmatched start marker at line 10"
- **AND** the Edit operation SHALL be blocked with this error

#### Scenario: Mismatched markers (end without start)

- **GIVEN** a file with:
  ```python
  def function():
      pass
  # <!-- conclaude-uneditable:end -->  # Line 15 (no start marker)
  ```
- **WHEN** the file is parsed
- **THEN** parsing SHALL fail with an error
- **AND** the error message SHALL indicate "Unmatched end marker at line 15"
- **AND** the Edit operation SHALL be blocked with this error

### Requirement: Edit Operation Validation

The system SHALL intercept Edit tool operations in the PreToolUse hook and block edits that overlap uneditable ranges.

#### Scenario: Edit fully inside protected range

- **GIVEN** a file with protected range at lines 10-20
- **AND** an Edit operation with old_string spanning lines 12-15
- **WHEN** the PreToolUse hook is triggered
- **THEN** the edit SHALL be blocked
- **AND** the error message SHALL include "lines 10-20"
- **AND** the file path SHALL be included in the error

#### Scenario: Edit partially overlaps protected range

- **GIVEN** a file with protected range at lines 10-20
- **AND** an Edit operation with old_string spanning lines 15-25
- **WHEN** the PreToolUse hook is triggered
- **THEN** the edit SHALL be blocked (partial overlap blocks entire edit)
- **AND** the error message SHALL indicate the protected range

#### Scenario: Edit outside protected range

- **GIVEN** a file with protected range at lines 10-20
- **AND** an Edit operation with old_string spanning lines 30-35
- **WHEN** the PreToolUse hook is triggered
- **THEN** the edit SHALL be allowed
- **AND** no blocking message SHALL be returned

#### Scenario: Edit touches start or end marker lines

- **GIVEN** a file with markers at lines 10 (start) and 20 (end)
- **AND** an Edit operation that includes line 10 or line 20 in old_string
- **WHEN** the PreToolUse hook is triggered
- **THEN** the edit SHALL be blocked (marker lines are part of protected range)

#### Scenario: Multiple ranges with first range overlapping

- **GIVEN** a file with protected ranges at lines 5-10 and 30-40
- **AND** an Edit operation with old_string spanning lines 8-12
- **WHEN** the PreToolUse hook is triggered
- **THEN** the edit SHALL be blocked due to overlap with first range (5-10)
- **AND** the error SHALL reference the first overlapping range

### Requirement: Write Tool Exemption

The system SHALL NOT block Write tool operations based on uneditable ranges (only Edit tool is blocked).

#### Scenario: Write tool with protected ranges present

- **GIVEN** a file with protected range at lines 10-20
- **AND** a Write operation that overwrites the entire file
- **WHEN** the PreToolUse hook is triggered
- **THEN** the write SHALL be allowed (no blocking)
- **AND** the entire file content SHALL be replaced

#### Scenario: Edit tool with protected ranges

- **GIVEN** a file with protected range at lines 10-20
- **AND** an Edit operation overlapping the range
- **WHEN** the PreToolUse hook is triggered
- **THEN** the edit SHALL be blocked
- **AND** the error SHALL indicate the protected range

### Requirement: Error Message Format

The system SHALL provide structured, informative error messages when Edit operations are blocked.

#### Scenario: Error includes protected line range

- **GIVEN** an Edit blocked due to protected range at lines 42-58
- **WHEN** the error message is generated
- **THEN** the message SHALL include "lines 42-58"
- **AND** the message SHALL include the file path
- **AND** the message SHALL indicate "Blocked Edit operation"

#### Scenario: Error includes custom message

- **GIVEN** a configuration with custom message: "Contact security team"
- **AND** an Edit blocked due to protected range
- **WHEN** the error message is generated
- **THEN** the custom message SHALL be appended to the error
- **AND** the custom message SHALL be on a separate line for readability

#### Scenario: Error with no custom message

- **GIVEN** a configuration with no custom message set
- **AND** an Edit blocked due to protected range at lines 10-20
- **WHEN** the error message is generated
- **THEN** the error SHALL include range info and file path only
- **AND** no additional custom text SHALL be appended

### Requirement: Language Support Extensibility

The system SHALL support multiple programming languages through configurable comment syntax mappings.

#### Scenario: Supported language file

- **GIVEN** a file with extension ".go" (supported language)
- **WHEN** uneditable markers are present
- **THEN** the file SHALL be parsed for ranges using Go comment syntax
- **AND** ranges SHALL be detected correctly

#### Scenario: Unsupported language file

- **GIVEN** a file with extension ".xyz" (unsupported language)
- **WHEN** an Edit operation is performed
- **THEN** no range detection SHALL occur (graceful fallback)
- **AND** the Edit SHALL proceed without range validation
- **AND** no error SHALL be thrown for unsupported language

#### Scenario: File without extension

- **GIVEN** a file named "Makefile" (no extension)
- **WHEN** an Edit operation is performed
- **THEN** no range detection SHALL occur
- **AND** the Edit SHALL proceed without range validation

### Requirement: Performance Constraints

The system SHALL parse files for uneditable ranges efficiently without significantly impacting Edit operation latency.

#### Scenario: Small file parsing

- **GIVEN** a file with 100 lines and 2 protected ranges
- **WHEN** the file is parsed for ranges
- **THEN** parsing SHALL complete in under 10ms
- **AND** no noticeable delay SHALL occur in Edit validation

#### Scenario: Large file parsing

- **GIVEN** a file with 10,000 lines and 5 protected ranges
- **WHEN** the file is parsed for ranges
- **THEN** parsing SHALL complete in under 100ms
- **AND** Edit validation SHALL remain responsive

#### Scenario: File with no markers

- **GIVEN** a file with no uneditable markers
- **WHEN** the file is parsed
- **THEN** parsing SHALL return an empty range list quickly
- **AND** no performance overhead SHALL be introduced

### Requirement: Configuration Validation

The system SHALL validate uneditable range configuration and fail fast with clear errors if misconfigured.

#### Scenario: Valid configuration

- **GIVEN** a configuration:
  ```yaml
  rules:
    uneditableRanges:
      enabled: true
      message: "Custom error"
  ```
- **WHEN** the configuration is loaded
- **THEN** validation SHALL pass
- **AND** the configuration SHALL be usable

#### Scenario: Invalid enabled field type

- **GIVEN** a configuration with `enabled: "yes"` (string instead of boolean)
- **WHEN** the configuration is loaded
- **THEN** deserialization SHALL fail
- **AND** an error message SHALL indicate type mismatch

#### Scenario: Invalid message field type

- **GIVEN** a configuration with `message: 123` (number instead of string)
- **WHEN** the configuration is loaded
- **THEN** deserialization SHALL fail
- **AND** an error message SHALL indicate type mismatch
