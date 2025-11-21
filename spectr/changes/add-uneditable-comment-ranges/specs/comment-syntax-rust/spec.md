# comment-syntax-rust Specification

## Purpose

Define Rust-specific comment syntax detection for uneditable range markers, supporting line comments (`//`), block comments (`/* */`), doc comments (`///`, `//!`), and outer doc comments (`/** */`).

## ADDED Requirements

### Requirement: Line Comment Detection

The system SHALL recognize uneditable markers within Rust line comments (`//`).

#### Scenario: Single-line comment with marker

- **GIVEN** a Rust file with:
  ```rust
  // <!-- conclaude-uneditable:start -->
  ```
- **WHEN** the file is parsed for uneditable markers
- **THEN** the start marker SHALL be detected
- **AND** the marker line SHALL be recorded as the start of a protected range

#### Scenario: Line comment with trailing content

- **GIVEN** a Rust file with:
  ```rust
  // <!-- conclaude-uneditable:start --> Generated code
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be detected (trailing content ignored)
- **AND** the range SHALL begin at this line

### Requirement: Doc Comment Detection

The system SHALL recognize uneditable markers within Rust doc comments (`///`, `//!`, `/** */`).

#### Scenario: Outer doc comment with marker

- **GIVEN** a Rust file with:
  ```rust
  /// <!-- conclaude-uneditable:start -->
  /// Auto-generated struct
  struct Generated {}
  /// <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the markers SHALL be detected within `///` comments
- **AND** a protected range from line 1 to line 4 SHALL be created

#### Scenario: Inner doc comment with marker

- **GIVEN** a Rust file with:
  ```rust
  //! <!-- conclaude-uneditable:start -->
  //! Module documentation
  //! <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the markers SHALL be detected within `//!` comments
- **AND** a protected range SHALL be created

#### Scenario: Block doc comment with marker

- **GIVEN** a Rust file with:
  ```rust
  /**
   * <!-- conclaude-uneditable:start -->
   * Generated API
   */
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be detected within `/** */` comment
- **AND** the range SHALL begin at line 2

### Requirement: Block Comment Detection

The system SHALL recognize uneditable markers within Rust block comments (`/* */`).

#### Scenario: Block comment with marker

- **GIVEN** a Rust file with:
  ```rust
  /* <!-- conclaude-uneditable:start --> */
  fn protected() {}
  /* <!-- conclaude-uneditable:end --> */
  ```
- **WHEN** the file is parsed
- **THEN** both markers SHALL be detected within `/* */` comments
- **AND** a protected range SHALL be created

#### Scenario: Multi-line block comment with marker

- **GIVEN** a Rust file with:
  ```rust
  /*
   * <!-- conclaude-uneditable:start -->
   * Protected section
   */
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be detected on line 2
- **AND** the range SHALL begin at line 2

### Requirement: File Extension Mapping

The system SHALL detect Rust files by their file extension and apply Rust comment syntax rules.

#### Scenario: .rs file extension

- **GIVEN** a file named "main.rs"
- **WHEN** the file is processed for uneditable ranges
- **THEN** Rust comment syntax rules SHALL be applied
- **AND** markers within `//`, `///`, `//!`, or `/* */` comments SHALL be detected

#### Scenario: .rs file with markers

- **GIVEN** a file "generated.rs" containing:
  ```rust
  // <!-- conclaude-uneditable:start -->
  pub struct GeneratedConfig {
      pub api_key: String,
  }
  // <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** a protected range from line 1 to line 5 SHALL be created
- **AND** edits overlapping this range SHALL be blocked

### Requirement: Marker Format Validation

The system SHALL only recognize correctly formatted markers within Rust comments.

#### Scenario: Marker in string literal (not detected)

- **GIVEN** a Rust file with:
  ```rust
  let marker = "// <!-- conclaude-uneditable:start -->";
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL NOT be detected (inside string, not comment)
- **AND** no protected range SHALL be created

#### Scenario: Marker in raw string literal (not detected)

- **GIVEN** a Rust file with:
  ```rust
  let marker = r#"<!-- conclaude-uneditable:start -->"#;
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL NOT be detected (inside raw string)
- **AND** no protected range SHALL be created

#### Scenario: Correctly formatted marker in comment

- **GIVEN** a Rust file with:
  ```rust
  // <!-- conclaude-uneditable:start -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be recognized
- **AND** a protected range SHALL start at this line

### Requirement: Nested Range Handling

The system SHALL handle nested ranges within Rust code structure.

#### Scenario: Nested impl block protection

- **GIVEN** a Rust file with:
  ```rust
  // <!-- conclaude-uneditable:start -->  // Line 1
  impl Service {
      // <!-- conclaude-uneditable:start -->  // Line 3
      pub fn authenticate(&self) -> bool {
          true
      }
      // <!-- conclaude-uneditable:end -->  // Line 7
  }
  // <!-- conclaude-uneditable:end -->  // Line 9
  ```
- **WHEN** the file is parsed
- **THEN** two ranges SHALL be created: (1-9) and (3-7)
- **AND** both ranges SHALL be marked as protected
- **AND** edits overlapping either range SHALL be blocked

### Requirement: Attribute and Macro Compatibility

The system SHALL correctly handle files with Rust attributes and macros.

#### Scenario: Marker with attributes

- **GIVEN** a Rust file with:
  ```rust
  // <!-- conclaude-uneditable:start -->
  #[derive(Debug, Clone)]
  pub struct Config {}
  // <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the range SHALL include lines with attributes
- **AND** edits SHALL be blocked correctly

#### Scenario: Marker around macro invocation

- **GIVEN** a Rust file with:
  ```rust
  // <!-- conclaude-uneditable:start -->
  macro_rules! generated {
      () => { println!("Generated"); }
  }
  // <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the range SHALL include the macro definition
- **AND** edits SHALL be blocked

### Requirement: Edge Case Handling

The system SHALL gracefully handle edge cases in Rust comment syntax.

#### Scenario: Nested block comments (Rust supports this)

- **GIVEN** a Rust file with:
  ```rust
  /* outer /* inner <!-- conclaude-uneditable:start --> */ */
  fn protected() {}
  /* <!-- conclaude-uneditable:end --> */
  ```
- **WHEN** the file is parsed
- **THEN** the start marker SHALL be detected within nested block comments
- **AND** a protected range SHALL be created

#### Scenario: Marker at module start

- **GIVEN** a Rust file with:
  ```rust
  // <!-- conclaude-uneditable:start -->
  mod generated {
      pub fn api_call() {}
  }
  // <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the range SHALL protect the entire module
- **AND** edits SHALL be blocked

### Requirement: Performance Characteristics

The system SHALL efficiently parse Rust files for uneditable markers.

#### Scenario: Large Rust file with multiple markers

- **GIVEN** a Rust file with 6,000 lines and 10 protected ranges
- **WHEN** the file is parsed for markers
- **THEN** parsing SHALL complete in under 60ms
- **AND** all 10 ranges SHALL be correctly identified

#### Scenario: Rust file with no markers

- **GIVEN** a Rust file with 2,000 lines and no uneditable markers
- **WHEN** the file is parsed
- **THEN** parsing SHALL complete quickly (under 30ms)
- **AND** an empty range list SHALL be returned
- **AND** no false positive detections SHALL occur
