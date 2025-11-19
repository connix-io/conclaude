# Specification: Language Support (Comment Syntax Detection)

## Purpose
Define language detection and comment syntax mapping capabilities to enable marker detection across all major programming languages.

## ADDED Requirements

### Requirement: Language Detection by File Extension
The system SHALL detect programming languages based on file extensions with high accuracy for common languages.

#### Scenario: Go file detection
- **WHEN** a file has extension `.go`
- **THEN** the system SHALL detect language as Go
- **AND** use `//` as the comment pattern

#### Scenario: Python file detection
- **WHEN** a file has extension `.py`
- **THEN** the system SHALL detect language as Python
- **AND** use `#` as the comment pattern

#### Scenario: JavaScript file detection
- **WHEN** a file has extension `.js` or `.jsx`
- **THEN** the system SHALL detect language as JavaScript
- **AND** use `//` as the comment pattern

#### Scenario: TypeScript file detection
- **WHEN** a file has extension `.ts` or `.tsx`
- **THEN** the system SHALL detect language as TypeScript
- **AND** use `//` as the comment pattern

#### Scenario: Rust file detection
- **WHEN** a file has extension `.rs`
- **THEN** the system SHALL detect language as Rust
- **AND** use `//` as the comment pattern

#### Scenario: Java file detection
- **WHEN** a file has extension `.java`
- **THEN** the system SHALL detect language as Java
- **AND** use `//` as the comment pattern

#### Scenario: C file detection
- **WHEN** a file has extension `.c` or `.h`
- **THEN** the system SHALL detect language as C
- **AND** use `//` as the primary comment pattern

#### Scenario: C++ file detection
- **WHEN** a file has extension `.cpp`, `.cc`, `.hpp`, or `.cxx`
- **THEN** the system SHALL detect language as C++
- **AND** use `//` as the primary comment pattern

#### Scenario: Ruby file detection
- **WHEN** a file has extension `.rb`
- **THEN** the system SHALL detect language as Ruby
- **AND** use `#` as the comment pattern

#### Scenario: PHP file detection
- **WHEN** a file has extension `.php`
- **THEN** the system SHALL detect language as PHP
- **AND** use `//` and `#` as comment patterns

#### Scenario: Shell script detection
- **WHEN** a file has extension `.sh`, `.bash`, or `.zsh`
- **THEN** the system SHALL detect language as Shell
- **AND** use `#` as the comment pattern

### Requirement: Unknown Language Fallback
The system SHALL provide fallback behavior for files with unknown or uncommon extensions.

#### Scenario: Unknown extension fallback
- **WHEN** a file has an unrecognized extension (e.g., `.xyz`)
- **THEN** the system SHALL attempt detection using all common comment patterns
- **AND** try patterns in order: `//`, `#`, `/*`

#### Scenario: No extension fallback
- **WHEN** a file has no extension (e.g., `Makefile`, `Dockerfile`)
- **THEN** the system SHALL attempt detection using all common comment patterns
- **AND** successfully find markers if they exist in any comment style

### Requirement: Comment Pattern Mapping
The system SHALL correctly map detected languages to their respective comment syntax patterns.

#### Scenario: Double-slash comment languages
- **WHEN** the language is Go, Rust, JavaScript, TypeScript, Java, C, or C++
- **THEN** the system SHALL use `//` as the line comment pattern
- **AND** markers in lines starting with `//` SHALL be detected

#### Scenario: Hash comment languages
- **WHEN** the language is Python, Ruby, or Shell
- **THEN** the system SHALL use `#` as the line comment pattern
- **AND** markers in lines starting with `#` SHALL be detected

#### Scenario: Multi-pattern languages
- **WHEN** the language supports multiple comment styles (e.g., C, PHP)
- **THEN** the system SHALL try all applicable patterns
- **AND** detect markers in any valid comment syntax

### Requirement: Marker Pattern Matching in Comments
The system SHALL correctly identify markers within language-specific comments, ignoring markers in strings or non-comment contexts.

#### Scenario: Marker in line comment
- **WHEN** a line contains `// <!-- conclaude-uneditable:start -->`
- **AND** the language uses `//` comments
- **THEN** the marker SHALL be detected

#### Scenario: Marker with leading whitespace
- **WHEN** a line contains `   // <!-- conclaude-uneditable:start -->` (indented)
- **AND** the language uses `//` comments
- **THEN** the marker SHALL be detected

#### Scenario: Marker with trailing content
- **WHEN** a line contains `// <!-- conclaude-uneditable:start --> - do not modify`
- **AND** the language uses `//` comments
- **THEN** the marker SHALL be detected

