# comment-syntax-svelte Specification

## Purpose

Define Svelte-specific comment syntax detection for uneditable range markers, supporting HTML comments (`<!-- -->`), JavaScript comments in script blocks (`//`, `/* */`), and CSS comments in style blocks (`/* */`).

## ADDED Requirements

### Requirement: HTML Comment Detection

The system SHALL recognize uneditable markers within Svelte HTML comments (`<!-- -->`).

#### Scenario: Single-line HTML comment with marker

- **GIVEN** a Svelte file with:
  ```svelte
  <!-- conclaude-uneditable:start -->
  ```
- **WHEN** the file is parsed for uneditable markers
- **THEN** the start marker SHALL be detected
- **AND** the marker line SHALL be recorded as the start of a protected range

#### Scenario: HTML comment in template section

- **GIVEN** a Svelte file with:
  ```svelte
  <div>
    <!-- conclaude-uneditable:start -->
    <p>Protected content</p>
    <!-- conclaude-uneditable:end -->
  </div>
  ```
- **WHEN** the file is parsed
- **THEN** both markers SHALL be detected
- **AND** a protected range from line 2 to line 4 SHALL be created

#### Scenario: HTML comment with trailing description

- **GIVEN** a Svelte file with:
  ```svelte
  <!-- conclaude-uneditable:start Auto-generated component markup -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be detected (trailing content ignored)
- **AND** the range SHALL begin at this line

### Requirement: JavaScript Comment Detection in Script Blocks

The system SHALL recognize uneditable markers within JavaScript comments in Svelte script blocks.

#### Scenario: Line comment in script block

- **GIVEN** a Svelte file with:
  ```svelte
  <script>
    // <!-- conclaude-uneditable:start -->
    export let name = '';
    // <!-- conclaude-uneditable:end -->
  </script>
  ```
- **WHEN** the file is parsed
- **THEN** both markers SHALL be detected within the script block
- **AND** a protected range from line 2 to line 4 SHALL be created

#### Scenario: Block comment in script block

- **GIVEN** a Svelte file with:
  ```svelte
  <script>
    /* <!-- conclaude-uneditable:start --> */
    const apiKey = 'SECRET';
    /* <!-- conclaude-uneditable:end --> */
  </script>
  ```
- **WHEN** the file is parsed
- **THEN** both markers SHALL be detected within block comments
- **AND** a protected range from line 2 to line 4 SHALL be created

#### Scenario: TypeScript script block with marker

- **GIVEN** a Svelte file with:
  ```svelte
  <script lang="ts">
    // <!-- conclaude-uneditable:start -->
    interface User { id: number; }
    // <!-- conclaude-uneditable:end -->
  </script>
  ```
- **WHEN** the file is parsed
- **THEN** markers SHALL be detected in TypeScript script blocks
- **AND** a protected range SHALL be created

#### Scenario: Module script with marker

- **GIVEN** a Svelte file with:
  ```svelte
  <script context="module">
    // <!-- conclaude-uneditable:start -->
    export const shared = true;
    // <!-- conclaude-uneditable:end -->
  </script>
  ```
- **WHEN** the file is parsed
- **THEN** markers SHALL be detected in module context scripts
- **AND** a protected range SHALL be created

### Requirement: CSS Comment Detection in Style Blocks

The system SHALL recognize uneditable markers within CSS comments in Svelte style blocks.

#### Scenario: CSS comment in style block

- **GIVEN** a Svelte file with:
  ```svelte
  <style>
    /* <!-- conclaude-uneditable:start --> */
    .button { color: blue; }
    /* <!-- conclaude-uneditable:end --> */
  </style>
  ```
- **WHEN** the file is parsed
- **THEN** both markers SHALL be detected within CSS comments
- **AND** a protected range from line 2 to line 4 SHALL be created

#### Scenario: Scoped style with marker

- **GIVEN** a Svelte file with:
  ```svelte
  <style scoped>
    /* <!-- conclaude-uneditable:start --> */
    :global(.container) { width: 100%; }
    /* <!-- conclaude-uneditable:end --> */
  </style>
  ```
- **WHEN** the file is parsed
- **THEN** markers SHALL be detected in scoped style blocks
- **AND** a protected range SHALL be created

### Requirement: File Extension Mapping

The system SHALL detect Svelte files by their file extension and apply Svelte comment syntax rules.

#### Scenario: .svelte file extension

- **GIVEN** a file named "Component.svelte"
- **WHEN** the file is processed for uneditable ranges
- **THEN** Svelte comment syntax rules SHALL be applied
- **AND** markers within `<!-- -->`, `//`, or `/* */` comments SHALL be detected

#### Scenario: .svelte file with mixed markers

