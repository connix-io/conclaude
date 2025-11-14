# preToolUse File Addition Prevention Specification

## Purpose
Define the enforcement behavior of the `preventAdditions` configuration setting within `preToolUse` to block Claude from creating files in specified directories using glob patterns.

## MODIFIED Requirements

### Requirement: File Addition Prevention via Glob Patterns
The system SHALL enforce the `preventAdditions` configuration by blocking `Write` tool operations that target paths matching configured glob patterns.

**Previous behavior (BROKEN):** `preventAdditions` field existed in schema but was never checked by the hook, causing silent failure.

**New behavior (FIXED):** `preventAdditions` patterns are enforced during PreToolUse hook execution for Write tool operations.

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

### Requirement: Write Tool Exclusivity
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

#### Scenario: Write tool to existing file allowed
- **GIVEN** configuration contains `preventAdditions: ["dist/**"]`
- **AND** file `dist/output.js` already exists
- **WHEN** Claude attempts to use Write tool to overwrite `dist/output.js`
- **THEN** the operation SHALL be allowed (Write can overwrite existing files)
- **AND** preventAdditions only blocks creation, not modification

### Requirement: Path Normalization and Matching
The system SHALL normalize file paths and match them against preventAdditions patterns using consistent path resolution.

#### Scenario: Relative paths normalize correctly
- **GIVEN** configuration contains `preventAdditions: ["dist/**"]`
- **WHEN** Claude attempts to create file with relative path `./dist/output.js`
- **THEN** the path SHALL normalize to `dist/output.js`
- **AND** the operation SHALL be blocked by pattern `"dist/**"`

#### Scenario: Absolute paths resolve correctly
- **GIVEN** configuration contains `preventAdditions: ["dist/**"]`
- **WHEN** Claude attempts to create file with absolute path `/home/user/project/dist/output.js`
- **THEN** the path SHALL resolve relative to working directory
- **AND** if resolved path is `dist/output.js`, it SHALL be blocked

#### Scenario: Parent directory references normalize
- **GIVEN** configuration contains `preventAdditions: ["dist/**"]`
- **WHEN** Claude attempts to create file `src/../dist/output.js`
- **THEN** the path SHALL normalize to `dist/output.js`
- **AND** the operation SHALL be blocked by pattern `"dist/**"`

### Requirement: Error Reporting for Blocked Operations
The system SHALL provide clear, actionable error messages when preventAdditions blocks a file creation operation.

#### Scenario: Error message includes all context
- **WHEN** preventAdditions blocks a Write operation
- **THEN** error message SHALL include:
  - The tool name (`Write`)
  - The matching glob pattern (e.g., `"dist/**"`)
  - The attempted file path (e.g., `dist/output.js`)
- **AND** error format SHALL be: `"Blocked {tool} operation: file matches preventAdditions pattern '{pattern}'. File: {path}"`

#### Scenario: Error example for directory pattern
- **GIVEN** configuration contains `preventAdditions: ["build/**"]`
- **WHEN** Claude attempts to create `build/output.js` with Write tool
- **THEN** error message SHALL be exactly: `"Blocked Write operation: file matches preventAdditions pattern 'build/**'. File: build/output.js"`

#### Scenario: Error example for file extension pattern
- **GIVEN** configuration contains `preventAdditions: ["*.log"]`
- **WHEN** Claude attempts to create `debug.log` with Write tool
- **THEN** error message SHALL be exactly: `"Blocked Write operation: file matches preventAdditions pattern '*.log'. File: debug.log"`

#### Scenario: Diagnostic logging for debugging
- **WHEN** preventAdditions blocks an operation
- **THEN** a diagnostic message SHALL be logged to stderr
- **AND** log message SHALL include: tool_name, file_path, and matching pattern
- **AND** log format SHALL be: `"PreToolUse blocked by preventAdditions rule: tool_name={tool}, file_path={path}, pattern={pattern}"`

### Requirement: Interaction with Other File Protection Rules
The system SHALL enforce `preventAdditions` alongside other file protection mechanisms independently.

#### Scenario: preventAdditions with preventRootAdditions
- **GIVEN** configuration contains:
  - `preventRootAdditions: true`
  - `preventAdditions: ["dist/**"]`
- **WHEN** Claude attempts to create root-level file `README.md`
- **THEN** operation SHALL be blocked by `preventRootAdditions` (checked first)
- **WHEN** Claude attempts to create `dist/output.js`
- **THEN** operation SHALL be blocked by `preventAdditions`
- **AND** both rules are evaluated independently

#### Scenario: preventAdditions with uneditableFiles
- **GIVEN** configuration contains:
  - `uneditableFiles: ["package.json"]`
  - `preventAdditions: ["dist/**"]`
- **WHEN** Claude attempts to Write to `package.json` (Edit or Write)
- **THEN** operation SHALL be blocked by `uneditableFiles` (checked first)
- **WHEN** Claude attempts to Write to `dist/output.js`
- **THEN** operation SHALL be blocked by `preventAdditions`
- **AND** both rules apply to Write tool independently

