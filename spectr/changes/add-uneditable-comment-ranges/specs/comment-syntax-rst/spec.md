# comment-syntax-rst Specification

## Purpose

Define reStructuredText (RST)-specific comment syntax detection for uneditable range markers, supporting RST comment blocks (`..`).

## ADDED Requirements

### Requirement: RST Comment Block Detection

The system SHALL recognize uneditable markers within RST comment blocks (`..`).

#### Scenario: Comment block with marker on same line

- **GIVEN** an RST file with:
  ```rst
  .. <!-- conclaude-uneditable:start -->
  ```
- **WHEN** the file is parsed for uneditable markers
- **THEN** the start marker SHALL be detected
- **AND** the marker line SHALL be recorded as the start of a protected range

#### Scenario: Comment block with marker on indented line

- **GIVEN** an RST file with:
  ```rst
  ..
     <!-- conclaude-uneditable:start -->
     This is a protected section
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be detected on the indented line
- **AND** the range SHALL begin at that line

#### Scenario: Inline comment with marker

- **GIVEN** an RST file with:
  ```rst
  .. <!-- conclaude-uneditable:start --> Protected content below
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be detected
- **AND** the range SHALL begin at this line

### Requirement: File Extension Mapping

The system SHALL detect RST files by their file extension and apply RST comment syntax rules.

#### Scenario: .rst file extension

- **GIVEN** a file named "documentation.rst"
- **WHEN** the file is processed for uneditable ranges
- **THEN** RST comment syntax rules SHALL be applied
- **AND** markers within `..` comments SHALL be detected

#### Scenario: .rest file extension

- **GIVEN** a file named "README.rest"
- **WHEN** the file is processed for uneditable ranges
- **THEN** RST comment syntax rules SHALL be applied
- **AND** markers SHALL be detected correctly

#### Scenario: .rst file with markers

- **GIVEN** a file "api.rst" containing:
  ```rst
  API Documentation
  =================

  .. <!-- conclaude-uneditable:start -->

  Generated API Reference
  -----------------------

  .. automodule:: myapp.api
     :members:

  .. <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** a protected range from line 4 to line 12 SHALL be created
- **AND** edits overlapping this range SHALL be blocked

### Requirement: Marker Format Validation

The system SHALL only recognize correctly formatted markers within RST comments.

#### Scenario: Marker in code block (not detected)

- **GIVEN** an RST file with:
  ```rst
  ::

      .. <!-- conclaude-uneditable:start -->
      code content
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL NOT be detected (inside code block, not comment)
- **AND** no protected range SHALL be created

#### Scenario: Marker in literal block (not detected)

- **GIVEN** an RST file with:
  ```rst
  Example::

    .. <!-- conclaude-uneditable:start -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL NOT be detected (inside literal block)
- **AND** no protected range SHALL be created

#### Scenario: Correctly formatted marker in comment

- **GIVEN** an RST file with:
  ```rst
  .. <!-- conclaude-uneditable:start -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be recognized
- **AND** a protected range SHALL start at this line

### Requirement: Nested Range Handling

The system SHALL handle nested ranges within RST document structure.

#### Scenario: Nested section protection

- **GIVEN** an RST file with:
  ```rst
  .. <!-- conclaude-uneditable:start -->  Line 1

  Chapter 1
  =========

  .. <!-- conclaude-uneditable:start -->  Line 6
  Section 1.1
  -----------

  Protected content.
  .. <!-- conclaude-uneditable:end -->  Line 11

  .. <!-- conclaude-uneditable:end -->  Line 13
  ```
- **WHEN** the file is parsed
- **THEN** two ranges SHALL be created: (1-13) and (6-11)
- **AND** both ranges SHALL be marked as protected
- **AND** edits overlapping either range SHALL be blocked

### Requirement: Edge Case Handling

The system SHALL gracefully handle edge cases in RST comment syntax.

#### Scenario: Marker at file start

- **GIVEN** an RST file with marker at line 1:
  ```rst
  .. <!-- conclaude-uneditable:start -->

  Document Title
  ==============

  .. <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the range SHALL start at line 1
- **AND** the range SHALL end at line 6

#### Scenario: Marker at file end

- **GIVEN** an RST file ending with:
  ```rst
  Final Section
  =============

  Content here.

  .. <!-- conclaude-uneditable:start -->
  .. Generated footer
  .. <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the range SHALL end at the last line of the file
- **AND** the range SHALL be valid

#### Scenario: Empty lines between markers

