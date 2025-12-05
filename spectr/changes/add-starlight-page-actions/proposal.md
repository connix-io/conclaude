# Change: Add Starlight Page Actions Plugin

## Why

The documentation site lacks interactive page-level actions that would improve usability for readers. The starlight-page-actions plugin provides three key features:
1. A "Copy Markdown" button that enables users to easily copy raw markdown content
2. An "Open" dropdown menu with options to launch pages in AI chat services (ChatGPT, Claude)
3. Automatic `llms.txt` generation containing all documentation URLs during builds

These features enhance documentation accessibility and enable better AI-assisted workflows for users.

## What Changes

- Add `starlight-page-actions` npm dependency to docs/package.json
- Import and configure the plugin in docs/astro.config.mjs
- Enable Copy Markdown button on all documentation pages
- Enable AI chat service integration dropdown
- Automatic generation of llms.txt file during site builds

## Impact

- Affected specs: `documentation`
- Affected code:
  - `docs/package.json` - add plugin dependency
  - `docs/astro.config.mjs` - import and configure plugin
- Build output: new `llms.txt` file generated automatically
- No breaking changes
