# conclaude

A Claude Code hook handler CLI tool that processes hook events from Claude Code by reading JSON payloads from stdin and executing handlers for each event type. The tool provides lifecycle hooks for tool usage, session management, and transcript processing with a powerful layered configuration system.

## Features

- **Comprehensive Hook System**: Handle all Claude Code lifecycle events (PreToolUse, PostToolUse, Stop, etc.)
- **Layered Configuration**: Project → Local → Global → Environment-specific configs with c12
- **Command Execution**: Run linting, testing, and validation commands during Stop hooks
- **File Protection**: Prevent unwanted root-level file creation via `preventRootAdditions` rule
- **Session Logging**: Winston-based logging with session-specific file output
- **Environment Awareness**: Support for development, production, and test-specific configurations

## Installation

```bash
bun install
```

## Configuration System

conclaude uses [c12](https://github.com/unjs/c12) for layered configuration management with the following priority order (high to low):

1. **Runtime Overrides** - Passed to configuration loader
2. **Project Config** - `conclaude.config.{ts,js,yaml,json}`
3. **Local RC File** - `./.conclaude` (project-specific overrides)
4. **Global RC File** - `~/.conclaude` (user's global preferences)
5. **Package.json** - `"conclaude"` field
6. **Schema Defaults** - Fallback configuration

### Configuration Schema

```typescript
interface ConclaudeConfig {
  stop: {
    run: string;  // Commands to execute during Stop hook
  };
  rules: {
    preventRootAdditions: boolean;  // Block file creation at repo root
  };
}
```

### Environment-Specific Overrides

Configurations support environment-specific sections:

- `$development` - Applied when `NODE_ENV=development`
- `$production` - Applied when `NODE_ENV=production`
- `$test` - Applied when `NODE_ENV=test`

## Hook Types

### PreToolUse Hook
Fired before Claude executes any tool. Enables:
- Tool execution blocking based on custom rules
- Input validation and security checks
- Root file creation prevention via `preventRootAdditions`

### PostToolUse Hook
Fired after tool execution. Enables:
- Result logging and analysis
- Performance monitoring
- Post-processing of tool outputs

### Stop Hook
Fired when Claude session terminates. Enables:
- Command execution (lint, test, build)
- Session cleanup and validation
- **Blocks session if any command fails**

### Other Hooks
- **UserPromptSubmit** - Process user input before Claude sees it
- **SessionStart** - Session initialization
- **SubagentStop** - Subagent completion handling
- **Notification** - System notification processing
- **PreCompact** - Transcript compaction preprocessing

## Configuration Examples

### Project Configuration (`conclaude.config.ts`)

```typescript
export default {
  stop: {
    run: `bun x tsc --noEmit`,
  },
  rules: {
    preventRootAdditions: true,
  },
  $development: {
    stop: {
      run: `echo "Development mode - skipping lint checks"`,
    },
  },
  $production: {
    stop: {
      run: `bun x tsc --noEmit
bun test
bun build`,
    },
  },
};
```

### Global User Config (`~/.conclaude`)

```yaml
stop:
  run: |
    bun run lint
    bun run test
    bun run build
rules:
  preventRootAdditions: false
```

### Project-Specific Override (`./.conclaude`)

```yaml
stop:
  run: echo "Project-specific commands"
rules:
  preventRootAdditions: false  # Allow root edits for this project
```

## Usage

### Claude Code Integration

conclaude is designed to be used as a hook handler in Claude Code. Configure it in your Claude Code settings:

```json
{
  "hooks": {
    "PreToolUse": "bun /path/to/conclaude/src/index.ts PreToolUse",
    "PostToolUse": "bun /path/to/conclaude/src/index.ts PostToolUse", 
    "Stop": "bun /path/to/conclaude/src/index.ts Stop"
  }
}
```

### Manual Testing

```bash
# Test Stop hook
echo '{"session_id":"test","transcript_path":"/tmp/test.jsonl","hook_event_name":"Stop","stop_hook_active":true}' | \
  bun src/index.ts Stop

# Test PreToolUse hook  
echo '{"session_id":"test","transcript_path":"/tmp/test.jsonl","hook_event_name":"PreToolUse","tool_name":"Write","tool_input":{"file_path":"test.txt"}}' | \
  bun src/index.ts PreToolUse
```

### Exit Codes

- **0**: Success - operation allowed to proceed
- **1**: Error - validation failure, parsing error, or handler crash
- **2**: Blocked - hook explicitly blocked the operation

## Hook Behavior Examples

### Stop Hook Command Execution

The Stop hook executes commands from `config.stop.run` sequentially:

```bash
# Configuration
stop:
  run: |
    bun x tsc --noEmit
    bun test

# Execution: If any command fails, the entire hook fails and blocks the session
✓ Command 1/2: bun x tsc --noEmit
✗ Command 2/2: bun test (exit code 1)
❌ Hook blocked: Command failed with exit code 1: bun test
```

### PreToolUse Root Protection

When `preventRootAdditions: true`, file-modifying tools are blocked at repo root:

```bash
# Blocked operations
Write → /repo/newfile.txt          ❌ Blocked
Edit → /repo/config.json           ❌ Blocked

# Allowed operations  
Write → /repo/.gitignore           ✓ Allowed (dotfile)
Write → /repo/src/component.tsx    ✓ Allowed (subdirectory)
Read → /repo/package.json          ✓ Allowed (read-only)
```

## Development

### Commands

```bash
# Type checking
bun x tsc --noEmit

# Build
bun build src/index.ts --target=bun

# Run directly
bun src/index.ts <hook-type>
```

### Project Structure

```
├── src/
│   ├── index.ts      # Main CLI with hook handlers
│   ├── config.ts     # Configuration loading with c12
│   ├── types.ts      # TypeScript payload definitions
│   └── logger.ts     # Winston logging configuration
├── conclaude.config.ts    # Project configuration
├── conclaude.schema.json  # JSON schema for validation
└── README.md
```

### Configuration Loading

Configuration is loaded once at startup using c12's layered system:

1. Base defaults from JSON schema
2. Package.json `conclaude` field
3. Global RC file (`~/.conclaude`)
4. Local RC file (`./.conclaude`) 
5. Project config file (`conclaude.config.*`)
6. Environment-specific overrides
7. Runtime overrides

### Adding New Hooks

1. Define payload interface in `types.ts`
2. Add handler function in `index.ts`
3. Create command definition
4. Register command with yargs CLI

## Architecture

### Hook Processing Flow

```
stdin JSON → readPayload() → validateFields() → handler() → HookResult → exit code
```

### Configuration Resolution

```
Schema Defaults → Package.json → Global RC → Local RC → Project Config → ENV Overrides
```

### Command Execution (Stop Hook)

```
config.stop.run → extractBashCommands() → Bun.spawn() → sequential execution → fail fast
```

## License

This project is part of the connix ecosystem and follows the same licensing terms.