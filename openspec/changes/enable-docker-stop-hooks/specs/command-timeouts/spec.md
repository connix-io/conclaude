# Specification: Command Execution Timeouts

**Capability:** command-timeouts
**Change:** enable-docker-stop-hooks
**Status:** Specification
**Version:** 1.0

## Summary
Add optional timeout enforcement for both host-based and Docker-containerized stop hook command execution. Prevents indefinite command runs and ensures predictable execution behavior.

## ADDED Requirements

### Requirement: Timeout Configuration
The system SHALL support optional execution timeout configuration for commands.

#### Scenario: Command with Timeout
When timeout is specified on a command:
```yaml
stop:
  commands:
    - run: "npm test"
      timeout: "5m"
```

Then:
1. Command execution limited to 5 minutes
2. If command doesn't complete within timeout, execution is terminated
3. Timeout applies to both host and Docker execution
4. Process receives SIGTERM (graceful termination)
5. If process doesn't exit, SIGKILL sent after grace period

#### Scenario: Timeout with Global Default
When global default timeout is set:
```yaml
stop:
  defaultTimeout: "10m"
  commands:
    - run: "npm test"
      image: "node:18"
      # Inherits 10m timeout from global default
```

Then:
1. Global timeout applied to command
2. Per-command timeout overrides global
3. Per-command `timeout: null` disables timeout for that command
4. Merging respects precedence: per-command > global > no timeout

#### Scenario: No Timeout (Unlimited)
When timeout is not specified or set to null:
```yaml
stop:
  commands:
    - run: "npm test"
      timeout: null
```

Then:
1. Command can run indefinitely
2. No timeout enforcement
3. Command termination based on completion only
4. Can still be interrupted by stop hook handler

#### Scenario: Timeout Format Parsing
When timeout is specified in various formats:
```yaml
stop:
  commands:
    - run: "test1"
      timeout: "30s"        # 30 seconds
    - run: "test2"
      timeout: "5m"         # 5 minutes
    - run: "test3"
      timeout: "1h"         # 1 hour
    - run: "test4"
      timeout: "2h30m"      # NOT valid: use single unit
    - run: "test5"
      timeout: null         # No timeout
```

Then:
1. Each timeout is parsed correctly
2. `s` suffix = seconds, `m` suffix = minutes, `h` suffix = hours
3. Only single unit allowed (not "2h30m")
4. Numeric part must be positive integer
5. `null` value disables timeout
6. Invalid formats cause validation error

---

### Requirement: Timeout Enforcement Mechanism
The system SHALL enforce timeouts consistently across both host and Docker execution contexts.

#### Scenario: Host Command Timeout
When a host command exceeds timeout:
```yaml
stop:
  commands:
    - run: "sleep 100"
      timeout: "5s"
```

Then:
1. Command starts on host
2. After 5 seconds, timeout triggers
3. SIGTERM sent to process
4. Grace period (2 seconds) waits for graceful shutdown
5. If process still running, SIGKILL sent
6. Exit code indicates timeout (e.g., 124 or custom code)
7. Output captured up to timeout point
8. User sees timeout message:
   ```
   Error: Command execution timed out after 5s
   Command: sleep 100
   Partial output:
   ...
   ```

#### Scenario: Docker Container Timeout
When a container command exceeds timeout:
```yaml
stop:
  commands:
    - run: "sleep 100"
      image: "alpine"
      timeout: "5s"
```

Then:
1. Container starts
2. Command executes in container
3. After 5 seconds, timeout triggers in orchestrator
4. Docker container receives stop signal
5. Grace period (2 seconds) for graceful shutdown
6. If container still running, kill signal sent
7. Container exit code captured
8. Output up to timeout captured
9. Container cleaned up
10. User sees same timeout message as host execution

#### Scenario: Timeout with Long Output
When command produces large output before timeout:
```yaml
stop:
  commands:
    - run: "npm test"
      timeout: "1m"
      maxOutputLines: 100
```

Then:
1. Command executes with 1 minute timeout
2. Output captured and limited to 100 lines
3. If timeout occurs, truncated output shown
4. Omitted lines count indicated
5. Message shows: "Command timed out after 1m. Showing 100 of 2043 output lines"

---

### Requirement: Timeout Grace Period
The system SHALL provide a grace period for processes to shut down gracefully before forced termination.

#### Scenario: Graceful Shutdown Before Kill
When process receives SIGTERM:
```yaml
stop:
  commands:
    - run: "node server.js"
      timeout: "30s"
```

Then:
1. SIGTERM sent to process
2. Process has 2 seconds grace period
3. Process can catch SIGTERM and clean up resources
4. Process can write final logs
5. If process exits cleanly, no SIGKILL
6. If process still running after grace period, SIGKILL sent
7. SIGKILL forces immediate termination

#### Scenario: Immediate Kill Without Grace (Future)
If extended for critical timeouts:
```yaml
stop:
  commands:
    - run: "npm test"
      timeout: "5m"
      gracePeriod: 0  # Immediate kill (future extension)
```