#### Scenario: Marker in string literal (should ignore)
- **WHEN** a line contains `const x = "<!-- conclaude-uneditable:start -->"` (in a string)
- **AND** the line does not start with a comment pattern
- **THEN** the marker SHALL NOT be detected
- **AND** the line SHALL be treated as regular code

#### Scenario: Marker in multi-line string (should ignore)
- **WHEN** content contains markers within triple-quoted strings (Python) or template literals (JS)
- **AND** the markers are not in actual comments
- **THEN** the markers SHALL NOT be detected

### Requirement: Case Sensitivity
The system SHALL use case-sensitive matching for marker detection to ensure consistency.

#### Scenario: Correct case marker
- **WHEN** a comment contains `<!-- conclaude-uneditable:start -->` (lowercase)
- **THEN** the marker SHALL be detected

#### Scenario: Incorrect case marker
- **WHEN** a comment contains `<!-- CONCLAUDE-UNEDITABLE:START -->` (uppercase)
- **THEN** the marker SHALL NOT be detected
- **AND** no protected range SHALL be created

### Requirement: Whitespace Handling
The system SHALL handle various whitespace configurations around markers.

#### Scenario: Marker with spaces
- **WHEN** a comment contains `//  <!-- conclaude-uneditable:start -->` (extra spaces)
- **THEN** the marker SHALL be detected

#### Scenario: Marker with tabs
- **WHEN** a comment contains `//\t<!-- conclaude-uneditable:start -->` (tab character)
- **THEN** the marker SHALL be detected

#### Scenario: Indented marker
- **WHEN** a marker line is indented to match code indentation
- **THEN** the marker SHALL be detected regardless of indentation level

### Requirement: Special File Types
The system SHALL support special file types that may not have standard extensions.

#### Scenario: Markdown files
- **WHEN** a file has extension `.md`
- **THEN** markers in HTML comments SHALL be detected
- **AND** `<!-- conclaude-uneditable:start -->` SHALL work natively

#### Scenario: HTML/XML files
- **WHEN** a file has extension `.html`, `.xml`, or `.svg`
- **THEN** markers in HTML comments SHALL be detected
- **AND** `<!-- conclaude-uneditable:start -->` SHALL work natively

#### Scenario: YAML files
- **WHEN** a file has extension `.yaml` or `.yml`
- **THEN** the system SHALL use `#` as the comment pattern
- **AND** markers SHALL be detected in lines starting with `#`

#### Scenario: JSON files
- **WHEN** a file has extension `.json`
- **THEN** the system SHALL NOT support markers (JSON has no comment syntax)
- **AND** return an empty protected ranges list

### Requirement: Language-Agnostic Marker Format
The system SHALL use a consistent marker format (`<!-- conclaude-uneditable:start/end -->`) across all languages, embedded in language-specific comments.

#### Scenario: Same marker in different languages
- **WHEN** the marker `<!-- conclaude-uneditable:start -->` is used in Go (`//`), Python (`#`), and HTML
- **THEN** the marker SHALL be detected in all three languages
- **AND** the marker format SHALL be identical in all cases

#### Scenario: Marker format consistency
- **WHEN** developers copy markers between different language files
- **THEN** the markers SHALL work without modification
- **AND** only the surrounding comment syntax SHALL differ

### Requirement: Performance Across Languages
The system SHALL efficiently detect markers across all supported languages with consistent performance.

#### Scenario: Language detection overhead
- **WHEN** detecting language for any file
- **THEN** detection SHALL complete in under 1ms
- **AND** add negligible overhead to marker parsing

#### Scenario: Pattern matching overhead
- **WHEN** scanning for markers using language-specific patterns
- **THEN** performance SHALL be equivalent across all languages
- **AND** no language SHALL have significantly slower marker detection

### Requirement: Error Handling for Unsupported Scenarios
The system SHALL gracefully handle scenarios where marker detection is not possible or meaningful.

#### Scenario: Binary file
- **WHEN** a file is detected as binary (e.g., `.png`, `.exe`)
- **THEN** marker detection SHALL be skipped
- **AND** an empty protected ranges list SHALL be returned
- **AND** no errors SHALL be raised

#### Scenario: Empty file
- **WHEN** a file is empty (0 bytes)
- **THEN** marker detection SHALL return empty protected ranges
- **AND** no errors SHALL be raised

#### Scenario: File read error
- **WHEN** a file cannot be read (permissions, not found, etc.)
- **THEN** marker detection SHALL propagate the error
- **AND** the PreToolUse hook SHALL fail with a clear error message
