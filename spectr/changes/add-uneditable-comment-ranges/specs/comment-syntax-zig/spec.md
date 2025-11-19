# comment-syntax-zig Specification

## Purpose

Define Zig-specific comment syntax detection for uneditable range markers, supporting line comments (`//`) and doc comments (`///`, `//!`).

## ADDED Requirements

### Requirement: Line Comment Detection

The system SHALL recognize uneditable markers within Zig line comments (`//`).

#### Scenario: Single-line comment with marker

- **GIVEN** a Zig file with:
  ```zig
  // <!-- conclaude-uneditable:start -->
  ```
- **WHEN** the file is parsed for uneditable markers
- **THEN** the start marker SHALL be detected
- **AND** the marker line SHALL be recorded as the start of a protected range

#### Scenario: Line comment with trailing content

- **GIVEN** a Zig file with:
  ```zig
  // <!-- conclaude-uneditable:start --> Auto-generated code
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be detected (trailing content ignored)
- **AND** the range SHALL begin at this line

### Requirement: Doc Comment Detection

The system SHALL recognize uneditable markers within Zig doc comments (`///`, `//!`).

#### Scenario: Outer doc comment with marker

- **GIVEN** a Zig file with:
  ```zig
  /// <!-- conclaude-uneditable:start -->
  /// Auto-generated struct documentation
  const Generated = struct {};
  /// <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the markers SHALL be detected within `///` comments
- **AND** a protected range from line 1 to line 4 SHALL be created

#### Scenario: Top-level doc comment with marker

- **GIVEN** a Zig file with:
  ```zig
  //! <!-- conclaude-uneditable:start -->
  //! Module documentation
  //! <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the markers SHALL be detected within `//!` comments
- **AND** a protected range SHALL be created

### Requirement: File Extension Mapping

The system SHALL detect Zig files by their file extension and apply Zig comment syntax rules.

#### Scenario: .zig file extension

- **GIVEN** a file named "main.zig"
- **WHEN** the file is processed for uneditable ranges
- **THEN** Zig comment syntax rules SHALL be applied
- **AND** markers within `//`, `///`, or `//!` comments SHALL be detected

#### Scenario: .zig file with markers

- **GIVEN** a file "generated.zig" containing:
  ```zig
  // <!-- conclaude-uneditable:start -->
  // Auto-generated bindings
  pub const API_VERSION = 1;
  // <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** a protected range from line 1 to line 4 SHALL be created
- **AND** edits overlapping this range SHALL be blocked

### Requirement: Marker Format Validation

The system SHALL only recognize correctly formatted markers within Zig comments.

#### Scenario: Marker in string literal (not detected)

- **GIVEN** a Zig file with:
  ```zig
  const marker = "// <!-- conclaude-uneditable:start -->";
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL NOT be detected (inside string, not comment)
- **AND** no protected range SHALL be created

#### Scenario: Correctly formatted marker in comment

- **GIVEN** a Zig file with:
  ```zig
  // <!-- conclaude-uneditable:start -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be recognized
- **AND** a protected range SHALL start at this line

### Requirement: Nested Range Handling

The system SHALL handle nested ranges within Zig code structure.

#### Scenario: Nested struct protection

- **GIVEN** a Zig file with:
  ```zig
  // <!-- conclaude-uneditable:start -->  // Line 1
  const Config = struct {
      // <!-- conclaude-uneditable:start -->  // Line 3
      api_key: []const u8,
      // <!-- conclaude-uneditable:end -->  // Line 5
  };
  // <!-- conclaude-uneditable:end -->  // Line 7
  ```
- **WHEN** the file is parsed
- **THEN** two ranges SHALL be created: (1-7) and (3-5)
- **AND** both ranges SHALL be marked as protected
- **AND** edits overlapping either range SHALL be blocked

### Requirement: Performance Characteristics

The system SHALL efficiently parse Zig files for uneditable markers.

#### Scenario: Large Zig file with multiple markers

- **GIVEN** a Zig file with 5,000 lines and 10 protected ranges
- **WHEN** the file is parsed for markers
- **THEN** parsing SHALL complete in under 50ms
- **AND** all 10 ranges SHALL be correctly identified

#### Scenario: Zig file with no markers

- **GIVEN** a Zig file with 2,000 lines and no uneditable markers
- **WHEN** the file is parsed
- **THEN** parsing SHALL complete quickly (under 30ms)
- **AND** an empty range list SHALL be returned
- **AND** no false positive detections SHALL occur
