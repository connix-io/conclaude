## Why
The gitWorktree configuration option adds unnecessary complexity to the codebase without providing clear value. Removing unused functionality simplifies the configuration schema and reduces maintenance burden.

## What Changes
- **BREAKING**: Remove gitWorktree configuration section from ConclaudeConfig struct
- Remove GitWorktreeConfig struct definition
- Remove gitWorktree references from default configuration files
- Update JSON schema to remove gitWorktree definitions
- Remove all related test coverage for gitWorktree functionality

## Impact
- Affected specs: configuration capability
- Affected code: src/config.rs, default-config.yaml, conclaude-schema.json, tests/config_tests.rs
- Breaking change for any users with gitWorktree configuration (acceptable per request)