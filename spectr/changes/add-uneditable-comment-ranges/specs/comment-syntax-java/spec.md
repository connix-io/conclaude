# comment-syntax-java Specification

## Purpose

Define Java-specific comment syntax detection for uneditable range markers, supporting line comments (`//`), block comments (`/* */`), and Javadoc comments (`/** */`).

## ADDED Requirements

### Requirement: Line Comment Detection

The system SHALL recognize uneditable markers within Java line comments (`//`).

#### Scenario: Single-line comment with marker

- **GIVEN** a Java file with:
  ```java
  // <!-- conclaude-uneditable:start -->
  ```
- **WHEN** the file is parsed for uneditable markers
- **THEN** the start marker SHALL be detected
- **AND** the marker line SHALL be recorded as the start of a protected range

#### Scenario: Line comment with trailing content

- **GIVEN** a Java file with:
  ```java
  // <!-- conclaude-uneditable:start --> Auto-generated code below
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be detected (trailing content ignored)
- **AND** the range SHALL begin at this line

### Requirement: Block Comment Detection

The system SHALL recognize uneditable markers within Java block comments (`/* */`).

#### Scenario: Block comment with marker

- **GIVEN** a Java file with:
  ```java
  /* <!-- conclaude-uneditable:start --> */
  ```
- **WHEN** the file is parsed
- **THEN** the start marker SHALL be detected within the block comment
- **AND** the range SHALL begin at this line

#### Scenario: Multi-line block comment with marker

- **GIVEN** a Java file with:
  ```java
  /*
   * <!-- conclaude-uneditable:start -->
   * Auto-generated method signatures
   */
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be detected on line 2
- **AND** the range SHALL begin at line 2

### Requirement: Javadoc Comment Detection

The system SHALL recognize uneditable markers within Javadoc comments (`/** */`).

#### Scenario: Javadoc comment with marker

- **GIVEN** a Java file with:
  ```java
  /**
   * <!-- conclaude-uneditable:start -->
   * @generated
   * Auto-generated class documentation
   */
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be detected on line 2
- **AND** the range SHALL begin at line 2

#### Scenario: Javadoc with marker and annotations

- **GIVEN** a Java file with:
  ```java
  /**
   * <!-- conclaude-uneditable:start -->
   * @param value the input value
   * @return processed result
   */
  public String process(String value) {
      return value;
  }
  // <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** both markers SHALL be detected
- **AND** a protected range from line 2 to line 9 SHALL be created

### Requirement: File Extension Mapping

The system SHALL detect Java files by their file extension and apply Java comment syntax rules.

#### Scenario: .java file extension

- **GIVEN** a file named "Application.java"
- **WHEN** the file is processed for uneditable ranges
- **THEN** Java comment syntax rules SHALL be applied
- **AND** markers within `//`, `/* */`, or `/** */` comments SHALL be detected

#### Scenario: .java file with markers

- **GIVEN** a file "Generated.java" containing:
  ```java
  // <!-- conclaude-uneditable:start -->
  // Auto-generated DTO
  public class GeneratedDTO {
      private String id;
      private String name;
  }
  // <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** a protected range from line 1 to line 7 SHALL be created
- **AND** edits overlapping this range SHALL be blocked

### Requirement: Marker Format Validation

The system SHALL only recognize correctly formatted markers within Java comments.

#### Scenario: Marker in string literal (not detected)

- **GIVEN** a Java file with:
  ```java
  String marker = "// <!-- conclaude-uneditable:start -->";
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL NOT be detected (inside string, not comment)
- **AND** no protected range SHALL be created

#### Scenario: Marker in annotation value (not detected)

- **GIVEN** a Java file with:
  ```java
  @SuppressWarnings("<!-- conclaude-uneditable:start -->")
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL NOT be detected (inside annotation string)
- **AND** no protected range SHALL be created

#### Scenario: Correctly formatted marker in comment

- **GIVEN** a Java file with:
  ```java
  // <!-- conclaude-uneditable:start -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be recognized
- **AND** a protected range SHALL start at this line

### Requirement: Nested Range Handling

The system SHALL handle nested ranges within Java code structure.

#### Scenario: Nested class method protection

- **GIVEN** a Java file with:
  ```java
  // <!-- conclaude-uneditable:start -->  // Line 1
  public class APIClient {
      // <!-- conclaude-uneditable:start -->  // Line 3
      public void authenticate() {
          // Authentication logic
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

The system SHALL gracefully handle edge cases in Java comment syntax.

#### Scenario: Inline comment after code

- **GIVEN** a Java file with:
  ```java
  int x = 5;  // <!-- conclaude-uneditable:start -->
  int y = 10;
  int z = 15;  // <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** both markers SHALL be detected (inline comments are valid)
