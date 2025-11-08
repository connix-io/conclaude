# Implementation Tasks

## 1. Design Decision
- [x] 1.1 Evaluate procedural macro vs build script approach
- [x] 1.2 Document chosen approach and rationale in design.md
- [x] 1.3 Confirm approach with user/team

## 2. Implementation (Option A: Procedural Macro)
- [x] 2.1 Create new `conclaude-field-derive` procedural macro crate
- [x] 2.2 Implement `#[derive(FieldList)]` or `#[field_list]` attribute macro
- [x] 2.3 Generate `impl FieldListProvider for T` that returns `Vec<&'static str>`
- [x] 2.4 Add macro as dependency in `Cargo.toml`
- [x] 2.5 Annotate config structs (`StopConfig`, `RulesConfig`, etc.) with derive
- [x] 2.6 Refactor `suggest_similar_fields` to call generated methods
- [x] 2.7 Remove hardcoded field list

## 2. Implementation (Option B: Build Script)
- [ ] 2.1 Create `build.rs` in project root
- [ ] 2.2 Use `syn` crate to parse `src/config.rs` AST
- [ ] 2.3 Extract field names from each config struct
- [ ] 2.4 Generate Rust code file with `const ALL_FIELDS: Vec<(&str, Vec<&str>)>`
- [ ] 2.5 Write generated code to `OUT_DIR/generated_fields.rs`
- [ ] 2.6 Include generated file in `src/config.rs` via `include!`
- [ ] 2.7 Refactor `suggest_similar_fields` to use generated constant
- [ ] 2.8 Add build dependencies (`syn`, `quote`, `proc-macro2`) to `Cargo.toml`
- [ ] 2.9 Remove hardcoded field list

## 3. Testing
- [x] 3.1 Add unit test verifying all struct fields appear in generated list
- [x] 3.2 Test `suggest_similar_fields` with known typos for each section
- [x] 3.3 Add integration test with intentionally wrong field names
- [x] 3.4 Verify existing validation tests still pass

## 4. Documentation
- [x] 4.1 Update inline comments in `src/config.rs` explaining auto-generation
- [x] 4.2 Add README or doc comments in procedural macro crate (if Option A)
- [ ] 4.3 Document build script behavior in code comments (if Option B)

## 5. Validation
- [x] 5.1 Run full test suite
- [x] 5.2 Manually test with invalid config files to verify suggestions
- [x] 5.3 Verify `cargo build` completes successfully
- [x] 5.4 Confirm no performance regression in validation

