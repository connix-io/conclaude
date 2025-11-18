# preToolUse Root Addition Prevention Specification

## Purpose

Define the refined enforcement behavior of the `preventRootAdditions` configuration setting to block only file **creation** at the repository root while allowing **modification** of existing root-level files.

## MODIFIED Requirements

### Requirement: Prevention of New Root-Level Files with Write Tool

The system SHALL block `Write` tool operations that create **new** files at the repository root level when `preventRootAdditions` is enabled. However, the system SHALL allow modifications to existing root-level files.

**Previous behavior:** Blocked all Write operations on root-level files, including updates to existing files (overly restrictive).

**New behavior:** Only blocks creation of new files at root; allows editing and overwriting existing root files (balanced protection).

#### Scenario: Block creation of new root-level file
- **GIVEN** configuration contains `rules.preventRootAdditions: true`
- **AND** file `README.md` does not exist
- **WHEN** Claude attempts to use Write tool to create `README.md` at root
- **THEN** the operation SHALL be blocked
- **AND** error message SHALL indicate preventRootAdditions rule prevents creating files at root

#### Scenario: Allow modification of existing root-level file
- **GIVEN** configuration contains `rules.preventRootAdditions: true`
- **AND** file `package.json` exists at root
- **WHEN** Claude attempts to use Write tool to overwrite/modify `package.json`
- **THEN** the operation SHALL be allowed
- **AND** no error message SHALL be generated for preventRootAdditions

#### Scenario: Allow editing existing root file with Edit tool
- **GIVEN** configuration contains `rules.preventRootAdditions: true`
- **AND** file `.env` exists at root
- **WHEN** Claude attempts to use Edit tool to modify `.env`
- **THEN** the operation SHALL be allowed by preventRootAdditions
- **AND** the operation may be subject to uneditableFiles checks but not preventRootAdditions

#### Scenario: Block creation of multiple root files
- **GIVEN** configuration contains `rules.preventRootAdditions: true`
- **WHEN** Claude attempts to create `tsconfig.json` at root (new file)
- **THEN** the operation SHALL be blocked
- **WHEN** Claude attempts to create `.gitignore` at root (new file)
- **THEN** the operation SHALL be blocked

#### Scenario: Distinguish between new and existing root files
- **GIVEN** configuration contains `rules.preventRootAdditions: true`
- **WHEN** Claude attempts to create new file `docker-compose.yml` (does not exist)
- **THEN** the operation SHALL be blocked
- **WHEN** the same file now exists and Claude attempts to update it
- **THEN** the operation SHALL be allowed (modification, not addition)

### Requirement: Root-Level Definition

The system SHALL determine "root-level" as the directory containing the configuration file (where `.conclaude.yaml` is located).

#### Scenario: Root is config directory
- **GIVEN** configuration file `.conclaude.yaml` is at `/project/.conclaude.yaml`
- **THEN** the repository root for preventRootAdditions checks is `/project/`
- **WHEN** Claude attempts to create `/project/README.md`
- **THEN** the operation SHALL be blocked (new root file)

#### Scenario: File in subdirectory allowed
- **GIVEN** configuration file at `/project/.conclaude.yaml`
- **WHEN** Claude attempts to create `/project/src/index.ts` (in subdirectory)
- **THEN** the operation SHALL be allowed (not at root level)

### Requirement: Write Tool Exclusivity

The system SHALL only apply `preventRootAdditions` checks to the `Write` tool. Edit and NotebookEdit tools are not affected by this rule.

#### Scenario: Edit tool not affected by preventRootAdditions
- **GIVEN** configuration contains `rules.preventRootAdditions: true`
- **WHEN** Claude uses Edit tool to modify file at root
- **THEN** the operation SHALL NOT be blocked by preventRootAdditions

#### Scenario: NotebookEdit tool not affected by preventRootAdditions
- **GIVEN** configuration contains `rules.preventRootAdditions: true`
- **WHEN** Claude uses NotebookEdit tool to modify root-level notebook
- **THEN** the operation SHALL NOT be blocked by preventRootAdditions

### Requirement: File Existence Detection

The system SHALL check if a file exists at the target path before determining whether to block a Write operation.

#### Scenario: File existence prevents blocking
- **GIVEN** configuration contains `rules.preventRootAdditions: true`
- **AND** file `Dockerfile` exists at root
- **WHEN** Claude attempts Write to `Dockerfile`
- **THEN** the system SHALL detect file exists
- **AND** the operation SHALL be allowed (modification, not addition)

#### Scenario: Non-existent file is blocked
- **GIVEN** configuration contains `rules.preventRootAdditions: true`
- **AND** file `docker-compose.yml` does NOT exist at root
- **WHEN** Claude attempts Write to `docker-compose.yml`
- **THEN** the system SHALL detect file does not exist
- **AND** the operation SHALL be blocked (new file at root)

### Requirement: Error Messages for Blocked Operations

The system SHALL provide clear error messages when preventRootAdditions blocks a Write operation to create a new root-level file.

#### Scenario: Error message for blocked root addition
- **GIVEN** configuration contains `rules.preventRootAdditions: true`
- **WHEN** Claude attempts to create new file `LICENSE` at root
- **THEN** error message SHALL indicate:
  - The tool name (`Write`)
  - That preventRootAdditions rule prevented the operation
  - The attempted file path (e.g., `LICENSE`)
