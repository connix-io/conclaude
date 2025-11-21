# comment-syntax-astro Specification

## Purpose

Define Astro-specific comment syntax detection for uneditable range markers, supporting HTML comments (`<!-- -->`), JavaScript/TypeScript comments (`//` and `/* */`), and JSX comments (`{/* */}`) across frontmatter and component content.

## ADDED Requirements

### Requirement: HTML Comment Detection

The system SHALL recognize uneditable markers within Astro HTML comments (`<!-- -->`).

#### Scenario: HTML comment with marker in template

- **GIVEN** an Astro file with:
  ```astro
  <!-- conclaude-uneditable:start -->
  <div class="generated">Content</div>
  <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed for uneditable markers
- **THEN** both markers SHALL be detected within HTML comments
- **AND** a protected range SHALL be created

#### Scenario: Multi-line HTML comment with marker

- **GIVEN** an Astro file with:
  ```astro
  <!--
    conclaude-uneditable:start
    Auto-generated section
  -->
  <section>...</section>
  <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the start marker SHALL be detected on line 2
- **AND** a protected range SHALL be created

#### Scenario: HTML comment with trailing content

- **GIVEN** an Astro file with:
  ```astro
  <!-- conclaude-uneditable:start Auto-generated components -->
  <Component />
  <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be detected (trailing content ignored)
- **AND** the range SHALL be created correctly

### Requirement: JavaScript/TypeScript Comment Detection

The system SHALL recognize uneditable markers within JavaScript/TypeScript line comments (`//`) and block comments (`/* */`) in Astro frontmatter and component scripts.

#### Scenario: Frontmatter line comment with marker

- **GIVEN** an Astro file with:
  ```astro
  ---
  // <!-- conclaude-uneditable:start -->
  const apiKey = "generated-key";
  // <!-- conclaude-uneditable:end -->
  ---
  ```
- **WHEN** the file is parsed
- **THEN** both markers SHALL be detected in frontmatter line comments
- **AND** a protected range SHALL be created for lines 2-4

#### Scenario: Frontmatter block comment with marker

- **GIVEN** an Astro file with:
  ```astro
  ---
  /* <!-- conclaude-uneditable:start --> */
  interface Props {
    id: number;
  }
  /* <!-- conclaude-uneditable:end --> */
  ---
  ```
- **WHEN** the file is parsed
- **THEN** both markers SHALL be detected in block comments
- **AND** a protected range SHALL be created

#### Scenario: Component script comment with marker

- **GIVEN** an Astro file with:
  ```astro
  ---
  import Component from './Component.astro';
  ---
  <script>
  // <!-- conclaude-uneditable:start -->
  const config = { debug: true };
  // <!-- conclaude-uneditable:end -->
  </script>
  ```
- **WHEN** the file is parsed
- **THEN** markers in the script tag SHALL be detected
- **AND** a protected range SHALL be created

### Requirement: File Extension Mapping

The system SHALL detect Astro files by their file extension and apply Astro-specific comment syntax rules.

#### Scenario: .astro file extension

- **GIVEN** a file named "Component.astro"
- **WHEN** the file is processed for uneditable ranges
- **THEN** Astro comment syntax rules SHALL be applied
- **AND** markers within HTML, JS/TS, and JSX comments SHALL be detected

#### Scenario: .astro file with mixed comment types

- **GIVEN** a file "Page.astro" containing markers in HTML and frontmatter comments
- **WHEN** the file is parsed
- **THEN** all markers SHALL be detected regardless of comment type
- **AND** multiple protected ranges SHALL be created correctly

### Requirement: Marker Format Validation

The system SHALL only recognize correctly formatted markers within valid Astro comments.

#### Scenario: Marker in string literal (not detected)

- **GIVEN** an Astro file with:
  ```astro
  ---
  const marker = "<!-- conclaude-uneditable:start -->";
  ---
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL NOT be detected (inside string, not comment)
- **AND** no protected range SHALL be created

#### Scenario: Marker in HTML attribute (not detected)

- **GIVEN** an Astro file with:
  ```astro
  <div data-comment="<!-- conclaude-uneditable:start -->">
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL NOT be detected (inside attribute value)
- **AND** no protected range SHALL be created

#### Scenario: Marker in template literal (not detected)

