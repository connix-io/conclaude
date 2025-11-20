# comment-syntax-csharp Specification

## Purpose

Define C#-specific comment syntax detection for uneditable range markers, supporting line comments (`//`), block comments (`/* */`), and XML documentation comments (`///` and `/** */`).

## ADDED Requirements

### Requirement: Line Comment Detection

The system SHALL recognize uneditable markers within C# line comments (`//`).

#### Scenario: Single-line comment with marker

- **GIVEN** a C# file with:
  ```csharp
  // <!-- conclaude-uneditable:start -->
  ```
- **WHEN** the file is parsed for uneditable markers
- **THEN** the start marker SHALL be detected
- **AND** the marker line SHALL be recorded as the start of a protected range

#### Scenario: Line comment with trailing content

- **GIVEN** a C# file with:
  ```csharp
  // <!-- conclaude-uneditable:start --> Auto-generated code below
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be detected (trailing content ignored)
- **AND** the range SHALL begin at this line

### Requirement: Block Comment Detection

The system SHALL recognize uneditable markers within C# block comments (`/* */`).

#### Scenario: Block comment with marker

- **GIVEN** a C# file with:
  ```csharp
  /* <!-- conclaude-uneditable:start --> */
  ```
- **WHEN** the file is parsed
- **THEN** the start marker SHALL be detected within the block comment
- **AND** the range SHALL begin at this line

#### Scenario: Multi-line block comment with marker

- **GIVEN** a C# file with:
  ```csharp
  /*
   * <!-- conclaude-uneditable:start -->
   * Auto-generated service methods
   */
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be detected on line 2
- **AND** the range SHALL begin at line 2

### Requirement: XML Documentation Comment Detection

The system SHALL recognize uneditable markers within C# XML documentation comments (`///` and `/** */`).

#### Scenario: XML doc comment with marker

- **GIVEN** a C# file with:
  ```csharp
  /// <!-- conclaude-uneditable:start -->
  /// <summary>
  /// Auto-generated API client
  /// </summary>
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be detected on line 1
- **AND** the range SHALL begin at line 1

#### Scenario: Multi-line XML doc with marker and tags

- **GIVEN** a C# file with:
  ```csharp
  /// <!-- conclaude-uneditable:start -->
  /// <summary>
  /// Processes the input data
  /// </summary>
  /// <param name="data">The input data</param>
  /// <returns>The processed result</returns>
  public string Process(string data)
  {
      return data;
  }
  // <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** both markers SHALL be detected
- **AND** a protected range from line 1 to line 11 SHALL be created

#### Scenario: Block-style XML doc comment with marker

- **GIVEN** a C# file with:
  ```csharp
  /**
   * <!-- conclaude-uneditable:start -->
   * <summary>
   * Generated class documentation
   * </summary>
   */
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be detected on line 2
- **AND** the range SHALL begin at line 2

### Requirement: File Extension Mapping

The system SHALL detect C# files by their file extension and apply C# comment syntax rules.

#### Scenario: .cs file extension

- **GIVEN** a file named "Program.cs"
- **WHEN** the file is processed for uneditable ranges
- **THEN** C# comment syntax rules SHALL be applied
- **AND** markers within `//`, `/* */`, or `///` comments SHALL be detected

#### Scenario: .csx file extension (C# script)

- **GIVEN** a file named "script.csx"
- **WHEN** the file is processed for uneditable ranges
- **THEN** C# comment syntax rules SHALL be applied
- **AND** markers SHALL be detected correctly

#### Scenario: .cs file with markers

- **GIVEN** a file "GeneratedClient.cs" containing:
  ```csharp
  // <!-- conclaude-uneditable:start -->
  // Auto-generated API client
  public class GeneratedClient
  {
      private readonly HttpClient _client;

      public GeneratedClient(HttpClient client)
      {
          _client = client;
      }
  }
  // <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** a protected range from line 1 to line 12 SHALL be created
- **AND** edits overlapping this range SHALL be blocked

### Requirement: Marker Format Validation

The system SHALL only recognize correctly formatted markers within C# comments.

#### Scenario: Marker in string literal (not detected)

- **GIVEN** a C# file with:
  ```csharp
  string marker = "// <!-- conclaude-uneditable:start -->";
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL NOT be detected (inside string, not comment)
- **AND** no protected range SHALL be created

#### Scenario: Marker in verbatim string (not detected)

