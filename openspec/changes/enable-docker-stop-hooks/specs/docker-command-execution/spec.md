# Specification: Docker Command Execution

**Capability:** docker-command-execution
**Change:** enable-docker-stop-hooks
**Status:** Specification
**Version:** 1.0

## Summary
Enable stop hook commands to execute within isolated Docker containers with full lifecycle management, resource controls, and security context configuration.

## ADDED Requirements

### Requirement: Docker Image Specification
The system SHALL support optional Docker image specification for commands to execute within isolated containers.

#### Scenario: Command with Docker Image
When a stop command specifies an `image` field:
```yaml
stop:
  commands:
    - run: "npm test"
      image: "node:18-alpine"
```

Then:
1. The command executes inside a container based on the specified image
2. The project directory is auto-mounted to `/workspace` in the container
3. The command runs with `/bin/sh -c "<command>"` inside the container
4. The container is cleaned up after execution (success or failure)

#### Scenario: Command without Docker Image
When a stop command does NOT specify an `image` field:
```yaml
stop:
  commands:
    - run: "npm test"
```

Then:
1. The command executes on the host system (backward compatible)
2. No container is created
3. Existing host execution behavior is maintained

---

### Requirement: Container Lifecycle Management
The system SHALL properly manage container lifecycle including creation, startup, execution, and cleanup operations.

#### Scenario: Successful Container Execution
When a command executes successfully in a Docker container:
1. Container is created with proper configuration
2. Container is started
3. Command executes to completion
4. Output is captured (stdout/stderr)
5. Exit code is preserved
6. Container is removed

#### Scenario: Container Cleanup on Failure
When a container execution fails (command error, timeout, infrastructure failure):
1. Container is still removed (cleanup guaranteed)
2. Error details are captured and returned to user
3. Partial output is included in error message

#### Scenario: Container Not Found
When the specified image is not available:
1. Docker attempts to pull the image (if configured)
2. If pull fails or image doesn't exist: clear error message
3. Suggest common solutions (e.g., `docker pull <image>`)

---

### Requirement: Working Directory Configuration
The system SHALL support optional working directory configuration for commands executing within containers.

#### Scenario: Default Working Directory
When no `workdir` is specified:
```yaml
stop:
  commands:
    - run: "npm test"
      image: "node:18"
```

Then:
1. Container works from image's default working directory
2. Project directory mounted at `/workspace`
3. If command references relative paths, they resolve from image default

#### Scenario: Custom Working Directory
When `workdir` is specified:
```yaml
stop:
  commands:
    - run: "npm test"
      image: "node:18"
      workdir: "/app"
```

Then:
1. Container working directory is set to `/app`
2. Command executes from that directory
3. `workdir` must be absolute path (e.g., `/app`, not `./app`)

---

### Requirement: Volume Mounting and Path Management
The system SHALL support volume mounting and automatic project directory mounting to enable file access in containers.

#### Scenario: Auto-Mount Project Directory
When a command has an image specified:
```yaml
stop:
  commands:
    - run: "npm test"
      image: "node:18"
```

Then:
1. Project directory (CWD) is automatically mounted to `/workspace`
2. Mount is read-write by default
3. All project files accessible in container at `/workspace/<path>`
4. File modifications persist to host

#### Scenario: Custom Bind Mount (Read-Write)
When user specifies custom bind mounts:
```yaml
stop:
  commands:
    - run: "npm test"
      image: "node:18"
      mounts:
        - type: bind
          source: "."
          target: "/app"
          readonly: false
```

Then:
1. Source path (relative or absolute) is mounted to target in container
2. `readonly: false` allows writing to mounted files
3. Changes in container persist on host

#### Scenario: Read-Only Mount
When `readonly` is specified:
```yaml
stop:
  commands:
    - run: "npm test"
      image: "node:18"
      mounts:
        - type: bind
          source: "./node_modules"
          target: "/app/node_modules"
          readonly: true
```

Then:
1. Mount is read-only inside container
2. Container cannot modify mounted files
3. Host files protected from accidental modification

#### Scenario: Temporary File System Mount
When tmpfs mount is specified:
```yaml
stop:
  commands:
    - run: "npm test"
      image: "node:18"
      mounts:
        - type: tmpfs
          target: "/tmp"
```

Then:
1. Temporary filesystem mounted at `/tmp`
2. Files in tmpfs not persisted
3. Each container execution gets clean tmpfs

---

### Requirement: Environment Variable Passing
The system SHALL support passing environment variables to containerized commands with interpolation and merging capabilities.

#### Scenario: Basic Environment Variables
When environment variables are specified:
```yaml
stop:
  commands:
    - run: "npm test"
      image: "node:18"
      env:
        - "NODE_ENV=test"
        - "DEBUG=true"
```

