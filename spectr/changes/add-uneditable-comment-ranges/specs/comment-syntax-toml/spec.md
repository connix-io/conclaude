# comment-syntax-toml Specification

## Purpose

Define TOML-specific comment syntax detection for uneditable range markers, supporting line comments (`#`).

## ADDED Requirements

### Requirement: Line Comment Detection

The system SHALL recognize uneditable markers within TOML line comments (`#`).

#### Scenario: Single-line comment with marker

- **GIVEN** a TOML file with:
  ```toml
  # <!-- conclaude-uneditable:start -->
  ```
- **WHEN** the file is parsed for uneditable markers
- **THEN** the start marker SHALL be detected
- **AND** the marker line SHALL be recorded as the start of a protected range

#### Scenario: Line comment with trailing content

- **GIVEN** a TOML file with:
  ```toml
  # <!-- conclaude-uneditable:start --> Auto-generated configuration
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be detected (trailing content ignored)
- **AND** the range SHALL begin at this line

### Requirement: File Extension Mapping

The system SHALL detect TOML files by their file extension and apply TOML comment syntax rules.

#### Scenario: .toml file extension

- **GIVEN** a file named "config.toml"
- **WHEN** the file is processed for uneditable ranges
- **THEN** TOML comment syntax rules SHALL be applied
- **AND** markers within `#` comments SHALL be detected

#### Scenario: Cargo.toml file

- **GIVEN** a file named "Cargo.toml"
- **WHEN** the file is processed for uneditable ranges
- **THEN** TOML comment syntax rules SHALL be applied
- **AND** markers SHALL be detected correctly

#### Scenario: .toml file with markers

- **GIVEN** a file "app.toml" containing:
  ```toml
  # <!-- conclaude-uneditable:start -->
  # Auto-generated database configuration
  [database]
  host = "localhost"
  port = 5432
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** a protected range from line 1 to line 6 SHALL be created
- **AND** edits overlapping this range SHALL be blocked

### Requirement: Marker Format Validation

The system SHALL only recognize correctly formatted markers within TOML comments.

#### Scenario: Marker in string value (not detected)

- **GIVEN** a TOML file with:
  ```toml
  message = "# <!-- conclaude-uneditable:start -->"
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL NOT be detected (inside string, not comment)
- **AND** no protected range SHALL be created

#### Scenario: Marker in multi-line string (not detected)

- **GIVEN** a TOML file with:
  ```toml
  description = """
  # <!-- conclaude-uneditable:start -->
  Some description
  """
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL NOT be detected (inside string)
- **AND** no protected range SHALL be created

#### Scenario: Correctly formatted marker in comment

- **GIVEN** a TOML file with:
  ```toml
  # <!-- conclaude-uneditable:start -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be recognized
- **AND** a protected range SHALL start at this line

### Requirement: Nested Range Handling

The system SHALL handle nested ranges within TOML structure.

#### Scenario: Nested table protection

- **GIVEN** a TOML file with:
  ```toml
  # <!-- conclaude-uneditable:start -->  # Line 1
  [server]
  # <!-- conclaude-uneditable:start -->  # Line 3
  host = "localhost"
  port = 8080
  # <!-- conclaude-uneditable:end -->  # Line 6

  [server.ssl]
  enabled = true
  # <!-- conclaude-uneditable:end -->  # Line 10
  ```
- **WHEN** the file is parsed
- **THEN** two ranges SHALL be created: (1-10) and (3-6)
- **AND** both ranges SHALL be marked as protected
- **AND** edits overlapping either range SHALL be blocked

### Requirement: Edge Case Handling

The system SHALL gracefully handle edge cases in TOML comment syntax.

#### Scenario: Inline comment after value

- **GIVEN** a TOML file with:
  ```toml
  name = "app"  # <!-- conclaude-uneditable:start -->
  version = "1.0"
  tag = "latest"  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** both markers SHALL be detected (inline comments are valid)
- **AND** a protected range from line 1 to line 3 SHALL be created

#### Scenario: Marker at file start

- **GIVEN** a TOML file with marker at line 1:
  ```toml
  # <!-- conclaude-uneditable:start -->
  title = "My App"
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the range SHALL start at line 1
- **AND** the range SHALL end at line 3

#### Scenario: Marker at file end

- **GIVEN** a TOML file ending with:
  ```toml
  [lastSection]
  value = "data"
  # <!-- conclaude-uneditable:start -->
  # Generated metadata
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the range SHALL end at the last line of the file
- **AND** the range SHALL be valid

#### Scenario: Empty lines between markers

- **GIVEN** a TOML file with:
  ```toml
  # <!-- conclaude-uneditable:start -->


  [protected]
  enabled = true


  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the range SHALL include all lines between markers (including empty lines)
- **AND** edits within the range SHALL be blocked

### Requirement: Cargo.toml Compatibility

The system SHALL correctly handle TOML files used in Rust's Cargo.

#### Scenario: Cargo.toml package section with marker

