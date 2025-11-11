# Implementation Tasks: Docker Image Support for Stop Hooks

**Change ID:** `enable-docker-stop-hooks`
**Total Estimated Effort:** 2-3 weeks
**Phases:** 5 (Infrastructure → Core → Advanced → Testing → Polish)

## Phase 1: Core Infrastructure (Days 1-2)

### 1.1 Add Bollard Dependency
**Status:** pending
**Effort:** 0.5 day
**Description:** Add `bollard` crate to `Cargo.toml` with latest stable version and feature flags

**Acceptance Criteria:**
- [ ] `cargo build` succeeds
- [ ] Bollard compiles without errors
- [ ] Documentation available via `cargo doc`

**Tasks:**
- Update `Cargo.toml` with bollard dependency
- Run `cargo build` to verify
- Document version choice in code comments

---

### 1.2 Extend Types for Docker Configuration
**Status:** pending
**Effort:** 1 day
**Description:** Add new Rust types to `src/types.rs` for Docker-specific configuration

**Acceptance Criteria:**
- [ ] New `DockerConfig` struct with all required fields
- [ ] New enums: `MountType`, `NetworkMode`
- [ ] `StopCommand` extended with optional Docker fields
- [ ] Serialization/deserialization works with serde
- [ ] Code compiles without warnings
- [ ] Documentation comments on all public types

**Detailed Requirements:**

```rust
// New types to add to src/types.rs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockerConfig {
    pub image: String,
    pub workdir: Option<String>,
    pub env: Option<Vec<String>>,
    pub mounts: Option<Vec<DockerMount>>,
    pub network: Option<String>,
    pub timeout: Option<String>,
    pub memory: Option<String>,
    pub cpus: Option<String>,
    pub user: Option<String>,
    pub seccomp: Option<String>,
    pub apparmor: Option<String>,
    pub capabilities: Option<DockerCapabilities>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockerMount {
    pub r#type: String,          // "bind", "volume", "tmpfs"
    pub source: String,          // Host path
    pub target: String,          // Container path
    pub readonly: Option<bool>,
    pub options: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockerCapabilities {
    pub add: Option<Vec<String>>,
    pub drop: Option<Vec<String>>,
}

// Extend StopCommand in src/types.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StopCommand {
    pub run: String,
    pub message: Option<String>,
    pub showStdout: Option<bool>,
    pub showStderr: Option<bool>,
    pub maxOutputLines: Option<u32>,
    // NEW DOCKER FIELDS:
    pub image: Option<String>,
    pub workdir: Option<String>,
    pub env: Option<Vec<String>>,
    pub mounts: Option<Vec<DockerMount>>,
    pub network: Option<String>,
    pub timeout: Option<String>,
    pub memory: Option<String>,
    pub cpus: Option<String>,
    pub user: Option<String>,
    pub seccomp: Option<String>,
    pub apparmor: Option<String>,
    pub capabilities: Option<DockerCapabilities>,
}
```

**Validation Rules:**
- timeout format: "{number}[s|m|h]" or "null"
- memory format: "{number}[b|k|m|g]"
- cpus: positive decimal number
- image: non-empty string
- workdir: absolute path starting with "/"

**Tasks:**
1. Add all new types to `src/types.rs`
2. Implement `Default` trait where appropriate
3. Add validation helper functions
4. Add extensive documentation comments
5. Run `cargo check` to verify types compile
6. Add unit tests for validation helpers

---

### 1.3 Update Schema Configuration
**Status:** pending
**Effort:** 1 day
**Description:** Update `schema.json` and `default-config.yaml` with new Docker fields

**Acceptance Criteria:**
- [ ] `schema.json` includes all Docker fields with proper validation
- [ ] `default-config.yaml` has commented examples of all Docker options
- [ ] Schema validation works: `openspec validate` succeeds
- [ ] Examples are realistic and reference valid Docker images
- [ ] Documentation explains each field clearly

**Detailed Requirements:**

