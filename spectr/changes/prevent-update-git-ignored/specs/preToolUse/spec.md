# preToolUse Git-Ignored File Protection Specification

## Purpose
Define the `preventUpdateGitIgnored` configuration setting within `preToolUse` to allow blocking Claude from modifying or creating files that are git-ignored.

## ADDED Requirements

### Requirement: Git-Ignored File Protection Configuration
The system SHALL provide an optional `preventUpdateGitIgnored` boolean field in the `preToolUse` configuration to block Claude from modifying or creating files that match entries in `.gitignore`.

#### Scenario: preventUpdateGitIgnored enabled
- **WHEN** `preToolUse.preventUpdateGitIgnored` is set to `true`
- **THEN** the system SHALL check if any requested file operation targets a path that matches an entry in `.gitignore`
- **AND** if matched, the operation SHALL be blocked with a clear error message
- **AND** if not matched, the operation SHALL proceed normally

#### Scenario: preventUpdateGitIgnored disabled
- **WHEN** `preToolUse.preventUpdateGitIgnored` is set to `false`
- **THEN** git-ignore rules SHALL NOT be evaluated
- **AND** Claude SHALL be allowed to create or modify files freely (subject to other restrictions)
- **AND** existing behavior is preserved

#### Scenario: Default behavior
- **WHEN** `preToolUse.preventUpdateGitIgnored` is not specified in configuration
- **THEN** the system SHALL default to `preventUpdateGitIgnored: false`
- **AND** git-ignored files are not automatically protected

### Requirement: Git-Ignore Pattern Matching
The system SHALL correctly evaluate files against `.gitignore` patterns using git-standard semantics.

#### Scenario: Simple pattern match
- **WHEN** `.gitignore` contains `node_modules`
- **AND** Claude attempts to modify `node_modules/package.json`
- **AND** `preventUpdateGitIgnored: true`
- **THEN** the operation SHALL be blocked
- **AND** an error message SHALL indicate the file is git-ignored

#### Scenario: Glob pattern match
- **WHEN** `.gitignore` contains `*.log`
- **AND** Claude attempts to create `debug.log`
- **AND** `preventUpdateGitIgnored: true`
- **THEN** the operation SHALL be blocked

#### Scenario: Directory pattern match
- **WHEN** `.gitignore` contains `.env` (exact filename)
- **AND** Claude attempts to modify `.env` in the repository root
- **AND** `preventUpdateGitIgnored: true`
- **THEN** the operation SHALL be blocked

#### Scenario: Nested .gitignore files
- **WHEN** repository contains `.gitignore` at root and `src/.gitignore` in a subdirectory
- **AND** `src/.gitignore` contains `local-config.json`
- **AND** Claude attempts to modify `src/local-config.json`
- **AND** `preventUpdateGitIgnored: true`
- **THEN** the operation SHALL be blocked

#### Scenario: Negation patterns
- **WHEN** `.gitignore` contains `*.log` followed by `!important.log`
- **AND** Claude attempts to modify `important.log`
- **AND** `preventUpdateGitIgnored: true`
- **THEN** the operation SHALL NOT be blocked (negation pattern allows it)

#### Scenario: Comments in .gitignore
- **WHEN** `.gitignore` contains `# Comment` on a line
- **AND** Claude attempts to create a file named `# Comment`
- **AND** `preventUpdateGitIgnored: true`
- **THEN** the operation SHALL proceed (comment lines are not patterns)

### Requirement: File Operation Blocking Scope
The system SHALL block Read, Write, and Edit operations that target git-ignored paths. Glob operations are NOT blocked.

#### Scenario: Block Read operation
- **WHEN** `.gitignore` contains `.env`
- **AND** Claude uses `Read` tool to read `.env`
- **AND** `preventUpdateGitIgnored: true`
- **THEN** the operation SHALL be blocked before execution

#### Scenario: Block Write operation (file creation)
- **WHEN** `.gitignore` contains `*.tmp`
- **AND** Claude uses `Write` tool to create `session.tmp`
- **AND** `preventUpdateGitIgnored: true`
- **THEN** the operation SHALL be blocked before execution

#### Scenario: Block Edit operation (file modification)
- **WHEN** `.gitignore` contains `config.local`
- **AND** Claude uses `Edit` tool to modify existing `config.local` file
- **AND** `preventUpdateGitIgnored: true`
- **THEN** the operation SHALL be blocked before execution

#### Scenario: Allow Glob operations
- **WHEN** `.gitignore` contains `node_modules/`
- **AND** Claude uses `Glob` tool with pattern `**/*.js`
- **AND** `preventUpdateGitIgnored: true`
- **THEN** the operation SHALL be allowed (Glob is not blocked)
- **AND** Glob results may include git-ignored files

#### Scenario: Allow operations on non-ignored files
- **WHEN** `.gitignore` contains `*.log`
- **AND** Claude attempts to Read or Write `src/main.ts`
- **AND** `preventUpdateGitIgnored: true`
- **THEN** the operation SHALL be allowed (file is not ignored)

### Requirement: Error Reporting for Blocked Operations
The system SHALL provide clear, actionable error messages when blocking git-ignored file operations.

