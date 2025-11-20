# comment-syntax-jsonc Specification

## Purpose

Define JSONC (JSON with Comments)-specific comment syntax detection for uneditable range markers, supporting line comments (`//`) and block comments (`/* */`).

## ADDED Requirements

### Requirement: Line Comment Detection

The system SHALL recognize uneditable markers within JSONC line comments (`//`).

#### Scenario: Single-line comment with marker

- **GIVEN** a JSONC file with:
  ```jsonc
  // <!-- conclaude-uneditable:start -->
  ```
- **WHEN** the file is parsed for uneditable markers
- **THEN** the start marker SHALL be detected
- **AND** the marker line SHALL be recorded as the start of a protected range

#### Scenario: Line comment with trailing content

- **GIVEN** a JSONC file with:
  ```jsonc
  // <!-- conclaude-uneditable:start --> Auto-generated configuration
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be detected (trailing content ignored)
- **AND** the range SHALL begin at this line

### Requirement: Block Comment Detection

The system SHALL recognize uneditable markers within JSONC block comments (`/* */`).

#### Scenario: Block comment with marker

- **GIVEN** a JSONC file with:
  ```jsonc
  /* <!-- conclaude-uneditable:start --> */
  ```
- **WHEN** the file is parsed
- **THEN** the start marker SHALL be detected within the block comment
- **AND** the range SHALL begin at this line

#### Scenario: Multi-line block comment with marker

- **GIVEN** a JSONC file with:
  ```jsonc
  /*
   * <!-- conclaude-uneditable:start -->
   * Auto-generated settings
   */
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be detected on line 2
- **AND** the range SHALL begin at line 2

### Requirement: File Extension Mapping

The system SHALL detect JSONC files by their file extension and apply JSONC comment syntax rules.

#### Scenario: .jsonc file extension

- **GIVEN** a file named "settings.jsonc"
- **WHEN** the file is processed for uneditable ranges
- **THEN** JSONC comment syntax rules SHALL be applied
- **AND** markers within `//` or `/* */` comments SHALL be detected

#### Scenario: .jsonc file with markers

- **GIVEN** a file "tsconfig.jsonc" containing:
  ```jsonc
  {
    // <!-- conclaude-uneditable:start -->
    "compilerOptions": {
      "target": "ES2020",
      "module": "commonjs"
    },
    // <!-- conclaude-uneditable:end -->
    "include": ["src/**/*"]
  }
  ```
- **WHEN** the file is parsed
- **THEN** a protected range from line 2 to line 7 SHALL be created
- **AND** edits overlapping this range SHALL be blocked

### Requirement: Marker Format Validation

The system SHALL only recognize correctly formatted markers within JSONC comments.

#### Scenario: Marker in string value (not detected)

- **GIVEN** a JSONC file with:
  ```jsonc
  {
    "comment": "// <!-- conclaude-uneditable:start -->"
  }
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL NOT be detected (inside string, not comment)
- **AND** no protected range SHALL be created

#### Scenario: Marker in property name (not detected)

- **GIVEN** a JSONC file with:
  ```jsonc
  {
    "<!-- conclaude-uneditable:start -->": "value"
  }
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL NOT be detected (inside string key)
- **AND** no protected range SHALL be created

#### Scenario: Correctly formatted marker in comment

- **GIVEN** a JSONC file with:
  ```jsonc
  // <!-- conclaude-uneditable:start -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be recognized
- **AND** a protected range SHALL start at this line

### Requirement: Nested Range Handling

The system SHALL handle nested ranges within JSONC structure.

#### Scenario: Nested object protection

- **GIVEN** a JSONC file with:
  ```jsonc
  {
    // <!-- conclaude-uneditable:start -->  // Line 2
    "database": {
      // <!-- conclaude-uneditable:start -->  // Line 4
      "host": "localhost",
      "port": 5432
      // <!-- conclaude-uneditable:end -->  // Line 7
    },
    "cache": {}
    // <!-- conclaude-uneditable:end -->  // Line 10
  }
  ```
- **WHEN** the file is parsed
- **THEN** two ranges SHALL be created: (2-10) and (4-7)
- **AND** both ranges SHALL be marked as protected
- **AND** edits overlapping either range SHALL be blocked

### Requirement: Edge Case Handling

The system SHALL gracefully handle edge cases in JSONC comment syntax.

#### Scenario: Inline comment after property

- **GIVEN** a JSONC file with:
  ```jsonc
  {
    "name": "app",  // <!-- conclaude-uneditable:start -->
    "version": "1.0.0",
    "description": "test"  // <!-- conclaude-uneditable:end -->
  }
  ```
- **WHEN** the file is parsed
- **THEN** both markers SHALL be detected (inline comments are valid)
- **AND** a protected range from line 2 to line 4 SHALL be created

#### Scenario: Marker at file start

- **GIVEN** a JSONC file with marker at line 1:
  ```jsonc
  // <!-- conclaude-uneditable:start -->
  {
    "config": true
  }
  // <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the range SHALL start at line 1
