# conclaude

**The guardrails your Claude Code sessions need.**

Picture this: You're deep in a coding session with Claude, building something amazing. Claude is writing tests, refactoring code, fixing bugs—it's incredibly productive. But then you notice something troubling. Your carefully configured linting rules? Ignored. Your test suite that was green this morning? Now broken. Root-level files appearing where they shouldn't. The AI is powerful, but it doesn't know your project's rules.

This is the story of why `conclaude` exists.

Born from real developer frustration, conclaude transforms chaotic AI coding sessions into controlled, validated workflows. It's not just another CLI tool—it's your project's guardian, ensuring that every Claude Code session respects your standards, follows your rules, and maintains your code quality.

## The Problem We Solve

AI-assisted coding is revolutionary, but it comes with a challenge: **How do you maintain control and quality standards when an AI is making rapid changes to your codebase?**

Without guardrails, Claude Code sessions can:
- Break existing linting and formatting rules
- Create files in wrong locations
- Skip essential validation steps
- Leave your project in an inconsistent state
- Bypass your carefully crafted development workflows

**conclaude changes this story.**

## How conclaude Changes Everything

Imagine starting every Claude Code session knowing that:

✅ **Your linting rules will be respected** - No more broken formatting or style violations  
✅ **Your tests must pass** - Sessions only complete when your test suite is green  
✅ **Your files stay organized** - No mysterious root-level files cluttering your project  
✅ **Your workflows are enforced** - Build processes, validation, and quality checks run automatically  
✅ **Everything is logged** - Complete visibility into what Claude did during each session  

This isn't just wishful thinking—it's what conclaude delivers every single time.

## What Makes conclaude Different

While other tools try to bolt-on AI safety as an afterthought, conclaude was built from the ground up specifically for Claude Code workflows. Here's what sets it apart:

🎯 **Purpose-Built for Claude Code**: Native integration with Claude's lifecycle hooks—no hacks, no workarounds  
⚡ **Zero Configuration Friction**: Simple YAML config that just works, powered by cosmiconfig  
🛡️ **Fail-Fast Protection**: Catches problems immediately, not after damage is done  
🔄 **Extensible Hook System**: Handle PreToolUse, PostToolUse, Stop, and more lifecycle events  
📊 **Session-Aware Logging**: Every action is tracked with session context for complete auditability  

## Core Capabilities

- **Comprehensive Hook System**: Handle all Claude Code lifecycle events with precision
- **YAML Configuration**: Human-readable configuration that scales with your team
- **Command Execution**: Run your existing lint, test, and build commands automatically
- **File Protection**: Prevent unwanted file creation with intelligent pattern matching
- **Session Logging**: Winston-based logging with session-specific file output
- **Infinite Monitoring**: Optional continuous monitoring for long-running development sessions

## Real-World Scenarios

### Scenario 1: The "Oops, My Tests Are Broken" Prevention

**Before conclaude:**
```
Developer: "Claude, add user authentication to my app"
Claude: *writes beautiful auth code*
Developer: *tries to deploy*
CI/CD: ❌ 47 test failures, linting errors everywhere
Developer: *spends 2 hours fixing what should have been caught*
```

**With conclaude:**
```
Developer: "Claude, add user authentication to my app"
Claude: *writes beautiful auth code*
conclaude: ✅ All tests pass, linting clean
Developer: *deploys confidently*
```

### Scenario 2: The "Where Did This File Come From?" Mystery

**The Problem:**
You're reviewing Claude's work and find `config.json`, `temp.js`, and `debug.log` scattered in your project root. Your clean directory structure is now a mess.

**The conclaude Solution:**
```yaml
# .conclaude.yaml
rules:
  preventRootAdditions: true  # No more mystery files!
```

Claude tries to create a root file → conclaude blocks it → Claude puts it in the right place.

### Scenario 3: The "Continuous Refactoring" Workflow

**The Vision:**
You're pair programming with Claude for hours, making incremental improvements. You want validation after every change, not just at the end.

**The Setup:**
```yaml
# .conclaude.yaml
stop:
  infinite: true
  run: |
    bun x tsc --noEmit
    bun test --silent
  infiniteMessage: "🛡️ Monitoring active - your code stays clean!"
```

Now every small change gets validated immediately. No surprises at the end of a long session.

## Getting Started

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

## Configuration: Your Project's Rulebook

Think of conclaude's configuration as your project's constitution—the fundamental rules that govern how Claude Code can interact with your codebase. It's designed to be simple enough to set up in minutes, yet powerful enough to handle complex enterprise workflows.

### How Configuration Works

