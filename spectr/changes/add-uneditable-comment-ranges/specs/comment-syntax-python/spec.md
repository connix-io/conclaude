# comment-syntax-python Specification

## Purpose

Define Python-specific comment syntax detection for uneditable range markers, supporting line comments (`#`) and docstrings.

## ADDED Requirements

### Requirement: Line Comment Detection

The system SHALL recognize uneditable markers within Python line comments (`#`).

#### Scenario: Single-line comment with marker

- **GIVEN** a Python file with:
  ```python
  # <!-- conclaude-uneditable:start -->
  ```
- **WHEN** the file is parsed for uneditable markers
- **THEN** the start marker SHALL be detected
- **AND** the marker line SHALL be recorded as the start of a protected range

#### Scenario: Indented comment with marker

- **GIVEN** a Python file with:
  ```python
  def function():
      # <!-- conclaude-uneditable:start -->
      pass
      # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** both markers SHALL be detected regardless of indentation
- **AND** a protected range from line 2 to line 4 SHALL be created

#### Scenario: Comment with leading whitespace

- **GIVEN** a Python file with:
  ```python
          # <!-- conclaude-uneditable:start -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be detected after trimming leading whitespace
- **AND** the range SHALL begin at this line

### Requirement: File Extension Mapping

The system SHALL detect Python files by their file extension and apply Python comment syntax rules.

#### Scenario: .py file extension

- **GIVEN** a file named "script.py"
- **WHEN** the file is processed for uneditable ranges
- **THEN** Python comment syntax rules SHALL be applied
- **AND** markers within `#` comments SHALL be detected

#### Scenario: .pyw file extension (Windows Python)

- **GIVEN** a file named "gui.pyw"
- **WHEN** the file is processed for uneditable ranges
- **THEN** Python comment syntax rules SHALL be applied
- **AND** markers SHALL be detected correctly

#### Scenario: .py file with markers

- **GIVEN** a file "models.py" containing:
  ```python
  # <!-- conclaude-uneditable:start -->
  # Auto-generated database models
  class User(Model):
      id = IntegerField()
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** a protected range from line 1 to line 5 SHALL be created
- **AND** edits overlapping this range SHALL be blocked

### Requirement: Marker Format Validation

The system SHALL only recognize correctly formatted markers within Python comments.

#### Scenario: Marker in string literal (not detected)

- **GIVEN** a Python file with:
  ```python
  message = "# <!-- conclaude-uneditable:start -->"
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL NOT be detected (inside string, not comment)
- **AND** no protected range SHALL be created

#### Scenario: Marker in docstring (not detected)

- **GIVEN** a Python file with:
  ```python
  """
  <!-- conclaude-uneditable:start -->
  This is a docstring
  """
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL NOT be detected (docstrings are not line comments)
- **AND** no protected range SHALL be created

#### Scenario: Correctly formatted marker in comment

- **GIVEN** a Python file with:
  ```python
  # <!-- conclaude-uneditable:start -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be recognized
- **AND** a protected range SHALL start at this line

### Requirement: Nested Range Handling

The system SHALL handle nested ranges within Python code structure.

#### Scenario: Nested function protection

- **GIVEN** a Python file with:
  ```python
  # <!-- conclaude-uneditable:start -->  # Line 1
  class AuthService:
      # <!-- conclaude-uneditable:start -->  # Line 3
      def verify_token(self, token):
          return True
      # <!-- conclaude-uneditable:end -->  # Line 6
  # <!-- conclaude-uneditable:end -->  # Line 7
  ```
- **WHEN** the file is parsed
- **THEN** two ranges SHALL be created: (1-7) and (3-6)
- **AND** both ranges SHALL be marked as protected
- **AND** edits overlapping either range SHALL be blocked

#### Scenario: Nested ranges with mixed indentation

- **GIVEN** a Python file with nested ranges at different indentation levels
- **WHEN** the file is parsed
- **THEN** all markers SHALL be detected regardless of indentation
- **AND** ranges SHALL be correctly paired by nesting order (not indentation)

### Requirement: Edge Case Handling

The system SHALL gracefully handle edge cases in Python comment syntax.

#### Scenario: Multiple comments before code

- **GIVEN** a Python file with:
  ```python
  # First comment
  # <!-- conclaude-uneditable:start -->
  # Protected section
  def function():
      pass
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the range SHALL start at line 2 (marker line)
- **AND** the range SHALL end at line 6 (end marker line)

#### Scenario: Inline comment (not at line start)

- **GIVEN** a Python file with:
  ```python
  x = 5  # <!-- conclaude-uneditable:start -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be detected (comments can appear after code)
- **AND** the range SHALL start at this line

#### Scenario: Marker at file start

- **GIVEN** a Python file with marker at line 1:
  ```python
  # <!-- conclaude-uneditable:start -->
  import os
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the range SHALL start at line 1
- **AND** the range SHALL end at line 3

#### Scenario: Marker at file end

- **GIVEN** a Python file ending with:
  ```python
  def last_function():
      pass
  # <!-- conclaude-uneditable:start -->
  # Protected footer
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the range SHALL end at the last line of the file
- **AND** the range SHALL be valid

#### Scenario: Empty lines between markers

- **GIVEN** a Python file with:
  ```python
  # <!-- conclaude-uneditable:start -->


  def protected_function():
      pass


  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the range SHALL include all lines between markers (including empty lines)
- **AND** edits within the range SHALL be blocked

### Requirement: Shebang and Encoding Compatibility

The system SHALL correctly handle Python files with shebang lines and encoding declarations.

#### Scenario: File with shebang

- **GIVEN** a Python file starting with:
  ```python
  #!/usr/bin/env python3
  # <!-- conclaude-uneditable:start -->
  import sys
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 2 SHALL be detected
- **AND** a protected range from line 2 to line 4 SHALL be created

#### Scenario: File with encoding declaration

- **GIVEN** a Python file with:
  ```python
  # -*- coding: utf-8 -*-
  # <!-- conclaude-uneditable:start -->
  def function():
      pass
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 2 SHALL be detected (after encoding declaration)
- **AND** a protected range from line 2 to line 5 SHALL be created

#### Scenario: Shebang, encoding, and marker

- **GIVEN** a Python file with:
  ```python
  #!/usr/bin/env python3
  # -*- coding: utf-8 -*-
  # <!-- conclaude-uneditable:start -->
  import os
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 3 SHALL be detected
- **AND** a protected range from line 3 to line 5 SHALL be created

### Requirement: Performance Characteristics

The system SHALL efficiently parse Python files for uneditable markers.

#### Scenario: Large Python file with multiple markers

- **GIVEN** a Python file with 10,000 lines and 15 protected ranges
- **WHEN** the file is parsed for markers
- **THEN** parsing SHALL complete in under 100ms
- **AND** all 15 ranges SHALL be correctly identified

#### Scenario: Python file with no markers

- **GIVEN** a Python file with 2,000 lines and no uneditable markers
- **WHEN** the file is parsed
- **THEN** parsing SHALL complete quickly (under 30ms)
- **AND** an empty range list SHALL be returned
- **AND** no false positive detections SHALL occur

#### Scenario: File with many comments but no markers

- **GIVEN** a Python file with 500 comment lines but no uneditable markers
- **WHEN** the file is parsed
- **THEN** parsing SHALL scan all comments efficiently
- **AND** no protected ranges SHALL be created
- **AND** performance SHALL remain acceptable (under 50ms)
