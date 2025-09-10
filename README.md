# Conclaude Go Implementation

This is a complete Go implementation of the conclaude CLI using the cobra-cli framework. It maintains full compatibility with the original Rust implementation while following idiomatic Go patterns and practices.

## Features

- **Full compatibility** with the original Rust conclaude CLI
- **Cobra-powered CLI** with rich command structure and help system
- **Idiomatic Go code** following Go best practices and conventions
- **Comprehensive configuration management** with YAML support and JSON Schema validation
- **Robust hook processing** for all Claude Code hook events
- **Colored terminal output** for better user experience
- **Structured logging** with configurable log levels
- **Multi-platform support** with cross-compilation capabilities

## Architecture

### Core Components

1. **`cmd/`** - Cobra command definitions
   - `root.go` - Root command and global flags
   - `hooks.go` - All hook command handlers
   - `init.go` - Initialization command
   - `generate_schema.go` - Schema generation command  
   - `visualize.go` - Configuration visualization command

2. **`internal/config/`** - Configuration management
   - Configuration structs matching Rust implementation
   - YAML loading and parsing
   - Default configuration generation
   - File path extraction and validation utilities

3. **`internal/types/`** - Core data types
   - Hook payload structures
   - Result types with proper JSON marshaling
   - Base payload validation

4. **`internal/hooks/`** - Hook processing logic
   - Cached configuration loading
   - Stdin payload reading and validation
   - Hook result handling with proper exit codes
   - File validation and tool usage rules

5. **`internal/schema/`** - JSON Schema support
   - Schema generation using reflection
   - YAML language server header generation
   - Configuration validation

6. **`internal/logger/`** - Logging infrastructure
   - Session-specific loggers
   - Configurable log levels and outputs
   - Temporary file logging support

## Commands

The Go implementation provides identical commands to the Rust version:

### Hook Commands
- `PreToolUse` - Process pre-tool-use hook events
- `PostToolUse` - Process post-tool-use hook events  
- `Notification` - Process notification hook events
- `UserPromptSubmit` - Process user prompt submit hook events
- `SessionStart` - Process session start hook events
- `Stop` - Process stop hook events
- `SubagentStop` - Process subagent stop hook events
- `PreCompact` - Process pre-compact hook events

### Management Commands
- `init` - Initialize conclaude configuration and Claude Code hooks
- `generate-schema` - Generate JSON Schema for conclaude configuration
- `visualize` - Visualize file/directory settings from configuration

### Global Flags
- `--verbose, -v` - Enable verbose logging output
- `--disable-file-logging` - Disable logging to temporary files

## Installation

### Prerequisites

- Go 1.22 or later
- Make (for build automation)

### Building from Source

```bash
# Clone the repository
git clone <repository-url>
cd conclaude/go-implementation

# Download dependencies
make deps

# Build the application
make build

# Install globally
make install
```

### Cross-Platform Building

```bash
# Build for all supported platforms
make build-all
```

This creates binaries for:
- Linux (amd64, arm64)
- macOS (amd64, arm64) 
- Windows (amd64)

## Usage

The Go implementation is used identically to the Rust version:

### Initialize Configuration

```bash
conclaude init
```

### Generate JSON Schema

```bash
conclaude generate-schema --output schema.json --validate
```

### Visualize Configuration

```bash
# Show all rules
conclaude visualize

# Show specific rule
conclaude visualize --rule uneditableFiles --show-matches
```

### Hook Processing

Hook commands are typically called by Claude Code automatically:

```bash
echo '{"session_id":"test","transcript_path":"/tmp/test","hook_event_name":"PreToolUse","tool_name":"Edit","tool_input":{"file_path":"test.go"}}' | conclaude PreToolUse
```

## Configuration

Configuration is managed through `.conclaude.yaml` files with the same structure as the Rust version:

```yaml
# yaml-language-server: $schema=https://raw.githubusercontent.com/connix-io/conclaude/main/conclaude-schema.json

stop:
  run: ""
  commands: []
  infinite: false
  rounds: null

rules:
  preventRootAdditions: true
  uneditableFiles: []
  toolUsageValidation: []

preToolUse:
  preventAdditions: []
  preventGeneratedFileEdits: true
  generatedFileMessage: null

gitWorktree:
  enabled: false
  autoCreatePR: false
  autoCreatePRCommand: null
  autoCreatePRTemplate: null
```

## Development

### Development Workflow

```bash
# Format code
make fmt

# Lint code  
make lint

# Run tests
make test

# Build for development (with race detection)
make dev

# Run full CI pipeline
make ci
```

### Adding New Features

1. **Add command**: Create new command file in `cmd/`
2. **Add configuration**: Extend config structs in `internal/config/`
3. **Add types**: Define new types in `internal/types/`
4. **Add processing**: Implement logic in `internal/hooks/`
5. **Update schema**: Schema generation is automatic via reflection

### Testing

The implementation includes comprehensive test coverage:

```bash
# Run tests with coverage
make coverage

# Check for vulnerabilities  
make security
```

## Key Design Decisions

### Compatibility with Rust Implementation

- **Identical CLI interface**: All commands, flags, and behaviors match exactly
- **Same configuration format**: Uses identical YAML structure and validation rules  
- **Same hook processing**: Implements identical hook validation and processing logic
- **Same exit codes**: Uses same exit codes (0=success, 1=error, 2=blocked)

### Go-Specific Improvements

- **Structured logging**: Uses logrus for structured, configurable logging
- **Better error handling**: Leverages Go's explicit error handling patterns
- **Type safety**: Strong typing with compile-time safety
- **Concurrent safety**: Thread-safe configuration caching with sync patterns
- **Memory efficiency**: Efficient JSON marshaling with custom implementations

### Cobra Integration

- **Rich CLI experience**: Automatic help generation, command completion support  
- **Consistent flag handling**: Global and command-specific flags with proper validation
- **Subcommand organization**: Logical grouping of related functionality
- **Version management**: Built-in version handling with build-time injection

## Performance Characteristics

- **Fast startup**: Minimal dependencies, optimized for CLI use
- **Low memory usage**: Efficient data structures and memory management
- **Cached configuration**: Avoids repeated file I/O with thread-safe caching
- **Concurrent processing**: Where applicable, uses Go's concurrency primitives

## Compatibility Matrix

| Feature | Rust Implementation | Go Implementation | Status |
|---------|-------------------|------------------|--------|
| All hook commands | ✅ | ✅ | ✅ Full compatibility |
| Configuration loading | ✅ | ✅ | ✅ Identical behavior |
| Schema generation | ✅ | ✅ | ✅ Same output format |
| Visualization | ✅ | ✅ | ✅ Same functionality |
| File validation | ✅ | ✅ | ✅ Identical rules |
| Tool usage rules | ✅ | ✅ | ✅ Same validation |
| Logging | ✅ | ✅ | ✅ Enhanced with structure |
| Exit codes | ✅ | ✅ | ✅ Identical behavior |

## Contributing

1. Follow Go coding standards and use `gofmt`
2. Maintain compatibility with the Rust implementation
3. Add tests for new functionality
4. Update documentation for changes
5. Run the full CI pipeline before submitting PRs

## License

MIT License - same as the original Rust implementation.