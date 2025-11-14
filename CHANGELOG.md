# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed

- Fixed preventAdditions setting in preToolUse which was non-functional - now correctly blocks Write operations matching configured glob patterns

## [0.2.1] - 2025-11-13

### Bug Fixes

- Fix changelog workflow to use git-cliff action (#75)

Replace nix-based approach with proven git-cliff GitHub action from official repo. Update cliff.toml template to use remote.github variables instead of env vars. ([a8932e5](https://github.com/connix-io/conclaude/commit/a8932e54839d4c39e977995ff851487690bf65c9))

## [0.2.0] - 2025-11-13

## [0.1.9] - 2025-11-13

## [0.1.8] - 2025-11-12

## [0.1.7] - 2025-11-03

### Features

- Comprehensive updates to configuration, hooks, and testing modules (#48) ([6f5a2be](https://github.com/connix-io/conclaude/commit/6f5a2be63e22da16b78f9c1ef89ba8dfa8d16dd0))
- [**breaking**] Remove gitWorktree configuration support (#53) ([bece5f4](https://github.com/connix-io/conclaude/commit/bece5f47a2dc61efccab3c71be88188f866a2859))
- Comprehensive openspec framework integration and hook reconfiguration (#54) ([922384a](https://github.com/connix-io/conclaude/commit/922384a39aa7996308612193881938754ed5aa51))
- Replace logging system with notification system and add command timeout (#58) ([e2fca5f](https://github.com/connix-io/conclaude/commit/e2fca5fa87dd79589858fc0e2dac6526422dafda))

### Documentation

- Enhance README.md with comprehensive installation guide and version update ([16464b7](https://github.com/connix-io/conclaude/commit/16464b74e56138116596ec8eded4fb223c6d3c78))

## [0.1.6] - 2025-10-08

## [0.1.5] - 2025-10-08

## [0.1.4] - 2025-10-08

### Features

- Migrate to cargo-dist and add SessionEnd hook (#46) ([59f39b1](https://github.com/connix-io/conclaude/commit/59f39b12e20f83332e1a7d49d0de1780cdd296b2))

## [0.1.3] - 2025-10-06

### Bug Fixes

- Resolve GitHub Actions workflow failures ([3074abd](https://github.com/connix-io/conclaude/commit/3074abdeef28b46bfbad8422d77984e26987cac3))
- Escape remaining backticks in schema.yml workflow ([615d1cd](https://github.com/connix-io/conclaude/commit/615d1cd6f99893ea822d2862dce6633aa254c1a0))
- Rewrite Create summary step using heredoc ([5da3742](https://github.com/connix-io/conclaude/commit/5da37427a690e5555df80b5eefa851f870ac42e5))
- Make permissions field optional in ClaudeSettings ([1a5565c](https://github.com/connix-io/conclaude/commit/1a5565c5b0fe3a54e5b8bca8062c27f7e8ad5b2c))
- Make hooks field optional in ClaudeSettings ([21b774b](https://github.com/connix-io/conclaude/commit/21b774b0df85f6b4453bb3e7d894e6df9891c390))
- Prevent includeCoAuthoredBy from serializing as null ([379152c](https://github.com/connix-io/conclaude/commit/379152c7307bfa8589175caa58c942a2914dccca))
- Fix grep stop config hooks having default values and not being checked if empty/set to null ([677fa63](https://github.com/connix-io/conclaude/commit/677fa63c43a304ffb1f4628a48931ab55309b071))

### Testing

- Trigger workflow testing ([9768d8b](https://github.com/connix-io/conclaude/commit/9768d8b8817e3bd9355831347c9a8af7cb19ea6a))
- Trigger Auto-update JSON Schema workflow ([c26d842](https://github.com/connix-io/conclaude/commit/c26d8423448209ca3918408cc4e4e6e7cd1001a3))
- Verify final workflow fixes ([c5f8a44](https://github.com/connix-io/conclaude/commit/c5f8a44eaadf7fc9d2c2ae4ac53df918a3f979db))
- Final verification of workflow fixes ([629f509](https://github.com/connix-io/conclaude/commit/629f509605998715493904b8ef2c2ac58dfe8bb5))

### Miscellaneous Tasks

- Auto-update JSON schema ([54970ef](https://github.com/connix-io/conclaude/commit/54970efb47fe63ba1f18e36c78d036b7e99d1c4a))
- Nix-powered CI and release; remove schema auto-update (#31) ([30d4b87](https://github.com/connix-io/conclaude/commit/30d4b87b2a559b9e503fab79a1d27cb61a87778e))
- Ci/nix actions (#32)

* ci: add Nix-powered CI and release; remove schema auto-update workflows

* docs(README): add CI/Release badges and Releases section with download instructions

* fix .claude/settings.json parser ([8a7b8ac](https://github.com/connix-io/conclaude/commit/8a7b8ace54296d5d7df295b2f3a1aca73d9744bb))

### Fix

- Handle empty grep rules arrays correctly (#35) ([df5660d](https://github.com/connix-io/conclaude/commit/df5660dc26713e42358ce802b9e25aebb5c565cd))

## [0.1.0] - 2025-09-05

### Bug Fixes

- Fix dev dep for types of yargs ([2dec02c](https://github.com/connix-io/conclaude/commit/2dec02c11ebda64568f0ece427d24b65d7e9b3b4))
- Fix bin packaging ([c39dc29](https://github.com/connix-io/conclaude/commit/c39dc29e33673c7d7e15c1f0902daa28f79d4a46))
- Fix package.json merge conflicts ([a5cf069](https://github.com/connix-io/conclaude/commit/a5cf0699765088333915c99f3bdebf8485294a8f))
- Fix removal of package-lock.json ([6914d3d](https://github.com/connix-io/conclaude/commit/6914d3daf292c82408496f82cd70b857e42a7daf))
- Fix npm hash for nix packaging ([5bd751f](https://github.com/connix-io/conclaude/commit/5bd751faa8e5a5bafc8b9a8e87290e76c2f8bda8))
- Fix infinite mode ([110f472](https://github.com/connix-io/conclaude/commit/110f4722c9c042ead0e6bdd92bf0cdd2ba54d763))

<!-- generated by git-cliff -->
