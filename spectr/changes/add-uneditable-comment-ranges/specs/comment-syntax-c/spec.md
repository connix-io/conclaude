# comment-syntax-c Specification

## Purpose

Define C/C++-specific comment syntax detection for uneditable range markers, supporting both line comments (`//`) and block comments (`/* */`).

## ADDED Requirements

### Requirement: Line Comment Detection

The system SHALL recognize uneditable markers within C line comments (`//`).

#### Scenario: Single-line comment with marker

- **GIVEN** a C file with:
  ```c
  // <!-- conclaude-uneditable:start -->
  ```
- **WHEN** the file is parsed for uneditable markers
- **THEN** the start marker SHALL be detected
- **AND** the marker line SHALL be recorded as the start of a protected range

#### Scenario: Line comment with trailing content

- **GIVEN** a C file with:
  ```c
  // <!-- conclaude-uneditable:start --> Auto-generated code below
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be detected (trailing content ignored)
- **AND** the range SHALL begin at this line

### Requirement: Block Comment Detection

The system SHALL recognize uneditable markers within C block comments (`/* */`).

#### Scenario: Block comment with marker

- **GIVEN** a C file with:
  ```c
  /* <!-- conclaude-uneditable:start --> */
  ```
- **WHEN** the file is parsed
- **THEN** the start marker SHALL be detected within the block comment
- **AND** the range SHALL begin at this line

#### Scenario: Multi-line block comment with marker

- **GIVEN** a C file with:
  ```c
  /*
   * <!-- conclaude-uneditable:start -->
   * Auto-generated function prototypes
   */
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be detected on line 2
- **AND** the range SHALL begin at line 2

#### Scenario: Doxygen-style comment with marker

- **GIVEN** a C file with:
  ```c
  /**
   * <!-- conclaude-uneditable:start -->
   * @brief Auto-generated API
   */
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be detected on line 2
- **AND** the range SHALL begin at line 2

### Requirement: File Extension Mapping

The system SHALL detect C and C++ files by their file extension and apply C comment syntax rules.

#### Scenario: .c file extension

- **GIVEN** a file named "main.c"
- **WHEN** the file is processed for uneditable ranges
- **THEN** C comment syntax rules SHALL be applied
- **AND** markers within `//` or `/* */` comments SHALL be detected

#### Scenario: .h file extension (C header)

- **GIVEN** a file named "api.h"
- **WHEN** the file is processed for uneditable ranges
- **THEN** C comment syntax rules SHALL be applied
- **AND** markers SHALL be detected correctly

#### Scenario: .cpp, .cxx, .cc file extensions (C++)

- **GIVEN** files named "app.cpp", "util.cxx", "lib.cc"
- **WHEN** the files are processed for uneditable ranges
- **THEN** C comment syntax rules SHALL be applied (C++ uses same syntax)
- **AND** markers SHALL be detected in all files

#### Scenario: .hpp, .hxx file extensions (C++ headers)

- **GIVEN** files named "interface.hpp", "template.hxx"
- **WHEN** the files are processed for uneditable ranges
- **THEN** C comment syntax rules SHALL be applied
- **AND** markers SHALL be detected correctly

#### Scenario: .c file with markers

- **GIVEN** a file "generated.c" containing:
  ```c
  // <!-- conclaude-uneditable:start -->
  // Auto-generated function implementations
  void generated_init(void) {
      // ...
  }
  // <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** a protected range from line 1 to line 6 SHALL be created
- **AND** edits overlapping this range SHALL be blocked

### Requirement: Marker Format Validation

The system SHALL only recognize correctly formatted markers within C comments.

#### Scenario: Marker in string literal (not detected)

- **GIVEN** a C file with:
  ```c
  const char* marker = "// <!-- conclaude-uneditable:start -->";
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL NOT be detected (inside string, not comment)
- **AND** no protected range SHALL be created

#### Scenario: Marker in preprocessor directive (not detected)

- **GIVEN** a C file with:
  ```c
  #define MARKER "<!-- conclaude-uneditable:start -->"
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL NOT be detected (inside macro definition)
- **AND** no protected range SHALL be created

#### Scenario: Correctly formatted marker in comment

- **GIVEN** a C file with:
  ```c
  // <!-- conclaude-uneditable:start -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be recognized
- **AND** a protected range SHALL start at this line

### Requirement: Nested Range Handling

The system SHALL handle nested ranges within C code structure.

#### Scenario: Nested struct protection

- **GIVEN** a C file with:
  ```c
  // <!-- conclaude-uneditable:start -->  // Line 1
  typedef struct {
      // <!-- conclaude-uneditable:start -->  // Line 3
      int critical_field;
      // <!-- conclaude-uneditable:end -->  // Line 5
      int other_field;
  } Config;
  // <!-- conclaude-uneditable:end -->  // Line 8
  ```