- **AND** error format SHALL be: `"Blocked Write operation: preventRootAdditions rule prevents creating files at repository root. File: LICENSE"`

#### Scenario: Error message example
- **WHEN** preventRootAdditions blocks `README.md`
- **THEN** error message SHALL be exactly: `"Blocked Write operation: preventRootAdditions rule prevents creating files at repository root. File: README.md"`

### Requirement: Interaction with Other File Protection Rules

The system SHALL enforce `preventRootAdditions` alongside other file protection mechanisms independently.

#### Scenario: preventRootAdditions with uneditableFiles
- **GIVEN** configuration contains:
  - `rules.preventRootAdditions: true`
  - `rules.uneditableFiles: ["package.json"]`
- **WHEN** Claude attempts to create new `README.md` at root (does not exist)
- **THEN** the operation SHALL be blocked by preventRootAdditions
- **WHEN** Claude attempts to edit `package.json` at root (exists)
- **THEN** the operation SHALL be blocked by uneditableFiles (takes precedence)
- **WHEN** Claude attempts to edit `.env` at root (exists, not in uneditableFiles)
- **THEN** the operation SHALL be allowed

#### Scenario: preventRootAdditions does not affect preventAdditions
- **GIVEN** configuration contains:
  - `rules.preventRootAdditions: true`
  - `preToolUse.preventAdditions: ["dist/**"]`
- **WHEN** Claude attempts to create `/dist/output.js`
- **THEN** the operation SHALL be blocked by preventAdditions (not root-level)
- **WHEN** Claude attempts to create root-level `.env`
- **THEN** the operation SHALL be blocked by preventRootAdditions
- **AND** both rules apply independently

### Requirement: Backwards Compatibility

The system SHALL maintain backwards compatibility with existing configurations using `preventRootAdditions`.

#### Scenario: Existing configuration works unchanged
- **GIVEN** configuration with `rules.preventRootAdditions: true` from previous version
- **WHEN** updated to new version with refined semantics
- **THEN** the configuration SHALL work unchanged
- **AND** no migration or changes required
- **AND** behavior becomes more permissive (allows edits)

#### Scenario: No configuration changes needed
- **GIVEN** users with preventRootAdditions enabled
- **WHEN** they update to refined version
- **THEN** they immediately gain ability to edit root files
- **AND** no explicit configuration changes required

### Requirement: Path Resolution and Existence Checking

The system SHALL correctly resolve file paths and determine existence before making prevention decisions.

#### Scenario: Relative paths resolve correctly
- **GIVEN** configuration at `/project/.conclaude.yaml`
- **WHEN** Claude attempts Write to `./package.json` (relative path)
- **THEN** the path SHALL resolve to `/project/package.json`
- **AND** if file exists, operation is allowed

#### Scenario: Symlinks resolve to actual file
- **GIVEN** configuration at `/project/.conclaude.yaml`
- **WHEN** Claude attempts Write to symlink at root
- **THEN** the system SHALL resolve symlink to actual location
- **AND** existence check uses resolved path

#### Scenario: Case-sensitive file systems
- **GIVEN** configuration at `/project/.conclaude.yaml`
- **AND** Linux environment (case-sensitive)
- **WHEN** Claude attempts Write to `Package.json` vs `package.json`
- **THEN** these SHALL be treated as different files
- **AND** existence check respects OS case-sensitivity

### Requirement: Edge Case Handling

The system SHALL handle edge cases gracefully.

#### Scenario: Parent directories created automatically
- **GIVEN** configuration contains `rules.preventRootAdditions: true`
- **WHEN** Claude attempts Write to `subdir/file.txt` (parent dir doesn't exist)
- **THEN** the operation SHALL be allowed (not at root)
- **AND** parent directory creation is separate from preventRootAdditions

#### Scenario: File without write permissions
- **GIVEN** configuration contains `rules.preventRootAdditions: true`
- **AND** existing file at root with no write permissions
- **WHEN** Claude attempts Write to modify that file
- **THEN** preventRootAdditions SHALL allow the operation
- **AND** the Write tool failure is OS-level permission issue, not preventRootAdditions

#### Scenario: Overwrite with same contents
- **GIVEN** configuration contains `rules.preventRootAdditions: true`
- **AND** file `config.json` exists at root
- **WHEN** Claude attempts Write with identical contents
- **THEN** the operation SHALL be allowed (file exists)

## REMOVED Requirements

None. This change refines existing functionality, does not remove features.

## ADDED Requirements

### Requirement: File Existence Check for Root Additions

The system SHALL check if a target file exists to distinguish between creation and modification operations.

#### Scenario: Existence check prevents false positives
- **GIVEN** configuration contains `rules.preventRootAdditions: true`
- **WHEN** determining whether to block a Write operation
- **THEN** the system SHALL check if the file exists
- **AND** only block if file does NOT exist at root

---

**Summary:** preventRootAdditions now correctly allows modifications to existing root-level files while maintaining protection against creating new files at the root. This preserves the semantic meaning of "preventRootAdditions" (prevent adding/creating files at root) while enabling practical workflows that require updating configuration files.
