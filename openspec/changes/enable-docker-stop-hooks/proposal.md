# Proposal: Docker Image Support for Stop Hooks

**Change ID:** `enable-docker-stop-hooks`
**Status:** Proposal
**Author:** Claude Code
**Date:** 2025-11-11

## Executive Summary

Enable stop hook commands to execute within isolated Docker containers instead of only on the host system. This provides security isolation, reproducible environments, and dependency management without requiring breaking changes to existing configurations.

**Key benefits:**
- Run commands in clean, isolated container environments
- Enforce resource limits (memory, CPU) per command
- Manage environment variables and secrets safely
- Reproducible test/validation environments regardless of host setup
- Optional timeout enforcement for safety
- Full backward compatibility (opt-in via `image` field)

## Why

Users need to launch stop hook commands from specified Docker images to ensure consistency, isolation, and reproducibility of command execution across different environments and hosts.

## Motivation

### Current State
Stop hooks currently execute all commands directly on the host system using `/bin/bash -c`. This creates several challenges:

1. **Environment contamination**: Commands inherit all host environment variables and system state
2. **Dependency assumptions**: No guaranteed runtime environment (Node, Python, etc. versions)
3. **Resource exhaustion**: No limits on memory, CPU, or execution time
4. **Cross-platform inconsistency**: Different results on macOS vs Linux vs Windows
5. **Security risks**: Commands can access host filesystem and networks without restrictions

### User Request
Users need to launch stop hook commands from specified Docker images to ensure consistency, isolation, and reproducibility.

## What Changes

1. **Configuration Schema** - Add Docker-specific fields to StopCommand (image, workdir, env, mounts, network, timeout, memory, cpus, user, seccomp, apparmor, capabilities)
2. **Hook Execution** - Add Docker container execution path alongside host execution
3. **Timeout Support** - Add optional timeout enforcement for both host and containerized commands (default 5 minutes)
4. **Type System** - Extend StopCommand struct and add new Docker-related types
5. **Dependencies** - Add Bollard crate for native Docker API client
6. **Error Handling** - Graceful Docker availability handling with clear error messages
7. **Documentation** - Configuration examples and troubleshooting guide

## Scope

### What's Included
- Docker container execution for stop hook commands
- Opt-in per-command (only when `image` field is specified)
- Full Docker configuration: volumes, environment, network, resources, security
- Timeout support for both host and containerized execution
- Auto-mount of project directory to container `/workspace`
- Path translation and file validation in containerized context
- Comprehensive error handling and resource cleanup

### What's NOT Included (Future Work)
- Docker image building/tagging
- Multi-stage container workflows
- Container networking between stop commands
- Kubernetes/orchestrator integration
- Image registry authentication (beyond Docker credentials)

## Design Overview

See `design.md` for detailed architecture.

**Key decisions:**
1. Use Bollard crate (native Rust Docker API) for reliability and performance
2. Commands without `image` field continue to run on host (backward compatible)
3. Project directory auto-mounted read-write to `/workspace` by default
4. Sensible defaults with full override capability
5. Timeout defaults to 5 minutes, can be disabled via `timeout: null`

## Affected Components

1. **Configuration Schema** (`schema.json`)
   - New fields in `StopCommand`: `image`, `workdir`, `env`, `mounts`, `network`, `timeout`, `memory`, `cpus`, `user`, `seccomp`, `apparmor`

2. **Types** (`src/types.rs`)
   - Extend `StopCommand` struct with new Docker fields
   - New enums: `MountType`, `NetworkMode`, `SecurityContext`

3. **Config Parsing** (`src/config.rs`)
   - Validate Docker-specific fields
   - Path resolution for mounted volumes
   - Environment variable interpolation

4. **Hook Execution** (`src/hooks.rs`)
   - New `execute_in_docker()` function
   - Docker client initialization and cleanup
   - Container lifecycle management
   - Output capture and error handling

5. **Dependencies** (`Cargo.toml`)
   - Add `bollard` crate (Docker API)
   - Add `futures` for async handling (if needed)

6. **Tests** (`tests/`)
   - Integration tests with real Docker containers
   - Unit tests for configuration parsing
   - Error scenario testing

## Success Criteria

- [ ] Commands with `image` field execute in Docker containers
- [ ] Commands without `image` field continue to execute on host (backward compatible)
- [ ] Project directory automatically mounted to container `/workspace`
- [ ] Timeouts work for both host and containerized execution
- [ ] Full test coverage for Docker execution paths
- [ ] Zero breaking changes to existing configurations
- [ ] Clear error messages when Docker is unavailable
- [ ] Configuration validation catches invalid Docker options

## Timeline
Estimated 2-3 weeks for full implementation including testing and documentation.

## References
- Current hook system: `src/hooks.rs` (lines 625-730 for command execution)
- Configuration: `src/config.rs` and `schema.json`
- Related specs: See `openspec/specs/` for existing hook architecture

## Next Steps
1. Review and approve this proposal
2. Review detailed design in `design.md`
3. Review implementation tasks in `tasks.md`
4. Begin implementation following task order