Then:
1. Variables are passed to container
2. Variables are available to the executed command
3. Variables override image defaults

#### Scenario: Environment Variable Interpolation
When environment variables reference host environment:
```yaml
stop:
  commands:
    - run: "npm test"
      image: "node:18"
      env:
        - "BUILD_ID=${BUILD_ID}"
        - "API_KEY=${API_KEY}"
```

Then:
1. `${VAR}` references are interpolated from host environment
2. If host variable doesn't exist, error is reported
3. Interpolation happens before passing to container

#### Scenario: Merging with Global Defaults
When environment variables are specified both globally and per-command:
```yaml
stop:
  defaultEnv:
    - "CI=true"
    - "LOG_LEVEL=info"
  commands:
    - run: "npm test"
      image: "node:18"
      env:
        - "NODE_ENV=test"
```

Then:
1. Global environment variables are merged with command-specific ones
2. Command-specific variables override global defaults
3. Final environment contains: `CI=true`, `LOG_LEVEL=info`, `NODE_ENV=test`

---

### Requirement: Network Configuration
The system SHALL support configuration of container network mode (bridge, host, none).

#### Scenario: Default Network Mode
When no network is specified:
```yaml
stop:
  commands:
    - run: "npm test"
      image: "node:18"
```

Then:
1. Container uses default Docker network (bridge)
2. Container can reach external networks

#### Scenario: Host Network Mode
When network is set to "host":
```yaml
stop:
  commands:
    - run: "npm test"
      image: "node:18"
      network: "host"
```

Then:
1. Container shares host network stack
2. Container ports directly accessible on host
3. Performance optimal for network operations

#### Scenario: No Network Mode
When network is set to "none":
```yaml
stop:
  commands:
    - run: "npm test"
      image: "node:18"
      network: "none"
```

Then:
1. Container has no network access
2. Only loopback interface available
3. Useful for security-sensitive commands

---

### Requirement: Resource Limits
The system SHALL support configuration of container resource limits (memory and CPU).

#### Scenario: Memory Limit
When memory limit is specified:
```yaml
stop:
  commands:
    - run: "npm test"
      image: "node:18"
      memory: "512m"
```

Then:
1. Container memory limited to 512MB
2. If memory exceeded, container killed
3. Process receives OOM signal
4. Valid formats: `256m`, `1g`, `512MB` (case-insensitive)

#### Scenario: CPU Limit
When CPU limit is specified:
```yaml
stop:
  commands:
    - run: "npm test"
      image: "node:18"
      cpus: "1.5"
```

Then:
1. Container limited to 1.5 CPU cores
2. Value can be decimal (e.g., `0.5` for half core)
3. Docker throttles CPU time accordingly

#### Scenario: Memory and CPU Combined
When both limits specified:
```yaml
stop:
  commands:
    - run: "npm test"
      image: "node:18"
      memory: "512m"
      cpus: "1"
```

Then:
1. Both limits are enforced independently
2. Container cannot exceed either resource
3. OOM killer or CPU throttling triggered as needed

---

### Requirement: Security Context
The system SHALL support configuration of container security context (user, seccomp, AppArmor, capabilities).

#### Scenario: User Context
When user is specified:
```yaml
stop:
  commands:
    - run: "npm test"
      image: "node:18"
      user: "node"
```

Then:
1. Container executes as specified user
2. Can be username (from image) or UID:GID (e.g., "1000:1000")
3. Useful for permission management

#### Scenario: Seccomp Profile
When seccomp profile is specified:
```yaml
stop:
  commands:
    - run: "npm test"
      image: "node:18"
      seccomp: "default"
```

Then:
1. Docker seccomp profile is applied
2. Restricts system calls available to container
3. Profiles: `default`, `unconfined`, or path to custom profile

#### Scenario: AppArmor Profile
When apparmor profile is specified:
```yaml
stop:
  commands:
    - run: "npm test"
      image: "node:18"
      apparmor: "docker-default"
```

Then:
1. AppArmor profile applied to container (Linux only)
2. Provides mandatory access control
3. Restricts container capabilities

#### Scenario: Dropped Capabilities
When capabilities are dropped:
```yaml
stop:
  commands:
    - run: "npm test"
      image: "node:18"
      capabilities:
        drop:
          - "ALL"
```

Then:
1. All capabilities removed from container
2. Container runs with minimal privileges
3. Suitable for sandboxing untrusted workloads

#### Scenario: Added Capabilities
When capabilities are added:
```yaml
stop:
  commands:
    - run: "npm test"
      image: "node:18"
      capabilities:
        drop:
          - "ALL"
        add:
          - "NET_BIND_SERVICE"
```

Then:
1. Specified capabilities are added to container
2. Dropped first, then added
3. Allows minimal privilege elevation when needed

