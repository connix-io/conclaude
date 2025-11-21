# comment-syntax-tsx Specification

## Purpose

Define TSX-specific comment syntax detection for uneditable range markers, supporting line comments (`//`), block comments (`/* */`), and JSX comments (`{/* */}`).

## ADDED Requirements

### Requirement: Line Comment Detection

The system SHALL recognize uneditable markers within TSX line comments (`//`).

#### Scenario: Single-line comment with marker

- **GIVEN** a TSX file with:
  ```tsx
  // <!-- conclaude-uneditable:start -->
  ```
- **WHEN** the file is parsed for uneditable markers
- **THEN** the start marker SHALL be detected
- **AND** the marker line SHALL be recorded as the start of a protected range

#### Scenario: Line comment with trailing code comment

- **GIVEN** a TSX file with:
  ```tsx
  // <!-- conclaude-uneditable:start --> Generated component props
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be detected (trailing content ignored)
- **AND** the range SHALL begin at this line

### Requirement: Block Comment Detection

The system SHALL recognize uneditable markers within TSX block comments (`/* */`).

#### Scenario: Block comment with marker

- **GIVEN** a TSX file with:
  ```tsx
  /* <!-- conclaude-uneditable:start --> */
  ```
- **WHEN** the file is parsed
- **THEN** the start marker SHALL be detected within the block comment
- **AND** the range SHALL begin at this line

#### Scenario: JSDoc-style comment with marker

- **GIVEN** a TSX file with:
  ```tsx
  /**
   * <!-- conclaude-uneditable:start -->
   * @generated
   */
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be detected on line 2
- **AND** the range SHALL begin at line 2

### Requirement: JSX Comment Detection

The system SHALL recognize uneditable markers within JSX comments (`{/* */}`).

#### Scenario: JSX comment with marker in component

- **GIVEN** a TSX file with:
  ```tsx
  {/* <!-- conclaude-uneditable:start --> */}
  <Component />
  {/* <!-- conclaude-uneditable:end --> */}
  ```
- **WHEN** the file is parsed
- **THEN** both markers SHALL be detected within `{/* */}` comments
- **AND** a protected range SHALL be created

#### Scenario: JSX comment with marker inside JSX element

- **GIVEN** a TSX file with:
  ```tsx
  <div>
    {/* <!-- conclaude-uneditable:start --> */}
    <span>Generated content</span>
    {/* <!-- conclaude-uneditable:end --> */}
  </div>
  ```
- **WHEN** the file is parsed
- **THEN** both markers SHALL be detected
- **AND** a protected range SHALL be created for lines 2-4

#### Scenario: JSX comment with marker and description

- **GIVEN** a TSX file with:
  ```tsx
  {/* <!-- conclaude-uneditable:start --> Auto-generated form fields */}
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be detected (trailing content ignored)
- **AND** the range SHALL begin at this line

#### Scenario: Multiple JSX comments in component tree

- **GIVEN** a TSX file with:
  ```tsx
  return (
    <>
      {/* <!-- conclaude-uneditable:start --> */}
      <Header />
      {/* <!-- conclaude-uneditable:end --> */}
      {/* <!-- conclaude-uneditable:start --> */}
      <Footer />
      {/* <!-- conclaude-uneditable:end --> */}
    </>
  );
  ```
- **WHEN** the file is parsed
- **THEN** four markers SHALL be detected
- **AND** two protected ranges SHALL be created

### Requirement: File Extension Mapping

The system SHALL detect TSX files and apply TSX comment syntax rules.

#### Scenario: .tsx file extension

- **GIVEN** a file named "Component.tsx"
- **WHEN** the file is processed for uneditable ranges
- **THEN** TSX comment syntax rules SHALL be applied
- **AND** markers within `//`, `/* */`, or `{/* */}` comments SHALL be detected

#### Scenario: .tsx file with React components

- **GIVEN** a file named "App.tsx" containing React components
- **WHEN** the file is processed for uneditable ranges
- **THEN** all three comment types SHALL be recognized
- **AND** markers SHALL be detected in any comment format

### Requirement: Marker Format Validation

The system SHALL only recognize correctly formatted markers within TSX comments.

#### Scenario: Marker in string literal (not detected)

- **GIVEN** a TSX file with:
  ```tsx
  const marker = "// <!-- conclaude-uneditable:start -->";
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL NOT be detected (inside string, not comment)
- **AND** no protected range SHALL be created

#### Scenario: Marker in template literal (not detected)

- **GIVEN** a TSX file with:
  ```tsx
  const html = `<!-- conclaude-uneditable:start -->`;
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL NOT be detected (inside template literal)
- **AND** no protected range SHALL be created

#### Scenario: Marker in JSX attribute string (not detected)

- **GIVEN** a TSX file with:
  ```tsx
  <div data-marker="<!-- conclaude-uneditable:start -->" />
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL NOT be detected (inside JSX attribute)
- **AND** no protected range SHALL be created

#### Scenario: Correctly formatted marker in TSX comment

- **GIVEN** a TSX file with:
  ```tsx
  {/* <!-- conclaude-uneditable:start --> */}
  <GeneratedComponent />
  {/* <!-- conclaude-uneditable:end --> */}
  ```
- **WHEN** the file is parsed
- **THEN** both markers SHALL be recognized
- **AND** a protected range SHALL be created

### Requirement: Nested Range Handling

The system SHALL handle nested ranges within TSX component structure.

#### Scenario: Nested component protection

- **GIVEN** a TSX file with:
  ```tsx
  // <!-- conclaude-uneditable:start -->  // Line 1
  function FormComponent() {
    return (
      {/* <!-- conclaude-uneditable:start --> */}  // Line 4
      <input type="text" />
      {/* <!-- conclaude-uneditable:end --> */}  // Line 6
    );
  }
  // <!-- conclaude-uneditable:end -->  // Line 9
  ```
- **WHEN** the file is parsed
- **THEN** two ranges SHALL be created: (1-9) and (4-6)
- **AND** both ranges SHALL be marked as protected
- **AND** edits overlapping either range SHALL be blocked

#### Scenario: Inline comment after JSX code

- **GIVEN** a TSX file with:
  ```tsx
  <div>  {/* <!-- conclaude-uneditable:start --> */}
    <span>Content</span>
  </div>  {/* <!-- conclaude-uneditable:end --> */}
  ```
- **WHEN** the file is parsed
- **THEN** both markers SHALL be detected (inline JSX comments are valid)
- **AND** a protected range from line 1 to line 3 SHALL be created

### Requirement: Performance Characteristics

The system SHALL efficiently parse TSX files for uneditable markers.

#### Scenario: Large TSX file with multiple markers

- **GIVEN** a TSX file with 6,000 lines and 10 protected ranges using mixed comment types
- **WHEN** the file is parsed for markers
- **THEN** parsing SHALL complete in under 80ms
- **AND** all 10 ranges SHALL be correctly identified

#### Scenario: TSX file with no markers

- **GIVEN** a TSX file with 2,500 lines and no uneditable markers
- **WHEN** the file is parsed
- **THEN** parsing SHALL complete quickly (under 40ms)
- **AND** an empty range list SHALL be returned
- **AND** no false positive detections SHALL occur