- **WHEN** the file is parsed
- **THEN** two ranges SHALL be created: (1-8) and (3-5)
- **AND** both ranges SHALL be marked as protected
- **AND** edits overlapping either range SHALL be blocked

### Requirement: Edge Case Handling

The system SHALL gracefully handle edge cases in C comment syntax.

#### Scenario: Inline comment after code

- **GIVEN** a C file with:
  ```c
  int x = 5;  // <!-- conclaude-uneditable:start -->
  int y = 10;
  int z = 15;  // <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** both markers SHALL be detected (inline comments are valid)
- **AND** a protected range from line 1 to line 3 SHALL be created

#### Scenario: Marker at file start

- **GIVEN** a C file with marker at line 1:
  ```c
  // <!-- conclaude-uneditable:start -->
  #include <stdio.h>
  // <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the range SHALL start at line 1
- **AND** the range SHALL end at line 3

#### Scenario: Marker at file end

- **GIVEN** a C file ending with:
  ```c
  void last_function(void) {
  }
  // <!-- conclaude-uneditable:start -->
  // Generated footer
  // <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the range SHALL end at the last line of the file
- **AND** the range SHALL be valid

#### Scenario: Empty lines between markers

- **GIVEN** a C file with:
  ```c
  // <!-- conclaude-uneditable:start -->


  void protected_function(void) {
  }


  // <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the range SHALL include all lines between markers (including empty lines)
- **AND** edits within the range SHALL be blocked

### Requirement: Preprocessor and Header Guard Compatibility

The system SHALL correctly handle C files with preprocessor directives and header guards.

#### Scenario: File with header guards

- **GIVEN** a C header file with:
  ```c
  #ifndef API_H
  #define API_H
  // <!-- conclaude-uneditable:start -->
  void generated_api(void);
  // <!-- conclaude-uneditable:end -->
  #endif
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 3 SHALL be detected
- **AND** a protected range from line 3 to line 5 SHALL be created

#### Scenario: File with include guards and marker

- **GIVEN** a C file with:
  ```c
  #pragma once
  // <!-- conclaude-uneditable:start -->
  typedef struct {
      int id;
  } Generated;
  // <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 2 SHALL be detected
- **AND** a protected range from line 2 to line 6 SHALL be created

### Requirement: C++-Specific Syntax Handling

The system SHALL handle C++-specific comment patterns and constructs.

#### Scenario: Template with protected implementation

- **GIVEN** a C++ file with:
  ```cpp
  // <!-- conclaude-uneditable:start -->
  template <typename T>
  class Generated {
  public:
      T getValue() const { return value_; }
  private:
      T value_;
  };
  // <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 1 SHALL be detected
- **AND** a protected range from line 1 to line 9 SHALL be created

#### Scenario: Namespace with protected content

- **GIVEN** a C++ file with:
  ```cpp
  namespace generated {
  // <!-- conclaude-uneditable:start -->
  void auto_init();
  // <!-- conclaude-uneditable:end -->
  }  // namespace generated
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 2 SHALL be detected
- **AND** a protected range from line 2 to line 4 SHALL be created

### Requirement: Doxygen Comment Support

The system SHALL recognize markers within Doxygen documentation comments.

#### Scenario: Doxygen block comment with marker

- **GIVEN** a C file with:
  ```c
  /**
   * <!-- conclaude-uneditable:start -->
   * @file generated.h
   * @brief Auto-generated API definitions
   */
  void api_function(void);
  // <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 2 SHALL be detected
- **AND** a protected range from line 2 to line 7 SHALL be created

#### Scenario: Doxygen line comment with marker

- **GIVEN** a C++ file with:
  ```cpp
  /// <!-- conclaude-uneditable:start -->
  /// @brief Generated class
  class GeneratedClass {};
  /// <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 1 SHALL be detected
- **AND** a protected range from line 1 to line 4 SHALL be created

### Requirement: Performance Characteristics

The system SHALL efficiently parse C/C++ files for uneditable markers.

#### Scenario: Large C file with multiple markers

- **GIVEN** a C file with 8,000 lines and 12 protected ranges
- **WHEN** the file is parsed for markers
- **THEN** parsing SHALL complete in under 80ms
- **AND** all 12 ranges SHALL be correctly identified

#### Scenario: C file with no markers

- **GIVEN** a C file with 3,000 lines and no uneditable markers
- **WHEN** the file is parsed
- **THEN** parsing SHALL complete quickly (under 40ms)
- **AND** an empty range list SHALL be returned
- **AND** no false positive detections SHALL occur

#### Scenario: Header file with many comments but no markers

- **GIVEN** a C header file with 500 lines of Doxygen comments but no uneditable markers
- **WHEN** the file is parsed
- **THEN** parsing SHALL scan all comments efficiently
- **AND** no protected ranges SHALL be created
- **AND** performance SHALL remain acceptable (under 50ms)
