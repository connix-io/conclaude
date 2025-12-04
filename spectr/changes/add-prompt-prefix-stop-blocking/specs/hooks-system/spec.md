## ADDED Requirements

### Requirement: Prompt Prefix Stop Blocking Configuration

The system SHALL support configuring prompt-prefix-based stop blocking through a `promptPrefixBlocking` section in the `stop` configuration.

#### Scenario: Valid configuration with prefixes and messages

- **GIVEN** a `.conclaude.yaml` with:
  ```yaml
  stop:
    promptPrefixBlocking:
      prefixes:
        - "ULTRATHINK*"
        - "DEEPWORK*"
      messages:
        - text: "Continue working on the task"
          times: 2
        - text: "Make sure all decisions are documented"
  ```
- **WHEN** the configuration is loaded
- **THEN** the `prompt_prefix_blocking` field SHALL be populated
- **AND** `prefixes` SHALL contain the glob patterns
- **AND** `messages` SHALL contain the blocking messages with their times

#### Scenario: Configuration without promptPrefixBlocking

- **GIVEN** a `.conclaude.yaml` without `promptPrefixBlocking` section
- **WHEN** the configuration is loaded
- **THEN** `prompt_prefix_blocking` SHALL be `None`
- **AND** stop blocking based on prompt prefix SHALL be disabled

#### Scenario: Message with default times value

- **GIVEN** a message configuration without explicit `times` field
- **WHEN** the configuration is loaded
- **THEN** the message `times` SHALL default to 1

### Requirement: Initial Prompt Tracking with Database Persistence

The system SHALL track the initial prompt (first 100 characters) submitted in each session, persisting to database for session continuity across restarts.

#### Scenario: First UserPromptSubmit stores initial prompt to database

- **GIVEN** a new session with session_id "abc123"
- **AND** `promptPrefixBlocking` is configured
- **WHEN** the first `UserPromptSubmit` hook fires with prompt "ULTRATHINK help me build a feature"
- **THEN** the first 100 characters of the prompt SHALL be stored in the `prompt_prefix_sessions` table
- **AND** `queue_position` SHALL be initialized to 0
- **AND** `times_remaining` SHALL be initialized from the first message's `times` value
- **AND** the stored prompt SHALL be available for subsequent Stop hook events

#### Scenario: Subsequent prompts do not overwrite initial prompt

- **GIVEN** a session with initial prompt already stored in database
- **WHEN** additional `UserPromptSubmit` hooks fire with different prompts
- **THEN** the stored initial prompt SHALL NOT be overwritten
- **AND** only the first prompt SHALL be used for prefix matching

#### Scenario: Session without promptPrefixBlocking configured

- **GIVEN** `promptPrefixBlocking` is not configured
- **WHEN** `UserPromptSubmit` hook fires
- **THEN** the system SHALL NOT store the prompt in the database
- **AND** no database operations SHALL be performed

#### Scenario: Process restart preserves session state

- **GIVEN** a session with stored initial prompt and partial message queue progress
- **WHEN** the conclaude process restarts
- **AND** a Stop hook fires for the same session_id
- **THEN** the system SHALL retrieve the stored state from the database
- **AND** message queue iteration SHALL resume from the saved position

#### Scenario: Database disabled prevents prompt prefix blocking

- **GIVEN** `database.enabled` is set to false in configuration
- **AND** `promptPrefixBlocking` is configured
- **WHEN** `UserPromptSubmit` hook fires
- **THEN** the system SHALL NOT attempt to store the prompt
- **AND** prompt prefix blocking SHALL be effectively disabled
- **AND** an appropriate warning MAY be logged

### Requirement: Glob Pattern Prefix Matching

The system SHALL match the first 100 characters of the initial prompt against configured glob patterns (case-sensitive) to determine if stop blocking should activate.

#### Scenario: Exact prefix match with wildcard

- **GIVEN** a configured prefix pattern "ULTRATHINK*"
- **AND** an initial prompt "ULTRATHINK help me build a complex feature"
- **WHEN** the Stop hook fires
- **THEN** the pattern SHALL match against the first 100 characters
- **AND** stop blocking SHALL activate