- **GIVEN** a C# file with:
  ```csharp
  string verbatim = @"
  // <!-- conclaude-uneditable:start -->
  Some content
  ";
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL NOT be detected (inside verbatim string)
- **AND** no protected range SHALL be created

#### Scenario: Marker in interpolated string (not detected)

- **GIVEN** a C# file with:
  ```csharp
  string interpolated = $"// <!-- conclaude-uneditable:start --> {value}";
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL NOT be detected (inside interpolated string)
- **AND** no protected range SHALL be created

#### Scenario: Marker in attribute value (not detected)

- **GIVEN** a C# file with:
  ```csharp
  [Description("<!-- conclaude-uneditable:start -->")]
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL NOT be detected (inside attribute string)
- **AND** no protected range SHALL be created

#### Scenario: Correctly formatted marker in comment

- **GIVEN** a C# file with:
  ```csharp
  // <!-- conclaude-uneditable:start -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be recognized
- **AND** a protected range SHALL start at this line

### Requirement: Nested Range Handling

The system SHALL handle nested ranges within C# code structure.

#### Scenario: Nested class method protection

- **GIVEN** a C# file with:
  ```csharp
  // <!-- conclaude-uneditable:start -->  // Line 1
  public class ApiService
  {
      // <!-- conclaude-uneditable:start -->  // Line 4
      public async Task<string> AuthenticateAsync()
      {
          return await Task.FromResult("token");
      }
      // <!-- conclaude-uneditable:end -->  // Line 8
  }
  // <!-- conclaude-uneditable:end -->  // Line 10
  ```
- **WHEN** the file is parsed
- **THEN** two ranges SHALL be created: (1-10) and (4-8)
- **AND** both ranges SHALL be marked as protected
- **AND** edits overlapping either range SHALL be blocked

### Requirement: Edge Case Handling

The system SHALL gracefully handle edge cases in C# comment syntax.

#### Scenario: Inline comment after code

- **GIVEN** a C# file with:
  ```csharp
  int x = 5;  // <!-- conclaude-uneditable:start -->
  int y = 10;
  int z = 15;  // <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** both markers SHALL be detected (inline comments are valid)
- **AND** a protected range from line 1 to line 3 SHALL be created

#### Scenario: Marker at file start

- **GIVEN** a C# file with marker at line 1:
  ```csharp
  // <!-- conclaude-uneditable:start -->
  using System;
  // <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the range SHALL start at line 1
- **AND** the range SHALL end at line 3

#### Scenario: Marker at file end

- **GIVEN** a C# file ending with:
  ```csharp
  public void LastMethod()
  {
  }
  // <!-- conclaude-uneditable:start -->
  // Generated footer
  // <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the range SHALL end at the last line of the file
- **AND** the range SHALL be valid

#### Scenario: Empty lines between markers

- **GIVEN** a C# file with:
  ```csharp
  // <!-- conclaude-uneditable:start -->


  public void ProtectedMethod()
  {
  }


  // <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the range SHALL include all lines between markers (including empty lines)
- **AND** edits within the range SHALL be blocked

### Requirement: Namespace and Using Compatibility

The system SHALL correctly handle C# files with namespace declarations and using statements.

#### Scenario: File with namespace declaration

- **GIVEN** a C# file with:
  ```csharp
  namespace MyApp.Services;

  // <!-- conclaude-uneditable:start -->
  using System.Net.Http;
  // <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 3 SHALL be detected
- **AND** a protected range from line 3 to line 5 SHALL be created

#### Scenario: File with global using statements and marker

- **GIVEN** a C# file with:
  ```csharp
  // <!-- conclaude-uneditable:start -->
  global using System;
  global using System.Linq;
  // <!-- conclaude-uneditable:end -->

  namespace MyApp;
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 1 SHALL be detected
- **AND** a protected range from line 1 to line 4 SHALL be created

#### Scenario: File-scoped namespace with marker

- **GIVEN** a C# file with:
  ```csharp
  namespace MyApp.Generated;

  // <!-- conclaude-uneditable:start -->
  public record UserDto(int Id, string Name);
  // <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 3 SHALL be detected
- **AND** a protected range from line 3 to line 5 SHALL be created

### Requirement: C#-Specific Syntax Handling

The system SHALL handle C#-specific patterns and constructs.

#### Scenario: Interface with protected methods

- **GIVEN** a C# file with:
  ```csharp
  // <!-- conclaude-uneditable:start -->
  public interface IGeneratedService
  {
      Task<string> ExecuteAsync();
      string GetData();
  }
  // <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 1 SHALL be detected
- **AND** a protected range from line 1 to line 7 SHALL be created

#### Scenario: Record type with protected definition