**schema.json updates (stopCommandSchema):**
```json
{
  "type": "object",
  "properties": {
    "run": {"type": "string", "description": "Command to execute"},
    "message": {"type": "string"},
    "showStdout": {"type": "boolean"},
    "showStderr": {"type": "boolean"},
    "maxOutputLines": {"type": "integer", "minimum": 1, "maximum": 10000},

    // NEW DOCKER FIELDS
    "image": {
      "type": "string",
      "description": "Docker image to run command in. If specified, command runs in container; otherwise runs on host"
    },
    "workdir": {
      "type": "string",
      "description": "Working directory in container"
    },
    "env": {
      "type": "array",
      "items": {"type": "string"},
      "description": "Environment variables as KEY=VALUE"
    },
    "mounts": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "type": {"enum": ["bind", "volume", "tmpfs"]},
          "source": {"type": "string"},
          "target": {"type": "string"},
          "readonly": {"type": "boolean"},
          "options": {"type": "array", "items": {"type": "string"}}
        },
        "required": ["type", "target"]
      }
    },
    "network": {
      "type": "string",
      "description": "Network mode: bridge, host, none, or container:<name>"
    },
    "timeout": {
      "type": ["string", "null"],
      "pattern": "^\\d+[smh]$|null$",
      "description": "Timeout duration (e.g., '5m', '300s'). null for no timeout"
    },
    "memory": {
      "type": "string",
      "pattern": "^\\d+[bkmg]?$",
      "description": "Memory limit (e.g., '512m', '1g')"
    },
    "cpus": {
      "type": "string",
      "pattern": "^\\d+(\\.\\d+)?$",
      "description": "CPU limit (e.g., '1', '0.5')"
    },
    "user": {
      "type": "string",
      "description": "User to run container as (UID:GID or username)"
    },
    "seccomp": {
      "type": "string",
      "description": "Seccomp profile (e.g., 'default', 'unconfined')"
    },
    "apparmor": {
      "type": "string",
      "description": "AppArmor profile"
    },
    "capabilities": {
      "type": "object",
      "properties": {
        "add": {"type": "array", "items": {"type": "string"}},
        "drop": {"type": "array", "items": {"type": "string"}}
      }
    }
  },
  "required": ["run"]
}
```

**default-config.yaml updates:**
Add comprehensive examples in stop section:
```yaml
stop:
  # Global defaults for Docker commands
  defaultTimeout: "5m"
  defaultEnv:
    - "CI=true"

  commands:
    # Host execution (no image)
    - run: "npm test"
      message: "Unit tests failed"

    # Basic Docker execution
    - run: "npm test"
      image: "node:18-alpine"
      timeout: "10m"

    # Advanced Docker options
    - run: "python -m pytest"
      image: "python:3.11"
      workdir: "/app"
      env:
        - "PYTHONUNBUFFERED=1"
        - "PYTEST_ARGS=--verbose"
      timeout: "15m"
      memory: "512m"
      cpus: "2"

    # Custom mounts and security
    - run: "./build.sh"
      image: "node:18"
      mounts:
        - type: bind
          source: "."
          target: "/workspace"
          readonly: false
        - type: tmpfs
          target: "/tmp"
      user: "node"
      seccomp: "default"
      network: "bridge"
```

**Tasks:**
1. Update `schema.json` with Docker properties
2. Add validation patterns for timeout, memory, cpus
3. Update `default-config.yaml` with examples
4. Add field descriptions and documentation
5. Run `openspec validate` to verify schema
6. Test schema validation with valid/invalid examples

---

## Phase 2: Command Execution (Days 3-4)

### 2.1 Create Docker Client Manager
**Status:** pending
**Effort:** 1 day
**Description:** Create new module `src/docker.rs` with Docker client lifecycle management

**Acceptance Criteria:**
- [ ] Docker client initializes on demand
- [ ] Connection failures handled gracefully
- [ ] Docker version checked (minimum 1.40+ API)
- [ ] Clear error messages if Docker unavailable
- [ ] Client can be mocked for testing
- [ ] Documentation on Docker requirements

**Detailed Implementation:**

