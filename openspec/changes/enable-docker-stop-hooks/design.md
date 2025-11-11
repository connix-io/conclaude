# Design: Docker Image Support for Stop Hooks

## Architecture Overview

```
┌─────────────────────────────────────────────┐
│  Stop Hook Handler (src/hooks.rs)           │
├─────────────────────────────────────────────┤
│  execute_stop_commands()                    │
│    ├─ For each StopCommand:                 │
│    ├─ Check if image field present          │
│    ├─ Route to:                             │
│    │  ├─ execute_in_docker() if image ∃     │
│    │  └─ execute_on_host() if image ∅       │
│    └─ Collect output & exit codes           │
├─────────────────────────────────────────────┤
│  execute_in_docker()                        │
│    ├─ Docker Client (Bollard)               │
│    ├─ Container Lifecycle:                  │
│    │  ├─ Create with config                 │
│    │  ├─ Mount volumes                      │
│    │  ├─ Start container                    │
│    │  ├─ Execute command                    │
│    │  ├─ Stream output                      │
│    │  └─ Cleanup on completion              │
│    └─ Timeout enforcement                   │
├─────────────────────────────────────────────┤
│  execute_on_host()                          │
│    ├─ Existing Bash execution               │
│    ├─ Add timeout support                   │
│    └─ Return exit code & output             │
└─────────────────────────────────────────────┘
```

## Key Design Decisions

### 1. Docker Client Choice: Bollard

**Decision:** Use Bollard crate (native Rust Docker API client)

**Rationale:**
- **Reliability**: Direct API calls, no subprocess spawning
- **Performance**: No shell overhead, faster container management
- **Type Safety**: Full Rust type system for Docker operations
- **Async Native**: Works seamlessly with tokio runtime (already used)
- **Error Handling**: Structured error types vs parsing CLI output

**Alternative Considered:**
- Docker CLI wrapper: Simple but slow, brittle error handling, subprocess overhead
- containerd/runc: Lower level, more control but significantly more complex

### 2. Backward Compatibility via Opt-in

**Decision:** Commands WITHOUT `image` field execute on host; commands WITH `image` run in Docker

**Rationale:**
- Zero breaking changes to existing configurations
- Gradual adoption - users opt-in per command
- No need for major version bump
- Existing workflows continue unchanged

```yaml
stop:
  commands:
    # Continues to run on host (no image specified)
    - run: "npm test"

    # Runs in Docker (image specified)
    - run: "npm test"
      image: "node:18-alpine"
```

### 3. Auto-Mount Strategy

**Decision:** Project directory auto-mounted read-write to `/workspace` when image is specified

**Rationale:**
- Commands can access project files without configuration
- Sensible default matches common Docker practice
- Users can override or add additional mounts
- Maintains consistency with CWD on host execution

**Mount Behavior:**
```
Host: /home/user/project (current directory)
  ↓
Container: /workspace (read-write by default)

User can override or add mounts:
- Read-only mount for immutable files
- Different container paths
- Additional volumes
```

### 4. Timeout Handling

**Decision:** Add optional timeout with 5-minute default; applies to host AND container execution

**Motivation:**
- Prevents infinite-running commands (currently missing)
- Applies consistently across both execution modes
- Users can disable with `timeout: null` if needed
- Individual per-command control

**Implementation:**
- Host execution: Use `tokio::time::timeout()`
- Docker execution: Use Docker API timeout + OS-level signals
- Display clear timeout messages to user

### 5. Configuration Inheritance & Defaults

**Decision:** Hierarchical defaults with full override capability

```yaml
stop:
  # Global defaults (optional)
  defaultTimeout: "5m"
  defaultEnv:
    - "CI=true"

  commands:
    - run: "npm test"
      image: "node:18"
      # Uses global defaults, can override:
      timeout: "10m"  # Override default
      env:
        - "NODE_ENV=test"
        - "CI=true"   # Merged with defaults
```

### 6. Path Translation Strategy

**Challenge:** Commands reference host paths (CWD) but run in container with different paths

**Solution:** Context-aware path translation
- File validation rules adapted for container context
- Generated file checks use container paths
- Root addition prevention works in `/workspace` context
- User paths in `mounts` translate automatically

**Example:**
```yaml
stop:
  commands:
    - run: "npm test"
      image: "node:18"
      # Host: /home/user/project/src/index.js
      # Container: /workspace/src/index.js
      # File validation rules still work!
```

### 7. Security Context

**Decision:** Reasonable defaults with override capability

```yaml
commands:
  - run: "npm test"
    image: "node:18"
    user: "node"              # Run as specific user (default: image default)
    seccomp: "default"        # Docker seccomp profile
    apparmor: "docker-default" # AppArmor profile
    capabilities:
      drop: ["ALL"]
      add: ["NET_BIND_SERVICE"]
```