- **GIVEN** an Astro file with:
  ```astro
  ---
  const html = `<!-- conclaude-uneditable:start -->`;
  ---
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL NOT be detected (inside template literal)
- **AND** no protected range SHALL be created

#### Scenario: Correctly formatted marker in JSX comment

- **GIVEN** an Astro file with:
  ```astro
  {/* <!-- conclaude-uneditable:start --> */}
  <Component prop={value} />
  {/* <!-- conclaude-uneditable:end --> */}
  ```
- **WHEN** the file is parsed
- **THEN** both markers SHALL be detected within JSX comments
- **AND** a protected range SHALL be created

### Requirement: Nested Range Handling

The system SHALL handle nested ranges within Astro file structure.

#### Scenario: Nested ranges in frontmatter and template

- **GIVEN** an Astro file with:
  ```astro
  ---
  // <!-- conclaude-uneditable:start -->  // Line 2
  const data = fetchData();
  // <!-- conclaude-uneditable:end -->  // Line 4
  ---
  <!-- conclaude-uneditable:start -->  // Line 6
  <div>{data}</div>
  <!-- conclaude-uneditable:end -->  // Line 8
  ```
- **WHEN** the file is parsed
- **THEN** two separate ranges SHALL be created: (2-4) and (6-8)
- **AND** both ranges SHALL be marked as protected
- **AND** edits overlapping either range SHALL be blocked

#### Scenario: Nested ranges within component

- **GIVEN** an Astro file with:
  ```astro
  <!-- conclaude-uneditable:start -->  // Line 1
  <article>
    <!-- conclaude-uneditable:start -->  // Line 3
    <header>Title</header>
    <!-- conclaude-uneditable:end -->  // Line 5
  </article>
  <!-- conclaude-uneditable:end -->  // Line 7
  ```
- **WHEN** the file is parsed
- **THEN** two ranges SHALL be created: (1-7) and (3-5)
- **AND** both ranges SHALL be correctly nested
- **AND** edits overlapping either range SHALL be blocked

### Requirement: Astro-Specific Syntax Handling

The system SHALL correctly handle Astro-specific syntax features when detecting markers.

#### Scenario: Frontmatter delimiters with markers

- **GIVEN** an Astro file with:
  ```astro
  ---
  // <!-- conclaude-uneditable:start -->
  const title = "Generated";
  // <!-- conclaude-uneditable:end -->
  ---
  <h1>{title}</h1>
  ```
- **WHEN** the file is parsed
- **THEN** markers within frontmatter (between ---) SHALL be detected
- **AND** the range SHALL be limited to lines 2-4 (within frontmatter)

#### Scenario: Component import section with markers

- **GIVEN** an Astro file with:
  ```astro
  ---
  // <!-- conclaude-uneditable:start -->
  import Layout from '../layouts/Layout.astro';
  import Card from '../components/Card.astro';
  // <!-- conclaude-uneditable:end -->
  ---
  ```
- **WHEN** the file is parsed
- **THEN** the range SHALL protect the import section
- **AND** edits to imports SHALL be blocked

#### Scenario: JSX expression with markers

- **GIVEN** an Astro file with:
  ```astro
  {/* <!-- conclaude-uneditable:start --> */}
  {items.map(item => (
    <Card title={item.title} />
  ))}
  {/* <!-- conclaude-uneditable:end --> */}
  ```
- **WHEN** the file is parsed
- **THEN** markers in JSX comments SHALL be detected
- **AND** the JSX expression SHALL be protected

#### Scenario: Inline script with markers

- **GIVEN** an Astro file with:
  ```astro
  <script>
    // <!-- conclaude-uneditable:start -->
    document.addEventListener('DOMContentLoaded', () => {
      console.log('Generated event handler');
    });
    // <!-- conclaude-uneditable:end -->
  </script>
  ```
- **WHEN** the file is parsed
- **THEN** markers within inline script tags SHALL be detected
- **AND** the script content SHALL be protected

#### Scenario: Style tag (markers not in CSS comments)

- **GIVEN** an Astro file with:
  ```astro
  <style>
    /* <!-- conclaude-uneditable:start --> */
    .generated { color: red; }
    /* <!-- conclaude-uneditable:end --> */
  </style>
  ```
- **WHEN** the file is parsed
- **THEN** markers in CSS block comments SHALL be detected
- **AND** the style rules SHALL be protected

### Requirement: Performance Characteristics

The system SHALL efficiently parse Astro files for uneditable markers across multiple comment syntaxes.

#### Scenario: Large Astro file with multiple markers

- **GIVEN** an Astro file with 5,000 lines, frontmatter, components, and 10 protected ranges
- **WHEN** the file is parsed for markers
- **THEN** parsing SHALL complete in under 100ms
- **AND** all 10 ranges SHALL be correctly identified across all comment types

#### Scenario: Astro file with no markers

- **GIVEN** an Astro file with 2,000 lines and no uneditable markers
- **WHEN** the file is parsed
- **THEN** parsing SHALL complete quickly (under 50ms)
- **AND** an empty range list SHALL be returned
- **AND** no false positive detections SHALL occur

#### Scenario: Complex Astro file with mixed syntax

- **GIVEN** an Astro file with frontmatter, JSX, HTML, and inline scripts
- **WHEN** the file is parsed with markers in each section
- **THEN** all markers SHALL be detected efficiently
- **AND** parsing SHALL complete in under 80ms
- **AND** ranges SHALL be correctly identified by their comment type context
