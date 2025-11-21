# comment-syntax-nim Specification

## Purpose

Define Nim-specific comment syntax detection for uneditable range markers, supporting line comments (`#`) and block comments (`#[ ]#`).

## ADDED Requirements

### Requirement: Line Comment Detection

The system SHALL recognize uneditable markers within Nim line comments (`#`).

#### Scenario: Single-line comment with marker

- **GIVEN** a Nim file with:
  ```nim
  # <!-- conclaude-uneditable:start -->
  ```
- **WHEN** the file is parsed for uneditable markers
- **THEN** the start marker SHALL be detected
- **AND** the marker line SHALL be recorded as the start of a protected range

#### Scenario: Line comment with leading whitespace

- **GIVEN** a Nim file with:
  ```nim
      # <!-- conclaude-uneditable:start -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be detected after trimming leading whitespace
- **AND** the range SHALL begin at this line

#### Scenario: Line comment with trailing content

- **GIVEN** a Nim file with:
  ```nim
  # <!-- conclaude-uneditable:start --> Auto-generated code
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be detected (trailing content ignored)
- **AND** the range SHALL begin at this line

### Requirement: Block Comment Detection

The system SHALL recognize uneditable markers within Nim block comments (`#[ ]#`).

#### Scenario: Block comment with marker on single line

- **GIVEN** a Nim file with:
  ```nim
  #[ <!-- conclaude-uneditable:start --> ]#
  ```
- **WHEN** the file is parsed
- **THEN** the start marker SHALL be detected within the block comment
- **AND** the range SHALL begin at this line

#### Scenario: Multi-line block comment with marker

- **GIVEN** a Nim file with:
  ```nim
  #[
    <!-- conclaude-uneditable:start -->
    Auto-generated API bindings
  ]#
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be detected on line 2
- **AND** the range SHALL begin at line 2

#### Scenario: Block comment with end marker

- **GIVEN** a Nim file with:
  ```nim
  #[
    Protected section
    <!-- conclaude-uneditable:end -->
  ]#
  ```
- **WHEN** the file is parsed
- **THEN** the end marker SHALL be detected on line 3
- **AND** the range SHALL end at line 3

### Requirement: File Extension Mapping

The system SHALL detect Nim files by their file extension and apply Nim comment syntax rules.

#### Scenario: .nim file extension

- **GIVEN** a file named "main.nim"
- **WHEN** the file is processed for uneditable ranges
- **THEN** Nim comment syntax rules SHALL be applied
- **AND** markers within `#` or `#[ ]#` comments SHALL be detected

#### Scenario: .nims file extension (NimScript)

- **GIVEN** a file named "config.nims"
- **WHEN** the file is processed for uneditable ranges
- **THEN** Nim comment syntax rules SHALL be applied
- **AND** markers SHALL be detected using the same comment syntax as .nim files

#### Scenario: .nimble file extension (package files)

- **GIVEN** a file named "mypackage.nimble" containing:
  ```nim
  # Package

  # <!-- conclaude-uneditable:start -->
  version       = "0.1.0"
  author        = "Generated"
  description   = "Auto-generated package"
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** a protected range from line 3 to line 6 SHALL be created
- **AND** edits overlapping this range SHALL be blocked

### Requirement: Marker Format Validation

The system SHALL only recognize correctly formatted markers within Nim comments.

#### Scenario: Marker in string literal (not detected)

- **GIVEN** a Nim file with:
  ```nim
  let marker = "# <!-- conclaude-uneditable:start -->"
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL NOT be detected (inside string, not comment)
- **AND** no protected range SHALL be created

#### Scenario: Malformed marker in comment

- **GIVEN** a Nim file with:
  ```nim
  # <-- conclaude-uneditable:start -->  # Missing !
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL NOT be recognized (malformed)
- **AND** no protected range SHALL be created

#### Scenario: Correctly formatted marker in comment

- **GIVEN** a Nim file with:
  ```nim
  # <!-- conclaude-uneditable:start -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be recognized
- **AND** a protected range SHALL start at this line

### Requirement: Nested Range Handling

The system SHALL handle nested ranges within Nim code structure.

#### Scenario: Nested type protection

- **GIVEN** a Nim file with:
  ```nim
  # <!-- conclaude-uneditable:start -->  # Line 1
  type
    Config = object
      # <!-- conclaude-uneditable:start -->  # Line 4
      apiKey: string
      # <!-- conclaude-uneditable:end -->  # Line 6
  # <!-- conclaude-uneditable:end -->  # Line 7
  ```
- **WHEN** the file is parsed
- **THEN** two ranges SHALL be created: (1-7) and (4-6)
- **AND** both ranges SHALL be marked as protected
- **AND** edits overlapping either range SHALL be blocked

#### Scenario: Deeply nested ranges

- **GIVEN** a Nim file with three levels of nesting (outer, middle, inner)
- **WHEN** the file is parsed
- **THEN** all three ranges SHALL be detected
- **AND** each range SHALL have the correct nesting level recorded
- **AND** edits overlapping any range SHALL be blocked

### Requirement: Edge Case Handling

The system SHALL gracefully handle edge cases in Nim comment syntax.

#### Scenario: Empty lines between markers

- **GIVEN** a Nim file with:
  ```nim
  # <!-- conclaude-uneditable:start -->


  proc protected() =
    discard


  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the range SHALL include all lines between markers (including empty lines)
- **AND** edits within the range SHALL be blocked

#### Scenario: Multiple markers on single line (invalid)

- **GIVEN** a Nim file with:
  ```nim
  # <!-- conclaude-uneditable:start --> <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** both markers SHALL be detected on the same line
- **AND** the range SHALL start and end at the same line (line N to line N)

#### Scenario: Marker at file start

- **GIVEN** a Nim file with marker at line 1:
  ```nim
  # <!-- conclaude-uneditable:start -->
  import std/tables
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the range SHALL start at line 1
- **AND** the range SHALL end at line 3

#### Scenario: Nested block comments (Nim supports this)

- **GIVEN** a Nim file with:
  ```nim
  #[ outer #[ inner <!-- conclaude-uneditable:start --> ]# ]#
  proc protected() = discard
  #[ <!-- conclaude-uneditable:end --> ]#
  ```
- **WHEN** the file is parsed
- **THEN** the start marker SHALL be detected within nested block comments
- **AND** a protected range SHALL be created

### Requirement: Performance Characteristics

The system SHALL efficiently parse Nim files for uneditable markers.

#### Scenario: Large Nim file with multiple markers

- **GIVEN** a Nim file with 5,000 lines and 10 protected ranges
- **WHEN** the file is parsed for markers
- **THEN** parsing SHALL complete in under 50ms
- **AND** all 10 ranges SHALL be correctly identified

#### Scenario: Nim file with no markers

- **GIVEN** a Nim file with 1,000 lines and no uneditable markers
- **WHEN** the file is parsed
- **THEN** parsing SHALL complete quickly (under 20ms)
- **AND** an empty range list SHALL be returned
- **AND** no false positive detections SHALL occur