- **GIVEN** a Cargo.toml file with:
  ```toml
  # <!-- conclaude-uneditable:start -->
  [package]
  name = "myapp"
  version = "0.1.0"
  edition = "2021"
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 1 SHALL be detected
- **AND** a protected range from line 1 to line 6 SHALL be created

#### Scenario: Cargo.toml dependencies with marker

- **GIVEN** a Cargo.toml file with:
  ```toml
  [package]
  name = "myapp"

  # <!-- conclaude-uneditable:start -->
  [dependencies]
  serde = "1.0"
  tokio = { version = "1", features = ["full"] }
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 4 SHALL be detected
- **AND** a protected range from line 4 to line 8 SHALL be created

### Requirement: pyproject.toml Compatibility

The system SHALL handle markers in Python's pyproject.toml files.

#### Scenario: pyproject.toml build-system with marker

- **GIVEN** a pyproject.toml file with:
  ```toml
  # <!-- conclaude-uneditable:start -->
  [build-system]
  requires = ["setuptools>=45", "wheel"]
  build-backend = "setuptools.build_meta"
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 1 SHALL be detected
- **AND** a protected range from line 1 to line 5 SHALL be created

#### Scenario: pyproject.toml tool section with marker

- **GIVEN** a pyproject.toml file with:
  ```toml
  [project]
  name = "mypackage"

  # <!-- conclaude-uneditable:start -->
  [tool.black]
  line-length = 88
  target-version = ['py38']
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 4 SHALL be detected
- **AND** a protected range from line 4 to line 8 SHALL be created

### Requirement: TOML-Specific Syntax Handling

The system SHALL handle TOML-specific patterns and constructs.

#### Scenario: Array of tables with marker

- **GIVEN** a TOML file with:
  ```toml
  # <!-- conclaude-uneditable:start -->
  [[products]]
  name = "Hammer"
  sku = 738594937

  [[products]]
  name = "Nail"
  sku = 284758393
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 1 SHALL be detected
- **AND** a protected range from line 1 to line 9 SHALL be created

#### Scenario: Inline table with marker

- **GIVEN** a TOML file with:
  ```toml
  # <!-- conclaude-uneditable:start -->
  point = { x = 1, y = 2 }
  color = { r = 255, g = 0, b = 0 }
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 1 SHALL be detected
- **AND** a protected range from line 1 to line 4 SHALL be created

#### Scenario: Dotted keys with marker

- **GIVEN** a TOML file with:
  ```toml
  # <!-- conclaude-uneditable:start -->
  server.host = "localhost"
  server.port = 8080
  server.ssl.enabled = true
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 1 SHALL be detected
- **AND** a protected range from line 1 to line 5 SHALL be created

#### Scenario: Array with marker

- **GIVEN** a TOML file with:
  ```toml
  # <!-- conclaude-uneditable:start -->
  integers = [ 1, 2, 3 ]
  colors = [ "red", "yellow", "green" ]
  nested = [ [ 1, 2 ], [ 3, 4, 5 ] ]
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 1 SHALL be detected
- **AND** a protected range from line 1 to line 5 SHALL be created

### Requirement: Date and Time Handling

The system SHALL handle TOML date-time values with markers.

#### Scenario: Date-time values with marker

- **GIVEN** a TOML file with:
  ```toml
  # <!-- conclaude-uneditable:start -->
  odt1 = 1979-05-27T07:32:00Z
  odt2 = 1979-05-27T00:32:00-07:00
  ldt1 = 1979-05-27T07:32:00
  ld1 = 1979-05-27
  lt1 = 07:32:00
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 1 SHALL be detected
- **AND** a protected range from line 1 to line 7 SHALL be created

### Requirement: Table Header Sections

The system SHALL handle markers around TOML table headers.

#### Scenario: Standard table with marker

- **GIVEN** a TOML file with:
  ```toml
  # <!-- conclaude-uneditable:start -->
  [database]
  host = "localhost"
  port = 5432

  [database.pool]
  min = 1
  max = 10
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 1 SHALL be detected
- **AND** a protected range from line 1 to line 9 SHALL be created

#### Scenario: Array of tables header with marker

- **GIVEN** a TOML file with:
  ```toml
  # <!-- conclaude-uneditable:start -->
  [[servers]]
  name = "alpha"
  ip = "10.0.0.1"

  [[servers]]
  name = "beta"
  ip = "10.0.0.2"
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 1 SHALL be detected
- **AND** a protected range from line 1 to line 9 SHALL be created

### Requirement: Performance Characteristics

The system SHALL efficiently parse TOML files for uneditable markers.

#### Scenario: Large TOML file with multiple markers

- **GIVEN** a TOML file with 2,500 lines and 8 protected ranges
- **WHEN** the file is parsed for markers
- **THEN** parsing SHALL complete in under 45ms
- **AND** all 8 ranges SHALL be correctly identified

#### Scenario: TOML file with no markers

- **GIVEN** a TOML file with 1,500 lines and no uneditable markers
- **WHEN** the file is parsed
- **THEN** parsing SHALL complete quickly (under 25ms)
- **AND** an empty range list SHALL be returned
- **AND** no false positive detections SHALL occur

#### Scenario: File with many comments but no markers

- **GIVEN** a TOML file with 200 comment lines but no uneditable markers
- **WHEN** the file is parsed
- **THEN** parsing SHALL scan all comments efficiently
- **AND** no protected ranges SHALL be created
- **AND** performance SHALL remain acceptable (under 35ms)
