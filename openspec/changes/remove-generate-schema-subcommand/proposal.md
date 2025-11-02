# Remove generate-schema Subcommand Proposal

## Why

The `generate-schema` subcommand is currently tightly coupled to the main CLI binary, making schema generation dependent on the full `conclaude` build and release cycle. Schema generation is a development-time tool that should be part of the build/release automation rather than a user-facing command. This coupling creates unnecessary complexity in the CLI surface area and complicates the release process.

## What Changes

- **BREAKING**: Remove the `GenerateSchema` subcommand from the main CLI
- Create a standalone Rust script (`scripts/generate-schema.rs`) for schema generation
- Update GitHub Actions release workflow to run the schema generation script
- Update documentation to reflect that schema is auto-generated during releases
- Ensure the schema file (`conclaude-schema.json`) is still published to GitHub releases

## Impact

### Affected Specs
- `cli` - CLI interface changes (removal of subcommand)

### Affected Code
- `src/main.rs` - Remove `GenerateSchema` command enum variant and handler
- `src/schema.rs` - Module remains but is used only by the external script
- `.github/workflows/release.yml` - Add schema generation step
- `README.md` - Update documentation to remove `generate-schema` command references
- New file: `scripts/generate-schema.rs` - Standalone schema generation script

### User Impact
- **BREAKING**: Users can no longer run `conclaude generate-schema` manually
- The schema file will continue to be available at the same GitHub release URL
- No impact on IDE autocomplete (schema URL remains the same)
- Developers wanting to regenerate schema locally will use `cargo run --bin generate-schema` instead

### Migration Path
For users who previously ran `conclaude generate-schema`:
- If using the schema for IDE support: No action needed, schema remains at the same URL
- If generating schema locally: Use `cargo run --bin generate-schema` instead
- If automating schema generation: Update scripts to use the new Rust script directly
