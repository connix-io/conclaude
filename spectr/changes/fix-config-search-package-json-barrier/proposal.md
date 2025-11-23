# Change: Remove package.json Barrier from Config Search

## Why

The config file search in `src/config.rs:458-461` stops prematurely when it encounters a `package.json` file, incorrectly assuming it has reached the "project root." This prevents discovery of configuration files located in parent directories above the package.json.

**Real-world failure scenario:**
```
/home/user/.conclaude.yaml          <- Config exists here
/home/user/project/package.json      <- Search stops here (wrong assumption)
/home/user/project/src/              <- Working directory
```

Users expect cosmiconfig-like behavior where the search continues through parent directories until reaching the filesystem root or maximum depth limit, regardless of package.json presence. The current implementation breaks this expectation for monorepos, nested projects, and user-level configs.

## What Changes

- Remove the package.json barrier check from `get_config_search_paths()` in `src/config.rs:458-461`
- Maintain the existing 12-level maximum search depth limit
- Search continues through all parent directories until filesystem root or max depth
- Update tests to verify config discovery above package.json boundaries
- Update documentation to reflect corrected search behavior
- Create new `config-discovery` capability spec to document this behavior

**BREAKING**: This changes the documented search behavior. The archived CLI spec currently states the system "SHALL stop at package.json boundaries."

## Impact

### Affected Specs
- New spec: `specs/config-discovery` (new capability for config file discovery)

### Affected Code
- `src/config.rs:444-479` - `get_config_search_paths()` function
- `tests/config_tests.rs` - Add tests for package.json scenarios
- `README.md:911` - Update config search documentation
- `spectr/project.md:59,77` - Update cosmiconfig description

### Migration
**Low impact**: Most users will benefit from improved config discovery. Edge case where users relied on package.json as an explicit search boundary is unlikely. No configuration file changes required.
