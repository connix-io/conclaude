## Context
Users need to run stop commands from specific directories other than where conclaude is invoked. For example, monorepo users may want to run commands from the repository root (found via `$(git rev-parse --show-toplevel)`), or package-specific commands need to run from `packages/frontend`.

The feature needs to support dynamic path resolution using bash interpolation to handle various use cases like:
- Finding project root: `$(git rev-parse --show-toplevel)`
- Using environment variables: `$HOME/projects/myapp`
- Tilde expansion: `~/workspace/backend`
- Relative paths: `packages/frontend` (relative to config file)

## Goals / Non-Goals
**Goals:**
- Allow commands to execute from configurable directories
- Support dynamic path resolution via bash interpolation
- Maintain backward compatibility (optional field)
- Clear error messages for path resolution failures

**Non-Goals:**
- Changing how commands are executed (still use bash -c)
- Modifying environment variable handling
- Cross-platform shell support beyond bash (Windows users already use bash via WSL or Git Bash)

## Decisions

### Decision: Full bash interpolation support
**Chosen approach**: Support environment variables, command substitution, and tilde expansion via bash -c execution.

**Why**: Maximum flexibility for users. Examples:
- `workingDir: "$(git rev-parse --show-toplevel)"` - find repo root
- `workingDir: "$HOME/projects/backend"` - use env vars
- `workingDir: "~/workspace"` - tilde expansion
- `workingDir: "packages/$PACKAGE_NAME"` - dynamic package dirs

**Implementation**: Use bash -c to expand the path string before validating and using it:
```rust
let expanded_path = Command::new("bash")
    .arg("-c")
    .arg(format!("echo -n {}", shell_escaped_workingDir))
    .output()?;
```

**Alternatives considered**:
- Environment variables only: Too limiting, users commonly need `$(git rev-parse --show-toplevel)`
- No interpolation: Users would need wrapper scripts, less ergonomic

### Decision: Interpolate at command execution time
**Chosen approach**: Expand paths fresh for each command execution.

**Why**:
- Environment variables might change between commands
- Command substitution like `$(git rev-parse --show-toplevel)` could change if git state changes
- More dynamic and flexible

**Trade-off**: Slightly slower (bash subprocess per command), but stop commands are infrequent (session end or subagent end).

**Alternatives considered**:
- Interpolate at config load: Faster but static values only

### Decision: Relative paths relative to config file
**Chosen approach**: Resolve relative paths (after interpolation) relative to the directory containing `.conclaude.yaml`.

**Why**:
- Predictable: `workingDir: "packages/frontend"` always means the same directory regardless of where Claude session starts
- Config-centric: Paths are defined relative to the config, making configs portable
- Consistent with other tools (e.g., tsconfig.json paths are relative to tsconfig location)

**Example**:
- Config at: `/repo/.conclaude.yaml`
- `workingDir: "packages/frontend"` → `/repo/packages/frontend`
- `workingDir: "../other-repo"` → `/other-repo`

**Alternatives considered**:
- Relative to payload cwd: Less predictable, depends on where session started

### Decision: Strict error handling
**Chosen approach**:
- If bash interpolation fails → fail command with clear error
- If interpolation results in empty string → fail command with clear error
- If resolved path doesn't exist → fail command with clear error
- If resolved path is a file not a directory → fail command with clear error

**Why**: Clear failures are better than silent incorrect behavior. Users can quickly identify and fix configuration issues.

**Graceful degradation**: Other commands in the list continue executing (consistent with existing command failure behavior).

## Implementation Approach

### Path resolution pipeline:
1. **Start with raw `workingDir` string** from config
2. **Bash interpolation** (at execution time):
   ```rust
   bash -c "echo -n {shell_escaped_workingDir}"
   ```
3. **Check for empty result** → error if empty
4. **Resolve relative paths**:
   - If absolute (starts with `/` or `~` after expansion) → use as-is
   - If relative → resolve relative to config file directory
5. **Validate directory exists** → error if not found or not a directory
6. **Use `.current_dir()` in TokioCommand**

### Error messages:
- "workingDir interpolation failed: <bash error>"
- "workingDir interpolation resulted in empty path"
- "workingDir does not exist: /resolved/path"
- "workingDir is not a directory: /resolved/path"

### Code locations:
- `src/config.rs`: Add `workingDir: Option<String>` to `StopCommand` and `SubagentStopCommand`
- `src/hooks.rs`: Add path resolution helper function `resolve_working_dir()`
- `src/hooks.rs`: Modify `execute_stop_commands` and `execute_subagent_stop_commands` to call `.current_dir()` when workingDir is set

## Risks / Trade-offs

**Risk**: Bash interpolation adds subprocess overhead
- **Mitigation**: Stop commands are infrequent (session end), minimal performance impact

**Risk**: Command substitution could hang or fail
- **Mitigation**: Strict error handling with clear messages, user can debug their command

**Risk**: Cross-platform compatibility (Windows)
- **Mitigation**: Windows users already use bash (WSL, Git Bash) for conclaude commands. Document bash requirement.

**Risk**: Security - arbitrary command execution in workingDir
- **Mitigation**: Users already control the config file and all commands. If they can set `run:`, they can set `workingDir:`. No additional security risk.

## Migration Plan
- **Backward compatible**: New optional field, existing configs work unchanged
- **No migration needed**: Users opt-in by adding `workingDir` field
- **Documentation**: Add examples to README or config docs

## Open Questions
None - all decisions made based on user input.