Then:
1. On timeout, SIGKILL sent immediately
2. No grace period for shutdown
3. Useful for untrustworthy commands
4. Note: This is future extension, not in v1

---

### Requirement: Timeout Interaction with Output Limiting
The system SHALL support combined timeout and output limiting with independent enforcement.

#### Scenario: Both Timeout and Output Limit
When both are specified:
```yaml
stop:
  commands:
    - run: "npm test"
      timeout: "5m"
      maxOutputLines: 500
```

Then:
1. Command has 5 minute timeout
2. Output limited to 500 lines
3. Both limits enforced independently
4. If timeout occurs first: timeout message shown
5. If output exceeds 500 lines: both messages shown
6. Output truncated to 500 lines AND timeout reported

#### Scenario: No Output Limit with Timeout
When output limit not specified:
```yaml
stop:
  commands:
    - run: "npm test"
      timeout: "5m"
```

Then:
1. All output captured (no line limit)
2. Timeout enforced at 5 minutes
3. Large output shown in full (within reason)
4. Memory managed for long outputs

---

### Requirement: Timeout Error Reporting
The system SHALL provide clear error messages with context when timeout occurs.

#### Scenario: Timeout Error Message
When command execution times out:
```
Error: Command execution timed out

Command: npm test
Timeout: 5m
Duration: 5m 0.234s

Exit Status: Timeout (signal 15: SIGTERM)

Partial Output:
> npm test
npm WARN test ...
Tests running...
(output truncated - timed out before completion)

To fix:
1. Increase timeout if command takes longer
2. Optimize command execution
3. Run command with more resources
4. Set timeout: null to disable (not recommended)
```

---

### Requirement: Default Timeout Behavior
The system SHALL apply sensible default timeout values when not explicitly specified.

#### Scenario: Default Timeout When Not Specified
When no timeout specified globally or per-command:
```yaml
stop:
  commands:
    - run: "npm test"
      image: "node:18"
      # No timeout specified
```

Then:
1. Default timeout of 5 minutes applied
2. Command can complete if within 5 minutes
3. Timeout enforced unless explicitly disabled
4. User can change with global `defaultTimeout`
5. User can disable per-command with `timeout: null`

#### Scenario: Global Default Override
When global default is set:
```yaml
stop:
  defaultTimeout: "30m"
  commands:
    - run: "npm test"
      image: "node:18"
      # Uses 30m from global default
```

Then:
1. Global default applied to all commands
2. Commands without timeout use global default
3. Commands with explicit timeout override global
4. Global default can be set once for all commands

#### Scenario: Disable Timeout
When timeout explicitly disabled:
```yaml
stop:
  defaultTimeout: "5m"  # Global default
  commands:
    - run: "long-running-test"
      image: "node:18"
      timeout: null     # No timeout for this command
```

Then:
1. Per-command `timeout: null` overrides global
2. Command can run indefinitely
3. Only explicitly set to null, not just omitted
4. Useful for background processes or long operations

---

### Requirement: Timeout Validation
The system SHALL validate timeout format and values with helpful error messages.

#### Scenario: Valid Timeout Format
When timeout format is valid:
```yaml
timeout: "30s"
timeout: "5m"
timeout: "2h"
timeout: null
```

Then:
1. All formats accepted
2. Numeric value must be positive (> 0)
3. Suffix must be s, m, or h
4. No spaces between value and suffix
5. `null` special value for disabled

#### Scenario: Invalid Timeout Format
When timeout format is invalid:
```yaml
timeout: "5x"         # Invalid suffix
timeout: "-5m"        # Negative value
timeout: "5"          # Missing suffix
timeout: "0s"         # Zero value
timeout: "5m30s"      # Multiple units
timeout: 300          # Should be string
```

Then:
1. Configuration validation fails
2. Clear error messages provided
3. Suggests correct format
4. Examples provided: "Valid: '30s', '5m', '2h', null"

---

### Requirement: Timeout Behavior Consistency
The system SHALL ensure timeout behavior is identical across host and Docker execution contexts.

#### Scenario: Host and Docker Same Timeout Behavior
When same command runs on host and in Docker:
```yaml
stop:
  commands:
    - run: "npm test"
      timeout: "5m"

    - run: "npm test"
      image: "node:18"
      timeout: "5m"
```

Then:
1. Both commands have 5 minute timeout
2. Timeout behavior identical
3. Same error messages on timeout
4. Same exit code handling
5. User cannot tell difference in timeout behavior
6. Difference only in execution context

---

### Requirement: Resource Limit vs Timeout Distinction
The system SHALL distinguish between timeout and resource limits with independent enforcement.

#### Scenario: Timeout vs Memory Limit
When both timeout and memory limit specified:
```yaml
stop:
  commands:
    - run: "npm test"
      image: "node:18"
      timeout: "10m"         # Time limit
      memory: "512m"         # Memory limit
```

Then:
1. Timeout limit: execution duration
2. Memory limit: memory usage
3. Both can trigger independently
4. Timeout kills if exceeds time
5. Memory limit kills if exceeds memory
6. Whichever triggers first stops command
7. Error message identifies which limit triggered

