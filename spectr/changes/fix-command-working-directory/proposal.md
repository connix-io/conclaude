# Change: Fix Command Working Directory to Use Config File Directory

## Why

Commands defined in `.conclaude.yaml` configuration files currently execute with the working directory set to Claude's session directory (wherever Claude Code is invoked from), rather than the directory containing the configuration file. This causes commands like `npm test` or `cargo build` to fail when the configuration file is located in a parent directory, as they run from the wrong project context.

## What Changes

- **Execution behavior**: All configured commands (`stop.commands`, `subagentStop.commands`) SHALL execute with their current working directory set to the parent directory of the configuration file
- **Environment variable**: Commands SHALL receive `CONCLAUDE_CONFIG_DIR` environment variable pointing to the config file's parent directory
- **Config path propagation**: The configuration file path must be passed through to command execution functions
- **Function signature updates**: `execute_stop_commands` and `execute_subagent_stop_commands` will accept the config directory path as a parameter
- **TokioCommand enhancement**: Add `.current_dir()` and `.env("CONCLAUDE_CONFIG_DIR", ...)` to all command spawning calls
- **No additional logging**: Working directory changes will NOT be logged (minimal output preferred)

## Impact

- Affected specs: `execution`
- Affected code: `src/hooks.rs` (command execution functions)
- Breaking change: **No** - this fixes incorrect behavior; commands will now run from the expected directory
- Backward compatibility: Commands that previously worked by coincidence (config in same dir as session) continue to work
