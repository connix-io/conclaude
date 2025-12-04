# Change: Add configurable working directory for stop commands

## Why
Users need to run stop commands (like linters, test runners, or build tools) from specific directories other than where conclaude is invoked. For example, a command like `npm test` may need to run from a specific `packages/frontend` subdirectory, or a monorepo user may want to run commands from the project root regardless of where the session started.

## What Changes
- Add an optional `workingDir` field to `StopCommand` configuration
- Add an optional `workingDir` field to `SubagentStopCommand` configuration
- When specified, commands execute from that directory instead of the default (Claude's cwd)
- Support both absolute paths and relative paths (resolved relative to config file location)
- **Support full bash interpolation**: environment variables (`$HOME`, `$CONCLAUDE_SESSION_ID`), command substitution (`$(git rev-parse --show-toplevel)`), and tilde expansion (`~/project`)
- Interpolation happens at command execution time (per command) for dynamic values
- Clear error messages when interpolation fails or results in invalid directory

## Impact
- Affected specs: `execution`
- Affected code: `src/config.rs`, `src/hooks.rs`
- Configuration schema changes (non-breaking - new optional field)
