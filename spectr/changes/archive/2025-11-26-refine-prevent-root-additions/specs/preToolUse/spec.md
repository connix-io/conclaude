# Spec: Refine preventRootAdditions to Allow Root File Edits

**Capability:** `preToolUse`
**Status:** Draft
**Version:** 1.0.0

## Overview

Refines the `preventRootAdditions` enforcement behavior to distinguish between file **creation** (blocked) and file **modification** (allowed) at the repository root. This change enables practical workflows that require updating root-level configuration files (e.g., `package.json`, `.env`, `tsconfig.json`) while maintaining protection against accidental file creation at the root.

## MODIFIED Requirements

### Requirement: Root-Level File Addition Prevention

The system SHALL prevent Claude from creating **new** files at the repository root when `preventRootAdditions` is enabled. However, the system SHALL allow modifications to existing root-level files.

**Previous behavior:** Blocked all file creation and modification at root level (overly restrictive).

**New behavior:** Only blocks creation of new files at root; allows editing and overwriting existing root files (balanced protection).

#### Scenario: Prevent root additions enabled
- **WHEN** `preToolUse.preventRootAdditions` is set to `true`
- **AND** the target file does NOT exist at repository root
- **THEN** Claude SHALL NOT be allowed to create the new file
- **AND** any attempt to create such files SHALL result in an error message explaining the restriction

#### Scenario: Allow modification of existing root files
- **WHEN** `preToolUse.preventRootAdditions` is set to `true`
- **AND** the target file already exists at repository root
- **THEN** Claude SHALL be allowed to modify/overwrite the existing file
- **AND** no preventRootAdditions error SHALL be generated

#### Scenario: Prevent root additions disabled
- **WHEN** `preToolUse.preventRootAdditions` is set to `false`
- **THEN** Claude SHALL be allowed to create or modify files at the repository root
- **AND** all file operations in subdirectories remain subject to other restrictions

#### Scenario: Default behavior
- **WHEN** `preToolUse.preventRootAdditions` is not specified in configuration
- **THEN** the system SHALL default to `preventRootAdditions: true`
- **AND** root-level file creation SHALL be prevented by default
- **AND** existing root files may still be modified

## ADDED Requirements

### Requirement: File Existence Check for Root Additions

The system SHALL check if a target file exists at the resolved path before determining whether to block a Write operation under preventRootAdditions.

#### Scenario: Existence check prevents false positives
- **GIVEN** configuration contains `preToolUse.preventRootAdditions: true`
- **WHEN** determining whether to block a Write operation
- **THEN** the system SHALL check if the file exists at the resolved path
- **AND** only block if file does NOT exist at root

#### Scenario: File existence allows write
- **GIVEN** configuration contains `preToolUse.preventRootAdditions: true`
- **AND** file `package.json` exists at root
- **WHEN** Claude attempts to use Write tool to overwrite/modify `package.json`
- **THEN** the operation SHALL be allowed
- **AND** no error message SHALL be generated for preventRootAdditions

#### Scenario: Non-existent file is blocked
- **GIVEN** configuration contains `preToolUse.preventRootAdditions: true`
- **AND** file `docker-compose.yml` does NOT exist at root
- **WHEN** Claude attempts Write to `docker-compose.yml`
- **THEN** the system SHALL detect file does not exist
- **AND** the operation SHALL be blocked (new file at root)

---

**Summary:** preventRootAdditions now correctly allows modifications to existing root-level files while maintaining protection against creating new files at the root. This preserves the semantic meaning of "preventRootAdditions" (prevent adding/creating files at root) while enabling practical workflows that require updating configuration files.

## Related Specs

- **Modifies:** `preToolUse` - Refines preventRootAdditions enforcement logic
- **Works with:** `uneditableFiles` - Can be combined with preventRootAdditions for fine-grained control
