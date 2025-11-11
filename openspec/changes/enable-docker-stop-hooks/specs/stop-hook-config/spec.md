# Specification: Stop Hook Configuration Schema Extensions

**Capability:** stop-hook-config
**Change:** enable-docker-stop-hooks
**Status:** Specification
**Version:** 1.0

## Summary
Define configuration schema extensions for the stop hook to support Docker image specification and related Docker-specific options.

## ADDED Requirements

### Requirement: Configuration Schema Validation
The system SHALL validate Docker-specific fields in StopCommand structure through schema.json updates.

#### Scenario: Basic Docker Field (Image)
When configuration includes Docker image specification:
```json
{
  "stop": {
    "commands": [
      {
        "run": "npm test",
        "image": "node:18-alpine"
      }
    ]
  }
}
```

Then:
1. Configuration validates successfully
2. Image field accepts valid Docker image references
3. Image field is optional (backward compatible)
4. Empty image string is rejected

#### Scenario: Invalid Image Reference
When image field contains invalid reference:
```yaml
stop:
  commands:
    - run: "npm test"
      image: ""
```

Then:
1. Configuration validation fails
2. Error message: "image field cannot be empty"
3. User prompted to either remove image or provide valid reference

#### Scenario: Complete Docker Configuration
When all Docker options are specified:
```yaml
stop:
  commands:
    - run: "npm test"
      image: "node:18-alpine"
      workdir: "/app"
      env:
        - "NODE_ENV=test"
      mounts:
        - type: bind
          source: "."
          target: "/workspace"
      network: "bridge"
      timeout: "5m"
      memory: "512m"
      cpus: "1.5"
      user: "node"
      seccomp: "default"
      apparmor: "docker-default"
      capabilities:
        drop: ["ALL"]
        add: ["NET_BIND_SERVICE"]
```

Then:
1. Full configuration validates
2. All fields are optional
3. Schema enforces type constraints
4. No field is required when image is specified

---

### Requirement: Field-Level Validation Rules
The system SHALL enforce specific validation rules for each Docker-specific configuration field.

#### Scenario: Working Directory Validation
When workdir is specified:
```yaml
stop:
  commands:
    - run: "npm test"
      image: "node:18"
      workdir: "/app"
```

Then:
1. workdir must be absolute path (start with `/`)
2. workdir must not contain `..` (prevent escape)
3. workdir must not be empty string if specified
4. Error message: "workdir must be absolute path (e.g., /app)"

#### Scenario: Environment Variable Format
When env variables are specified:
```yaml
stop:
  commands:
    - run: "npm test"
      env:
        - "KEY=value"
        - "BUILD_ID=${BUILD_ID}"
```

Then:
1. Each env variable must be string in format `KEY=VALUE`
2. Keys must not contain `=` character (before first `=`)
3. Values can be empty (e.g., `KEY=`)
4. Interpolation markers `${VAR}` are allowed
5. Error message: "env variables must be KEY=VALUE format"

#### Scenario: Mount Configuration Validation
When mounts are specified:
```yaml
stop:
  commands:
    - run: "npm test"
      mounts:
        - type: bind
          source: "/host/path"
          target: "/container/path"
```

Then:
1. Mount type must be: `bind`, `volume`, or `tmpfs`
2. Mount target is required for all types
3. Mount source required for `bind` type
4. Mount source required for `volume` type
5. Mount source not needed for `tmpfs` type
6. Readonly defaults to false if not specified
7. Error message: "mount type must be bind, volume, or tmpfs"

#### Scenario: Network Mode Validation
When network is specified:
```yaml
stop:
  commands:
    - run: "npm test"
      network: "bridge"
```

Then:
1. Network must be one of: `bridge`, `host`, `none`, or `container:<name>`
2. For `container:<name>`, name must be valid container reference
3. Error message: "network must be bridge, host, none, or container:<name>"

