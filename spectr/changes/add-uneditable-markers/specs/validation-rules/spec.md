# Specification: Validation Rules (Uneditable Markers)

## Purpose
Define validation rules for detecting and enforcing uneditable code range markers in source files, protecting specific line ranges from modification by Claude Code.

## ADDED Requirements

### Requirement: Marker Detection in Files
The system SHALL detect `conclaude-uneditable` markers embedded in source code comments across all file types.

#### Scenario: Markers present in file
- **WHEN** a file contains `<!-- conclaude-uneditable:start -->` and `<!-- conclaude-uneditable:end -->` markers in comments
- **THEN** the system SHALL extract the protected line ranges
- **AND** the ranges SHALL include the marker lines themselves (inclusive)

#### Scenario: No markers in file
- **WHEN** a file does not contain any `conclaude-uneditable` markers
- **THEN** the system SHALL return an empty list of protected ranges
- **AND** no validation errors SHALL be raised

#### Scenario: Multiple protected ranges in single file
- **WHEN** a file contains multiple pairs of start/end markers
- **THEN** the system SHALL extract all protected ranges independently
- **AND** each range SHALL be validated separately

### Requirement: Protected Range Extraction
The system SHALL extract protected line ranges from detected markers and represent them with start and end line numbers (1-indexed, inclusive).

#### Scenario: Simple protected range
- **WHEN** markers are on lines 10 and 15
- **THEN** the protected range SHALL be lines 10-15 (inclusive)
- **AND** any edit touching lines 10-15 SHALL be blocked

#### Scenario: Single-line range
- **WHEN** start and end markers are on consecutive lines (e.g., lines 5 and 6)
- **THEN** the protected range SHALL be lines 5-6
- **AND** the range SHALL be valid and enforced

#### Scenario: Large range
- **WHEN** markers span hundreds of lines (e.g., lines 1 and 500)
- **THEN** the system SHALL extract the range correctly
- **AND** performance SHALL remain acceptable (< 10ms parsing time)

### Requirement: Edit Operation Validation
The system SHALL validate `Edit` and `Write` operations against protected ranges and block operations that overlap with protected code.

#### Scenario: Edit overlaps protected range
- **WHEN** an `Edit` operation targets lines 12-14
- **AND** a protected range is lines 10-20
- **THEN** the system SHALL block the operation
- **AND** return a clear error message indicating the protected range

#### Scenario: Edit outside protected range
- **WHEN** an `Edit` operation targets lines 5-8
- **AND** a protected range is lines 10-20
- **THEN** the system SHALL allow the operation
- **AND** no error SHALL be returned

#### Scenario: Edit partially overlaps start of range
- **WHEN** an `Edit` operation targets lines 8-12
- **AND** a protected range is lines 10-20
- **THEN** the system SHALL block the operation
- **AND** indicate that lines 10-12 are protected

#### Scenario: Edit partially overlaps end of range
- **WHEN** an `Edit` operation targets lines 18-22
- **AND** a protected range is lines 10-20
- **THEN** the system SHALL block the operation
- **AND** indicate that lines 18-20 are protected

#### Scenario: Edit fully contains protected range
- **WHEN** an `Edit` operation targets lines 5-25
- **AND** a protected range is lines 10-20
- **THEN** the system SHALL block the operation
- **AND** indicate the protected range is within the edit scope

#### Scenario: Edit is fully contained by protected range
- **WHEN** an `Edit` operation targets lines 12-15
- **AND** a protected range is lines 10-20
- **THEN** the system SHALL block the operation
- **AND** indicate the entire edit is within protected range

### Requirement: Write Operation Validation
The system SHALL block `Write` operations that would overwrite files containing protected ranges.

#### Scenario: Write to file with protected ranges
- **WHEN** a `Write` operation targets a file
- **AND** the file already exists with protected ranges
- **THEN** the system SHALL block the operation
- **AND** indicate which protected ranges would be affected

