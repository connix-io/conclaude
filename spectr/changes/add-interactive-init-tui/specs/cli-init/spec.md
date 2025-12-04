# CLI Init Capability

The `conclaude init` command initializes conclaude configuration files and Claude Code hook settings for a project.

## ADDED Requirements

### Requirement: Interactive Mode Flag

The `init` command SHALL accept an `--interactive` or `-i` flag that activates a terminal-based configuration wizard.

#### Scenario: Interactive mode activation
- **WHEN** user runs `conclaude init --interactive`
- **THEN** the TUI wizard is launched instead of writing defaults immediately

#### Scenario: Short flag activation
- **WHEN** user runs `conclaude init -i`
- **THEN** the TUI wizard is launched

#### Scenario: Silent mode default
- **WHEN** user runs `conclaude init` without the interactive flag
- **THEN** the default configuration is written immediately (current behavior preserved)

### Requirement: TUI Wizard Navigation

The TUI wizard SHALL provide keyboard-based navigation through configuration screens.

#### Scenario: Forward navigation
- **WHEN** user completes a screen and presses Enter on "Next"
- **THEN** the wizard advances to the next configuration screen

#### Scenario: Backward navigation
- **WHEN** user presses Shift+Tab or selects "Back"
- **THEN** the wizard returns to the previous screen

#### Scenario: Cancel and exit
- **WHEN** user presses Escape or 'q'
- **THEN** a confirmation prompt appears asking to discard changes
- **AND** if confirmed, the wizard exits without writing any files

#### Scenario: Skip to review
- **WHEN** user presses F10 or selects "Skip to Review"
- **THEN** remaining screens use default values and the review screen is shown

### Requirement: Core Protections Screen

The TUI wizard SHALL include a screen for configuring `preToolUse` protection settings.

#### Scenario: Toggle preventRootAdditions
- **WHEN** user toggles the `preventRootAdditions` option
- **THEN** the configuration reflects the new boolean value

#### Scenario: Toggle preventGeneratedFileEdits
- **WHEN** user toggles the `preventGeneratedFileEdits` option
- **THEN** the configuration reflects the new boolean value

#### Scenario: Toggle preventUpdateGitIgnored
- **WHEN** user toggles the `preventUpdateGitIgnored` option
- **THEN** the configuration reflects the new boolean value

### Requirement: Uneditable Files Screen

The TUI wizard SHALL include a screen for managing uneditable file patterns.

#### Scenario: Add pattern
- **WHEN** user enters a glob pattern and presses Enter
- **THEN** the pattern is added to the `uneditableFiles` list

#### Scenario: Remove pattern
- **WHEN** user selects a pattern and presses Delete or 'd'
- **THEN** the pattern is removed from the list

#### Scenario: Default patterns shown
- **WHEN** the Uneditable Files screen is first displayed
- **THEN** default patterns `.conclaude.yml` and `.conclaude.yaml` are pre-populated

### Requirement: Stop Hook Screen

The TUI wizard SHALL include a screen for configuring stop hook behavior.

#### Scenario: Toggle infinite mode
- **WHEN** user toggles the `infinite` option to true
- **THEN** the configuration sets `infinite: true`

#### Scenario: Set rounds value
- **WHEN** user enters a number in the `rounds` field
- **THEN** the configuration sets `rounds` to that value

#### Scenario: Mutual exclusivity hint
- **WHEN** user enables `infinite` mode
- **THEN** the `rounds` field is disabled with a hint that they are mutually exclusive

### Requirement: Notifications Screen

The TUI wizard SHALL include a screen for configuring system notifications.

#### Scenario: Toggle notifications enabled
- **WHEN** user toggles the `enabled` option
- **THEN** the configuration reflects the new value

#### Scenario: Select hook triggers
- **WHEN** user selects hooks from a multi-select list
- **THEN** only selected hooks are added to the `hooks` array

#### Scenario: Wildcard option
- **WHEN** user selects "All hooks (*)"
- **THEN** the `hooks` array contains `["*"]`

### Requirement: Review and Confirm Screen

The TUI wizard SHALL display a summary screen before writing the configuration file.

#### Scenario: Review summary display
- **WHEN** user reaches the review screen
- **THEN** a summary of all configured options is displayed

#### Scenario: Confirm and write
- **WHEN** user selects "Confirm" on the review screen
- **THEN** the configuration is written to `.conclaude.yaml`
- **AND** Claude Code settings are updated in `.claude/settings.json`

#### Scenario: Edit from review
- **WHEN** user selects "Edit" on the review screen
- **THEN** user is returned to the first wizard screen to make changes

### Requirement: Flag Compatibility

The `--interactive` flag SHALL work in combination with existing `init` flags.

#### Scenario: Custom config path with interactive
- **WHEN** user runs `conclaude init -i --config-path ./custom.yaml`
- **THEN** the TUI wizard generates config to the specified path

#### Scenario: Force flag with interactive
- **WHEN** user runs `conclaude init -i --force`
- **THEN** existing files are overwritten without additional confirmation

#### Scenario: Custom schema URL with interactive
- **WHEN** user runs `conclaude init -i --schema-url https://example.com/schema.json`
- **THEN** the generated YAML includes the custom schema URL header