#### Scenario: Multiple prefix patterns with first match

- **GIVEN** configured patterns ["FOCUS*", "ULTRATHINK*", "DEEPWORK*"]
- **AND** an initial prompt "DEEPWORK on this task"
- **WHEN** the Stop hook fires
- **THEN** the "DEEPWORK*" pattern SHALL match
- **AND** stop blocking SHALL activate

#### Scenario: No pattern match allows stop

- **GIVEN** a configured prefix pattern "ULTRATHINK*"
- **AND** an initial prompt "Help me build a feature"
- **WHEN** the Stop hook fires
- **THEN** no pattern SHALL match
- **AND** the Stop hook SHALL proceed normally without blocking

#### Scenario: Case-sensitive pattern matching

- **GIVEN** a configured prefix pattern "ULTRATHINK*"
- **AND** an initial prompt "ultrathink help me"
- **WHEN** the Stop hook fires
- **THEN** the pattern SHALL NOT match (case-sensitive)
- **AND** the Stop hook SHALL proceed normally

#### Scenario: Long prompt truncated to 100 characters

- **GIVEN** a configured prefix pattern "ULTRATHINK*"
- **AND** an initial prompt that is 500 characters long, starting with "ULTRATHINK"
- **WHEN** the Stop hook fires
- **THEN** only the first 100 characters SHALL be used for matching
- **AND** the pattern SHALL match successfully

### Requirement: Message Queue Iteration

The system SHALL iterate through configured messages, sending each one the specified number of times before advancing.

#### Scenario: First stop blocked with first message

- **GIVEN** matching prefix and messages:
  ```yaml
  messages:
    - text: "Continue working"
      times: 2
    - text: "Document your decisions"
  ```
- **AND** this is the first Stop hook for this session
- **WHEN** the Stop hook fires
- **THEN** the hook SHALL be blocked
- **AND** the returned message SHALL be "Continue working"
- **AND** the remaining times for first message SHALL be decremented to 1

#### Scenario: Same message sent multiple times

- **GIVEN** first message with times: 2 and remaining times: 1
- **WHEN** the Stop hook fires again
- **THEN** the hook SHALL be blocked
- **AND** the returned message SHALL be "Continue working"
- **AND** the remaining times SHALL be decremented to 0
- **AND** the queue SHALL advance to the next message

#### Scenario: Advancing to next message

- **GIVEN** first message exhausted (times remaining: 0)
- **AND** second message "Document your decisions" with times: 1
- **WHEN** the Stop hook fires
- **THEN** the hook SHALL be blocked
- **AND** the returned message SHALL be "Document your decisions"

#### Scenario: All messages exhausted allows stop

- **GIVEN** all messages have been sent their configured number of times
- **WHEN** the Stop hook fires
- **THEN** the Stop hook SHALL proceed normally (not blocked)
- **AND** the database record SHALL be retained for audit purposes (no automatic cleanup)

### Requirement: Integration with Existing Stop Hook Features

The prompt prefix stop blocking SHALL integrate correctly with existing stop hook features like commands, infinite mode, and rounds.

#### Scenario: Prompt prefix blocking runs before commands

- **GIVEN** `promptPrefixBlocking` is configured with matching prefix
- **AND** `stop.commands` are also configured
- **WHEN** the Stop hook fires and messages remain
- **THEN** the stop SHALL be blocked with the prefix message
- **AND** `stop.commands` SHALL NOT be executed

#### Scenario: Prompt prefix blocking exhausted then commands run

- **GIVEN** `promptPrefixBlocking` messages are exhausted
- **AND** `stop.commands` are configured
- **WHEN** the Stop hook fires
- **THEN** `stop.commands` SHALL execute normally
- **AND** their pass/fail status SHALL determine the hook result

#### Scenario: Prompt prefix blocking with infinite mode disabled

- **GIVEN** `promptPrefixBlocking` is configured
- **AND** `stop.infinite` is false
- **WHEN** prefix matches and messages remain
- **THEN** prompt prefix blocking SHALL still function
- **AND** infinite mode setting SHALL NOT affect prefix blocking behavior