### 8. Resource Limits

**Decision:** Optional but recommended; no defaults (container defaults apply)

```yaml
commands:
  - run: "npm test"
    image: "node:18"
    memory: "512m"           # Memory limit
    cpus: "1.0"              # CPU limit
    # Users must explicitly set if they want limits
```

### 9. Environment Variable Handling

**Features:**
- Pass environment variables to container
- Support interpolation of host env vars
- Safe secret handling (no logging of sensitive vars)
- Merge with global defaults

```yaml
commands:
  - run: "npm test"
    image: "node:18"
    env:
      - "NODE_ENV=test"
      - "BUILD_ID=${BUILD_ID}"  # Interpolate from host
      - "API_KEY=${API_KEY}"    # From host, not logged
```

### 10. Network Configuration

**Decision:** Support multiple network modes

```yaml
commands:
  - run: "npm test"
    image: "node:18"
    network: "bridge"  # bridge, host, none, container:<name>
```

### 11. Error Handling & Cleanup

**Principles:**
- Always cleanup containers (success or failure)
- Preserve container logs for debugging
- Clear error messages to user
- Distinguish between command failure vs infrastructure failure

**Cleanup Strategy:**
```rust
// Pseudo-code
let container_id = create_container(config)?;
let result = match run_container(&container_id, command).await {
    Ok(output) => Ok(output),
    Err(e) => {
        capture_logs(&container_id); // For debugging
        Err(e)
    }
};
// ALWAYS cleanup
cleanup_container(&container_id).await?;
result
```

## Implementation Phases

### Phase 1: Core Infrastructure
1. Add Bollard dependency
2. Extend type system for Docker config
3. Basic Docker client initialization
4. Container creation and cleanup

### Phase 2: Command Execution
1. Implement `execute_in_docker()`
2. Volume/mount handling
3. Environment variable passing
4. Command execution and output capture

### Phase 3: Advanced Features
1. Resource limits (memory, CPU)
2. Timeout enforcement
3. Security context (user, capabilities)
4. Network mode configuration

### Phase 4: Testing & Validation
1. Integration tests with real Docker
2. Error scenario testing
3. Configuration validation
4. Performance testing
5. Documentation

### Phase 5: Polish & Documentation
1. Error message improvements
2. Configuration examples
3. Troubleshooting guide
4. Migration guide for existing users

## Fallback & Graceful Degradation

**If Docker is unavailable:**
1. Detect at startup (optional check)
2. Allow host-only execution
3. Commands with `image` field will fail with clear error
4. Commands without `image` field work fine

**Clear Error Messages:**
```
Error: Docker image specified but Docker daemon not available
  Command: npm test
  Image: node:18

  Solutions:
  1. Install Docker Desktop
  2. Start Docker daemon: docker daemon
  3. Check permissions: docker ps
  4. Remove 'image' field to run on host instead
```

## Dependency Management

**New Crate: bollard**
- Latest stable version at time of implementation
- Handles Docker API communication
- Async/await support via tokio

**Existing Dependencies Leveraged:**
- tokio: Async runtime (already used)
- serde/serde_json: Configuration (already used)
- anyhow/thiserror: Error handling (already used)

## Testing Strategy

### Unit Tests
- Configuration parsing with Docker fields
- Path translation logic
- Environment variable interpolation
- Timeout calculation

### Integration Tests
- Pull image (or use local)
- Create and run container
- Verify output capture
- Test timeout enforcement
- Cleanup verification

### Error Scenario Tests
- Missing image
- Container startup failure
- Command failure in container
- Timeout trigger
- Docker daemon unavailable
- Permission denied
- Resource exhaustion

### Performance Tests
- Container startup time
- Output streaming latency
- Memory usage with large output
- Cleanup performance

## Compatibility Notes

### Operating Systems
- Linux: Full support (Docker native)
- macOS: Full support (Docker Desktop)
- Windows: Full support (Docker Desktop)

### Existing Features
- File validation rules: ✅ Work in container context
- Generated file protection: ✅ Adapted for container paths
- Root addition prevention: ✅ Works in `/workspace`
- Output limiting: ✅ Applied to container output
- Error messages: ✅ Enhanced with Docker context

### Breaking Changes
None. This is fully backward compatible.

## Future Considerations

1. **Image Caching**: Pre-pull images for performance
2. **Custom Networks**: Connect containers for multi-command workflows
3. **Docker Compose**: Support docker-compose.yml for complex setups
4. **Kubernetes**: Support for kubelet execution (future)
5. **Registry Auth**: Support private image registries
6. **Image Signing**: Verify image signatures for security

## References

- Bollard documentation: https://docs.rs/bollard/
- Docker API: https://docs.docker.com/engine/api/
- Docker CLI reference: https://docs.docker.com/engine/reference/commandline/docker/