conclaude finds your rules automatically using [cosmiconfig](https://github.com/cosmiconfig/cosmiconfig), looking for:

1. `.conclaude.yaml` - Your main configuration file (recommended)
2. `.conclaude.yml` - Alternative YAML extension

No complex setup, no environment variables to manage. Just drop a `.conclaude.yaml` file in your project root and you're protected.

### Your First Configuration

Here's what a real-world configuration looks like:

```yaml
# .conclaude.yaml - Your project's guardrails

# Commands that MUST pass before any session ends
stop:
  run: |
    bun x tsc --noEmit    # TypeScript check
    bun test              # All tests must pass
    bun run lint          # Code style enforced
  
# File protection rules
rules:
  preventRootAdditions: true    # Keep project root clean
  uneditableFiles:              # Protect critical files
    - "./package.json"          # Don't touch dependencies
    - "*.lock"                  # Lock files are sacred
    - ".env*"                   # Secrets stay secret
```

**What this accomplishes:**
- 🛡️ Claude can't break your TypeScript compilation
- ✅ All tests must pass before session completion  
- 🎨 Your linting rules are automatically enforced
- 📁 No surprise files cluttering your project root
- 🔒 Critical configuration files stay untouched

### Advanced Scenarios

#### Continuous Monitoring During Long Sessions
```yaml
# Perfect for refactoring sessions or long pair-programming
stop:
  run: |
    bun x tsc --noEmit
    bun test --silent
  infinite: true  # Validate after every change
  infiniteMessage: "🔍 Watching your code quality..."
```

#### Enterprise-Grade Protection
```yaml
# Maximum security for production codebases
stop:
  run: |
    npm audit --audit-level moderate
    bun x tsc --noEmit
    bun test --coverage
    bun run lint
    bun run build

rules:
  preventRootAdditions: true
  uneditableFiles:
    - "./package.json"
    - "./package-lock.json"
    - ".env*"
    - "dist/**"
    - "build/**"
    - "node_modules/**"
    - ".github/workflows/**"
```

### Configuration Reference

<details>
<summary>Complete Configuration Schema</summary>

```typescript
interface ConclaudeConfig {
  stop: {
    run: string;                    // Shell commands to execute
    infinite?: boolean;             // Keep running infinitely (default: false)  
    infiniteMessage?: string;       // Custom message for infinite mode
  };
  rules: {
    preventRootAdditions: boolean;  // Block file creation at repo root
    uneditableFiles: string[];      // Glob patterns for protected files
  };
}
```
</details>

## Understanding the Hook System

conclaude taps into Claude Code's lifecycle through strategic intervention points called "hooks." Think of hooks as security checkpoints in your development workflow—each one serves a specific purpose in keeping your codebase safe and consistent.

### The Three Critical Moments

#### 🚦 PreToolUse Hook: The Gatekeeper
*Fired the moment before Claude tries to use any tool*

**What it protects against:**
- Claude creating files in your project root (when you prefer organized subdirectories)
- Modifications to protected files like `package.json` or `.env` files
- Any tool usage that violates your project's rules

**Real example:** Claude wants to create `debug.log` in your project root, but your `preventRootAdditions` rule blocks it. Claude adapts and creates `logs/debug.log` instead.

#### 📊 PostToolUse Hook: The Observer  
*Fired immediately after Claude completes any tool operation*

**What it enables:**
- Complete audit trail of every change Claude makes
- Performance monitoring (how long did that operation take?)
- Session-specific logging with full context
- Post-processing and validation of tool results

**Real example:** After Claude edits a file, PostToolUse logs exactly what changed, when, and in which session—giving you complete traceability.

#### ⚡ Stop Hook: The Validator (Most Important)
*Fired when Claude thinks the session is complete*

**This is where the magic happens.** The Stop hook is your last line of defense and your quality assurance engine:

- **Runs your validation commands** (lint, test, build, etc.)
- **Blocks session completion** if any check fails  
- **Forces Claude to fix issues** before you see "success"
- **Optional infinite mode** for continuous validation during long sessions

**Real example:** Claude finishes implementing a feature. Stop hook runs your tests, finds 3 failures, blocks completion. Claude sees the errors and fixes them automatically. Only then does the session complete successfully.

### Supporting Cast of Hooks

- **UserPromptSubmit** - Intercept and potentially modify your prompts before Claude sees them
- **SessionStart** - Initialize logging, set up monitoring, prepare your workspace  
- **SubagentStop** - Handle completion of Claude's internal subprocesses
- **Notification** - Process and potentially filter system notifications
- **PreCompact** - Prepare transcripts before they're compressed or archived

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
