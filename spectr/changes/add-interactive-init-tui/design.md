# Design: Interactive TUI Mode for `conclaude init`

## Context

The `conclaude init` command currently writes a complete default configuration to `.conclaude.yaml` without asking for user preferences. While functional, this approach:

1. Requires users to understand the full configuration schema before customization
2. Forces manual YAML editing after initialization
3. Does not expose available options to new users

This design adds an optional interactive TUI wizard that guides users through configuration setup step-by-step.

## Goals

- Provide a guided, discoverable configuration experience
- Allow users to customize settings before file generation
- Preserve current silent mode as default (no breaking changes)
- Support terminal environments with ratatui/crossterm

## Non-Goals

- Web-based configuration UI
- GUI (graphical) configuration
- Automatic configuration detection from existing projects
- Real-time config file watching/editing

## Decisions

### TUI Framework: ratatui + crossterm

**Decision**: Use `ratatui` (formerly `tui-rs`) with `crossterm` backend for cross-platform terminal rendering.

**Rationale**:
- ratatui is the most actively maintained Rust TUI library (fork of tui-rs)
- crossterm provides cross-platform terminal manipulation (Windows, macOS, Linux)
- Both are widely used in the Rust ecosystem (e.g., gitui, bottom, diskonaut)
- Minimal footprint; both compile quickly

**Alternatives considered**:
- `cursive`: More widget-heavy, larger dependency footprint
- `termion`: Linux/macOS only, no Windows support
- Plain stdin prompts: Poor UX for complex configuration

### Wizard Steps

The TUI wizard progresses through these screens:

1. **Welcome** - Introduction and navigation instructions
2. **Core Protections** - `preToolUse` settings (preventRootAdditions, preventGeneratedFileEdits, etc.)
3. **Uneditable Files** - Add glob patterns for protected files
4. **Stop Hook** - Configure stop commands, infinite mode, rounds
5. **Notifications** - Enable/disable, select hook triggers
6. **Subagent Hooks** - Pattern-based subagent stop commands (optional, advanced)
7. **Review & Confirm** - Show summary, generate file

### Navigation Model

- **Tab/Shift+Tab**: Move between form fields
- **Enter**: Select/confirm current field
- **Arrow keys**: Navigate lists/options
- **q/Esc**: Cancel and exit without saving
- **F10**: Skip to review (accept defaults for remaining screens)

### State Management

TUI state is stored in a `TuiState` struct that mirrors `ConclaudeConfig`:

```rust
struct TuiState {
    current_screen: Screen,
    config: ConclaudeConfig,
    cursor_position: usize,
    // per-screen state for lists, inputs, etc.
}
```

On confirmation, `TuiState.config` is serialized to YAML and written to disk.

### Error Handling

- Invalid input is highlighted inline with error messages
- Users cannot proceed to next screen until current screen validates
- On Esc/q, prompt for confirmation before discarding changes

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| Terminal compatibility issues | Use crossterm (tested on Windows, macOS, Linux) |
| Complex state management | Keep wizard linear; avoid deep nesting |
| Increased binary size | ratatui/crossterm add ~300-500KB; acceptable for CLI tool |
| Maintenance burden | Keep TUI code isolated in `src/tui/` module |

## Migration Plan

No migration needed. This is an additive feature:

1. Add `--interactive` / `-i` flag to `Init` command
2. Default behavior unchanged (silent mode)
3. Interactive mode activated only when flag is present

## Open Questions

1. Should we support a `--preset` flag for common configurations (e.g., `--preset minimal`, `--preset strict`)?
2. Should the TUI support importing an existing config file for editing?

These can be addressed in follow-up changes.
