# conclaude

A Claude Code hook handler CLI tool that processes hook events from Claude Code by reading JSON payloads from stdin and executing handlers for each event type. The tool provides lifecycle hooks for tool usage, session management, and transcript processing with YAML-based configuration.

## Features

- **Comprehensive Hook System**: Handle all Claude Code lifecycle events (PreToolUse, PostToolUse, Stop, etc.)
- **YAML Configuration**: Simple, readable YAML configuration files with cosmiconfig
- **Command Execution**: Run linting, testing, and validation commands during Stop hooks
- **File Protection**: Prevent unwanted root-level file creation via `preventRootAdditions` rule
- **Session Logging**: Winston-based logging with session-specific file output
- **Pattern Matching**: Glob pattern support for file protection rules

## Installation

### Global Installation (Recommended)

```bash
# Install globally with bun
bun install -g conclaude

# Or install from npm
npm install -g conclaude
```

### Nix Flake Installation

```bash
# Use the flake directly
nix run github:connix-io/conclaude -- --help
```

#### Adding conclaude to your development shell

Add conclaude as a flake input and include it in your development shell:

```nix
# flake.nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    conclaude.url = "github:connix-io/conclaude";
  };

  outputs = { self, nixpkgs, conclaude, ... }:
    let
      system = "x86_64-linux"; # or your system
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      devShells.default = pkgs.mkShell {
        packages = [
          conclaude.packages.${system}.default
          # your other packages...
        ];
        
        shellHook = ''
          echo "conclaude available in development environment"
          conclaude --help
        '';
      };
    };
}
```

Then enter the development shell:

```bash
nix develop
```

### Development Installation

```bash
# Clone and install for development
git clone https://github.com/connix-io/conclaude.git
cd conclaude
bun install
```

## Configuration System