---

### Requirement: Timeout Logging and Debugging
The system SHALL properly log timeout events with context for debugging purposes.

#### Scenario: Timeout in Session Logs
When command times out:
1. Timeout logged with timestamp
2. Command and timeout duration logged
3. Exit code/signal logged
4. Output (partial) preserved
5. Session can be reviewed for timeout events
6. Logs include timeout details in structured format

---

## MODIFIED Requirements

### Requirement: Host Command Execution Enhancement
The system SHALL extend host command execution with timeout support.

#### Scenario: Existing Host Execution with New Timeout
When existing host command now has timeout:
```yaml
stop:
  commands:
    - run: "npm test"
      timeout: "5m"
```

Then:
1. Command executed on host (no Docker)
2. Timeout enforced (new feature)
3. Existing output capture works
4. Existing error handling works
5. Backward compatible (timeout optional)

---

## REMOVED Requirements

(This is an additive capability - no requirements removed)

---

## Configuration Schema

```yaml
stop:
  # Global timeout default (optional)
  defaultTimeout: string?       # Format: {number}[smh] or null

  commands:
    - run: string               # Command to execute
      timeout: string?          # Format: {number}[smh] or null
      # ... other fields ...
```

### Validation Rules

| Field | Format | Examples | Default |
|-------|--------|----------|---------|
| timeout | `{number}[smh]` or `null` | `30s`, `5m`, `2h`, `null` | `5m` |
| defaultTimeout | `{number}[smh]` or `null` | `30s`, `5m`, `2h`, `null` | `5m` |

### Timeout Precedence

1. Per-command timeout (highest priority)
2. Global defaultTimeout
3. Default 5 minutes (lowest priority)

---

## Implementation Details

### Timeout Mechanism - Host Execution
```
1. Parse timeout value to Duration
2. Spawn command
3. Create timeout_handle = tokio::time::sleep(duration)
4. Race: command completion vs timeout
5. If command wins: return output
6. If timeout wins:
   - Send SIGTERM to process
   - Wait grace_period (2s)
   - If still running: Send SIGKILL
   - Return timeout error with partial output
```

### Timeout Mechanism - Docker Execution
```
1. Create Docker container
2. Start container
3. Create timeout_handle = tokio::time::sleep(duration)
4. Race: container completion vs timeout
5. If container wins: return output
6. If timeout wins:
   - Send stop signal to container
   - Wait grace_period (2s)
   - If still running: Send kill signal
   - Remove container
   - Return timeout error with partial output
```

### Grace Period Behavior
- Grace period: 2 seconds (hardcoded, configurable in future)
- Process receives SIGTERM
- Process has grace_period to exit
- After grace_period, SIGKILL sent
- SIGKILL forces immediate termination

### Exit Code Mapping
- Normal completion: command exit code
- Timeout: 124 (SIGTERM) or 137 (SIGKILL)
- Infrastructure error: non-zero error code

---

## Related Capabilities
- `docker-command-execution`: Docker timeout enforcement
- `stop-hook-config`: Configuration schema for timeout field

---

## Testing Strategy

### Unit Tests
- Timeout parsing and validation
- Format validation (valid/invalid)
- Default timeout merging
- Precedence calculation

### Integration Tests
- Host command timeout enforcement
- Docker command timeout enforcement
- Timeout with large output
- Grace period behavior (process cleanup)
- SIGTERM vs SIGKILL handling

### Edge Case Tests
- Command completes before timeout
- Command exceeds timeout
- Command ignores SIGTERM (SIGKILL test)
- Very short timeout (1s)
- Very long timeout (24h)
- Zero timeout (invalid)
- Negative timeout (invalid)
- No timeout specification
- null timeout specification
- Global default + per-command override
- Timeout with output capture
- Timeout with output limiting

### Performance Tests
- Timeout accuracy (verify actual vs specified)
- Cleanup performance
- No resource leaks on timeout

---

## Success Criteria

- [ ] Timeout enforced for host execution
- [ ] Timeout enforced for Docker execution
- [ ] Timeout format validated
- [ ] Default timeout applied (5m)
- [ ] Per-command override works
- [ ] Global default override works
- [ ] `timeout: null` disables timeout
- [ ] Grace period provides clean shutdown
- [ ] Clear error messages on timeout
- [ ] Exit codes correct
- [ ] Full test coverage
- [ ] Zero backward compatibility issues
- [ ] Documentation complete
- [ ] Examples provided

---

## Future Enhancements

1. **Configurable Grace Period**: Allow per-command grace period override
2. **Timeout Warnings**: Warn before actual timeout (e.g., at 90%)
3. **Timeout Callbacks**: Execute cleanup commands on timeout
4. **Per-Stage Timeouts**: Different timeouts for setup vs execution
5. **Timeout Metrics**: Track timeout frequency and patterns
6. **Smart Timeouts**: Learn reasonable timeouts from historical runs
