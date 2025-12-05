# Implementation Tasks

## 1. Dependency Installation
- [ ] 1.1 Add `starlight-page-actions` to docs/package.json dependencies
- [ ] 1.2 Run package manager install (npm/pnpm) to update lockfile

## 2. Configuration
- [ ] 2.1 Import `starlightPageActions` in docs/astro.config.mjs
- [ ] 2.2 Add plugin to Starlight's plugins array in configuration

## 3. Validation
- [ ] 3.1 Run `npm run build` in docs/ directory to verify build succeeds
- [ ] 3.2 Verify llms.txt is generated in build output
- [ ] 3.3 Run `npm run dev` and visually verify page action buttons appear
- [ ] 3.4 Test "Copy Markdown" button functionality
- [ ] 3.5 Test "Open" dropdown menu appears and contains AI service links

## 4. Documentation
- [ ] 4.1 Update docs/README.md if it documents plugin usage or dependencies
