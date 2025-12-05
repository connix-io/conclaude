# Change: Fix workingDir interpolation for paths with spaces

## Why
workingDir interpolation currently drops spaces because the bash command is constructed without quoting when the value appears simple. Paths like "/tmp/my project" get split by bash, causing commands to run from the wrong directory or fail even when the path exists.

## What Changes
- Adjust bash interpolation quoting to preserve literal spaces while still supporting environment variables, tilde expansion, and command substitution
- Add tests covering workingDir values with spaces for stop and subagent stop handling
- Update the execution spec to require correct handling of space-containing workingDir values

## Impact
- Affected specs: execution
- Affected code: src/hooks.rs, tests in src/hooks.rs