- **GIVEN** an RST file with:
  ```rst
  .. <!-- conclaude-uneditable:start -->



  Protected Section
  -----------------



  .. <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the range SHALL include all lines between markers (including empty lines)
- **AND** edits within the range SHALL be blocked

### Requirement: Sphinx Directive Compatibility

The system SHALL correctly handle RST files with Sphinx directives.

#### Scenario: Sphinx autodoc directive with marker

- **GIVEN** an RST file with:
  ```rst
  .. <!-- conclaude-uneditable:start -->
  .. automodule:: mypackage
     :members:
     :undoc-members:
  .. <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 1 SHALL be detected
- **AND** a protected range from line 1 to line 5 SHALL be created

#### Scenario: Sphinx toctree with marker

- **GIVEN** an RST file with:
  ```rst
  .. <!-- conclaude-uneditable:start -->
  .. toctree::
     :maxdepth: 2

     intro
     tutorial
  .. <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 1 SHALL be detected
- **AND** a protected range from line 1 to line 7 SHALL be created

### Requirement: RST-Specific Syntax Handling

The system SHALL handle RST-specific patterns and constructs.

#### Scenario: Admonition with marker

- **GIVEN** an RST file with:
  ```rst
  .. <!-- conclaude-uneditable:start -->
  .. note::
     This is an important note.
     It spans multiple lines.
  .. <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 1 SHALL be detected
- **AND** a protected range from line 1 to line 5 SHALL be created

#### Scenario: Table directive with marker

- **GIVEN** an RST file with:
  ```rst
  .. <!-- conclaude-uneditable:start -->
  .. list-table:: Generated Table
     :header-rows: 1

     * - Column 1
       - Column 2
     * - Value 1
       - Value 2
  .. <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 1 SHALL be detected
- **AND** a protected range from line 1 to line 9 SHALL be created

#### Scenario: Code block directive with marker

- **GIVEN** an RST file with:
  ```rst
  .. <!-- conclaude-uneditable:start -->
  .. code-block:: python
     :linenos:

     def example():
         return True
  .. <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 1 SHALL be detected
- **AND** a protected range from line 1 to line 7 SHALL be created

#### Scenario: Reference with marker

- **GIVEN** an RST file with:
  ```rst
  .. <!-- conclaude-uneditable:start -->
  .. _my-reference:

  Referenced Section
  ------------------
  .. <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 1 SHALL be detected
- **AND** a protected range from line 1 to line 6 SHALL be created

### Requirement: Multi-line Comment Block Handling

The system SHALL handle multi-line RST comment blocks correctly.

#### Scenario: Multi-line comment with marker

- **GIVEN** an RST file with:
  ```rst
  ..
     <!-- conclaude-uneditable:start -->
     This is a multi-line comment
     that continues here
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be detected on line 2
- **AND** the range SHALL begin at line 2

#### Scenario: Comment block with blank line separator

- **GIVEN** an RST file with:
  ```rst
  .. <!-- conclaude-uneditable:start -->

  Content starts here.

  .. <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** both markers SHALL be detected
- **AND** a protected range from line 1 to line 5 SHALL be created

### Requirement: Substitution and Link Compatibility

The system SHALL handle RST substitutions and links with markers.

#### Scenario: Substitution definition with marker

- **GIVEN** an RST file with:
  ```rst
  .. <!-- conclaude-uneditable:start -->
  .. |version| replace:: 1.0.0
  .. |date| replace:: 2024-01-01
  .. <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 1 SHALL be detected
- **AND** a protected range from line 1 to line 4 SHALL be created

#### Scenario: Hyperlink target with marker

- **GIVEN** an RST file with:
  ```rst
  .. <!-- conclaude-uneditable:start -->
  .. _Python: https://python.org
  .. _Django: https://djangoproject.com
  .. <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 1 SHALL be detected
- **AND** a protected range from line 1 to line 4 SHALL be created

### Requirement: Performance Characteristics

The system SHALL efficiently parse RST files for uneditable markers.

#### Scenario: Large RST file with multiple markers

- **GIVEN** an RST file with 4,000 lines and 8 protected ranges
- **WHEN** the file is parsed for markers
- **THEN** parsing SHALL complete in under 60ms
- **AND** all 8 ranges SHALL be correctly identified

#### Scenario: RST file with no markers

- **GIVEN** an RST file with 2,000 lines and no uneditable markers
- **WHEN** the file is parsed
- **THEN** parsing SHALL complete quickly (under 35ms)
- **AND** an empty range list SHALL be returned
- **AND** no false positive detections SHALL occur

#### Scenario: File with many directives but no markers

- **GIVEN** an RST file with 300 lines of Sphinx directives but no uneditable markers
- **WHEN** the file is parsed
- **THEN** parsing SHALL scan all comments efficiently
- **AND** no protected ranges SHALL be created
- **AND** performance SHALL remain acceptable (under 45ms)
