# Architecture: preventUpdateGitIgnored File Protection

## Overview

The `preventUpdateGitIgnored` feature adds a third layer to conclaude's file protection system, allowing users to automatically protect git-ignored files from modification. This document explains the architectural decisions and trade-offs.

## File Protection System Architecture

The system now has three complementary file protection mechanisms:

```
preToolUse.preventRootAdditions
├─ Structural protection
├─ Blocks creation/modification at repository root level
└─ Example: Prevents .env, README.md at root

preToolUse.uneditableFiles
├─ Explicit pattern protection
├─ Blocks files matching user-specified glob patterns
└─ Example: Blocks Cargo.toml, package.json, *.lock

preToolUse.preventUpdateGitIgnored
├─ Source-of-truth protection
├─ Blocks files matching .gitignore patterns automatically
└─ Example: Blocks node_modules/, .env, *.log per .gitignore
```

### Design Rationale

1. **Single Responsibility**: Each protection mechanism has a distinct purpose:
   - `preventRootAdditions` = structure (where)
   - `uneditableFiles` = explicit (what)
   - `preventUpdateGitIgnored` = automatic (via .gitignore)

2. **Composability**: Users can combine rules as needed:
   - Minimal: Just `preventRootAdditions: true`
   - Explicit: Add specific `uneditableFiles` patterns
   - Automatic: Enable `preventUpdateGitIgnored` for git-aware protection

3. **No Redundancy**: Each rule is independent; overlaps are not problematic:
   - File protection is idempotent (blocking twice has no additional effect)
   - Error messages indicate which rule applied
   - Performance impact is minimal with proper caching

## Implementation Architecture

### Module Structure

```
src/
├─ config.rs          (PreToolUseConfig struct)
├─ gitignore.rs       (new module)
│  ├─ GitIgnore struct
│  ├─ load_git_ignores()
│  ├─ is_ignored()
│  └─ pattern matching logic
├─ hooks.rs           (handle_pre_tool_use updated)
└─ types.rs           (error types)
```

### Git-Ignore Detection Flow

```
PreToolUse Hook Execution
    ↓
Check preventUpdateGitIgnored enabled?
    ├─ NO  → Skip git-ignore checks
    │       └─ Proceed with other validations
    │
    └─ YES → Load .gitignore rules (cached)
            ↓
            Check if file path matches any pattern
            ├─ MATCH    → Block with error message
            │            └─ Include pattern in error
            │
            └─ NO MATCH → Proceed with other validations
```

### No Persistent Caching Needed

Since conclaude runs fresh for each hook execution (not a long-running process), there's no need for complex caching strategies:
- `.gitignore` is loaded fresh on each hook run
- No session state is maintained between runs
- Minimal performance impact even without caching

### Error Message Design

When a git-ignored file operation is blocked, the error message should:

1. **Be clear and specific**:
   ```
   File operation blocked: Path is git-ignored

   File: src/.env
   Matched pattern in .gitignore: .env

   This file is protected by 'preventUpdateGitIgnored: true'

   To allow modifications:
   1. Remove the pattern from .gitignore
   2. Use a negation pattern (e.g., !.env)
   3. Set preventUpdateGitIgnored: false in your config
   ```

2. **Include matching pattern(s)**:
   - Show which `.gitignore` line matched
   - Helpful for debugging complex patterns

3. **Suggest remediation**:
   - Options to unblock the file
   - Acknowledge the safety feature

## Operation-Level Control

The feature blocks three specific operations:
- **Read** - Prevent reading git-ignored files
- **Write** - Prevent creating git-ignored files
- **Edit** - Prevent modifying git-ignored files

**Glob operations are NOT blocked** - Users can still discover and list files, but can't read/write/edit them.

## Integration Points

### With Other File Protection Rules

The three protection rules are evaluated independently, and if any blocks the operation, it's rejected:

```rust
// Pseudocode: preToolUse hook execution
if is_git_ignored_file && prevent_update_git_ignored {
    return Err("File is git-ignored")
}
if is_root_level_file && prevent_root_additions {
    return Err("Root-level files protected")
}
if matches_uneditable_patterns && uneditable_files {
    return Err("File is in uneditable list")
}
// If no rules block it, proceed
```

