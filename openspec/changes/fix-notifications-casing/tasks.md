## 1. Update NotificationsConfig struct
- [x] 1.1 Add serde(rename = "showErrors") to show_errors field
- [x] 1.2 Add serde(rename = "showSuccess") to show_success field
- [x] 1.3 Add serde(rename = "showSystemEvents") to show_system_events field
- [x] 1.4 Keep existing serde(default) attributes on all fields

## 2. Update error message
- [x] 2.1 Update line 174 error message to use camelCase field names
- [x] 2.2 Replace "show_errors, show_success, show_system_events" with "showErrors, showSuccess, showSystemEvents"

## 3. Validation
- [x] 3.1 Run openspec validate for this change
- [x] 3.2 Test configuration serialization/deserialization
- [x] 3.3 Verify error messages display correctly with new field names