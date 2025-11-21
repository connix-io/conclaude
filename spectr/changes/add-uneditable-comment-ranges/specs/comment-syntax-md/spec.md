# comment-syntax-md Specification

## Purpose

Define Markdown-specific comment syntax detection for uneditable range markers, supporting HTML-style comments (`<!-- -->`), handling edge cases like code blocks, inline code, and YAML frontmatter, and supporting `.md`, `.markdown`, and `.mdx` file extensions.

## ADDED Requirements

### Requirement: HTML Comment Detection

The system SHALL recognize uneditable markers within HTML-style comments (`<!-- -->`).

#### Scenario: Single-line HTML comment with marker

- **GIVEN** a Markdown file with:
  ```markdown
  <!-- conclaude-uneditable:start -->
  ```
- **WHEN** the file is parsed for uneditable markers
- **THEN** the start marker SHALL be detected
- **AND** the marker line SHALL be recorded as the start of a protected range

#### Scenario: Multi-line HTML comment with marker

- **GIVEN** a Markdown file with:
  ```markdown
  <!--
    conclaude-uneditable:start
    Generated content below
  -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be detected on line 2
- **AND** the range SHALL begin at line 2

#### Scenario: HTML comment with trailing content

- **GIVEN** a Markdown file with:
  ```markdown
  <!-- conclaude-uneditable:start - Auto-generated section -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be detected (trailing content ignored)
- **AND** the range SHALL begin at this line

#### Scenario: HTML comment with start and end markers

- **GIVEN** a Markdown file with:
  ```markdown
  <!-- conclaude-uneditable:start -->
  ## Generated Documentation
  This is auto-generated content.
  <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** both markers SHALL be detected
- **AND** a protected range from line 1 to line 4 SHALL be created

### Requirement: File Extension Mapping

The system SHALL detect Markdown files by their file extensions and apply HTML comment syntax rules.

#### Scenario: .md file extension

- **GIVEN** a file named "README.md"
- **WHEN** the file is processed for uneditable ranges
- **THEN** HTML comment syntax rules SHALL be applied
- **AND** markers within `<!-- -->` comments SHALL be detected

#### Scenario: .markdown file extension

- **GIVEN** a file named "CHANGELOG.markdown"
- **WHEN** the file is processed for uneditable ranges
- **THEN** HTML comment syntax rules SHALL be applied
- **AND** markers within `<!-- -->` comments SHALL be detected

#### Scenario: .mdx file extension

- **GIVEN** a file named "component.mdx" (MDX = Markdown + JSX)
- **WHEN** the file is processed for uneditable ranges
- **THEN** HTML comment syntax rules SHALL be applied
- **AND** markers within `<!-- -->` comments SHALL be detected
- **AND** markers in JSX comments SHALL NOT be detected (only HTML-style comments)

### Requirement: Marker Format Validation

The system SHALL only recognize correctly formatted markers within HTML comments, NOT within code blocks or inline code.

#### Scenario: Marker in fenced code block (not detected)

- **GIVEN** a Markdown file with:
  ````markdown
  ```
  <!-- conclaude-uneditable:start -->
  This is example code
  ```
  ````
- **WHEN** the file is parsed
- **THEN** the marker SHALL NOT be detected (inside code block)
- **AND** no protected range SHALL be created

#### Scenario: Marker in inline code (not detected)

- **GIVEN** a Markdown file with:
  ```markdown
  Use `<!-- conclaude-uneditable:start -->` to mark sections.
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL NOT be detected (inside inline code)
- **AND** no protected range SHALL be created

#### Scenario: Marker in indented code block (not detected)

- **GIVEN** a Markdown file with:
  ```markdown
      <!-- conclaude-uneditable:start -->
      function example() {}
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL NOT be detected (4-space indent = code block)
- **AND** no protected range SHALL be created

#### Scenario: Correctly formatted marker in HTML comment

- **GIVEN** a Markdown file with:
  ```markdown
  <!-- conclaude-uneditable:start -->
  # Protected Section
  <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** both markers SHALL be recognized
- **AND** a protected range SHALL be created

### Requirement: Nested Range Handling

The system SHALL handle nested ranges within Markdown content structure.

