## ADDED Requirements

### Requirement: Page-Level Actions UI

The documentation site SHALL provide interactive page-level actions that enable users to copy content and open pages in external services.

#### Scenario: Copy Markdown button appears

- **WHEN** a user views any documentation page
- **THEN** a "Copy Markdown" button is visible in the page interface

#### Scenario: Copy Markdown button copies raw content

- **WHEN** a user clicks the "Copy Markdown" button
- **THEN** the raw markdown source of the current page is copied to the clipboard

#### Scenario: Open dropdown menu appears

- **WHEN** a user views any documentation page
- **THEN** an "Open" dropdown menu is visible in the page interface

#### Scenario: AI service options available

- **WHEN** a user clicks the "Open" dropdown menu
- **THEN** options to open the page in AI chat services (ChatGPT, Claude) are displayed

### Requirement: LLMs.txt Generation

The documentation site SHALL automatically generate an llms.txt file containing all documentation URLs during the build process.

#### Scenario: llms.txt generated on build

- **WHEN** the documentation site build process runs (astro build)
- **THEN** an llms.txt file is created in the build output directory

#### Scenario: llms.txt contains all page URLs

- **WHEN** the llms.txt file is generated
- **THEN** it contains URLs for all pages defined in the Starlight sidebar configuration

#### Scenario: llms.txt follows proper format

- **WHEN** the llms.txt file is generated
- **THEN** it follows the properly formatted structure expected by AI services

### Requirement: Starlight Plugin Integration

The documentation site SHALL integrate the starlight-page-actions plugin through the standard Starlight plugin system.

#### Scenario: Plugin dependency installed

- **WHEN** the docs project dependencies are installed
- **THEN** the `starlight-page-actions` package is available in node_modules

#### Scenario: Plugin configured in Astro config

- **WHEN** the Astro configuration is loaded
- **THEN** `starlightPageActions()` is included in the Starlight plugins array

#### Scenario: Plugin executes during build

- **WHEN** the documentation build runs
- **THEN** the starlight-page-actions plugin hooks execute and inject UI components
