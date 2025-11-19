# Implementation Tasks

## 1. Create External Schema Generation Script
- [x] 1.1 Create `scripts/` directory in repository root
- [x] 1.2 Create `scripts/Cargo.toml` with workspace dependency on `conclaude`
- [x] 1.3 Create `scripts/generate-schema.rs` with main function
- [x] 1.4 Import and call `conclaude::schema::generate_config_schema()`
- [x] 1.5 Import and call `conclaude::schema::write_schema_to_file()`
- [x] 1.6 Add success message output to match removed subcommand UX
- [x] 1.7 Add error handling with clear error messages
- [x] 1.8 Test script locally: `cargo run --manifest-path scripts/Cargo.toml --bin generate-schema`
- [x] 1.9 Verify generated `conclaude-schema.json` matches previous output

## 2. Update Main CLI Code
- [x] 2.1 Remove `GenerateSchema` variant from `Commands` enum in `src/main.rs`
- [x] 2.2 Remove `handle_generate_schema` function from `src/main.rs`
- [x] 2.3 Remove `Commands::GenerateSchema` match arm in `main()`
- [x] 2.4 Verify `src/schema.rs` module remains unchanged (needed by script)
- [x] 2.5 Ensure schema functions are public in `src/lib.rs` for script access
- [x] 2.6 Run `cargo build` to verify no compilation errors
- [x] 2.7 Run `cargo clippy` to verify no warnings
- [x] 2.8 Run `conclaude --help` and verify generate-schema is not listed

## 3. Update Library Exports
- [x] 3.1 Review `src/lib.rs` to ensure `schema` module is publicly exported
- [x] 3.2 Add documentation comments indicating schema module is for tooling
- [x] 3.3 Verify external script can import `conclaude::schema` functions
- [x] 3.4 Run `cargo doc` to ensure module documentation is correct

## 4. Create Schema Upload Workflow
- [x] 4.1 Create `.github/workflows/upload-schema.yml` file
- [x] 4.2 Configure trigger: `on: release: types: [published]`
- [x] 4.3 Add `permissions: contents: write` for release uploads
- [x] 4.4 Add checkout step with `actions/checkout@v4`
- [x] 4.5 Add Rust installation step with `dtolnay/rust-toolchain@stable`
- [x] 4.6 Add schema generation step: `cargo build --release --bin generate-schema`
- [x] 4.7 Add schema execution step: `./target/release/generate-schema`
- [x] 4.8 Add upload step: `gh release upload $TAG conclaude-schema.json --clobber`
- [x] 4.9 Test workflow on a draft/test release to verify it works

## 5. Update Documentation
- [x] 5.1 Open `README.md` and remove all `generate-schema` command references
- [x] 5.2 Update "Available Commands" section to remove generate-schema
- [x] 5.3 Update "Configuration Reference" section (remove manual generation instructions)
- [x] 5.4 Verify schema URL documentation remains correct
- [x] 5.5 Add note that schema is uploaded automatically via GitHub Actions
- [x] 5.6 Search README for any other `generate-schema` mentions and update
- [x] 5.7 Update `CLAUDE.md` if it references schema generation
- [x] 5.8 Update any OpenSpec documentation referencing the subcommand

## 6. Update Tests
- [x] 6.1 Review tests in `src/schema.rs` - verify they still work
- [x] 6.2 Add integration test for external script if needed
- [x] 6.3 Run `cargo test` to verify all tests pass
- [x] 6.4 Run `cargo test --all-features` to verify comprehensive coverage
- [x] 6.5 Remove any tests specifically for the CLI subcommand

## 7. Update Root Cargo.toml
- [x] 7.1 Add `[[bin]]` entry for generate-schema binary
- [x] 7.2 Set name = "generate-schema" and path = "scripts/generate-schema.rs"
- [x] 7.3 Verify `cargo build --bin generate-schema` works correctly
- [x] 7.4 Verify `cargo test` still works correctly

## 8. Cleanup and Verification
- [x] 8.1 Run `cargo fmt` to format all code
- [x] 8.2 Run `cargo clippy -- -D warnings` to ensure no warnings
- [x] 8.3 Run full test suite: `cargo test --all-features`
- [x] 8.4 Generate schema locally and verify it works: `cargo run --bin generate-schema`
- [x] 8.5 Verify CLI help text no longer shows generate-schema
- [x] 8.6 Attempt to run `conclaude generate-schema` and verify error message
- [x] 8.7 Review all modified files for consistency

## 9. Release Verification (Post-Merge)
- [ ] 9.1 Monitor release workflow when PR is merged
- [ ] 9.2 Verify upload-schema workflow triggers and completes successfully
- [ ] 9.3 Verify `conclaude-schema.json` is in release assets
- [ ] 9.4 Verify schema URL is accessible after release
- [ ] 9.5 Test YAML language server integration with new schema
- [ ] 9.6 Confirm no user-facing breakage for IDE autocomplete

## 10. Archive Change (After Deployment)
- [ ] 10.1 Verify the change is deployed and stable
- [ ] 10.2 Run `openspec archive remove-generate-schema-subcommand --yes`
- [ ] 10.3 Update `specs/cli/spec.md` with final CLI requirements if needed
- [ ] 10.4 Run `openspec validate --strict` to verify archived state
- [ ] 10.5 Create separate PR for archiving if required
