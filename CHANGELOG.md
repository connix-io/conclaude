# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **JSON Schema Auto-Generation**: Implemented automatic JSON schema generation and publishing
  - Added `ts-json-schema-generator` dev dependency for TypeScript-to-JSON schema conversion
  - New `schema:generate` npm script to generate schema from `ConclaudeConfig` type
  - Auto-generated `schema.json` provides IDE autocomplete and validation for `.conclaude.yaml` files
  - GitHub Actions workflow (`.github/workflows/schema.yml`) automatically generates and publishes schema on changes
  - Schema validation integrated into CI pipeline to ensure consistency
  - YAML Language Server integration via schema directive in generated configuration files

### Changed
- **Init Command Enhancement**: Updated `conclaude init` to include YAML language server schema directive
  - Generated `.conclaude.yaml` files now include `# yaml-language-server: $schema=https://raw.githubusercontent.com/conneroisu/conclaude/main/schema.json`
  - Enables real-time validation and autocomplete in VS Code and other YAML-aware editors
  - Improves developer experience when configuring conclaude
- **CI/CD Pipeline**: Enhanced existing workflows with schema validation
  - Added schema generation validation to `ci.yml` workflow
  - Ensures schema stays in sync with TypeScript definitions

### Dependencies
- Added `ts-json-schema-generator` ^2.3.0 for automated schema generation from TypeScript types

### Developer Experience
- IDE autocomplete and validation for `.conclaude.yaml` configuration files
- Automatic schema updates when TypeScript configuration interfaces change
- Better configuration documentation through generated JSON schema descriptions

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