`conclaude` uses [cosmiconfig](https://github.com/cosmiconfig/cosmiconfig) with YAML configuration files. The system searches for configuration in the following order:

1. `.conclaude.yaml` - Primary configuration file
2. `.conclaude.yml` - Alternative YAML extension

### Configuration Schema

```yaml
# .conclaude.yaml
stop:
  run: |
    nix develop -c "lint"
    bun test
  infinite: false  # Optional: run once and exit (default)
  infiniteMessage: "Validation complete"  # Optional: custom message

rules:
  preventRootAdditions: true
  uneditableFiles:
    - "./package.json"
    - "*.lock"
    - ".env*"
```

### Configuration Schema

```typescript
interface ConclaudeConfig {
  stop: {
    run: string;                    // Commands to execute during Stop hook
    infinite?: boolean;             // Keep running infinitely (default: false)
    infiniteMessage?: string;       // Message to display when in infinite mode
  };
  rules: {
    preventRootAdditions: boolean;  // Block file creation at repo root
    uneditableFiles: string[];      // Glob patterns for protected files
  };
}
```

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
- **Infinite mode**: Optionally keep running indefinitely for continuous monitoring

### Other Hooks
- **UserPromptSubmit** - Process user input before Claude sees it
- **SessionStart** - Session initialization
- **SubagentStop** - Subagent completion handling
- **Notification** - System notification processing
- **PreCompact** - Transcript compaction preprocessing

## Configuration Examples

### Basic Configuration (`.conclaude.yaml`)

```yaml
# Commands to run during Stop hook
stop:
  run: |
    bun x tsc --noEmit
    bun test
    bun build

# Validation rules
rules:
  # Block file creation at repository root
  preventRootAdditions: true
  
  # Files that cannot be edited (glob patterns)
  uneditableFiles:
    - "./package.json"
    - "./bun.lockb"
    - ".env*"
    - "*.lock"
```

### Development Configuration Example

```yaml
# Minimal checks for development
stop:
  run: |
    echo "Running development checks..."
    bun x tsc --noEmit

rules:
  preventRootAdditions: false  # Allow root edits during development
  uneditableFiles:
    - "./package.json"  # Still protect package.json
```

### Production Configuration Example

```yaml
# Comprehensive validation for production
stop:
  run: |
    echo "Running production validation..."
    bun x tsc --noEmit
    bun test
    bun run lint
    bun run build

rules:
  preventRootAdditions: true
  uneditableFiles:
    - "./package.json"
    - "./bun.lockb"
    - ".env*"
    - "dist/**"
    - "node_modules/**"
```

### Infinite Mode Configuration Example

```yaml
# Continuous monitoring with infinite mode
stop:
  run: |
    echo "Starting continuous monitoring..."
    bun x tsc --noEmit
    bun test
  infinite: true
  infiniteMessage: "Monitoring active - press Ctrl+C to stop"

rules:
  preventRootAdditions: false  # Allow file changes during development
  uneditableFiles:
    - "./package.json"
```

## Usage

### Claude Code Integration

conclaude is designed to be used as a hook handler in Claude Code. After global installation, use the `conclaude init` command to automatically configure Claude Code hooks:

```bash
# Initialize conclaude in your project
conclaude init
```

This creates:
- `.conclaude.yaml` - Your project configuration
- `.claude/settings.json` - Claude Code hook configuration

#### Manual Claude Code Configuration

If you prefer manual setup, configure hooks in your Claude Code settings:

```json
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "",
        "hooks": [
          {
            "type": "command",
            "command": "conclaude PreToolUse"
          }
        ]
      }
    ],
    "PostToolUse": [
      {
        "matcher": "",
        "hooks": [
          {
            "type": "command",
            "command": "conclaude PostToolUse"
          }
        ]
      }
    ],
    "Stop": [
      {
        "matcher": "",
        "hooks": [
          {
            "type": "command",
            "command": "conclaude Stop"
          }
        ]
      }
    ]
  }
}
```

### Initialize Configuration

```bash
# Create initial .conclaude.yaml configuration
conclaude init

# Force overwrite existing configuration
conclaude init --force

# Specify custom paths
conclaude init --config-path ./custom.yaml --claude-path ./.claude-custom
```

### Manual Testing

```bash
# Test Stop hook
echo '{"session_id":"test","transcript_path":"/tmp/test.jsonl","hook_event_name":"Stop","stop_hook_active":true}' | \
  conclaude Stop

# Test PreToolUse hook  
echo '{"session_id":"test","transcript_path":"/tmp/test.jsonl","hook_event_name":"PreToolUse","tool_name":"Write","tool_input":{"file_path":"test.txt"}}' | \
  conclaude PreToolUse

# Get help
conclaude --help
conclaude Stop --help
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
bun run lint

# Run tests
bun test

# Build for distribution
bun run build

# Run hooks directly (development)
bun src/index.ts <hook-type>

# Use Nix development environment
nix develop -c lint    # Run linting
nix develop -c tests   # Run tests
```

### Project Structure

```
├── src/
│   ├── index.ts      # Main CLI with hook handlers
│   ├── config.ts     # Configuration loading with cosmiconfig
│   ├── types.ts      # TypeScript payload definitions
│   └── logger.ts     # Winston logging configuration
├── .conclaude.yaml        # YAML configuration file
├── flake.nix              # Nix development environment
├── package.json           # Package configuration
└── README.md
```

### Configuration Loading

Configuration is loaded using cosmiconfig with YAML files:

1. `.conclaude.yaml` - Primary configuration file
2. `.conclaude.yml` - Alternative YAML extension

If no configuration file is found, conclaude will throw an error requiring you to run `conclaude init` first.

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
.conclaude.yaml → YAML parsing → ConclaudeConfig interface
```

### Command Execution (Stop Hook)

```
config.stop.run → extractBashCommands() → Bun.spawn() → sequential execution → fail fast
```

## License

This project is part of the connix ecosystem and follows the same licensing terms.