#### Scenario: Nested ranges in sections

- **GIVEN** a Markdown file with:
  ```markdown
  <!-- conclaude-uneditable:start -->  # Line 1
  # Outer Section
  ## Subsection
  <!-- conclaude-uneditable:start -->  # Line 4
  ### Protected Subsection
  Content here
  <!-- conclaude-uneditable:end -->    # Line 7
  ## Another Section
  <!-- conclaude-uneditable:end -->    # Line 9
  ```
- **WHEN** the file is parsed
- **THEN** two ranges SHALL be created: (1-9) and (4-7)
- **AND** both ranges SHALL be marked as protected
- **AND** edits overlapping either range SHALL be blocked

#### Scenario: Nested ranges in list items

- **GIVEN** a Markdown file with:
  ```markdown
  <!-- conclaude-uneditable:start -->
  - Item 1
    <!-- conclaude-uneditable:start -->
    - Nested protected item
    <!-- conclaude-uneditable:end -->
  - Item 2
  <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** two nested ranges SHALL be created
- **AND** both ranges SHALL be protected

### Requirement: Markdown-Specific Handling

The system SHALL correctly handle Markdown-specific constructs including frontmatter, code blocks, and inline code.

#### Scenario: YAML frontmatter with HTML comment marker

- **GIVEN** a Markdown file with:
  ```markdown
  ---
  title: "Generated Document"
  # YAML comments do NOT use HTML syntax
  ---
  <!-- conclaude-uneditable:start -->
  # Content
  <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** only the HTML comment markers SHALL be detected (lines 5-7)
- **AND** YAML comment syntax SHALL be ignored
- **AND** a protected range from line 5 to line 7 SHALL be created

#### Scenario: Marker before and after code fence

- **GIVEN** a Markdown file with:
  ```markdown
  <!-- conclaude-uneditable:start -->
  ```javascript
  // This is code
  ```
  <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** both markers SHALL be detected (outside code block)
- **AND** a protected range SHALL include the code fence
- **AND** content inside the fenced block SHALL be protected

#### Scenario: Marker in blockquote

- **GIVEN** a Markdown file with:
  ```markdown
  > <!-- conclaude-uneditable:start -->
  > This is a protected quote.
  > <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** both markers SHALL be detected (HTML comments work in blockquotes)
- **AND** a protected range SHALL be created

#### Scenario: Mixed content with code and text

- **GIVEN** a Markdown file with:
  ```markdown
  <!-- conclaude-uneditable:start -->
  ## API Reference

  Use `api.call()` to invoke the API.

  ```javascript
  api.call({ param: 'value' });
  ```
  <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** markers SHALL be detected
- **AND** the entire range including inline code and code blocks SHALL be protected

#### Scenario: Language-specific code block with marker inside (not detected)

- **GIVEN** a Markdown file with:
  ````markdown
  ```html
  <!-- conclaude-uneditable:start -->
  <div>Example HTML</div>
  ```
  ````
- **WHEN** the file is parsed
- **THEN** the marker SHALL NOT be detected (inside fenced code block)
- **AND** no protected range SHALL be created

#### Scenario: Marker in table

- **GIVEN** a Markdown file with:
  ```markdown
  | Column 1 | Column 2 |
  |----------|----------|
  | `<!-- conclaude-uneditable:start -->` | Example |
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL NOT be detected (inside inline code within table)
- **AND** no protected range SHALL be created

### Requirement: Performance Characteristics

The system SHALL efficiently parse Markdown files for uneditable markers.

#### Scenario: Large Markdown file with multiple markers

- **GIVEN** a Markdown file with 8,000 lines and 12 protected ranges
- **WHEN** the file is parsed for markers
- **THEN** parsing SHALL complete in under 70ms
- **AND** all 12 ranges SHALL be correctly identified
- **AND** code blocks SHALL be properly excluded from detection

#### Scenario: Markdown file with no markers

- **GIVEN** a Markdown file with 3,000 lines and no uneditable markers
- **WHEN** the file is parsed
- **THEN** parsing SHALL complete quickly (under 35ms)
- **AND** an empty range list SHALL be returned
- **AND** no false positive detections SHALL occur (code blocks properly ignored)