### Git Repository Detection

Assumptions:
1. `.gitignore` is in the repository root (standard practice)
2. Nested `.gitignore` files in subdirectories are supported
3. If no `.gitignore` exists, all files are treated as non-ignored
4. Feature gracefully handles non-git repositories

### Tool Execution Context

The check must happen:
- **BEFORE** the tool is executed
- **IN** the `preToolUse` hook handler
- **FOR** file paths provided to the tool
- **INDEPENDENT** of tool type (Read, Write, Edit, Glob, etc.)

## Performance Considerations

### Time Complexity
- **Load .gitignore**: O(n) where n = number of patterns in `.gitignore`
- **Per-file check**: O(n) worst case, but typically much faster with early termination
- **Acceptable overhead**: `.gitignore` files are small (typically <500 patterns), minimal impact on hook latency

### Space Complexity
- Typical `.gitignore`: 50-200 patterns = negligible memory
- Parsed rules: ~10-50 KB for most projects
- No problematic scaling

### Optimization Opportunities (Future)
1. Compile glob patterns to regex for faster matching (if profiling shows it's needed)
2. Use path prefixes to skip checks for unrelated files
3. Early termination on first match

## Git Semantics Compliance

The implementation must respect git-ignore semantics:

1. **Patterns**: Support glob syntax (*, ?, **, etc.)
2. **Anchoring**: Leading `/` anchors to root, no `/` allows anywhere
3. **Negation**: `!pattern` overrides previous matches
4. **Directories**: Trailing `/` matches directories only
5. **Comments**: Lines starting with `#` are ignored
6. **Escaping**: Backslash escapes special characters
7. **Whitespace**: Trailing whitespace is significant

**Implementation**: Use `gitignore` crate (or similar) rather than reimplementing.

## Backward Compatibility

- **Default**: `preventUpdateGitIgnored: false` (disabled)
- **Existing configs**: Work unchanged
- **New projects**: Can opt-in by setting to `true`
- **Schema**: Additive change, non-breaking

## Edge Cases and Handling

### Non-Git Repository
- **Behavior**: Treat all files as non-ignored (no blocking)
- **Rationale**: Feature degrades gracefully in non-git context

### No .gitignore File
- **Behavior**: Treat all files as non-ignored
- **Rationale**: Explicit opt-in via configuration, not implicit

### Corrupt or Unparseable .gitignore
- **Behavior**: Log warning, treat as no patterns (fail open)
- **Rationale**: Better to allow operations than block by accident

### Symlinks and Relative Paths
- **Behavior**: Canonicalize paths before matching
- **Rationale**: Consistent behavior regardless of path representation

### Large .gitignore Files
- **Behavior**: Load and evaluate efficiently
- **Rationale**: Typical repos have <500 patterns; minimal performance impact even without caching

## Future Extensions

1. **Logging and Audit**: Track blocked operations for debugging
2. **Per-file exceptions**: Allow selectively allowing certain git-ignored files if needed
3. **Pattern whitelisting**: Allow inline overrides for specific patterns (requires more complex config)
4. **Stricter modes**: Add optional modes to block Glob as well (future enhancement)

## Risks and Mitigations

| Risk | Mitigation |
|------|-----------|
| Git-ignore parsing bugs cause false positives | Use well-tested `gitignore` crate; comprehensive tests |
| Performance regression on large files | Cache rules; profile on real repos before release |
| Confusion with `uneditableFiles` | Document differences; provide migration examples |
| Breaking if repo isn't git-tracked | Fail open (treat as no patterns) |

## Testing Strategy

### Unit Tests
- Pattern matching: Simple, glob, negation, anchored, nested
- Edge cases: Corrupt gitignore, no gitignore, nested .gitignore
- Caching: Invalidation on file change

### Integration Tests
- Combined with other file protection rules
- With real repositories
- Performance benchmarks

### Property-Based Tests (Optional)
- Gitignore semantics compliance
- Pattern matching consistency with `git check-ignore`