- **GIVEN** a C# file with:
  ```csharp
  // <!-- conclaude-uneditable:start -->
  public record Person(string Name, int Age);
  // <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 1 SHALL be detected
- **AND** a protected range from line 1 to line 3 SHALL be created

#### Scenario: Property with protected getter/setter

- **GIVEN** a C# file with:
  ```csharp
  // <!-- conclaude-uneditable:start -->
  public string Name { get; init; }
  public int Age { get; set; }
  // <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 1 SHALL be detected
- **AND** a protected range from line 1 to line 4 SHALL be created

#### Scenario: Nullable reference types with marker

- **GIVEN** a C# file with:
  ```csharp
  #nullable enable
  // <!-- conclaude-uneditable:start -->
  public string? OptionalValue { get; set; }
  public string RequiredValue { get; set; } = string.Empty;
  // <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 2 SHALL be detected
- **AND** a protected range from line 2 to line 5 SHALL be created

#### Scenario: LINQ query with protected implementation

- **GIVEN** a C# file with:
  ```csharp
  // <!-- conclaude-uneditable:start -->
  var results = from user in users
                where user.Active
                select user.Name;
  // <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 1 SHALL be detected
- **AND** a protected range from line 1 to line 5 SHALL be created

#### Scenario: Async method with marker

- **GIVEN** a C# file with:
  ```csharp
  // <!-- conclaude-uneditable:start -->
  public async Task<int> ProcessAsync(string input)
  {
      await Task.Delay(100);
      return input.Length;
  }
  // <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 1 SHALL be detected
- **AND** a protected range from line 1 to line 7 SHALL be created

### Requirement: Attribute Compatibility

The system SHALL correctly handle markers in C# code with attributes.

#### Scenario: Attribute on class with marker

- **GIVEN** a C# file with:
  ```csharp
  // <!-- conclaude-uneditable:start -->
  [Serializable]
  [DataContract]
  public class GeneratedModel
  {
      [DataMember]
      public string Name { get; set; }
  }
  // <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 1 SHALL be detected
- **AND** a protected range from line 1 to line 9 SHALL be created

#### Scenario: Method with multiple attributes

- **GIVEN** a C# file with:
  ```csharp
  // <!-- conclaude-uneditable:start -->
  [HttpGet]
  [Authorize]
  [ProducesResponseType(200)]
  public async Task<IActionResult> GetData()
  {
      return Ok();
  }
  // <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 1 SHALL be detected
- **AND** a protected range from line 1 to line 9 SHALL be created

### Requirement: XML Documentation Tag Compatibility

The system SHALL correctly handle markers in XML documentation with standard tags.

#### Scenario: XML doc with standard tags

- **GIVEN** a C# file with:
  ```csharp
  /// <!-- conclaude-uneditable:start -->
  /// <summary>
  /// Auto-generated service class
  /// </summary>
  /// <remarks>
  /// Generated from OpenAPI specification
  /// </remarks>
  public class AutoGeneratedService
  {
  }
  // <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 1 SHALL be detected
- **AND** a protected range from line 1 to line 11 SHALL be created

#### Scenario: XML doc with param and returns tags

- **GIVEN** a C# file with:
  ```csharp
  /// <!-- conclaude-uneditable:start -->
  /// <param name="value">Input value</param>
  /// <returns>Processed result</returns>
  /// <exception cref="ArgumentException">Thrown when value is null</exception>
  public string Process(string value)
  {
      return value ?? throw new ArgumentException(nameof(value));
  }
  // <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 1 SHALL be detected
- **AND** a protected range from line 1 to line 9 SHALL be created

### Requirement: Performance Characteristics

The system SHALL efficiently parse C# files for uneditable markers.

#### Scenario: Large C# file with multiple markers

- **GIVEN** a C# file with 10,000 lines and 15 protected ranges
- **WHEN** the file is parsed for markers
- **THEN** parsing SHALL complete in under 100ms
- **AND** all 15 ranges SHALL be correctly identified

#### Scenario: C# file with no markers

- **GIVEN** a C# file with 3,000 lines and no uneditable markers
- **WHEN** the file is parsed
- **THEN** parsing SHALL complete quickly (under 40ms)
- **AND** an empty range list SHALL be returned
- **AND** no false positive detections SHALL occur

#### Scenario: File with many XML doc comments but no markers

- **GIVEN** a C# file with 500 lines of XML documentation comments but no uneditable markers
- **WHEN** the file is parsed
- **THEN** parsing SHALL scan all comments efficiently
- **AND** no protected ranges SHALL be created
- **AND** performance SHALL remain acceptable (under 50ms)