#### Scenario: Timeout Format Validation
When timeout is specified:
```yaml
stop:
  commands:
    - run: "npm test"
      timeout: "5m"
```

Then:
1. Format: `{number}[s|m|h]`
2. Number must be positive integer or decimal
3. Suffix required: `s` (seconds), `m` (minutes), or `h` (hours)
4. Special value `null` allows unlimited timeout
5. Examples: `30s`, `5m`, `2h`, `null`
6. Error message: "timeout must be format like '5m', '300s', or null"

#### Scenario: Memory Limit Format Validation
When memory is specified:
```yaml
stop:
  commands:
    - run: "npm test"
      memory: "512m"
```

Then:
1. Format: `{number}[b|k|m|g]` (case-insensitive)
2. Number can be integer or decimal
3. Units: `b` (bytes), `k` (kilobytes), `m` (megabytes), `g` (gigabytes)
4. Examples: `512m`, `1g`, `256m`, `1024b`
5. Default unit if omitted: bytes
6. Error message: "memory must be format like '512m' or '1g'"

#### Scenario: CPU Limit Format Validation
When cpus is specified:
```yaml
stop:
  commands:
    - run: "npm test"
      cpus: "1.5"
```

Then:
1. Value must be positive number (integer or decimal)
2. Represents number of CPU cores
3. Examples: `1`, `0.5`, `2.0`, `1.5`
4. Error message: "cpus must be positive number (e.g., 1, 0.5)"

#### Scenario: User Format Validation
When user is specified:
```yaml
stop:
  commands:
    - run: "npm test"
      user: "node"
```

Then:
1. User can be username string or UID:GID format
2. Username: alphanumeric string (e.g., `node`, `root`)
3. UID:GID format: `{uid}:{gid}` where both numeric (e.g., `1000:1000`)
4. UID or GID can be omitted for defaults (e.g., `1000:`, `:1000`)
5. Error message: "user must be username or UID:GID format"

#### Scenario: Seccomp Profile Validation
When seccomp is specified:
```yaml
stop:
  commands:
    - run: "npm test"
      seccomp: "default"
```

Then:
1. Must be: `default`, `unconfined`, or path to profile file
2. Path profiles start with `/` and must be valid files
3. Error message: "seccomp must be 'default', 'unconfined', or file path"

#### Scenario: AppArmor Profile Validation
When apparmor is specified:
```yaml
stop:
  commands:
    - run: "npm test"
      apparmor: "docker-default"
```

Then:
1. String value of profile name
2. Common values: `docker-default`, `unconfined`
3. Can be custom profile name if installed
4. No validation on value (profile validation at runtime)

#### Scenario: Capabilities Configuration
When capabilities are specified:
```yaml
stop:
  commands:
    - run: "npm test"
      capabilities:
        drop: ["ALL"]
        add: ["NET_BIND_SERVICE"]
```

Then:
1. Both `add` and `drop` are optional
2. Capability names must be valid Linux capabilities
3. Common capabilities: `CAP_NET_BIND_SERVICE`, `CAP_CHOWN`, `CAP_DAC_OVERRIDE`, etc.
4. `ALL` is special value for drop (drops all capabilities)
5. Drop is processed before add
6. Error message: "capability must be valid Linux capability name"

---

### Requirement: Global Default Configuration
The system SHALL support global default configuration for Docker commands with per-command override capability.

#### Scenario: Global Default Timeout
When defaultTimeout is specified:
```yaml
stop:
  defaultTimeout: "5m"
  commands:
    - run: "npm test"
      image: "node:18"
      # Uses default 5m timeout
```

Then:
1. Default timeout applied to all commands
2. Per-command timeout overrides global default
3. Per-command `timeout: null` disables timeout
4. Format same as command-level timeout

#### Scenario: Global Default Environment Variables
When defaultEnv is specified:
```yaml
stop:
  defaultEnv:
    - "CI=true"
    - "NODE_ENV=production"
  commands:
    - run: "npm test"
      image: "node:18"
      env:
        - "NODE_ENV=test"  # Overrides default
```

