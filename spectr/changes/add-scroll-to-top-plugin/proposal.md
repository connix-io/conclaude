# Change: Add Scroll-to-Top Plugin to Documentation Site

## Why
Long documentation pages require excessive scrolling to return to the top, degrading navigation UX. Adding a scroll-to-top button provides quick access to page headers and main navigation.

## What Changes
- Add `starlight-scroll-to-top` plugin to the Starlight documentation site
- Install npm dependency
- Configure plugin in `astro.config.mjs`
- Enable smooth scrolling and progress ring indicator

## Impact
- Affected specs: `documentation`
- Affected code: `docs/package.json`, `docs/astro.config.mjs`
- No breaking changes
- Purely additive UX enhancement