```rust
// New file: src/docker.rs

use bollard::Docker;
use anyhow::{Result, Context};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct DockerManager {
    client: Option<Arc<Docker>>,
    available: bool,
}

impl DockerManager {
    pub async fn new() -> Result<Self> {
        // Try to connect to Docker
        match Docker::connect_with_defaults() {
            Ok(client) => {
                // Check version
                if let Ok(version) = client.version().await {
                    if is_api_version_supported(&version.api_version) {
                        Ok(DockerManager {
                            client: Some(Arc::new(client)),
                            available: true,
                        })
                    } else {
                        Err(anyhow::anyhow!("Docker API version not supported"))
                    }
                } else {
                    Err(anyhow::anyhow!("Failed to connect to Docker"))
                }
            }
            Err(e) => {
                // Log warning but allow host execution
                Ok(DockerManager {
                    client: None,
                    available: false,
                })
            }
        }
    }

    pub fn is_available(&self) -> bool {
        self.available
    }

    pub fn get_client(&self) -> Result<Arc<Docker>> {
        self.client
            .clone()
            .context("Docker not available")
    }

    fn is_api_version_supported(version: &str) -> bool {
        // Check API version >= 1.40
        version >= "1.40"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_docker_manager_creation() {
        // Test with mock Docker
    }
}
```

**Tasks:**
1. Create `src/docker.rs` module
2. Implement `DockerManager` struct
3. Add Docker version validation
4. Handle connection failures gracefully
5. Create mock for testing (trait-based design)
6. Add comprehensive error messages
7. Add unit tests

---

### 2.2 Implement Container Execution
**Status:** pending
**Effort:** 1.5 days
**Description:** Implement `execute_in_docker()` function in `src/hooks.rs`

**Acceptance Criteria:**
- [ ] Container created with specified image
- [ ] Working directory set correctly
- [ ] Environment variables passed to container
- [ ] Command executes in container
- [ ] stdout/stderr captured and returned
- [ ] Container cleanup always occurs
- [ ] Exit codes preserved
- [ ] Timeout enforcement works
- [ ] Clear error messages on failure
- [ ] All async operations properly awaited

**Detailed Implementation:**

```rust
// In src/hooks.rs - new function

pub async fn execute_in_docker(
    docker_manager: &DockerManager,
    docker_config: &DockerConfig,
    command: &str,
    max_output_lines: Option<u32>,
    show_stdout: bool,
    show_stderr: bool,
) -> Result<CommandOutput> {
    let client = docker_manager.get_client()?;

    // Step 1: Prepare configuration
    let container_config = prepare_container_config(docker_config, command)?;
    let timeout_duration = parse_timeout(&docker_config.timeout)?;

    // Step 2: Create container
    let container_id = client.create_container::<&str>(
        Some(bollard::container::CreateContainerOptions {
            name: generate_container_name(),
            ..Default::default()
        }),
        container_config,
    )
    .await?
    .id;

    // Step 3: Start container with cleanup guarantee
    let result = match timeout_duration {
        Some(duration) => {
            tokio::time::timeout(
                duration,
                execute_container_with_timeout(&client, &container_id, docker_config)
            )
            .await
        }
        None => {
            execute_container_with_timeout(&client, &container_id, docker_config)
                .await
                .map(|r| r)
        }
    };

    // Step 4: ALWAYS cleanup container
    let _ = client.remove_container(&container_id, None).await;

    // Step 5: Handle result
    result.map_err(|e| anyhow::anyhow!("Container execution failed: {}", e))
}

async fn execute_container_with_timeout(
    client: &Docker,
    container_id: &str,
    config: &DockerConfig,
) -> Result<CommandOutput> {
    client.start_container::<String>(container_id, None).await?;

    // Stream and capture output
    let (stdout, stderr, exit_code) = capture_container_output(
        client,
        container_id,
    )
    .await?;

    Ok(CommandOutput {
        stdout,
        stderr,
        exit_code,
    })
}

fn prepare_container_config(
    docker_config: &DockerConfig,
    command: &str,
) -> Result<bollard::container::Config<String>> {
    let mut config = bollard::container::Config {
        image: Some(docker_config.image.clone()),
        cmd: Some(vec!["sh".to_string(), "-c".to_string(), command.to_string()]),
        working_dir: docker_config.workdir.clone(),
        env: docker_config.env.clone(),
        ..Default::default()
    };

    // Apply mounts, network, resource limits, security context, etc.
    apply_mounts(&mut config, &docker_config.mounts)?;
    apply_resource_limits(&mut config, docker_config)?;
    apply_security_context(&mut config, docker_config)?;

    Ok(config)
}

// ... Helper functions for mounts, resource limits, security context, etc.
```

