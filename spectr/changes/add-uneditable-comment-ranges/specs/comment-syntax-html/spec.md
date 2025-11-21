# comment-syntax-html Specification

## Purpose

Define HTML-specific comment syntax detection for uneditable range markers, supporting HTML comments (`<!-- -->`).

## ADDED Requirements

### Requirement: HTML Comment Detection

The system SHALL recognize uneditable markers within HTML comments (`<!-- -->`).

#### Scenario: Single-line comment with marker

- **GIVEN** an HTML file with:
  ```html
  <!-- conclaude-uneditable:start -->
  ```
- **WHEN** the file is parsed for uneditable markers
- **THEN** the start marker SHALL be detected
- **AND** the marker line SHALL be recorded as the start of a protected range

#### Scenario: Multi-line comment with marker

- **GIVEN** an HTML file with:
  ```html
  <!--
    conclaude-uneditable:start
    Protected content below
  -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be detected within the multi-line comment
- **AND** the range SHALL begin at the line containing the marker text

#### Scenario: Comment with trailing content

- **GIVEN** an HTML file with:
  ```html
  <!-- conclaude-uneditable:start Generated markup -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be detected (trailing content ignored)
- **AND** the range SHALL begin at this line

#### Scenario: Inline comment with marker

- **GIVEN** an HTML file with:
  ```html
  <div><!-- conclaude-uneditable:start --></div>
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be detected even when inline with other content
- **AND** the range SHALL start at this line

### Requirement: File Extension Mapping

The system SHALL detect HTML files by their file extension and apply HTML comment syntax rules.

#### Scenario: .html file extension

- **GIVEN** a file named "index.html"
- **WHEN** the file is processed for uneditable ranges
- **THEN** HTML comment syntax rules SHALL be applied
- **AND** markers within `<!-- -->` comments SHALL be detected

#### Scenario: .htm file extension

- **GIVEN** a file named "page.htm"
- **WHEN** the file is processed for uneditable ranges
- **THEN** HTML comment syntax rules SHALL be applied
- **AND** markers SHALL be detected correctly

#### Scenario: .html file with markers

- **GIVEN** a file "template.html" containing:
  ```html
  <!-- conclaude-uneditable:start -->
  <!-- Auto-generated navigation -->
  <nav>
    <a href="/">Home</a>
  </nav>
  <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** a protected range from line 1 to line 6 SHALL be created
- **AND** edits overlapping this range SHALL be blocked

### Requirement: Marker Format Validation

The system SHALL only recognize correctly formatted markers within HTML comments.

#### Scenario: Marker in string attribute (not detected)

- **GIVEN** an HTML file with:
  ```html
  <div data-comment="<!-- conclaude-uneditable:start -->"></div>
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL NOT be detected (inside attribute, not comment)
- **AND** no protected range SHALL be created

#### Scenario: Marker in script tag (not detected)

- **GIVEN** an HTML file with:
  ```html
  <script>
    const html = "<!-- conclaude-uneditable:start -->";
  </script>
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL NOT be detected (inside script, not HTML comment)
- **AND** no protected range SHALL be created

#### Scenario: Marker in style tag (not detected)

- **GIVEN** an HTML file with:
  ```html
  <style>
    /* <!-- conclaude-uneditable:start --> */
  </style>
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL NOT be detected (inside style, not HTML comment)
- **AND** no protected range SHALL be created

#### Scenario: Correctly formatted marker in HTML comment

- **GIVEN** an HTML file with:
  ```html
  <!-- conclaude-uneditable:start -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be recognized
- **AND** a protected range SHALL start at this line

### Requirement: Nested Range Handling

The system SHALL handle nested ranges within HTML structure.

#### Scenario: Nested section protection

- **GIVEN** an HTML file with:
  ```html
  <!-- conclaude-uneditable:start -->  <!-- Line 1 -->
  <header>
    <!-- conclaude-uneditable:start -->  <!-- Line 3 -->
    <nav>
      <a href="/">Home</a>
    </nav>
    <!-- conclaude-uneditable:end -->  <!-- Line 7 -->
  </header>
  <!-- conclaude-uneditable:end -->  <!-- Line 9 -->
  ```
