## Why

Stop hook commands can produce excessive output that clutters the Claude Code interface and consumes context window tokens unnecessarily. GitHub issue #49 requests the ability to limit command output to a specified number of lines, helping agents focus on relevant information and reducing distractions.

## What Changes

- Add optional `maxOutputLines` field to `StopCommand` configuration structure
- Implement output truncation logic in command execution
- Add clear truncation indicators when output is limited
- Maintain backward compatibility with existing configurations (no limit by default)

## Impact

- Affected specs: Command Execution
- Affected code: `src/config.rs` (StopCommand struct), `src/hooks.rs` (execute_stop_commands function)
- Breaking change: No (optional field with backward compatibility)
- Related issue: #49
