# comment-syntax-go Specification

## Purpose

Define Go-specific comment syntax detection for uneditable range markers, supporting both line comments (`//`) and block comments (`/* */`).

## ADDED Requirements

### Requirement: Line Comment Detection

The system SHALL recognize uneditable markers within Go line comments (`//`).

#### Scenario: Single-line comment with marker

- **GIVEN** a Go file with:
  ```go
  // <!-- conclaude-uneditable:start -->
  ```
- **WHEN** the file is parsed for uneditable markers
- **THEN** the start marker SHALL be detected
- **AND** the marker line SHALL be recorded as the start of a protected range

#### Scenario: Line comment with leading whitespace

- **GIVEN** a Go file with:
  ```go
      // <!-- conclaude-uneditable:start -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be detected after trimming leading whitespace
- **AND** the range SHALL begin at this line

#### Scenario: Line comment with trailing content

- **GIVEN** a Go file with:
  ```go
  // <!-- conclaude-uneditable:start --> DO NOT EDIT
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be detected (trailing content ignored)
- **AND** the range SHALL begin at this line

### Requirement: Block Comment Detection

The system SHALL recognize uneditable markers within Go block comments (`/* */`).

#### Scenario: Block comment with marker on single line

- **GIVEN** a Go file with:
  ```go
  /* <!-- conclaude-uneditable:start --> */
  ```
- **WHEN** the file is parsed
- **THEN** the start marker SHALL be detected within the block comment
- **AND** the range SHALL begin at this line

#### Scenario: Block comment with marker on separate line

- **GIVEN** a Go file with:
  ```go
  /*
   * <!-- conclaude-uneditable:start -->
   * Auto-generated code below
   */
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be detected on line 2
- **AND** the range SHALL begin at line 2

#### Scenario: Multi-line block comment with end marker

- **GIVEN** a Go file with:
  ```go
  /*
   * Protected section
   * <!-- conclaude-uneditable:end -->
   */
  ```
- **WHEN** the file is parsed
- **THEN** the end marker SHALL be detected on line 3
- **AND** the range SHALL end at line 3

### Requirement: File Extension Mapping

The system SHALL detect Go files by their file extension and apply Go comment syntax rules.

#### Scenario: .go file extension

- **GIVEN** a file named "main.go"
- **WHEN** the file is processed for uneditable ranges
- **THEN** Go comment syntax rules SHALL be applied
- **AND** markers within `//` or `/* */` comments SHALL be detected

#### Scenario: .go file with markers

- **GIVEN** a file "service.go" containing:
  ```go
  package main

  // <!-- conclaude-uneditable:start -->
  func GeneratedMethod() {
      // Auto-generated
  }
  // <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** a protected range from line 3 to line 7 SHALL be created
- **AND** edits overlapping this range SHALL be blocked

### Requirement: Marker Format Validation

The system SHALL only recognize correctly formatted markers within Go comments.

#### Scenario: Marker not in comment

- **GIVEN** a Go file with:
  ```go
  var marker = "<!-- conclaude-uneditable:start -->"
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL NOT be detected (not in a comment)
- **AND** no protected range SHALL be created

#### Scenario: Malformed marker in comment

- **GIVEN** a Go file with:
  ```go
  // <-- conclaude-uneditable:start -->  // Missing !
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL NOT be recognized (malformed)
- **AND** no protected range SHALL be created

#### Scenario: Correctly formatted marker in comment

- **GIVEN** a Go file with:
  ```go
  // <!-- conclaude-uneditable:start -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be recognized
- **AND** a protected range SHALL start at this line

### Requirement: Nested Comment Handling

The system SHALL handle nested ranges within Go code structure.

#### Scenario: Nested function protection

- **GIVEN** a Go file with:
  ```go
  // <!-- conclaude-uneditable:start -->  // Line 1
  type Config struct {
      // <!-- conclaude-uneditable:start -->  // Line 3
      APIKey string
      // <!-- conclaude-uneditable:end -->  // Line 5
  }
  // <!-- conclaude-uneditable:end -->  // Line 7
  ```
- **WHEN** the file is parsed
- **THEN** two ranges SHALL be created: (1-7) and (3-5)
- **AND** both ranges SHALL be marked as protected
- **AND** edits overlapping either range SHALL be blocked

#### Scenario: Deeply nested ranges

- **GIVEN** a Go file with three levels of nesting (outer, middle, inner)
- **WHEN** the file is parsed
- **THEN** all three ranges SHALL be detected
- **AND** each range SHALL have the correct nesting level recorded
- **AND** edits overlapping any range SHALL be blocked

### Requirement: Edge Case Handling

The system SHALL gracefully handle edge cases in Go comment syntax.

#### Scenario: Empty lines between markers

- **GIVEN** a Go file with:
  ```go
  // <!-- conclaude-uneditable:start -->


  func Protected() {}


  // <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the range SHALL include all lines between markers (including empty lines)
- **AND** edits within the range SHALL be blocked

#### Scenario: Multiple markers on single line (invalid)

- **GIVEN** a Go file with:
  ```go
  // <!-- conclaude-uneditable:start --> <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** both markers SHALL be detected on the same line
- **AND** the range SHALL start and end at the same line (line N to line N)

#### Scenario: Marker at file start

- **GIVEN** a Go file with marker at line 1:
  ```go
  // <!-- conclaude-uneditable:start -->
  package main
  // <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the range SHALL start at line 1
- **AND** the range SHALL end at line 3

#### Scenario: Marker at file end

- **GIVEN** a Go file with:
  ```go
  package main

  // <!-- conclaude-uneditable:start -->
  func Last() {}
  // <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the range SHALL end at the last line of the file
- **AND** the range SHALL be valid

### Requirement: Performance Characteristics

The system SHALL efficiently parse Go files for uneditable markers.

#### Scenario: Large Go file with multiple markers

- **GIVEN** a Go file with 5,000 lines and 10 protected ranges
- **WHEN** the file is parsed for markers
- **THEN** parsing SHALL complete in under 50ms
- **AND** all 10 ranges SHALL be correctly identified

#### Scenario: Go file with no markers

- **GIVEN** a Go file with 1,000 lines and no uneditable markers
- **WHEN** the file is parsed
- **THEN** parsing SHALL complete quickly (under 20ms)
- **AND** an empty range list SHALL be returned
- **AND** no false positive detections SHALL occur
