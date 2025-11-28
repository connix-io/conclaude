# Add preventUpdateGitIgnored Setting to preToolUse Configuration

## Why

Claude Code sessions sometimes need to ensure that git-ignored files are never modified, even if the tool execution would otherwise allow it. This is useful for:

- **Sensitive local configuration** - `.env` files, local secrets, credentials
- **Build artifacts and caches** - `node_modules/`, `.dist/`, `__pycache__/` directories
- **Development tools** - IDE settings, local test databases, temporary working files
- **Git workflow safety** - Ensuring generated files in `.gitignore` stay unmolested by Claude

Currently, there is no single configuration setting to blanket-prevent updates to git-ignored files. Users must manually enumerate them in `uneditableFiles` glob patterns, which is error-prone and doesn't automatically track `.gitignore` changes.

By adding a `preventUpdateGitIgnored` boolean flag to `preToolUse`, users can:
1. Enable automatic protection of all git-ignored files
2. Rely on their existing `.gitignore` as the source of truth
3. Avoid maintaining duplicate file protection lists

## What Changes

### Configuration Structure Changes
- Add `preventUpdateGitIgnored` boolean field to `preToolUse` configuration
- Default value: `false` (opt-in to preserve backward compatibility)
- When enabled, block Claude from creating or modifying any file that matches an entry in `.gitignore`

### Files Affected
- **src/config.rs** - Add field to `PreToolUseConfig` struct
- **src/default-config.yaml** - Add field to default configuration template
- **conclaude-schema.json** - Update JSON schema to include new field
- **src/hooks.rs** - Implement git-ignore detection and file blocking logic
- **Documentation** - Update examples to show the new setting

### Implementation Behavior
- When `preventUpdateGitIgnored: true`, before allowing file operations (Read, Write, Edit):
  1. Check if the file path matches any pattern in `.gitignore`
  2. If matched, block the operation with a clear error message
  3. If not matched, allow the operation to proceed
  - Glob operations are NOT blocked (users can still discover files)

- Respect `.gitignore` semantics:
  - Patterns should be evaluated using the same rules as `git check-ignore`
  - Support standard gitignore syntax (negation with `!`, comments with `#`, glob patterns)
  - Handle nested `.gitignore` files in subdirectories (git standard behavior)

### Impact
- **Breaking change**: No - this is an opt-in setting with default value `false`
- **Backward compatibility**: Full - existing configurations work unchanged
- **Migration path**: None required - users can opt-in at their own pace
- **Performance**: Minimal - git-ignore lookups cached per session

## Architecture Notes

The `preToolUse` section is the natural home for file protection policies. Adding `preventUpdateGitIgnored` consolidates the file protection strategy:
- `preventRootAdditions` - Structural protection (root level)
- `uneditableFiles` - Explicit pattern-based protection
- `preventUpdateGitIgnored` - Automatic source-of-truth protection (via `.gitignore`)

This creates a comprehensive, three-layer file protection system that users can compose according to their needs.

## Clarification Questions (None - Proposal is Self-Contained)

This proposal is straightforward:
- Single boolean flag with clear on/off semantics
- Behavior maps directly to git-ignore semantics
- No ambiguity in validation or error reporting