Then:
1. Default environment variables applied to all commands
2. Per-command env vars merged with defaults
3. Per-command values override defaults
4. Final env = default + per-command

---

### Requirement: Configuration File Format Support
The system SHALL support configuration in multiple formats (YAML, JSON, TOML).

#### Scenario: YAML Format
When configuration in YAML format:
```yaml
stop:
  commands:
    - run: "npm test"
      image: "node:18"
```

Then:
1. YAML is parsed correctly
2. Colon, dash, indentation syntax respected
3. Comments with `#` supported

#### Scenario: JSON Format
When configuration in JSON format:
```json
{
  "stop": {
    "commands": [
      {
        "run": "npm test",
        "image": "node:18"
      }
    ]
  }
}
```

Then:
1. JSON is parsed correctly
2. Quotes required around strings
3. No trailing commas allowed

#### Scenario: TOML Format
When configuration in TOML format:
```toml
[[stop.commands]]
run = "npm test"
image = "node:18"
```

Then:
1. TOML is parsed correctly
2. TOML table syntax respected

---

### Requirement: Configuration Examples and Documentation
The system SHALL include comprehensive configuration examples in default configuration documentation.

#### Scenario: Default Configuration Examples
When conclaude initializes with default configuration:
```yaml
stop:
  commands:
    # Example 1: Host execution (no Docker)
    - run: "npm test"
      message: "Tests failed"

    # Example 2: Basic Docker execution
    - run: "npm test"
      image: "node:18-alpine"

    # Example 3: Advanced Docker with mounts
    - run: "./build.sh"
      image: "node:18"
      workdir: "/app"
      env:
        - "NODE_ENV=production"
      mounts:
        - type: bind
          source: "."
          target: "/workspace"

    # Example 4: Python with resource limits
    - run: "python -m pytest"
      image: "python:3.11"
      timeout: "10m"
      memory: "512m"
      cpus: "2"
```

Then:
1. Default config includes 4+ working examples
2. Each example demonstrates key features
3. Examples are well-commented
4. Examples are valid and can be copy-pasted
5. Examples progress from simple to complex

---

## MODIFIED Requirements

### Requirement: Existing Stop Hook Backward Compatibility
The system SHALL maintain full backward compatibility with all existing stop hook configurations.

#### Scenario: Existing Configuration (No Docker)
When configuration uses existing format:
```yaml
stop:
  run: "npm test"  # Legacy format

stop:
  commands:        # Modern format
    - run: "npm test"
```

Then:
1. Configuration parses and validates
2. Commands execute on host (unchanged)
3. No Docker behavior triggered
4. No migration required

---

## REMOVED Requirements

(No requirements removed - this is additive)

---

## Schema Definition

### Root Configuration
```json
{
  "stop": {
    "type": "object",
    "properties": {
      "run": {"type": "string"},
      "commands": {
        "type": "array",
        "items": {"$ref": "#/definitions/stopCommand"}
      },
      "defaultTimeout": {"type": "string"},
      "defaultEnv": {
        "type": "array",
        "items": {"type": "string"}
      },
      "infinite": {"type": "boolean"},
      "infiniteMessage": {"type": "string"},
      "rounds": {"type": ["integer", "null"]}
    }
  }
}
```