- **AND** a protected range from line 1 to line 3 SHALL be created

#### Scenario: Marker at file start

- **GIVEN** a Java file with marker at line 1:
  ```java
  // <!-- conclaude-uneditable:start -->
  package com.example.generated;
  // <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the range SHALL start at line 1
- **AND** the range SHALL end at line 3

#### Scenario: Marker at file end

- **GIVEN** a Java file ending with:
  ```java
  public void lastMethod() {
  }
  // <!-- conclaude-uneditable:start -->
  // Generated footer
  // <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the range SHALL end at the last line of the file
- **AND** the range SHALL be valid

#### Scenario: Empty lines between markers

- **GIVEN** a Java file with:
  ```java
  // <!-- conclaude-uneditable:start -->


  public void protectedMethod() {
  }


  // <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the range SHALL include all lines between markers (including empty lines)
- **AND** edits within the range SHALL be blocked

### Requirement: Package and Import Compatibility

The system SHALL correctly handle Java files with package declarations and imports.

#### Scenario: File with package declaration

- **GIVEN** a Java file with:
  ```java
  package com.example.api;

  // <!-- conclaude-uneditable:start -->
  import java.util.List;
  // <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 3 SHALL be detected
- **AND** a protected range from line 3 to line 5 SHALL be created

#### Scenario: File with static imports and marker

- **GIVEN** a Java file with:
  ```java
  package com.example;

  // <!-- conclaude-uneditable:start -->
  import static org.junit.Assert.*;
  import static org.mockito.Mockito.*;
  // <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 3 SHALL be detected
- **AND** a protected range from line 3 to line 6 SHALL be created

### Requirement: Java-Specific Syntax Handling

The system SHALL handle Java-specific patterns and constructs.

#### Scenario: Interface with protected methods

- **GIVEN** a Java file with:
  ```java
  // <!-- conclaude-uneditable:start -->
  public interface GeneratedAPI {
      void execute();
      String getData();
  }
  // <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 1 SHALL be detected
- **AND** a protected range from line 1 to line 6 SHALL be created

#### Scenario: Enum with protected values

- **GIVEN** a Java file with:
  ```java
  // <!-- conclaude-uneditable:start -->
  public enum Status {
      ACTIVE,
      INACTIVE,
      PENDING
  }
  // <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 1 SHALL be detected
- **AND** a protected range from line 1 to line 7 SHALL be created

#### Scenario: Annotation type with marker

- **GIVEN** a Java file with:
  ```java
  // <!-- conclaude-uneditable:start -->
  @Retention(RetentionPolicy.RUNTIME)
  @Target(ElementType.METHOD)
  public @interface Generated {
      String value();
  }
  // <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 1 SHALL be detected
- **AND** a protected range from line 1 to line 7 SHALL be created

### Requirement: Javadoc Tag Compatibility

The system SHALL correctly handle markers in Javadoc with standard tags.

#### Scenario: Javadoc with standard tags

- **GIVEN** a Java file with:
  ```java
  /**
   * <!-- conclaude-uneditable:start -->
   * @author Generated
   * @version 1.0
   * @since 2024-01-01
   */
  public class AutoGenerated {
  }
  // <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 2 SHALL be detected
- **AND** a protected range from line 2 to line 9 SHALL be created

#### Scenario: Javadoc with @deprecated tag

- **GIVEN** a Java file with:
  ```java
  /**
   * <!-- conclaude-uneditable:start -->
   * @deprecated Use newMethod() instead
   */
  public void oldMethod() {
  }
  // <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 2 SHALL be detected
- **AND** a protected range from line 2 to line 7 SHALL be created

### Requirement: Performance Characteristics

The system SHALL efficiently parse Java files for uneditable markers.

#### Scenario: Large Java file with multiple markers

- **GIVEN** a Java file with 10,000 lines and 15 protected ranges
- **WHEN** the file is parsed for markers
- **THEN** parsing SHALL complete in under 100ms
- **AND** all 15 ranges SHALL be correctly identified

#### Scenario: Java file with no markers

- **GIVEN** a Java file with 3,000 lines and no uneditable markers
- **WHEN** the file is parsed
- **THEN** parsing SHALL complete quickly (under 40ms)
- **AND** an empty range list SHALL be returned
- **AND** no false positive detections SHALL occur

#### Scenario: File with many Javadoc comments but no markers

- **GIVEN** a Java file with 500 lines of Javadoc comments but no uneditable markers
- **WHEN** the file is parsed
- **THEN** parsing SHALL scan all comments efficiently
- **AND** no protected ranges SHALL be created
- **AND** performance SHALL remain acceptable (under 50ms)