- **AND** the range SHALL end at line 5

#### Scenario: Marker at file end

- **GIVEN** a JSONC file ending with:
  ```jsonc
  {
    "lastProperty": "value"
  }
  // <!-- conclaude-uneditable:start -->
  // Generated metadata
  // <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the range SHALL end at the last line of the file
- **AND** the range SHALL be valid

#### Scenario: Empty lines between markers

- **GIVEN** a JSONC file with:
  ```jsonc
  // <!-- conclaude-uneditable:start -->


  {
    "protected": true
  }


  // <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the range SHALL include all lines between markers (including empty lines)
- **AND** edits within the range SHALL be blocked

### Requirement: VS Code Configuration Compatibility

The system SHALL correctly handle JSONC files commonly used in VS Code configuration.

#### Scenario: settings.json with markers

- **GIVEN** a settings.json file with:
  ```jsonc
  {
    // <!-- conclaude-uneditable:start -->
    "editor.fontSize": 14,
    "editor.tabSize": 2,
    // <!-- conclaude-uneditable:end -->
    "workbench.colorTheme": "Default Dark+"
  }
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 2 SHALL be detected
- **AND** a protected range from line 2 to line 5 SHALL be created

#### Scenario: tsconfig.json with extends

- **GIVEN** a tsconfig.json file with:
  ```jsonc
  {
    // <!-- conclaude-uneditable:start -->
    "extends": "./tsconfig.base.json",
    "compilerOptions": {
      "outDir": "./dist"
    }
    // <!-- conclaude-uneditable:end -->
  }
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 2 SHALL be detected
- **AND** a protected range from line 2 to line 7 SHALL be created

### Requirement: JSONC-Specific Patterns

The system SHALL handle JSONC-specific configuration patterns.

#### Scenario: Array with protected items

- **GIVEN** a JSONC file with:
  ```jsonc
  {
    // <!-- conclaude-uneditable:start -->
    "scripts": [
      "build.sh",
      "test.sh"
    ],
    // <!-- conclaude-uneditable:end -->
    "env": {}
  }
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 2 SHALL be detected
- **AND** a protected range from line 2 to line 7 SHALL be created

#### Scenario: Trailing commas with marker

- **GIVEN** a JSONC file with:
  ```jsonc
  {
    // <!-- conclaude-uneditable:start -->
    "option1": true,
    "option2": false,
    // <!-- conclaude-uneditable:end -->
  }
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 2 SHALL be detected
- **AND** a protected range from line 2 to line 5 SHALL be created

#### Scenario: package.json with scripts

- **GIVEN** a package.json file with:
  ```jsonc
  {
    "name": "myapp",
    // <!-- conclaude-uneditable:start -->
    "scripts": {
      "build": "tsc",
      "test": "jest"
    },
    // <!-- conclaude-uneditable:end -->
    "dependencies": {}
  }
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 3 SHALL be detected
- **AND** a protected range from line 3 to line 8 SHALL be created

### Requirement: Block Comment Placement

The system SHALL handle block comments at various positions.

#### Scenario: Block comment at property level

- **GIVEN** a JSONC file with:
  ```jsonc
  {
    /* <!-- conclaude-uneditable:start --> */
    "protected": {
      "value": 123
    }
    /* <!-- conclaude-uneditable:end --> */
  }
  ```
- **WHEN** the file is parsed
- **THEN** both markers SHALL be detected
- **AND** a protected range from line 2 to line 6 SHALL be created

#### Scenario: Multi-line block with metadata

- **GIVEN** a JSONC file with:
  ```jsonc
  /*
   * <!-- conclaude-uneditable:start -->
   * Auto-generated by tool v1.0
   * Do not modify manually
   */
  {
    "generated": true
  }
  // <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** both markers SHALL be detected
- **AND** a protected range from line 2 to line 9 SHALL be created

### Requirement: Performance Characteristics

The system SHALL efficiently parse JSONC files for uneditable markers.

#### Scenario: Large JSONC file with multiple markers

- **GIVEN** a JSONC file with 3,000 lines and 8 protected ranges
- **WHEN** the file is parsed for markers
- **THEN** parsing SHALL complete in under 50ms
- **AND** all 8 ranges SHALL be correctly identified

#### Scenario: JSONC file with no markers

- **GIVEN** a JSONC file with 1,500 lines and no uneditable markers
- **WHEN** the file is parsed
- **THEN** parsing SHALL complete quickly (under 30ms)
- **AND** an empty range list SHALL be returned
- **AND** no false positive detections SHALL occur

#### Scenario: File with many comments but no markers

- **GIVEN** a JSONC file with 200 comment lines but no uneditable markers
- **WHEN** the file is parsed
- **THEN** parsing SHALL scan all comments efficiently
- **AND** no protected ranges SHALL be created
- **AND** performance SHALL remain acceptable (under 40ms)
