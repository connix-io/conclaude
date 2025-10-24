## Why
The NotificationsConfig struct in src/config.rs uses snake_case field names that serialize as snake_case, which is inconsistent with the project's camelCase convention for configuration fields.

## What Changes
- Add serde(rename = "...") attributes to NotificationsConfig fields:
  - `show_errors` → rename = "showErrors"
  - `show_success` → rename = "showSuccess"
  - `show_system_events` → rename = "showSystemEvents"
- Keep existing `serde(default)` attributes on each field
- Update error message at line 174 to reference camelCase field names ("showErrors"/"showSuccess"/"showSystemEvents")

## Impact
- Affected specs: config
- Affected code: src/config.rs (lines 87-106, 174)
- This is a **BREAKING** change for any existing configuration files using snake_case field names