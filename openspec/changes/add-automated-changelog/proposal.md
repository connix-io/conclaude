## Why

The project currently lacks an automated changelog generation system. Manual changelog maintenance is error-prone, inconsistent, and time-consuming. With the existing release workflow using cargo-dist and GitHub Actions, we need a complementary system to automatically generate conventional-commits-based changelogs when tags are pushed.

## What Changes

- Add git-cliff configuration (`cliff.toml`) to parse conventional commits and generate formatted changelogs
- Create GitHub Actions workflow (`.github/workflows/changelog.yml`) that runs on version tag pushes
- Configure the workflow to generate `CHANGELOG.md` automatically from git history
- Set up bot credentials for committing changelog updates back to main branch
- Ensure git-cliff integrates seamlessly with existing cargo-dist release workflow

## Impact

- Affected specs: `documentation` (new capability)
- Affected code:
  - New file: `.github/workflows/changelog.yml`
  - New file: `cliff.toml`
  - Generated file: `CHANGELOG.md`
- Dependencies: Requires git-cliff action in CI
- Branch protection: May require configuring bot permissions to push to main