---

### Requirement: Docker Availability Handling
The system SHALL gracefully handle Docker daemon unavailability with clear error messages and fallback behavior.

#### Scenario: Docker Daemon Not Available
When Docker is not running or not installed:
1. System detects Docker unavailability
2. Commands without `image` field execute on host normally
3. Commands with `image` field fail with clear error message:
   ```
   Error: Docker image specified but Docker daemon not available
     Command: npm test
     Image: node:18

     Solutions:
     1. Install Docker Desktop (https://docker.com)
     2. Start Docker daemon
     3. Check permissions: docker ps
     4. Remove 'image' field to run on host instead
   ```

#### Scenario: Docker Permissions
When user lacks Docker permissions:
1. Docker client connection fails
2. Clear error message indicates permission issue
3. Suggests: `sudo usermod -aG docker $USER`

---

### Requirement: Error Handling and Cleanup
The system SHALL properly handle errors and ensure cleanup of container resources in all execution paths.

#### Scenario: Command Execution Failure
When command inside container exits with error:
1. Exit code is captured and returned
2. stdout/stderr output included
3. Container is removed
4. Error is reported to user with command output

#### Scenario: Container Startup Failure
When container fails to start (image pull failure, config error):
1. Error details captured
2. Container cleanup attempted
3. Clear error message to user with suggestions
4. No partial state left behind

#### Scenario: Output Capture Failure
When output cannot be captured from container:
1. Container execution stops
2. Container is cleaned up
3. Error reported to user
4. Suggests checking Docker logs

---

## MODIFIED Requirements

(This is a new capability - no existing requirements modified)

---

## REMOVED Requirements

(This is a new capability - no requirements removed)

---

## Configuration Schema

```yaml
stop:
  commands:
    - run: string                          # Required: command to execute
      image: string?                       # Optional: Docker image (enables Docker execution)
      workdir: string?                     # Optional: working directory in container
      env: string[]?                       # Optional: environment variables (KEY=VALUE)
      mounts: Mount[]?                     # Optional: volume mounts
      network: string?                     # Optional: network mode (bridge, host, none)
      timeout: string?                     # Optional: timeout duration
      memory: string?                      # Optional: memory limit (512m, 1g)
      cpus: string?                        # Optional: CPU limit (1, 0.5)
      user: string?                        # Optional: user context (username or UID:GID)
      seccomp: string?                     # Optional: seccomp profile
      apparmor: string?                    # Optional: AppArmor profile
      capabilities: Capabilities?          # Optional: Linux capabilities
      message: string?                     # Existing: error message
      showStdout: boolean?                 # Existing: show stdout
      showStderr: boolean?                 # Existing: show stderr
      maxOutputLines: integer?             # Existing: max output lines

Mount:
  type: string                             # "bind", "volume", "tmpfs"
  source: string                           # Host path (for bind mounts)
  target: string                           # Container path
  readonly: boolean?                       # Read-only flag
  options: string[]?                       # Additional mount options

Capabilities:
  add: string[]?                           # Capabilities to add (NET_BIND_SERVICE, etc.)
  drop: string[]?                          # Capabilities to drop (ALL, NET_RAW, etc.)
```

---

## Related Capabilities
- `stop-hook-config`: Configuration schema extensions
- `command-timeouts`: Timeout enforcement for container execution

---

## Implementation Notes

1. **Bollard Crate**: Uses `bollard` Rust Docker API client
2. **Async Execution**: Container operations are async using tokio
3. **Cleanup Guarantee**: Try/finally pattern ensures cleanup
4. **Path Translation**: Host paths translated to container context
5. **Auto-Mount**: `/workspace` is auto-mounted unless overridden
6. **Timeout Support**: See `command-timeouts` spec
7. **Backward Compatible**: No changes to host execution
8. **Platform Support**: Linux, macOS, Windows (Docker Desktop)

---

## Testing Strategy

### Unit Tests
- Container configuration generation
- Path translation logic
- Environment variable interpolation
- Resource limit parsing
- Capability parsing

### Integration Tests
- Basic container execution
- Output capture
- Error handling
- Cleanup verification
- Resource limit enforcement
- Security context application
- Mount behavior
- Environment variable passing
- Network modes

### Error Scenario Tests
- Image not found
- Container startup failure
- Command failure
- Output capture failure
- Docker unavailable
- Permission denied
- Resource exhaustion

---

## Success Criteria

- [ ] Commands with `image` field execute in Docker
- [ ] Auto-mount of project directory works
- [ ] All Docker configuration options applied
- [ ] Container cleanup guaranteed
- [ ] Error messages clear and actionable
- [ ] Full test coverage
- [ ] Zero backward compatibility issues