### StopCommand Definition
```json
{
  "stopCommand": {
    "type": "object",
    "properties": {
      "run": {
        "type": "string",
        "description": "Command to execute (required)"
      },
      "image": {
        "type": "string",
        "description": "Docker image to run command in (optional, enables Docker execution)"
      },
      "workdir": {
        "type": "string",
        "pattern": "^/.*",
        "description": "Working directory in container (absolute path)"
      },
      "env": {
        "type": "array",
        "items": {"type": "string"},
        "description": "Environment variables (KEY=VALUE format)"
      },
      "mounts": {
        "type": "array",
        "items": {"$ref": "#/definitions/dockerMount"}
      },
      "network": {
        "type": "string",
        "description": "Network mode (bridge, host, none, container:name)"
      },
      "timeout": {
        "type": ["string", "null"],
        "pattern": "^\\d+[smh]$",
        "description": "Timeout duration (e.g., 5m, 300s) or null"
      },
      "memory": {
        "type": "string",
        "pattern": "^\\d+[bkmg]?$",
        "description": "Memory limit (e.g., 512m, 1g)"
      },
      "cpus": {
        "type": "string",
        "pattern": "^\\d+(\\.\\d+)?$",
        "description": "CPU limit (e.g., 1, 0.5)"
      },
      "user": {
        "type": "string",
        "description": "User context (username or UID:GID)"
      },
      "seccomp": {
        "type": "string",
        "description": "Seccomp profile (default, unconfined, or path)"
      },
      "apparmor": {
        "type": "string",
        "description": "AppArmor profile"
      },
      "capabilities": {
        "$ref": "#/definitions/dockerCapabilities"
      },
      "message": {"type": "string"},
      "showStdout": {"type": "boolean"},
      "showStderr": {"type": "boolean"},
      "maxOutputLines": {
        "type": "integer",
        "minimum": 1,
        "maximum": 10000
      }
    },
    "required": ["run"],
    "additionalProperties": false
  }
}
```

### DockerMount Definition
```json
{
  "dockerMount": {
    "type": "object",
    "properties": {
      "type": {
        "type": "string",
        "enum": ["bind", "volume", "tmpfs"],
        "description": "Mount type"
      },
      "source": {
        "type": "string",
        "description": "Host path (for bind and volume)"
      },
      "target": {
        "type": "string",
        "description": "Container path (required)"
      },
      "readonly": {
        "type": "boolean",
        "description": "Read-only mount"
      },
      "options": {
        "type": "array",
        "items": {"type": "string"},
        "description": "Additional mount options"
      }
    },
    "required": ["type", "target"],
    "additionalProperties": false
  }
}
```

### DockerCapabilities Definition
```json
{
  "dockerCapabilities": {
    "type": "object",
    "properties": {
      "add": {
        "type": "array",
        "items": {"type": "string"},
        "description": "Capabilities to add"
      },
      "drop": {
        "type": "array",
        "items": {"type": "string"},
        "description": "Capabilities to drop"
      }
    },
    "additionalProperties": false
  }
}
```

---

## Validation Strategy

### Configuration Validation
1. Parse YAML/JSON/TOML into structured format
2. Validate against JSON Schema
3. Validate individual field formats (timeout, memory, etc.)
4. Validate mount source paths exist (for bind mounts)
5. Report all validation errors (not just first)
6. Provide suggestions for common mistakes

### Error Messages
```
Error: Invalid configuration in .conclaude.yaml

Field: stop.commands[0].timeout
Issue: timeout must be format like '5m', '300s', or null
Value: "5x"

Suggestions:
- Use 's' for seconds, 'm' for minutes, 'h' for hours
- Examples: "30s", "5m", "2h"
- Use null to disable timeout
```

---

## Related Capabilities
- `docker-command-execution`: Docker command execution logic
- `command-timeouts`: Timeout handling for Docker and host execution

---

## Testing Strategy

### Unit Tests
- Schema validation with valid configurations
- Schema validation with invalid configurations
- Field format validation (timeout, memory, cpus, etc.)
- Mount configuration validation
- Global defaults merging
- Configuration parsing (YAML, JSON, TOML)

### Integration Tests
- End-to-end configuration loading
- Configuration examples validation
- Default configuration validation
- Migration from legacy format

---

## Success Criteria

- [ ] Schema.json validates all Docker fields
- [ ] All field validation rules enforced
- [ ] Backward compatible with existing configs
- [ ] Default configuration includes examples
- [ ] Error messages clear and helpful
- [ ] Configuration documentation complete
- [ ] Zero validation regressions