**Tasks:**
1. Implement `execute_in_docker()` function
2. Implement container creation logic
3. Implement output capture (stdout/stderr)
4. Implement timeout enforcement
5. Ensure cleanup always occurs (try/finally pattern)
6. Add comprehensive error handling
7. Add extensive logging for debugging
8. Add unit tests with mocked Docker

---

### 2.3 Implement Volume/Mount Handling
**Status:** pending
**Effort:** 1 day
**Description:** Handle Docker volume mounting and project directory auto-mount

**Acceptance Criteria:**
- [ ] Project directory auto-mounted to `/workspace` read-write
- [ ] User-specified mounts honored
- [ ] Bind mount paths validated
- [ ] Read-only mounts work correctly
- [ ] tmpfs mounts work correctly
- [ ] Volume mounts work correctly
- [ ] Path resolution correct on all platforms (Linux/macOS/Windows)
- [ ] File validation rules work with mounted paths
- [ ] Clear error messages for invalid mounts

**Detailed Implementation:**

```rust
// In src/docker.rs or src/hooks.rs

fn apply_mounts(
    config: &mut bollard::container::Config<String>,
    mounts: &Option<Vec<DockerMount>>,
) -> Result<()> {
    let mut host_config = config.host_config.get_or_insert_with(Default::default);
    let mut binds = Vec::new();

    // Step 1: Auto-mount project directory
    let cwd = std::env::current_dir()?;
    let cwd_str = cwd.to_string_lossy().to_string();
    binds.push(format!("{}:/workspace", cwd_str));

    // Step 2: Apply user-specified mounts
    if let Some(user_mounts) = mounts {
        for mount in user_mounts {
            validate_mount(mount)?;

            match mount.r#type.as_str() {
                "bind" => {
                    let bind_str = format_bind_mount(mount)?;
                    binds.push(bind_str);
                }
                "volume" => {
                    // Handle volume mounts
                }
                "tmpfs" => {
                    // Handle tmpfs mounts
                }
                _ => return Err(anyhow!("Unknown mount type: {}", mount.r#type)),
            }
        }
    }

    host_config.binds = Some(binds);
    Ok(())
}

fn validate_mount(mount: &DockerMount) -> Result<()> {
    // Validate mount source/target paths
    // Prevent escape attempts
    if mount.target.contains("..") {
        return Err(anyhow!("Mount target cannot contain '..'"));
    }
    Ok(())
}

fn format_bind_mount(mount: &DockerMount) -> Result<String> {
    let source = std::path::Path::new(&mount.source)
        .canonicalize()?
        .to_string_lossy()
        .to_string();

    let mut bind = format!("{}:{}", source, mount.target);

    if mount.readonly.unwrap_or(false) {
        bind.push_str(":ro");
    }

    Ok(bind)
}
```

**Tasks:**
1. Implement auto-mount logic for project directory
2. Implement user mount parsing and validation
3. Handle bind mounts (Linux/macOS/Windows)
4. Handle volume mounts
5. Handle tmpfs mounts
6. Implement path canonicalization
7. Add security checks (prevent directory traversal)
8. Add comprehensive error messages
9. Add unit tests

---

### 2.4 Update Hook Execution to Route Based on Image
**Status:** pending
**Effort:** 0.5 day
**Description:** Modify `execute_stop_commands()` in `src/hooks.rs` to route to Docker or host based on `image` field

**Acceptance Criteria:**
- [ ] Commands with `image` field route to `execute_in_docker()`
- [ ] Commands without `image` field route to host execution (existing)
- [ ] Docker unavailable errors handled gracefully
- [ ] Clear error messages when Docker needed but unavailable
- [ ] Backward compatibility maintained
- [ ] No changes to existing test suite failures

**Detailed Implementation:**

