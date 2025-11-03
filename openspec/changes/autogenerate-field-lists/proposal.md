# Autogenerate Field Lists from Struct Definitions

## Why

The `suggest_similar_fields` function in `src/config.rs:161-202` maintains a hardcoded `Vec<(&str, Vec<&str>)>` mapping section names to their allowed subfields. This approach is fragile because it can drift from the actual struct field definitions (`StopConfig`, `RulesConfig`, `PreToolUseConfig`, `NotificationsConfig`, `StopCommand`), causing misleading error suggestions when users provide unknown field names.

## What Changes

- Replace the hardcoded field list in `suggest_similar_fields` with an auto-generated version derived directly from the struct definitions
- Implement either:
  - **Option A (Preferred)**: A procedural macro derive that generates the field mapping at compile time
  - **Option B**: A build script that reflects on the structs and emits the field list to an included file

This ensures field validation suggestions always match the actual configuration schema.

## Impact

- **Affected specs**: `configuration` (new capability or extending existing validation spec)
- **Affected code**:
  - `src/config.rs:161-202` - Replace hardcoded field list
  - New procedural macro crate OR `build.rs` script
  - Struct definitions will need derive/attribute annotations if using macro approach
- **Breaking**: No user-facing breaking changes; internal implementation only
- **Benefits**: Eliminates drift between struct definitions and error suggestions, improves maintainability
