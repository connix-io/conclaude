# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.0.3] - 2025-08-28

### Added
- **User-friendly Stop Hook Error Messages**: Implemented intelligent error message formatting for Claude when stop hook commands fail
  - Added `identifyCommand()` function to classify commands (build, lint, test, typecheck) and provide context
  - Added `extractKeyErrors()` function to parse command output and extract relevant error information
  - Added `formatStopHookError()` function to create structured, actionable error messages for Claude
  - Error messages now include command description, purpose, key errors, and actionable suggestions
  - Added support for custom error messages via `customErrorMessage` configuration option
- **Stop Hook Configuration Enhancement**: Extended stop configuration with optional `customErrorMessage` field
  - Allows users to override automatic error formatting with custom messages
  - Updated init command template to document the new configuration option

### Changed
- **Improved Stop Hook Error Handling**: Stop hook failures now generate user-friendly messages instead of raw command output
  - Messages are structured with clear headings, context, extracted errors, and actionable suggestions
  - Command output is intelligently parsed to highlight the most relevant error information
  - Error messages are designed to help Claude understand what failed and how to fix it
  - Maintains full technical details in logs while presenting clean messages to Claude

### Fixed
- Stop hook error messages are now actionable and understandable for AI assistants instead of being purely technical

## [0.0.2] - 2025-08-21

### Added
- **Uneditable Files Configuration**: Added support for protecting files from Claude modifications using glob patterns
  - New `uneditableFiles` array in rules configuration
  - Supports minimatch glob patterns including wildcards (`*.json`), nested patterns (`src/**/*.ts`), brace expansion (`{package,tsconfig}.json`), and more
  - PreToolUse hook validation blocks Write, Edit, MultiEdit, and NotebookEdit operations on matching files
  - Tests patterns against original, relative, and resolved file paths for maximum compatibility
  - Comprehensive error messages showing which file was blocked and which pattern matched
  - Defensive error handling with detailed logging for debugging and audit purposes

### Changed
- Enhanced PreToolUse hook with uneditable files validation alongside existing `preventRootAdditions` rule
- Updated configuration templates in Init command to include `uneditableFiles` examples and documentation
- Refactored file path validation logic for better maintainability
- **flake.nix**: Updated to dynamically read version from package.json instead of hardcoded value

### Dependencies
- Added `minimatch` ^10.0.3 for powerful glob pattern matching
- Added `@types/minimatch` ^6.0.0 for TypeScript support

### Testing
- Added comprehensive tests for minimatch pattern matching
- Added tests for file path normalization scenarios
- All tests passing with improved coverage

## [0.0.1] - 2025-08-21

### Added
- Initial release of conclaude CLI tool
- Claude Code hook handler for all hook types (PreToolUse, PostToolUse, Stop, etc.)
- Configuration system with layered config support
- Stop hook with configurable command execution
- PreToolUse hook with `preventRootAdditions` rule
- Session-specific logging with Winston
- Init command for setting up conclaude configuration and Claude Code hooks
- Comprehensive TypeScript definitions for all hook payloads
- Transcript parsing utilities for conversation analysis
- Test suite with Bun test runner