- **WHEN** the file is parsed
- **THEN** two ranges SHALL be created: (1-9) and (3-7)
- **AND** both ranges SHALL be marked as protected
- **AND** edits overlapping either range SHALL be blocked

#### Scenario: Multiple non-nested ranges

- **GIVEN** an HTML file with:
  ```html
  <!-- conclaude-uneditable:start -->
  <header></header>
  <!-- conclaude-uneditable:end -->
  <main>Content</main>
  <!-- conclaude-uneditable:start -->
  <footer></footer>
  <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** two separate ranges SHALL be created
- **AND** both ranges SHALL be protected independently
- **AND** content between ranges SHALL remain editable

### Requirement: Edge Case Handling

The system SHALL gracefully handle edge cases in HTML comment syntax.

#### Scenario: Multi-line comment with marker not on first line

- **GIVEN** an HTML file with:
  ```html
  <!--
    This is a comment
    conclaude-uneditable:start
    Protected section
  -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be detected on line 3
- **AND** the range SHALL begin at line 3

#### Scenario: Conditional comment (IE-specific)

- **GIVEN** an HTML file with:
  ```html
  <!--[if IE]>
  <!-- conclaude-uneditable:start -->
  <div>IE only content</div>
  <!-- conclaude-uneditable:end -->
  <![endif]-->
  ```
- **WHEN** the file is parsed
- **THEN** markers SHALL be detected within conditional comment block
- **AND** a protected range SHALL be created

#### Scenario: Marker at file start

- **GIVEN** an HTML file with marker at line 1:
  ```html
  <!-- conclaude-uneditable:start -->
  <!DOCTYPE html>
  <html>
  <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the range SHALL start at line 1
- **AND** the range SHALL end at line 4

#### Scenario: Marker at file end

- **GIVEN** an HTML file ending with:
  ```html
  </body>
  <!-- conclaude-uneditable:start -->
  <!-- Auto-generated footer -->
  </html>
  <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the range SHALL end at the last line of the file
- **AND** the range SHALL be valid

#### Scenario: Empty lines between markers

- **GIVEN** an HTML file with:
  ```html
  <!-- conclaude-uneditable:start -->


  <div>Protected content</div>


  <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the range SHALL include all lines between markers (including empty lines)
- **AND** edits within the range SHALL be blocked

#### Scenario: Comment with leading/trailing whitespace

- **GIVEN** an HTML file with:
  ```html
      <!-- conclaude-uneditable:start -->
  <div>Protected</div>
      <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** both markers SHALL be detected regardless of indentation
- **AND** a protected range SHALL be created

#### Scenario: Marker in HTML head section

- **GIVEN** an HTML file with:
  ```html
  <head>
    <!-- conclaude-uneditable:start -->
    <meta charset="UTF-8">
    <title>Protected Title</title>
    <!-- conclaude-uneditable:end -->
  </head>
  ```
- **WHEN** the file is parsed
- **THEN** the range SHALL protect the meta tags and title
- **AND** edits SHALL be blocked correctly

### Requirement: Performance Characteristics

The system SHALL efficiently parse HTML files for uneditable markers.

#### Scenario: Large HTML file with multiple markers

- **GIVEN** an HTML file with 8,000 lines and 12 protected ranges
- **WHEN** the file is parsed for markers
- **THEN** parsing SHALL complete in under 80ms
- **AND** all 12 ranges SHALL be correctly identified

#### Scenario: HTML file with no markers

- **GIVEN** an HTML file with 3,000 lines and no uneditable markers
- **WHEN** the file is parsed
- **THEN** parsing SHALL complete quickly (under 40ms)
- **AND** an empty range list SHALL be returned
- **AND** no false positive detections SHALL occur

#### Scenario: File with many comments but no markers

- **GIVEN** an HTML file with 200 HTML comments but no uneditable markers
- **WHEN** the file is parsed
- **THEN** parsing SHALL scan all comments efficiently
- **AND** no protected ranges SHALL be created
- **AND** performance SHALL remain acceptable (under 50ms)
