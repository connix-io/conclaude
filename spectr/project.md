# Project Context

## Purpose
conclaude is a **Rust-based guardrail CLI for Claude Code sessions** that orchestrates configurable lifecycle hooks to enforce linting, testing, and workflow policies across projects.

**Goals:**
- Enforce consistent development workflows through session-aware hooks
- Validate and centralize YAML-based configuration management
- Run project-specific commands (tests, linters, workflows) during Claude Code sessions
- Protect sensitive files and enforce project rules without manual intervention

## Tech Stack
- **Language**: Rust (stable edition, current MSRV: TBD)
- **Configuration**: serde (serialization) + native Rust YAML parsing
- **Async Runtime**: tokio for I/O-bound operations
- **CLI Parsing**: clap for command-line argument handling
- **Schema Validation**: JSON Schema validation for configuration
- **Installation**: Shell scripts, npm packages, platform-specific binaries

## Project Conventions

### Code Style
- **Formatting**: Enforced via `rustfmt` (standard Rust formatting)
- **Linting**: Enforce `clippy` warnings and best practices
- **Naming Conventions**:
  - Functions/variables: `snake_case`
  - Types/structs: `PascalCase`
  - Constants: `SCREAMING_SNAKE_CASE`
- **Visibility**: Minimize public API surface; keep internals private unless necessary

### Architecture Patterns
- **Hook System**: Event-driven lifecycle hooks (SubagentStart, SubagentStop, SessionStart, etc.) that trigger at specific session points
- **Configuration Pipeline**: YAML discovery → validation against schema → runtime execution
- **File Protection Rules**: Configurable rules preventing modifications to protected files during sessions
- **Session-Aware Logging**: Contextualized logging tied to session IDs for debugging and telemetry

### Testing Strategy
- **Unit Tests**: Located in `#[cfg(test)]` modules alongside implementation code
- **Integration Tests**: End-to-end scenarios in `tests/` directory covering full workflows
- **Current Coverage**: Minimal to moderate (expanding as needed)
- **Manual Verification**: Ensure tests actually pass before committing (per CLAUDE.md requirement)

### Git Workflow
- **Commit Convention**: Conventional Commits with verb-led prefixes
  - `feat:` new features
  - `fix:` bug fixes
  - `refactor:` code restructuring
  - `chore:` tooling, dependencies, CI
  - `docs:` documentation updates
- **Branching**: Create feature/fix branches, merge via pull requests
- **Main Protection**: Main branch requires review and CI passing

## Domain Context
**Claude Code Context:**
- conclaude integrates with Claude Code sessions to enforce guardrails
- Hooks are executed at specific lifecycle points: session start, pre-tool-use, post-tool-use, subagent start/stop, etc.
- Configuration drives behavior without hardcoding policies
- File protection prevents accidental modifications to critical files (e.g., `.git/`, `node_modules/`, `Cargo.lock`)
- Multi-project support: conclaude discovers configuration by searching parent directories for `.conclaude.yaml` or `.conclaude.yml` files, continuing until reaching filesystem root or 12-level maximum depth

**Key Concepts:**
- **Hooks**: Configurable shell commands/scripts executed at session lifecycle events
- **Rules**: Preventive rules (e.g., prevent modifications to git-ignored files, protect certain directories)
- **Capabilities**: Discrete features tracked in specs (e.g., "Hook System", "Configuration Validation", "File Protection")
- **Changes**: Proposals for new features or breaking changes, tracked in `spectr/changes/`

## Important Constraints
- **Cross-Platform**: Must work on Linux, macOS, and Windows
- **Performance-Critical**: Hook execution must be fast; avoid blocking operations or expensive I/O
- **Schema Validation Strict**: Configuration YAML must adhere strictly to published schema; validation failures must fail fast with clear error messages
- **Subprocess Execution**: Shells and environment vary by OS; handle platform-specific quirks (e.g., `sh` vs `cmd.exe`)
- **Session Context**: Must preserve and use session IDs for logging and idempotency

## External Dependencies
- **Claude Code**: Invokes conclaude hooks during session lifecycle; receives hook outcomes (pass/fail/output)
- **Shell/Subprocess**: Executes configured shell commands and project scripts (npm, cargo, bash, etc.)
- **System Binaries**: Relies on user's PATH for executing commands (e.g., npm, cargo, python, linters)
