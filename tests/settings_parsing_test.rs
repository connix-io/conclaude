use serde_json;

// Replicate the structs from main.rs for testing
#[derive(serde::Serialize, serde::Deserialize)]
struct ClaudeHookConfig {
    #[serde(rename = "type")]
    config_type: String,
    command: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct ClaudeHookMatcher {
    matcher: String,
    hooks: Vec<ClaudeHookConfig>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct ClaudePermissions {
    allow: Vec<String>,
    deny: Vec<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct ClaudeSettings {
    #[serde(rename = "includeCoAuthoredBy")]
    include_co_authored_by: Option<bool>,
    permissions: Option<ClaudePermissions>,
    hooks: Option<std::collections::HashMap<String, Vec<ClaudeHookMatcher>>>,
}

#[test]
fn test_parse_settings_without_hooks_field() {
    // Test that settings can be parsed without the hooks field
    let json_without_hooks = r#"{
        "includeCoAuthoredBy": false,
        "permissions": {
            "allow": [],
            "deny": []
        }
    }"#;
    
    let result: Result<ClaudeSettings, _> = serde_json::from_str(json_without_hooks);
    assert!(result.is_ok(), "Should successfully parse settings without hooks field");
    
    let settings = result.unwrap();
    assert_eq!(settings.include_co_authored_by, Some(false));
    assert!(settings.permissions.is_some());
    assert!(settings.hooks.is_none());
}

#[test]
fn test_parse_settings_with_hooks_field() {
    // Test that settings can be parsed with the hooks field
    let json_with_hooks = r#"{
        "includeCoAuthoredBy": false,
        "permissions": {
            "allow": [],
            "deny": []
        },
        "hooks": {}
    }"#;
    
    let result: Result<ClaudeSettings, _> = serde_json::from_str(json_with_hooks);
    assert!(result.is_ok(), "Should successfully parse settings with hooks field");
    
    let settings = result.unwrap();
    assert_eq!(settings.include_co_authored_by, Some(false));
    assert!(settings.permissions.is_some());
    assert!(settings.hooks.is_some());
    assert_eq!(settings.hooks.unwrap().len(), 0);
}

#[test]
fn test_parse_minimal_settings() {
    // Test that minimal settings (all fields optional) can be parsed
    let json_minimal = r#"{}"#;
    
    let result: Result<ClaudeSettings, _> = serde_json::from_str(json_minimal);
    assert!(result.is_ok(), "Should successfully parse minimal settings");
    
    let settings = result.unwrap();
    assert!(settings.include_co_authored_by.is_none());
    assert!(settings.permissions.is_none());
    assert!(settings.hooks.is_none());
}

#[test]
fn test_parse_real_world_settings() {
    // Test with a more realistic Claude settings structure
    let json_real = r#"{
        "includeCoAuthoredBy": true,
        "permissions": {
            "allow": ["Write:**.md", "Read:**/*"],
            "deny": ["Bash:rm -rf"]
        }
    }"#;
    
    let result: Result<ClaudeSettings, _> = serde_json::from_str(json_real);
    assert!(result.is_ok(), "Should successfully parse real-world settings without hooks");
    
    let settings = result.unwrap();
    assert_eq!(settings.include_co_authored_by, Some(true));
    assert!(settings.permissions.is_some());
    
    let permissions = settings.permissions.unwrap();
    assert_eq!(permissions.allow.len(), 2);
    assert_eq!(permissions.deny.len(), 1);
    assert!(settings.hooks.is_none());
}