#### Scenario: Write to new file
- **WHEN** a `Write` operation creates a new file
- **AND** the file does not yet exist
- **THEN** the system SHALL allow the operation
- **AND** no marker validation SHALL be performed

### Requirement: Marker Pairing Validation
The system SHALL validate that markers are properly paired and report errors for malformed marker structures.

#### Scenario: Unclosed start marker
- **WHEN** a file contains `<!-- conclaude-uneditable:start -->` without a matching end marker
- **THEN** the system SHALL return a validation error
- **AND** block ALL edit operations on the file
- **AND** indicate the line number of the unclosed marker

#### Scenario: Unmatched end marker
- **WHEN** a file contains `<!-- conclaude-uneditable:end -->` without a preceding start marker
- **THEN** the system SHALL return a validation error
- **AND** block ALL edit operations on the file
- **AND** indicate the line number of the unmatched marker

#### Scenario: Properly paired markers
- **WHEN** all start markers have matching end markers in correct order
- **THEN** the system SHALL successfully extract protected ranges
- **AND** no validation errors SHALL be raised

### Requirement: Nested Marker Detection
The system SHALL detect nested or overlapping markers and report them as errors.

#### Scenario: Nested markers
- **WHEN** a file contains start-start-end-end marker sequence
- **THEN** the system SHALL return a validation error
- **AND** block ALL edit operations on the file
- **AND** indicate the line numbers of conflicting markers

#### Scenario: Overlapping markers
- **WHEN** marker ranges overlap (e.g., 1-10 and 5-15)
- **THEN** the system SHALL return a validation error
- **AND** indicate the line numbers of overlapping ranges

### Requirement: Error Message Clarity
The system SHALL provide clear, actionable error messages when blocking operations due to protected ranges.

#### Scenario: Blocked edit error message
- **WHEN** an edit is blocked due to protected range overlap
- **THEN** the error message SHALL include:
  - The operation type (Edit/Write)
  - The protected line range (start-end)
  - The file path
  - Indication that markers are present

#### Scenario: Malformed marker error message
- **WHEN** markers are malformed (unclosed, nested, etc.)
- **THEN** the error message SHALL include:
  - The specific error (unclosed/unmatched/nested)
  - The line number(s) involved
  - Guidance on how to fix the issue

### Requirement: Performance Requirements
The system SHALL efficiently detect and validate markers with minimal performance impact on hook execution.

#### Scenario: Small file parsing
- **WHEN** parsing a file under 1000 lines
- **THEN** marker detection SHALL complete in under 5ms

#### Scenario: Large file parsing
- **WHEN** parsing a file between 1000-10000 lines
- **THEN** marker detection SHALL complete in under 10ms

#### Scenario: Very large file parsing
- **WHEN** parsing a file over 10000 lines
- **THEN** marker detection SHALL complete in under 50ms
- **AND** not block other hook operations

### Requirement: PreToolUse Hook Integration
The system SHALL integrate marker validation into the existing PreToolUse hook workflow without breaking existing functionality.

#### Scenario: Integration with existing rules
- **WHEN** both `uneditableFiles` glob patterns and marker-based protection are configured
- **THEN** both SHALL be enforced independently
- **AND** if either blocks an operation, the operation SHALL be blocked
- **AND** the most specific error message SHALL be returned

#### Scenario: Marker validation before tool execution
- **WHEN** a PreToolUse hook is triggered for Edit/Write operations
- **THEN** marker validation SHALL occur before the tool executes
- **AND** blocked operations SHALL never reach the tool
- **AND** allowed operations SHALL proceed normally

#### Scenario: Non-Edit/Write tools unaffected
- **WHEN** a PreToolUse hook is triggered for tools other than Edit/Write (e.g., Read, Bash)
- **THEN** marker validation SHALL be skipped
- **AND** performance SHALL not be impacted
