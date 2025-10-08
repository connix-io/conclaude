# conclaude

**The guardrails your Claude Code sessions need.**

A high-performance Rust CLI tool that provides essential guardrails for Claude Code sessions through a comprehensive hook system. Conclaude ensures your AI coding sessions maintain code quality, follow project standards, and respect your development workflows.

[![Crates.io](https://img.shields.io/crates/v/conclaude)](https://crates.io/crates/conclaude)
[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![CI](https://github.com/connix-io/conclaude/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/connix-io/conclaude/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/connix-io/conclaude?display_name=tag&sort=semver)](https://github.com/connix-io/conclaude/releases)

Born from real developer frustration, conclaude transforms chaotic AI coding sessions into controlled, validated workflows. It's not just another CLI tool—it's your project's guardian, ensuring that every Claude Code session respects your standards, follows your rules, and maintains your code quality.

## Releases

Official builds are published on GitHub Releases when tags matching `v*` are pushed. The latest version is **v0.1.6**.

- All releases: https://github.com/connix-io/conclaude/releases

### Quick Install Options

#### Option 1: Shell Script (Recommended)

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/connix-io/conclaude/releases/download/v0.1.6/conclaude-installer.sh | sh
```

#### Option 2: PowerShell (Windows)

```powershell
powershell -ExecutionPolicy Bypass -c "irm https://github.com/connix-io/conclaude/releases/download/v0.1.6/conclaude-installer.ps1 | iex"
```

#### Option 3: NPM Package

```bash
npm install conclaude@0.1.6
```

#### Option 4: Manual Binary Download

```bash
# Linux x86_64 example
curl -L -o conclaude.tar.xz \
  https://github.com/connix-io/conclaude/releases/download/v0.1.6/conclaude-x86_64-unknown-linux-gnu.tar.xz
tar -xf conclaude.tar.xz
chmod +x conclaude && sudo mv conclaude /usr/local/bin/
conclaude --version
```

**Available platforms:** Apple Silicon macOS, Intel macOS, x64 Windows, ARM64 Linux, x64 Linux, x64 MUSL Linux.

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
✅ **Everything is configurably logged** - Complete visibility into what Claude did during each session  

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

## Installation

### From Crates.io (Recommended)

```bash
# Install from crates.io
cargo install conclaude
```

### From Source

```bash
# Clone and build from source
git clone https://github.com/conneroisu/conclaude.git
cd conclaude
cargo build --release

# The binary will be in target/release/conclaude
```

### Nix Flake Installation

```bash
# Use the flake directly
nix run github:conneroisu/conclaude -- --help
```

#### Adding conclaude to your development shell

Add conclaude as a flake input and include it in your development shell:

```nix
# flake.nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    conclaude.url = "github:conneroisu/conclaude";
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

### Development Setup

```bash
# Clone the repository
git clone https://github.com/conneroisu/conclaude.git
cd conclaude

# Build the project
cargo build

# Run tests
cargo test

# Install for development
cargo install --path .
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
    cargo fmt --check     # Code formatting check
    cargo clippy -- -D warnings  # Linting with warnings as errors
    cargo test            # All tests must pass
    cargo build           # Successful compilation required
  
# File protection rules
rules:
  preventRootAdditions: true    # Keep project root clean
  uneditableFiles:              # Protect critical files
    - "Cargo.toml"              # Don't modify package manifest
    - "Cargo.lock"              # Lock file is sacred
    - ".env*"                   # Secrets stay secret
    - "target/**"               # Build artifacts
```

**What this accomplishes:**
- 🛡️ Claude can't break your Rust compilation
- ✅ All tests must pass before session completion  
- 🎨 Your formatting and linting rules are automatically enforced
- 📁 No surprise files cluttering your project root
- 🔒 Critical configuration files stay untouched

### Advanced Scenarios

#### Continuous Monitoring During Long Sessions
```yaml
# Perfect for refactoring sessions or long pair-programming
stop:
  run: |
    cargo check --quiet   # Fast compilation check
    cargo test --quiet     # Silent test execution
  infinite: true           # Validate after every change
  infiniteMessage: "🔍 Watching your code quality..."
```

#### Enterprise-Grade Protection
```yaml
# Maximum security for production codebases
stop:
  run: |
    cargo audit              # Security audit
    cargo fmt --check        # Strict formatting
    cargo clippy -- -D warnings  # All clippy warnings as errors
    cargo test --all         # Test all packages
    cargo build --release    # Release build

rules:
  preventRootAdditions: true
  uneditableFiles:
    - "Cargo.toml"
    - "Cargo.lock"
    - ".env*"
    - "target/**"
    - ".github/workflows/**"
    - "src/lib.rs"            # Protect main library entry point
```

### Configuration Reference

The complete configuration schema is defined as Rust structs with serde serialization. Key sections include:

- **stop**: Commands and settings for session termination hooks
- **rules**: File protection and validation rules
- **preToolUse**: Pre-execution validation and controls
- **gitWorktree**: Git integration settings

Generate a complete JSON schema with: `conclaude generate-schema`

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
    cargo check
    cargo test
    cargo build

# Validation rules
rules:
  # Block file creation at repository root
  preventRootAdditions: true
  
  # Files that cannot be edited (glob patterns)
  uneditableFiles:
    - "Cargo.toml"
    - "Cargo.lock"
    - ".env*"
    - "target/**"
```

### Development Configuration Example

```yaml
# Minimal checks for development
stop:
  run: |
    echo "Running development checks..."
    cargo check

rules:
  preventRootAdditions: false  # Allow root edits during development
  uneditableFiles:
    - "Cargo.toml"  # Still protect Cargo.toml
```

### Production Configuration Example

```yaml
# Comprehensive validation for production
stop:
  run: |
    echo "Running production validation..."
    cargo fmt --check
    cargo clippy -- -D warnings
    cargo test
    cargo build --release

rules:
  preventRootAdditions: true
  uneditableFiles:
    - "Cargo.toml"
    - "Cargo.lock"
    - ".env*"
    - "target/**"
```

### Infinite Mode Configuration Example

```yaml
# Continuous monitoring with infinite mode
stop:
  run: |
    echo "Starting continuous monitoring..."
    cargo check
    cargo test
  infinite: true
  infiniteMessage: "Monitoring active - press Ctrl+C to stop"

rules:
  preventRootAdditions: false  # Allow file changes during development
  uneditableFiles:
    - "Cargo.toml"
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
    cargo check
    cargo test

# Execution: If any command fails, the entire hook fails and blocks the session
✓ Command 1/2: cargo check
✗ Command 2/2: cargo test (exit code 1)
❌ Hook blocked: Command failed with exit code 1: cargo test
```

### PreToolUse Root Protection

When `preventRootAdditions: true`, file-modifying tools are blocked at repo root:

```bash
# Blocked operations
Write → /repo/newfile.txt          ❌ Blocked
Edit → /repo/config.json           ❌ Blocked

# Allowed operations  
Write → /repo/.gitignore           ✓ Allowed (dotfile)
Write → /repo/src/component.rs     ✓ Allowed (subdirectory)
Read → /repo/Cargo.toml            ✓ Allowed (read-only)
```

## Development

### Commands

```bash
# Format code
cargo fmt

# Run linting
cargo clippy

# Run tests
cargo test

# Build for distribution
cargo build --release

# Run hooks directly (development)
cargo run -- <hook-type>

# Use Nix development environment
nix develop -c lint    # Run linting
nix develop -c tests   # Run tests
```

### Project Structure

```
├── src/
│   ├── main.rs             # Main CLI entry point
│   ├── config.rs           # Configuration loading and parsing
│   ├── types.rs            # Rust type definitions for payloads
│   ├── hooks.rs            # Hook handler implementations
│   ├── logger.rs           # Logging configuration
│   ├── schema.rs           # JSON Schema generation
│   ├── lib.rs              # Library exports
│   └── default-config.yaml # Default configuration template
├── .conclaude.yaml         # YAML configuration file
├── flake.nix               # Nix development environment
├── Cargo.toml              # Rust package manifest
└── README.md
```

### Configuration Loading

Configuration is loaded using native Rust YAML parsing with automatic directory tree search:

1. `.conclaude.yaml` - Primary configuration file
2. `.conclaude.yml` - Alternative YAML extension

The search starts from the current directory and moves up the directory tree until a configuration file is found or the project root (indicated by `package.json` presence) is reached.

If no configuration file is found, conclaude will display the searched locations and suggest running `conclaude init` to generate a template configuration.

### Adding New Hooks

1. Define payload struct in `src/types.rs`
2. Add handler function in `src/hooks.rs`
3. Create command variant in `src/main.rs`
4. Register command with clap CLI parser

## Architecture

### Hook Processing Flow

```
stdin JSON → read_payload_from_stdin() → validate_base_payload() → handler() → HookResult → exit code
```

### Configuration Resolution

```
.conclaude.yaml → serde_yaml parsing → ConclaudeConfig struct
```

### Command Execution (Stop Hook)

```
config.stop.run → extract_bash_commands() → tokio::process::Command → sequential execution → fail fast
```

## Features

### Hook System

- **PreToolUse**: Block or validate tool usage before execution
- **PostToolUse**: Log and analyze tool results after execution
- **Stop**: Run validation commands before session completion
- **SessionStart**: Initialize session-specific logging and setup
- **UserPromptSubmit**: Process and validate user input
- **Notification**: Handle system notifications and alerts
- **SubagentStop**: Manage subagent completion events
- **PreCompact**: Handle transcript compaction preparation

### Configuration Features

- **YAML Configuration**: Human-readable configuration with JSON Schema validation
- **File Protection**: Prevent edits to critical files using glob patterns
- **Root Directory Protection**: Keep project root clean from unwanted files
- **Command Validation**: Run custom validation commands (tests, linting, builds)
- **Infinite Mode**: Continuous monitoring for long development sessions
- **Rounds Mode**: Run validation for a specific number of iterations
- **Grep Rules**: Content-based validation using pattern matching
- **Tool Usage Validation**: Control which tools can operate on which files
- **Git Worktree Integration**: Automatic branch and PR management

### Performance & Reliability

- **Fast Startup**: Minimal overhead with efficient Rust implementation
- **Configuration Caching**: Avoid repeated file system operations
- **Session Logging**: Comprehensive logging with session-specific output
- **Error Recovery**: Graceful handling of command failures and invalid input
- **Cross-Platform**: Works on Linux, macOS, and Windows

## Command Line Interface

### Available Commands

```bash
# Initialize configuration
conclaude init [--force] [--config-path <path>] [--claude-path <path>]

# Generate JSON Schema
conclaude generate-schema [--output <path>] [--validate]

# Hook handlers (called by Claude Code)
conclaude PreToolUse
conclaude PostToolUse
conclaude Stop
conclaude SessionStart
conclaude UserPromptSubmit
conclaude Notification
conclaude SubagentStop
conclaude PreCompact

# Visualize configuration
conclaude visualize [--rule <rule-name>] [--show-matches]

# Global options
--verbose              # Enable debug logging
--disable-file-logging # Disable logging to temporary files
```

## Configuration Reference

### Complete Configuration Schema

```yaml
# Stop hook configuration
stop:
  # Simple command format
  run: |
    cargo check
    cargo test
    cargo build
  
  # Alternative: structured commands with custom messages
  commands:
    - run: "cargo test"
      message: "Tests failed - fix failing tests before continuing"
    - run: "cargo build"
      message: "Build failed - fix compilation errors"
  
  # Infinite mode - continue after successful validation
  infinite: false
  infiniteMessage: "Continue working on the task"
  
  # Rounds mode - run for specific iterations
  rounds: 3
  
  # Content validation rules
  grepRules:
    - filePattern: "**/*.rs"
      forbiddenPattern: "todo|fixme"
      description: "No TODO or FIXME comments allowed"

# File and directory protection rules
rules:
  # Prevent file creation at repository root
  preventRootAdditions: true
  
  # Files that cannot be edited (glob patterns)
  uneditableFiles:
    - "Cargo.toml"
    - "Cargo.lock"
    - ".env*"
    - "target/**"
  
  # Tool usage validation
  toolUsageValidation:
    - tool: "Write"
      pattern: "**/*.rs"
      action: "allow"
      message: "Writing to Rust files is allowed"
    - tool: "*"
      pattern: ".env*"
      action: "block"
      message: "Environment files cannot be modified"

# Pre-tool-use hook configuration
preToolUse:
  # Content validation before tool execution
  grepRules:
    - filePattern: "**/*.rs"
      forbiddenPattern: "unsafe"
      description: "Unsafe code blocks not allowed"
  
  # Additional directories to protect from additions
  preventAdditions:
    - "docs/"
    - "examples/"

# Git worktree integration
gitWorktree:
  enabled: false
  autoCreatePR: false
```

### Environment Variables

- `CONCLAUDE_LOG_LEVEL`: Set logging level (debug, info, warn, error)
- `CONCLAUDE_DISABLE_FILE_LOGGING`: Disable logging to temporary files

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development Setup

1. Fork the repository
2. Clone your fork: `git clone https://github.com/yourusername/conclaude.git`
3. Create a feature branch: `git checkout -b feature-name`
4. Make your changes and add tests
5. Run the test suite: `cargo test`
6. Run linting: `cargo clippy`
7. Format code: `cargo fmt`
8. Commit your changes: `git commit -am 'Add feature'`
9. Push to the branch: `git push origin feature-name`
10. Submit a Pull Request

### Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run integration tests
cargo test --test integration
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/) for performance and safety
- Uses [serde](https://serde.rs/) for JSON/YAML serialization
- CLI powered by [clap](https://clap.rs/)
- Async runtime provided by [tokio](https://tokio.rs/)
- Configuration validation with [schemars](https://docs.rs/schemars/)

---

**Note**: This is the Rust implementation of conclaude. For maximum performance and system integration, the Rust version is recommended over alternative implementations.
