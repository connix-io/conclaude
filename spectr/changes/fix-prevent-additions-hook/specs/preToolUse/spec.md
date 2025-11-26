# preToolUse File Addition Prevention Specification

## Purpose
Define the enforcement behavior of the `preventAdditions` configuration setting within `preToolUse` to block Claude from creating files in specified directories using glob patterns.

## ADDED Requirements

### Requirement: File Addition Prevention via Glob Patterns
The system SHALL enforce the `preventAdditions` configuration by blocking `Write` tool operations that create NEW files at paths matching configured glob patterns. Existing files can be overwritten.

**Previous behavior (BROKEN):** `preventAdditions` field existed in schema but was never checked by the hook, causing silent failure.

**New behavior (FIXED):** `preventAdditions` patterns are enforced during PreToolUse hook execution for Write tool operations creating new files.

#### Scenario: Exact directory pattern blocks file creation
- **GIVEN** configuration contains `preventAdditions: ["dist"]`
- **WHEN** Claude attempts to use Write tool to create file `dist/output.js`
- **THEN** the operation SHALL be blocked before execution
- **AND** error message SHALL indicate the file matches pattern `"dist"` and show the attempted path

#### Scenario: Recursive directory pattern blocks nested files
- **GIVEN** configuration contains `preventAdditions: ["build/**"]`
- **WHEN** Claude attempts to use Write tool to create file `build/nested/deep/file.js`
- **THEN** the operation SHALL be blocked
- **AND** error message SHALL indicate the file matches pattern `"build/**"`

#### Scenario: File extension pattern blocks files
- **GIVEN** configuration contains `preventAdditions: ["*.log"]`
- **WHEN** Claude attempts to use Write tool to create file `debug.log`
- **THEN** the operation SHALL be blocked
- **AND** error message SHALL indicate the file matches pattern `"*.log"`

#### Scenario: Multiple patterns all enforced
- **GIVEN** configuration contains `preventAdditions: ["dist/**", "build/**", "*.log"]`
- **WHEN** Claude attempts to create any file matching any of the patterns
- **THEN** the operation SHALL be blocked with appropriate pattern indicated
- **AND** Claude attempts to create file not matching any pattern (e.g., `src/main.rs`)
- **THEN** the operation SHALL be allowed to proceed

#### Scenario: Non-matching paths are allowed
- **GIVEN** configuration contains `preventAdditions: ["dist/**"]`
- **WHEN** Claude attempts to use Write tool to create file `src/components/Button.tsx`
- **THEN** the operation SHALL be allowed (no pattern match)
- **AND** no error message SHALL be generated

#### Scenario: Empty preventAdditions array allows all operations
- **GIVEN** configuration contains `preventAdditions: []`
- **WHEN** Claude attempts to use Write tool to create any file
- **THEN** the operation SHALL be allowed (no patterns to check)
- **AND** no preventAdditions validation SHALL occur

#### Scenario: Existing files can be overwritten
- **GIVEN** configuration contains `preventAdditions: ["docs/**"]`
- **AND** file `docs/README.md` already exists
- **WHEN** Claude attempts to use Write tool to overwrite `docs/README.md`
- **THEN** the operation SHALL be allowed (file already exists)
- **AND** preventAdditions only blocks creation of NEW files, not overwrites

### Requirement: Write Tool Exclusivity for preventAdditions
The system SHALL only apply `preventAdditions` checks to the `Write` tool, not to `Edit` or `NotebookEdit` tools.

#### Scenario: Edit tool bypasses preventAdditions
- **GIVEN** configuration contains `preventAdditions: ["dist/**"]`
- **AND** file `dist/existing.js` already exists
- **WHEN** Claude attempts to use Edit tool to modify `dist/existing.js`
- **THEN** the operation SHALL NOT be blocked by preventAdditions
- **AND** the operation may be subject to `uneditableFiles` checks but not preventAdditions

#### Scenario: NotebookEdit tool bypasses preventAdditions
- **GIVEN** configuration contains `preventAdditions: ["notebooks/**"]`
- **AND** file `notebooks/analysis.ipynb` exists
- **WHEN** Claude attempts to use NotebookEdit tool to modify the notebook
- **THEN** the operation SHALL NOT be blocked by preventAdditions
- **AND** preventAdditions validation SHALL not run for this tool

### Requirement: preventAdditions Error Reporting
The system SHALL provide clear, actionable error messages when preventAdditions blocks a file creation operation.

#### Scenario: Error message includes all context
- **WHEN** preventAdditions blocks a Write operation
- **THEN** error message SHALL include:
  - The tool name (`Write`)
  - The matching glob pattern (e.g., `"dist/**"`)
  - The attempted file path (e.g., `dist/output.js`)
- **AND** error format SHALL be: `"Blocked {tool} operation: file matches preToolUse.preventAdditions pattern '{pattern}'. File: {path}"`

#### Scenario: Diagnostic logging for debugging
- **WHEN** preventAdditions blocks an operation
- **THEN** a diagnostic message SHALL be logged to stderr
- **AND** log message SHALL include: tool_name, file_path, and matching pattern