- **GIVEN** a file "App.svelte" containing:
  ```svelte
  <!-- conclaude-uneditable:start -->
  <script>
    // Protected script
    export let data;
  </script>
  <div>Protected markup</div>
  <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** a protected range from line 1 to line 7 SHALL be created
- **AND** edits overlapping this range SHALL be blocked

### Requirement: Marker Format Validation

The system SHALL only recognize correctly formatted markers within Svelte comments.

#### Scenario: Marker in JavaScript string literal (not detected)

- **GIVEN** a Svelte file with:
  ```svelte
  <script>
    const marker = "<!-- conclaude-uneditable:start -->";
  </script>
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL NOT be detected (inside string, not comment)
- **AND** no protected range SHALL be created

#### Scenario: Marker in HTML attribute (not detected)

- **GIVEN** a Svelte file with:
  ```svelte
  <div title="<!-- conclaude-uneditable:start -->">
    Content
  </div>
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL NOT be detected (inside attribute value)
- **AND** no protected range SHALL be created

#### Scenario: Marker in template literal (not detected)

- **GIVEN** a Svelte file with:
  ```svelte
  <script>
    const html = `<!-- conclaude-uneditable:start -->`;
  </script>
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL NOT be detected (inside template literal)
- **AND** no protected range SHALL be created

#### Scenario: Marker in curly brace expression (not detected)

- **GIVEN** a Svelte file with:
  ```svelte
  <div>
    {comment = "<!-- conclaude-uneditable:start -->"}
  </div>
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL NOT be detected (inside expression)
- **AND** no protected range SHALL be created

### Requirement: Nested Range Handling

The system SHALL handle nested ranges within Svelte component structure.

#### Scenario: Nested ranges across sections

- **GIVEN** a Svelte file with:
  ```svelte
  <!-- conclaude-uneditable:start -->  <!-- Line 1 -->
  <script>
    // <!-- conclaude-uneditable:start -->  <!-- Line 3 -->
    export let config = {};
    // <!-- conclaude-uneditable:end -->  <!-- Line 5 -->
  </script>
  <div>Protected component</div>
  <!-- conclaude-uneditable:end -->  <!-- Line 8 -->
  ```
- **WHEN** the file is parsed
- **THEN** two ranges SHALL be created: (1-8) and (3-5)
- **AND** both ranges SHALL be marked as protected
- **AND** edits overlapping either range SHALL be blocked

#### Scenario: Multiple protected sections in one file

- **GIVEN** a Svelte file with separate protected script and markup sections
- **WHEN** the file is parsed
- **THEN** all markers SHALL be detected across different comment types
- **AND** ranges SHALL be correctly paired by nesting order

### Requirement: Svelte-Specific Syntax Handling

The system SHALL correctly handle Svelte-specific syntax features and patterns.

#### Scenario: Marker with reactive statement

- **GIVEN** a Svelte file with:
  ```svelte
  <script>
    // <!-- conclaude-uneditable:start -->
    $: doubled = count * 2;
    // <!-- conclaude-uneditable:end -->
  </script>
  ```
- **WHEN** the file is parsed
- **THEN** markers SHALL be detected around reactive statements
- **AND** a protected range SHALL be created

#### Scenario: Marker with slot content

- **GIVEN** a Svelte file with:
  ```svelte
  <!-- conclaude-uneditable:start -->
  <slot name="header">
    <h1>Default Header</h1>
  </slot>
  <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** markers SHALL be detected around slot definitions
- **AND** a protected range SHALL be created

#### Scenario: Marker with conditional blocks

- **GIVEN** a Svelte file with:
  ```svelte
  <!-- conclaude-uneditable:start -->
  {#if condition}
    <p>True branch</p>
  {:else}
    <p>False branch</p>
  {/if}
  <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** markers SHALL be detected around Svelte control flow
- **AND** the entire block SHALL be protected

#### Scenario: Marker with each block

- **GIVEN** a Svelte file with:
  ```svelte
  <!-- conclaude-uneditable:start -->
  {#each items as item}
    <li>{item.name}</li>
  {/each}
  <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** markers SHALL be detected around iteration blocks
- **AND** a protected range SHALL be created

### Requirement: Performance Characteristics

The system SHALL efficiently parse Svelte files for uneditable markers.

#### Scenario: Large Svelte file with multiple markers

- **GIVEN** a Svelte file with 5,000 lines and 10 protected ranges across script, style, and markup
- **WHEN** the file is parsed for markers
- **THEN** parsing SHALL complete in under 100ms
- **AND** all 10 ranges SHALL be correctly identified

#### Scenario: Svelte file with no markers

- **GIVEN** a Svelte file with 2,000 lines and no uneditable markers
- **WHEN** the file is parsed
- **THEN** parsing SHALL complete quickly (under 40ms)
- **AND** an empty range list SHALL be returned
- **AND** no false positive detections SHALL occur

#### Scenario: File with many comments but no markers

- **GIVEN** a Svelte file with 300 HTML and JavaScript comments but no uneditable markers
- **WHEN** the file is parsed
- **THEN** parsing SHALL scan all comments efficiently
- **AND** no protected ranges SHALL be created
- **AND** performance SHALL remain acceptable (under 60ms)
