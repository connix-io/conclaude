# Remove Legacy `stop.run` Configuration

**Status**: `proposed`
**Created**: 2025-11-08
**Capability**: `stop-config`

## Summary

Remove the deprecated top-level `stop.run` string field from the configuration schema, completing the migration to the structured `stop.commands[]` array format. This change eliminates the legacy single-string command syntax in favor of the more flexible and feature-rich command array structure.

## Motivation

The `stop.run` field was the original simple approach to specifying stop hook commands as a newline-delimited string. A previous enhancement introduced `stop.commands[]` as a structured array format that provides per-command configuration (messages, output control, timeout, etc.).

Currently, both formats coexist:
- **Legacy**: `stop.run: "cmd1\ncmd2"` - simple but limited
- **Modern**: `stop.commands: [{run: "cmd1"}, {run: "cmd2"}]` - structured with rich options

Maintaining both formats creates:
1. **Complexity**: Dual code paths in `collect_stop_commands()` that merge legacy and modern formats
2. **Confusion**: Users must choose between two ways to configure the same functionality
3. **Maintenance burden**: Schema, tests, and documentation must cover both approaches
4. **Inconsistency**: Legacy format lacks features available in modern format

Removing the legacy `stop.run` field simplifies the codebase, reduces cognitive load for users, and ensures all stop commands benefit from modern configuration capabilities.

## Breaking Changes

### Configuration Migration Required

Users currently using the legacy `stop.run` field must migrate to `stop.commands[]`:

**Before (Legacy)**:
```yaml
stop:
  run: |
    npm run lint
    npm run test
    npm run build
```

**After (Modern)**:
```yaml
stop:
  commands:
    - run: npm run lint
    - run: npm run test
    - run: npm run build
```

### Additional Configuration Options

The migration also enables users to leverage modern command features:

```yaml
stop:
  commands:
    - run: npm run lint
      message: "Linting code..."
      showStdout: false
    - run: npm run test
      message: "Running tests..."
      maxOutputLines: 50
    - run: npm run build
      message: "Building project..."
```

### Affected Files

This change impacts:
- Configuration schema (`schema.json`)
- Configuration structs (`src/config.rs`)
- Hook execution logic (`src/hooks.rs`)
- Default configuration template (`src/default-config.yaml`)
- Documentation (`README.md`)
- Test fixtures (`tests/*.rs`)

## Migration Path

1. **Detection**: Users with `stop.run` will receive clear error messages upon config validation
2. **Documentation**: README and migration guide will provide clear examples
3. **Automation**: Future enhancement could provide `conclaude migrate` command to auto-convert configs

## Alternatives Considered

1. **Keep both formats indefinitely**: Rejected due to ongoing maintenance burden and user confusion
2. **Deprecation warning period**: Considered, but the modern format has been available and the legacy format was always documented as temporary
3. **Auto-migration at runtime**: Rejected as it hides the breaking change and prevents users from learning the new format

## Related Changes

- Depends on: Previous change that introduced `stop.commands[]` structure
- Enables: Future simplification of command execution logic
- Blocks: None

## Validation

- All existing tests updated to use `stop.commands[]` format
- Schema validation enforces removal of `stop.run` field
- Comprehensive test coverage for edge cases (empty arrays, missing fields, etc.)