#### Scenario: preventAdditions does not affect Edit tool checks
- **GIVEN** configuration contains:
  - `uneditableFiles: ["*.lock"]`
  - `preventAdditions: ["dist/**"]`
- **WHEN** Claude attempts to Edit existing file `dist/config.js`
- **THEN** operation SHALL NOT be blocked by preventAdditions (Edit tool)
- **AND** operation SHALL be subject to uneditableFiles check only
- **WHEN** Claude attempts to Edit `package-lock.json`
- **THEN** operation SHALL be blocked by `uneditableFiles`

#### Scenario: Multiple rules can block same operation
- **GIVEN** configuration contains:
  - `preventRootAdditions: true`
  - `uneditableFiles: [".env"]`
  - `preventAdditions: [".env"]`
- **WHEN** Claude attempts to Write root-level `.env` file
- **THEN** operation SHALL be blocked by `preventRootAdditions` (checked first in order)
- **AND** if preventRootAdditions were disabled, `uneditableFiles` would block it next
- **AND** if uneditableFiles were empty, `preventAdditions` would block it last

### Requirement: Glob Pattern Semantics
The system SHALL support standard glob pattern matching for preventAdditions patterns.

#### Scenario: Single asterisk matches within directory level
- **GIVEN** configuration contains `preventAdditions: ["dist/*"]`
- **WHEN** Claude attempts to create `dist/output.js`
- **THEN** operation SHALL be blocked (matches single-level wildcard)
- **WHEN** Claude attempts to create `dist/nested/output.js`
- **THEN** operation SHALL NOT be blocked (requires `**` for recursive matching)

#### Scenario: Double asterisk matches recursively
- **GIVEN** configuration contains `preventAdditions: ["dist/**"]`
- **WHEN** Claude attempts to create `dist/nested/deep/output.js`
- **THEN** operation SHALL be blocked (recursive wildcard matches any depth)

#### Scenario: Question mark matches single character
- **GIVEN** configuration contains `preventAdditions: ["test?.log"]`
- **WHEN** Claude attempts to create `test1.log`
- **THEN** operation SHALL be blocked (single char wildcard matches)
- **WHEN** Claude attempts to create `test10.log`
- **THEN** operation SHALL NOT be blocked (only matches single character)

#### Scenario: Extension wildcard matches files
- **GIVEN** configuration contains `preventAdditions: ["*.log"]`
- **WHEN** Claude attempts to create `debug.log`, `error.log`, or `app.log`
- **THEN** all operations SHALL be blocked (extension pattern matches)
- **WHEN** Claude attempts to create `log.txt`
- **THEN** operation SHALL be allowed (extension doesn't match)

### Requirement: Edge Case Handling
The system SHALL handle edge cases and error conditions gracefully.

#### Scenario: Invalid glob pattern causes error
- **GIVEN** configuration contains `preventAdditions: ["[invalid"]` (malformed pattern)
- **WHEN** hook attempts to evaluate pattern
- **THEN** configuration loading or pattern evaluation SHALL fail with descriptive error
- **AND** error SHALL indicate the invalid pattern syntax

#### Scenario: Pattern with trailing slash for directories
- **GIVEN** configuration contains `preventAdditions: ["dist/"]`
- **WHEN** Claude attempts to create `dist/output.js` inside dist directory
- **THEN** operation SHALL be blocked (directory pattern matches)

#### Scenario: Case-sensitive matching
- **GIVEN** configuration contains `preventAdditions: ["Dist/**"]`
- **WHEN** Claude attempts to create `dist/output.js` (lowercase)
- **THEN** operation SHALL be allowed (pattern is case-sensitive, "Dist" != "dist")
- **WHEN** Claude attempts to create `Dist/output.js` (uppercase)
- **THEN** operation SHALL be blocked (exact case match)

### Requirement: Performance and Optimization
The system SHALL minimize performance impact of preventAdditions checking.

#### Scenario: Empty preventAdditions has zero overhead
- **GIVEN** configuration contains `preventAdditions: []`
- **WHEN** Write tool operations are performed
- **THEN** preventAdditions validation loop SHALL be skipped entirely
- **AND** no pattern matching overhead SHALL occur

#### Scenario: Early exit on first pattern match
- **GIVEN** configuration contains `preventAdditions: ["dist/**", "build/**", "tmp/**"]`
- **WHEN** Claude attempts to create `dist/output.js`
- **THEN** validation SHALL stop after matching first pattern `"dist/**"`
- **AND** remaining patterns SHALL NOT be evaluated (optimization)

#### Scenario: preventAdditions only checked for Write tool
- **GIVEN** configuration contains `preventAdditions: ["dist/**"]`
- **WHEN** Claude uses Edit, NotebookEdit, or Read tools
- **THEN** preventAdditions validation SHALL NOT run at all
- **AND** no performance impact occurs for non-Write operations

## REMOVED Requirements

None. This change fixes existing functionality, does not remove features.

## ADDED Requirements

None beyond the MODIFIED requirements above. This change restores intended behavior of existing `preventAdditions` field.