```rust
// In src/hooks.rs - modify execute_stop_commands()

async fn execute_stop_commands(
    config: &StopConfig,
    docker_manager: &DockerManager,
    session_id: &str,
) -> Result<()> {
    let commands = collect_stop_commands(config)?;
    let mut all_output = String::new();
    let mut any_failed = false;

    for cmd_config in commands {
        // Route based on presence of image field
        let output = if let Some(image) = &cmd_config.image {
            // Docker execution
            if !docker_manager.is_available() {
                return Err(anyhow!(
                    "Docker image specified but Docker is not available\n\
                     Command: {}\n\
                     Image: {}\n\n\
                     Solutions:\n\
                     1. Install Docker Desktop (https://docker.com)\n\
                     2. Start Docker daemon: docker daemon\n\
                     3. Check permissions: docker ps\n\
                     4. Remove 'image' field to run on host",
                    cmd_config.run, image
                ));
            }

            execute_in_docker(
                docker_manager,
                &docker_config,
                &cmd_config.run,
                cmd_config.maxOutputLines,
                cmd_config.showStdout.unwrap_or(false),
                cmd_config.showStderr.unwrap_or(false),
            )
            .await?
        } else {
            // Host execution (existing)
            execute_on_host(
                &cmd_config.run,
                cmd_config.maxOutputLines,
                cmd_config.showStdout.unwrap_or(false),
                cmd_config.showStderr.unwrap_or(false),
            )
            .await?
        };

        // Handle output and errors as before
        // ...
    }

    Ok(())
}
```

**Tasks:**
1. Update `execute_stop_commands()` routing logic
2. Add Docker availability check
3. Add clear error messages
4. Maintain backward compatibility
5. Update existing tests (if needed)
6. Add new tests for Docker routing

---

## Phase 3: Advanced Features (Days 5-6)

### 3.1 Implement Resource Limits
**Status:** pending
**Effort:** 0.5 day
**Description:** Support memory and CPU limits in Docker containers

**Acceptance Criteria:**
- [ ] Memory limits parsed correctly (512m, 1g, etc.)
- [ ] CPU limits parsed correctly (1.0, 0.5, etc.)
- [ ] Limits applied to Docker container
- [ ] Limits enforced by Docker
- [ ] Invalid limits caught and reported
- [ ] Unit tests verify parsing

---

### 3.2 Implement Network Configuration
**Status:** pending
**Effort:** 0.5 day
**Description:** Support network mode configuration (bridge, host, none)

**Acceptance Criteria:**
- [ ] Network modes correctly applied
- [ ] Custom network names supported
- [ ] Container-to-container networking functional
- [ ] Error handling for invalid modes

---

### 3.3 Implement Security Context
**Status:** pending
**Effort:** 0.5 day
**Description:** Support user, seccomp, AppArmor, and capabilities configuration

**Acceptance Criteria:**
- [ ] User/UID mapping works
- [ ] Seccomp profiles applied
- [ ] AppArmor profiles applied
- [ ] Capabilities added/dropped correctly
- [ ] Platform-specific handling (Windows vs Linux)

---

### 3.4 Implement Environment Variable Handling
**Status:** pending
**Effort:** 0.5 day
**Description:** Support environment variable passing and interpolation

**Acceptance Criteria:**
- [ ] Environment variables passed to container
- [ ] Host env var interpolation works (${VAR})
- [ ] Merging with global defaults works
- [ ] Sensitive variables not logged
- [ ] Invalid variable syntax caught

---

### 3.5 Add Timeout Support for Host Execution
**Status:** pending
**Effort:** 0.5 day
**Description:** Add timeout enforcement to existing host command execution

**Acceptance Criteria:**
- [ ] Timeout applied to host execution
- [ ] Processes killed on timeout
- [ ] Clear "command timed out" messages
- [ ] Timeout defaults to 5 minutes when not specified
- [ ] null timeout disables timeout
- [ ] Backward compatible (no timeout changes for existing configs)

---

## Phase 4: Testing & Validation (Days 7-8)

### 4.1 Integration Tests with Real Docker
**Status:** pending
**Effort:** 1.5 days
**Description:** Write comprehensive integration tests against real Docker daemon

