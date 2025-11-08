# Stop Configuration Capability

## REMOVED Requirements

### Requirement: Legacy stop.run Configuration Field

The system SHALL NOT support the deprecated top-level `stop.run` string field for specifying stop hook commands.

**Reason**: Superseded by structured `stop.commands[]` array format

#### Scenario: User specifies commands via top-level stop.run string field

**Given** a configuration file with `stop.run` as a newline-delimited string
**When** the configuration is parsed
**Then** the system SHALL reject the configuration with a validation error
**And** the error message SHALL indicate that `stop.run` is no longer supported
**And** the error message SHALL suggest migrating to `stop.commands[]` format

**Migration Example**:
```yaml
# REMOVED - No longer supported
stop:
  run: |
    npm run lint
    npm run test

# Use this instead
stop:
  commands:
    - run: npm run lint
    - run: npm run test
```

#### Scenario: Legacy stop.run extraction in command collection

**Given** the system is collecting stop commands for execution
**When** the configuration is processed
**Then** the system SHALL NOT extract commands from `stop.run` field
**And** the system SHALL ONLY process commands from `stop.commands[]` array

#### Scenario: Schema validation rejects stop.run field

**Given** a JSON schema validator for conclaude configuration
**When** a configuration contains `stop.run` field
**Then** the schema validation SHALL fail
**And** the schema SHALL NOT define `stop.run` as a valid property under `StopConfig`

#### Scenario: Default configuration template excludes stop.run

**Given** the default configuration template in `src/default-config.yaml`
**When** users reference or copy the default configuration
**Then** the template SHALL NOT include `stop.run` field
**And** the template SHALL demonstrate the `stop.commands[]` array format
**And** the template comments SHALL explain the structured command syntax

## MODIFIED Requirements

### Requirement: Stop Commands Configuration

The system SHALL use the structured `stop.commands[]` array format as the ONLY supported method for configuring stop hook commands.

**Modified**: Clarified as the ONLY supported format for stop commands (legacy `stop.run` removed)

#### Scenario: User configures stop commands using structured array

**Given** a configuration file with `stop.commands[]` array
**When** each command object contains a `run` field with a bash command
**Then** the system SHALL execute each command in order during stop hook
**And** the system SHALL support optional fields: `message`, `showStdout`, `showStderr`, `maxOutputLines`

**Example**:
```yaml
stop:
  commands:
    - run: npm run lint
      message: "Linting code..."
      showStdout: false
    - run: npm run test
      message: "Running tests..."
      maxOutputLines: 50
```

**Note**: This is now the ONLY supported way to configure stop commands. The legacy `stop.run` string field has been removed.

#### Scenario: Empty commands array is valid

**Given** a configuration file with `stop.commands: []`
**When** the stop hook is triggered
**Then** the system SHALL NOT execute any commands
**And** the system SHALL complete successfully without errors

#### Scenario: Missing commands field defaults to empty array

**Given** a configuration file with `stop:` section but no `commands` field
**When** the configuration is parsed
**Then** the system SHALL default `stop.commands` to an empty array
**And** the system SHALL NOT require the `commands` field to be present

## Implementation Notes

### Code Changes Required

1. **`src/config.rs`**:
   - Remove `run: String` field from `StopConfig` struct
   - Remove `#[serde(default)]` attribute for legacy `run` field
   - Keep `commands: Vec<StopCommand>` as the sole command source

2. **`src/hooks.rs`**:
   - In `collect_stop_commands()`, remove legacy extraction logic
   - Remove call to `extract_bash_commands()` for `config.stop.run`
   - Simplify function to only process `config.stop.commands`
   - Update unit tests to remove legacy format test cases

3. **`schema.json`**:
   - Remove `stop.run` property definition from `StopConfig` object
   - Remove default value specification for `run` field
   - Schema will auto-regenerate from updated Rust structs

4. **`src/default-config.yaml`**:
   - Remove `run: ""` field and associated comments
   - Ensure `commands: []` is shown as the default

### Test Changes Required

1. **`tests/config_tests.rs`**:
   - Replace all `stop.run: "..."` with `stop.commands: [{run: "..."}]`
   - Verify approximately 14 test fixtures need updating

2. **`tests/output_limiting_tests.rs`**:
   - Remove tests for legacy `stop.run` format
   - Remove tests for mixed legacy + modern format
   - Keep only `stop.commands[]` format tests
   - Ensure coverage of edge cases (empty arrays, missing fields)

3. **`src/hooks.rs` (test module)**:
   - Update unit tests for `collect_stop_commands()`
   - Remove legacy format test cases
   - Add test case verifying legacy `run` field causes error (if applicable)

### Documentation Changes Required

1. **`README.md`**:
   - Update all configuration examples (~15 occurrences)
   - Remove all references to `stop.run` string field
   - Update "Stop Hook Command Execution" section
   - Add migration note in breaking changes section

### Backward Compatibility

**Breaking Change**: This is a breaking change requiring user action.

**Detection**: Existing configs with `stop.run` will fail schema validation.

**Migration**: Users must convert:
```yaml
# Before
stop:
  run: |
    command1
    command2

# After
stop:
  commands:
    - run: command1
    - run: command2
```

**Future Enhancement**: Consider adding `conclaude migrate` command to automate this conversion.
