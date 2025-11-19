# comment-syntax-javascript Specification

## Purpose

Define JavaScript/TypeScript-specific comment syntax detection for uneditable range markers, supporting both line comments (`//`) and block comments (`/* */`).

## ADDED Requirements

### Requirement: Line Comment Detection

The system SHALL recognize uneditable markers within JavaScript line comments (`//`).

#### Scenario: Single-line comment with marker

- **GIVEN** a JavaScript file with:
  ```javascript
  // <!-- conclaude-uneditable:start -->
  ```
- **WHEN** the file is parsed for uneditable markers
- **THEN** the start marker SHALL be detected
- **AND** the marker line SHALL be recorded as the start of a protected range

#### Scenario: Line comment with trailing code comment

- **GIVEN** a JavaScript file with:
  ```javascript
  // <!-- conclaude-uneditable:start --> Auto-generated code below
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be detected (trailing content ignored)
- **AND** the range SHALL begin at this line

### Requirement: Block Comment Detection

The system SHALL recognize uneditable markers within JavaScript block comments (`/* */`).

#### Scenario: Block comment with marker

- **GIVEN** a JavaScript file with:
  ```javascript
  /* <!-- conclaude-uneditable:start --> */
  ```
- **WHEN** the file is parsed
- **THEN** the start marker SHALL be detected within the block comment
- **AND** the range SHALL begin at this line

#### Scenario: JSDoc-style comment with marker

- **GIVEN** a JavaScript file with:
  ```javascript
  /**
   * <!-- conclaude-uneditable:start -->
   * @generated
   */
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be detected on line 2
- **AND** the range SHALL begin at line 2

### Requirement: File Extension Mapping

The system SHALL detect JavaScript and TypeScript files and apply JavaScript comment syntax rules.

#### Scenario: .js file extension

- **GIVEN** a file named "app.js"
- **WHEN** the file is processed for uneditable ranges
- **THEN** JavaScript comment syntax rules SHALL be applied
- **AND** markers within `//` or `/* */` comments SHALL be detected

#### Scenario: .ts file extension (TypeScript)

- **GIVEN** a file named "service.ts"
- **WHEN** the file is processed for uneditable ranges
- **THEN** JavaScript comment syntax rules SHALL be applied (TypeScript uses same syntax)
- **AND** markers SHALL be detected correctly

#### Scenario: .jsx and .tsx file extensions

- **GIVEN** files named "Component.jsx" and "View.tsx"
- **WHEN** the files are processed for uneditable ranges
- **THEN** JavaScript comment syntax rules SHALL be applied
- **AND** markers SHALL be detected in both files

#### Scenario: .mjs and .cjs file extensions (ES modules)

- **GIVEN** files named "module.mjs" and "common.cjs"
- **WHEN** the files are processed for uneditable ranges
- **THEN** JavaScript comment syntax rules SHALL be applied
- **AND** markers SHALL be detected correctly

### Requirement: Marker Format Validation

The system SHALL only recognize correctly formatted markers within JavaScript comments.

#### Scenario: Marker in string literal (not detected)

- **GIVEN** a JavaScript file with:
  ```javascript
  const marker = "// <!-- conclaude-uneditable:start -->";
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL NOT be detected (inside string, not comment)
- **AND** no protected range SHALL be created

#### Scenario: Marker in template literal (not detected)

- **GIVEN** a JavaScript file with:
  ```javascript
  const html = `<!-- conclaude-uneditable:start -->`;
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL NOT be detected (inside template literal)
- **AND** no protected range SHALL be created

#### Scenario: Correctly formatted marker in comment

- **GIVEN** a JavaScript file with:
  ```javascript
  // <!-- conclaude-uneditable:start -->
  function generatedFunction() {}
  // <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** both markers SHALL be recognized
- **AND** a protected range SHALL be created

### Requirement: Nested Range Handling

The system SHALL handle nested ranges within JavaScript code structure.

#### Scenario: Nested class method protection

- **GIVEN** a JavaScript file with:
  ```javascript
  // <!-- conclaude-uneditable:start -->  // Line 1
  class APIClient {
      // <!-- conclaude-uneditable:start -->  // Line 3
      authenticate() {
          return true;
      }
      // <!-- conclaude-uneditable:end -->  // Line 7
  }
  // <!-- conclaude-uneditable:end -->  // Line 9
  ```
- **WHEN** the file is parsed
- **THEN** two ranges SHALL be created: (1-9) and (3-7)
- **AND** both ranges SHALL be marked as protected
- **AND** edits overlapping either range SHALL be blocked

### Requirement: Edge Case Handling

The system SHALL gracefully handle edge cases in JavaScript comment syntax.

#### Scenario: Inline comment after code

- **GIVEN** a JavaScript file with:
  ```javascript
  const x = 5; // <!-- conclaude-uneditable:start -->
  const y = 10;
  const z = 15; // <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** both markers SHALL be detected (inline comments are valid)
- **AND** a protected range from line 1 to line 3 SHALL be created

#### Scenario: Marker in JSX comment (special syntax)

- **GIVEN** a JSX file with:
  ```jsx
  {/* <!-- conclaude-uneditable:start --> */}
  <Component />
  {/* <!-- conclaude-uneditable:end --> */}
  ```
- **WHEN** the file is parsed
- **THEN** the markers SHALL be detected within `{/* */}` comments
- **AND** a protected range SHALL be created

### Requirement: Performance Characteristics

The system SHALL efficiently parse JavaScript/TypeScript files for uneditable markers.

#### Scenario: Large JavaScript file with multiple markers

- **GIVEN** a JavaScript file with 8,000 lines and 12 protected ranges
- **WHEN** the file is parsed for markers
- **THEN** parsing SHALL complete in under 80ms
- **AND** all 12 ranges SHALL be correctly identified

#### Scenario: JavaScript file with no markers

- **GIVEN** a JavaScript file with 3,000 lines and no uneditable markers
- **WHEN** the file is parsed
- **THEN** parsing SHALL complete quickly (under 40ms)
- **AND** an empty range list SHALL be returned
- **AND** no false positive detections SHALL occur