**Acceptance Criteria:**
- [ ] Tests can run with/without Docker available
- [ ] All major code paths tested
- [ ] Error scenarios covered
- [ ] Output capture verified
- [ ] Timeout enforcement verified
- [ ] Cleanup verified (containers don't leak)
- [ ] All tests pass consistently

**Test Cases:**
1. Basic container execution
2. Environment variable passing
3. Working directory configuration
4. Output capture (stdout/stderr)
5. Exit code handling
6. Timeout enforcement (trigger timeout)
7. Timeout enforcement (complete before timeout)
8. Resource limits
9. Network modes
10. User context
11. Container cleanup on success
12. Container cleanup on failure
13. Container cleanup on timeout
14. Image not found error
15. Docker unavailable error
16. Invalid configuration detection
17. File path translation for validation
18. Mount validation

---

### 4.2 Configuration Validation Tests
**Status:** pending
**Effort:** 0.5 day
**Description:** Test configuration parsing and validation

**Acceptance Criteria:**
- [ ] Valid configs parse correctly
- [ ] Invalid configs caught early
- [ ] Helpful error messages
- [ ] Examples from default-config.yaml validate

---

### 4.3 Error Scenario Testing
**Status:** pending
**Effort:** 1 day
**Description:** Test error handling and edge cases

**Test Cases:**
1. Docker daemon not running
2. Image not found
3. Insufficient permissions
4. Resource exhaustion
5. Container startup failure
6. Command failure in container
7. Invalid mount paths
8. Path traversal attempts
9. Invalid timeout format
10. Invalid resource limits

---

### 4.4 Performance Testing
**Status:** pending
**Effort:** 0.5 day
**Description:** Verify performance acceptable

**Benchmarks:**
- Container startup time
- Output streaming latency
- Memory usage with large output
- Cleanup performance

---

## Phase 5: Polish & Documentation (Days 9-10)

### 5.1 Documentation
**Status:** pending
**Effort:** 1 day
**Description:** Create comprehensive user documentation

**Documents:**
- Docker support overview
- Configuration examples (basic → advanced)
- Troubleshooting guide
- Performance considerations
- Migration guide for existing users
- API documentation (for code)

---

### 5.2 Error Message Improvements
**Status:** pending
**Effort:** 0.5 day
**Description:** Polish error messages and user guidance

**Goals:**
- Clear, actionable error messages
- Suggest solutions for common problems
- Include relevant configuration examples

---

### 5.3 Final Testing & QA
**Status:** pending
**Effort:** 0.5 day
**Description:** Full end-to-end testing and quality assurance

**Activities:**
- Manual testing across platforms
- Edge case verification
- Documentation accuracy check
- Example validation
- Performance spot checks

---

## Dependencies & Parallelization

### Dependency Graph
```
1.1 (Add dependency)
  ↓
1.2 (Types) → 1.3 (Schema) → 2.1 (Docker manager)
                                 ↓
2.2 (Container execution) ← 2.3 (Mounts)
  ↓
2.4 (Hook routing)
  ↓
3.1, 3.2, 3.3, 3.4 (Can run in parallel)
  ↓
4.1, 4.2, 4.3, 4.4 (Testing - some in parallel)
  ↓
5.1, 5.2, 5.3 (Polish - can run in parallel)
```

### Can Run in Parallel
- 1.2 and 1.3 (types and schema) - separate concerns
- 3.1, 3.2, 3.3, 3.4 (advanced features) - independent
- Some tests can run independently

---

## Validation Strategy

- Run `openspec validate enable-docker-stop-hooks --strict` after creating specs
- Run `cargo check` after each code phase
- Run `cargo test` after each code phase
- Run full test suite before marking complete
- Manual testing on all platforms before release

---

## Risk Mitigation

| Risk | Mitigation |
|------|-----------|
| Docker installation/setup issues | Clear error messages, fallback to host mode |
| Container resource leaks | Always cleanup in finally block |
| Complex configuration bugs | Extensive validation and tests |
| Performance overhead | Document container startup costs |
| Breaking existing workflows | Fully backward compatible, opt-in |
| Platform-specific issues | Test on Linux/macOS/Windows |

---

## Success Metrics

Upon completion:
- All 29+ tasks completed
- All code compiles without warnings
- All tests pass (unit + integration)
- Zero regressions in existing functionality
- Docker support fully functional
- Configuration validation comprehensive
- Documentation complete and accurate
- Code coverage > 85%
- Performance acceptable (< 2s container startup overhead)