#### Scenario: Blocked operation error message
- **WHEN** Claude attempts to modify a git-ignored file with `preventUpdateGitIgnored: true`
- **THEN** an error message SHALL be returned indicating:
  - The file path that was blocked
  - The reason (git-ignored status)
  - The `.gitignore` pattern(s) that matched
  - A suggestion to update `.gitignore` or disable the setting if needed

#### Scenario: Error includes matching pattern
- **WHEN** `.gitignore` contains `dist/` and Claude tries to write `dist/app.js`
- **THEN** the error message SHALL include the matching pattern `dist/`

#### Scenario: Error indicates setting responsible
- **WHEN** a file operation is blocked due to git-ignore
- **THEN** the error message SHALL clearly state that `preventUpdateGitIgnored` setting is enforcing this restriction

### Requirement: Git-Ignored File Configuration Validation
The system SHALL validate the `preventUpdateGitIgnore` boolean field.

#### Scenario: Valid boolean value
- **WHEN** `preToolUse.preventUpdateGitIgnored` is set to `true` or `false`
- **THEN** the configuration SHALL be accepted
- **AND** the setting SHALL be applied during pre-tool-use validation

#### Scenario: Invalid non-boolean value
- **WHEN** `preToolUse.preventUpdateGitIgnored` contains a non-boolean value (e.g., `"yes"`, `1`, `null`)
- **THEN** the configuration loading SHALL fail with a validation error
- **AND** the error message SHALL indicate the type mismatch and expected boolean value

#### Scenario: Missing field defaults to false
- **WHEN** `preToolUse.preventUpdateGitIgnored` is not specified
- **THEN** the system SHALL default to `false`
- **AND** no validation error SHALL occur

### Requirement: Git-Ignored Combined Protection Policies
The system SHALL enforce `preventUpdateGitIgnored` alongside existing file protection mechanisms.

#### Scenario: preventUpdateGitIgnored with preventRootAdditions
- **WHEN** both `preventRootAdditions: true` and `preventUpdateGitIgnored: true` are configured
- **AND** `.gitignore` contains `.env`
- **AND** Claude attempts to create `.env` (root-level git-ignored file)
- **THEN** the operation SHALL be blocked
- **AND** the error message SHALL indicate which restriction applied (or both)

#### Scenario: preventUpdateGitIgnored with uneditableFiles
- **WHEN** both `preventUpdateGitIgnored: true` and `uneditableFiles: ["*.lock"]` are configured
- **AND** `.gitignore` contains `node_modules/`
- **AND** Claude attempts to modify `node_modules/file.js`
- **THEN** the operation SHALL be blocked by git-ignore check

#### Scenario: preventUpdateGitIgnored with uneditableFiles overlap
- **WHEN** both `preventUpdateGitIgnored: true` and `uneditableFiles: [".env"]` are configured
- **AND** `.gitignore` also contains `.env`
- **AND** Claude attempts to modify `.env`
- **THEN** the operation SHALL be blocked
- **AND** the system evaluates both rules and applies the most restrictive result

#### Scenario: Multiple protection rules enforced
- **WHEN** `preventRootAdditions: true`, `uneditableFiles: ["Cargo.toml"]`, and `preventUpdateGitIgnored: true` are all configured
- **AND** `.gitignore` contains `dist/`
- **THEN** all three rules are evaluated independently for each file operation
- **AND** if any rule blocks the operation, it SHALL be denied

### Requirement: Git-Ignore Semantics Compliance
The system SHALL respect standard git-ignore semantics and behavior.

#### Scenario: Leading slash anchors to root
- **WHEN** `.gitignore` contains `/build` (leading slash)
- **AND** Claude attempts to modify `build/output.js` in the repository root
- **AND** `preventUpdateGitIgnored: true`
- **THEN** the operation SHALL be blocked
- **AND** a `build/` directory in subdirectories is not blocked by this rule

#### Scenario: Trailing slash matches directories only
- **WHEN** `.gitignore` contains `dist/` (trailing slash)
- **AND** Claude attempts to create a file named `dist` (as a file, not directory)
- **AND** `preventUpdateGitIgnored: true`
- **THEN** the operation may proceed (pattern matches directories only)

#### Scenario: Double asterisk matches nested levels
- **WHEN** `.gitignore` contains `src/**/*.test.ts`
- **AND** Claude attempts to modify `src/components/Button.test.ts`
- **AND** `preventUpdateGitIgnored: true`
- **THEN** the operation SHALL be blocked

#### Scenario: Exclamation negation overrides
- **WHEN** `.gitignore` contains:
  - `node_modules/`
  - `!node_modules/important-package/`
- **AND** Claude attempts to modify `node_modules/important-package/file.js`
- **AND** `preventUpdateGitIgnored: true`
- **THEN** the operation SHALL NOT be blocked (negation overrides the general rule)

### Requirement: Performance and Caching
The system SHALL cache git-ignore evaluation results to minimize performance impact.

#### Scenario: Git-ignore cache within session
- **WHEN** Claude performs multiple file operations within a session
- **THEN** git-ignore rules SHALL be loaded and parsed once per session (or when `.gitignore` changes)
- **AND** subsequent checks SHALL use cached rules for efficiency
- **AND** cache invalidation SHALL occur when `.gitignore` is modified

#### Scenario: No performance regression
- **WHEN** `preventUpdateGitIgnored` is set to `false`
- **THEN** no git-ignore checking code SHALL execute
- **AND** there SHALL be no performance impact on tool execution
