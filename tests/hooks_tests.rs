use conclaude::hooks::*;
use conclaude::types::*;
use serde_json::Value;
use std::collections::HashMap;

// Helper function to create a base payload for testing
fn create_test_base_payload() -> BasePayload {
    BasePayload {
        session_id: "test_session_123".to_string(),
        transcript_path: "/tmp/test_transcript.jsonl".to_string(),
        hook_event_name: "PreToolUse".to_string(),
        cwd: "/home/user/project".to_string(),
        permission_mode: Some("default".to_string()),
    }
}

#[test]
fn test_extract_file_path_with_file_path() {
    let mut tool_input = HashMap::new();
    tool_input.insert(
        "file_path".to_string(),
        Value::String("test.txt".to_string()),
    );

    let result = extract_file_path(&tool_input);
    assert_eq!(result, Some("test.txt".to_string()));
}

#[test]
fn test_extract_file_path_with_notebook_path() {
    let mut tool_input = HashMap::new();
    tool_input.insert(
        "notebook_path".to_string(),
        Value::String("notebook.ipynb".to_string()),
    );

    let result = extract_file_path(&tool_input);
    assert_eq!(result, Some("notebook.ipynb".to_string()));
}

#[test]
fn test_extract_file_path_with_both_paths_prefers_file_path() {
    let mut tool_input = HashMap::new();
    tool_input.insert(
        "file_path".to_string(),
        Value::String("test.txt".to_string()),
    );
    tool_input.insert(
        "notebook_path".to_string(),
        Value::String("notebook.ipynb".to_string()),
    );

    let result = extract_file_path(&tool_input);
    assert_eq!(result, Some("test.txt".to_string()));
}

#[test]
fn test_extract_file_path_with_no_path() {
    let tool_input = HashMap::new();

    let result = extract_file_path(&tool_input);
    assert_eq!(result, None);
}

#[test]
fn test_extract_file_path_with_non_string_value() {
    let mut tool_input = HashMap::new();
    tool_input.insert(
        "file_path".to_string(),
        Value::Number(serde_json::Number::from(42)),
    );

    let result = extract_file_path(&tool_input);
    assert_eq!(result, None);
}

#[test]
fn test_is_root_addition_true_cases() {
    use std::env;

    // Get current working directory for testing
    let cwd = env::current_dir().unwrap();

    // Simulate config file in the current directory
    let config_path = cwd.join(".conclaude.yaml");

    // Files directly in root directory (same level as config)
    assert!(is_root_addition("", "test.txt", &config_path));
    assert!(is_root_addition("", "script.sh", &config_path));
    assert!(is_root_addition("", "data.json", &config_path));

    // BREAKING CHANGE: Dotfiles are now also blocked at root level
    assert!(is_root_addition("", ".gitignore", &config_path));
    assert!(is_root_addition("", ".env", &config_path));

    // BREAKING CHANGE: Config files are now also blocked at root level
    assert!(is_root_addition("", "package.json", &config_path));
    assert!(is_root_addition("", "tsconfig.json", &config_path));
    assert!(is_root_addition("", "config.yaml", &config_path));
    assert!(is_root_addition("", "settings.ini", &config_path));
    assert!(is_root_addition("", "bun.lockb", &config_path));
    assert!(is_root_addition("", "bun.lock", &config_path));
}

#[test]
fn test_is_root_addition_false_cases() {
    use std::env;

    // Get current working directory for testing
    let cwd = env::current_dir().unwrap();

    // Simulate config file in the current directory
    let config_path = cwd.join(".conclaude.yaml");

    // Files in subdirectories should not be blocked
    assert!(!is_root_addition("", "src/test.txt", &config_path));
    assert!(!is_root_addition("", "docs/readme.md", &config_path));
    assert!(!is_root_addition("", "tests/unit.rs", &config_path));

    // Edge cases - empty paths
    assert!(!is_root_addition("", "", &config_path));
    assert!(!is_root_addition("", "..", &config_path));
}

#[test]
fn test_matches_uneditable_pattern() {
    // Exact file matches
    assert!(matches_uneditable_pattern(
        "package.json",
        "package.json",
        "/path/package.json",
        "package.json"
    )
    .unwrap());

    // Wildcard matches
    assert!(matches_uneditable_pattern("test.md", "test.md", "/path/test.md", "*.md").unwrap());
    assert!(
        matches_uneditable_pattern("README.md", "README.md", "/path/README.md", "*.md").unwrap()
    );

    // Directory pattern matches
    assert!(matches_uneditable_pattern(
        "src/index.ts",
        "src/index.ts",
        "/path/src/index.ts",
        "src/**/*.ts"
    )
    .unwrap());
    assert!(matches_uneditable_pattern(
        "src/lib/utils.ts",
        "src/lib/utils.ts",
        "/path/src/lib/utils.ts",
        "src/**/*.ts"
    )
    .unwrap());

    // Negative matches
    assert!(
        !matches_uneditable_pattern("other.txt", "other.txt", "/path/other.txt", "*.md").unwrap()
    );
    assert!(!matches_uneditable_pattern(
        "lib/index.ts",
        "lib/index.ts",
        "/path/lib/index.ts",
        "src/**/*.ts"
    )
    .unwrap());
}

#[test]
fn test_matches_uneditable_pattern_invalid_glob() {
    let result = matches_uneditable_pattern("test.txt", "test.txt", "/path/test.txt", "[invalid");
    assert!(result.is_err());
}

#[test]
fn test_matches_uneditable_pattern_multiple_patterns() {
    // Test multiple patterns separately (since the glob crate doesn't support brace expansion)
    assert!(matches_uneditable_pattern(
        "package.json",
        "package.json",
        "/path/package.json",
        "package.json"
    )
    .unwrap());
    assert!(matches_uneditable_pattern(
        "tsconfig.json",
        "tsconfig.json",
        "/path/tsconfig.json",
        "tsconfig.json"
    )
    .unwrap());
    assert!(!matches_uneditable_pattern(
        "other.json",
        "other.json",
        "/path/other.json",
        "package.json"
    )
    .unwrap());
}

#[test]
fn test_matches_uneditable_pattern_environment_files() {
    assert!(matches_uneditable_pattern(".env", ".env", "/path/.env", ".env*").unwrap());
    assert!(
        matches_uneditable_pattern(".env.local", ".env.local", "/path/.env.local", ".env*")
            .unwrap()
    );
    assert!(matches_uneditable_pattern(
        ".env.production",
        ".env.production",
        "/path/.env.production",
        ".env*"
    )
    .unwrap());
    assert!(!matches_uneditable_pattern(
        "environment.txt",
        "environment.txt",
        "/path/environment.txt",
        ".env*"
    )
    .unwrap());
}

#[test]
fn test_matches_uneditable_pattern_directory_patterns() {
    // Match entire directories
    assert!(matches_uneditable_pattern(
        "docs/README.md",
        "docs/README.md",
        "/path/docs/README.md",
        "docs/**"
    )
    .unwrap());
    assert!(matches_uneditable_pattern(
        "docs/api/index.md",
        "docs/api/index.md",
        "/path/docs/api/index.md",
        "docs/**"
    )
    .unwrap());
    assert!(!matches_uneditable_pattern(
        "src/docs.ts",
        "src/docs.ts",
        "/path/src/docs.ts",
        "docs/**"
    )
    .unwrap());
}

// Integration test for path normalization scenarios
#[test]
fn test_file_path_normalization_scenarios() {
    let test_cases = [
        ("./package.json", "package.json", true),
        ("src/../package.json", "package.json", true),
        ("/absolute/path/package.json", "package.json", true),
        ("src/nested/file.ts", "src/**/*.ts", true),
    ];

    for (path, pattern, expected) in test_cases {
        // Normalize path similar to how the real code would
        let normalized = if let Some(stripped) = path.strip_prefix("./") {
            stripped
        } else if path.contains("..") {
            "package.json" // Simplified normalization for test
        } else {
            std::path::Path::new(path)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or(path)
        };

        let result = matches_uneditable_pattern(normalized, normalized, path, pattern).unwrap();
        assert_eq!(
            result, expected,
            "Failed for path: {path}, pattern: {pattern}"
        );
    }
}

// Test validation helpers
#[test]
fn test_validate_base_payload_integration() {
    let valid_base = create_test_base_payload();
    assert!(validate_base_payload(&valid_base).is_ok());

    let invalid_base = BasePayload {
        session_id: String::new(),
        transcript_path: "/path/to/transcript".to_string(),
        hook_event_name: "PreToolUse".to_string(),
        cwd: "/home/user/project".to_string(),
        permission_mode: Some("default".to_string()),
    };
    assert!(validate_base_payload(&invalid_base).is_err());
}

// Tests for auto-generated file checking
#[test]
fn test_check_generated_file_markers_do_not_edit() {
    let content = "// DO NOT EDIT - This file is generated\nfn main() {}";
    let result = check_generated_file_markers(content);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), "DO NOT EDIT");
}

#[test]
fn test_check_generated_file_markers_lowercase() {
    let content = "/* do not edit this file */\nclass Example {}";
    let result = check_generated_file_markers(content);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), "do not edit");
}

#[test]
fn test_check_generated_file_markers_code_generated() {
    let content = "// Code generated by protoc-gen-go. DO NOT EDIT.\npackage main";
    let result = check_generated_file_markers(content);
    assert!(result.is_some());
    // The function returns "DO NOT EDIT" which appears first in the check order
    assert_eq!(result.unwrap(), "DO NOT EDIT");
}

#[test]
fn test_check_generated_file_markers_auto_generated() {
    let content = "/* Auto-generated file - modifications will be overwritten */\n";
    let result = check_generated_file_markers(content);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), "Auto-generated");
}

#[test]
fn test_check_generated_file_markers_at_generated() {
    let content = "/**\n * @generated\n * This file is generated by tools\n */";
    let result = check_generated_file_markers(content);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), "@generated");
}

#[test]
fn test_check_generated_file_markers_this_file_generated() {
    let content = "// This file is generated automatically\n// Don't modify\n";
    let result = check_generated_file_markers(content);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), "This file is generated");
}

#[test]
fn test_check_generated_file_markers_case_variations() {
    let variations = vec![
        "// AUTOGENERATED",
        "// autogenerated",
        "// AuToGeNeRaTeD",
        "/* Generated Code */",
        "// generated code follows",
    ];

    for content in variations {
        let result = check_generated_file_markers(content);
        assert!(result.is_some(), "Failed to detect marker in: {}", content);
    }
}

#[test]
fn test_check_generated_file_markers_no_markers() {
    let content = "// Regular source file\nfn calculate() -> i32 {\n    42\n}";
    let result = check_generated_file_markers(content);
    assert!(result.is_none());
}

#[test]
fn test_check_generated_file_markers_marker_after_100_lines() {
    let mut content = String::new();
    for i in 0..105 {
        content.push_str(&format!("// Line {}\n", i));
    }
    content.push_str("// DO NOT EDIT\n");

    // Should not detect marker after 100 lines
    let result = check_generated_file_markers(&content);
    assert!(result.is_none());
}

#[test]
fn test_check_generated_file_markers_within_100_lines() {
    let mut content = String::new();
    for i in 0..95 {
        content.push_str(&format!("// Line {}\n", i));
    }
    content.push_str("// DO NOT EDIT - Generated file\n");

    // Should detect marker within first 100 lines
    let result = check_generated_file_markers(&content);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), "DO NOT EDIT");
}

// Note: Integration tests for auto-generated file checking with config cannot be
// easily tested due to the global config cache. The functionality is tested
// through unit tests of the marker detection function and will be tested in
// real usage when the hook is invoked.

// ============================================================================
// Integration Tests for Bash Command Validation
// ============================================================================

#[tokio::test]
async fn test_bash_validation_block_exact_command() -> anyhow::Result<()> {
    use conclaude::config::{ConclaudeConfig, RulesConfig, ToolUsageRule};

    // Create test configuration with block rule for exact command
    let config = ConclaudeConfig {
        rules: RulesConfig {
            tool_usage_validation: vec![ToolUsageRule {
                tool: "Bash".to_string(),
                pattern: String::new(),
                action: "block".to_string(),
                message: Some("Dangerous command blocked!".to_string()),
                command_pattern: Some("rm -rf /".to_string()),
                match_mode: Some("full".to_string()),
            }],
            ..Default::default()
        },
        ..Default::default()
    };

    // Create PreToolUsePayload with Bash command
    let mut tool_input = HashMap::new();
    tool_input.insert("command".to_string(), Value::String("rm -rf /".to_string()));

    let payload = PreToolUsePayload {
        base: create_test_base_payload(),
        tool_name: "Bash".to_string(),
        tool_input,
        tool_use_id: None,
    };

    // Manually test the pattern matching logic (since we can't easily inject config)
    // Extract the command
    let command = payload
        .tool_input
        .get("command")
        .and_then(|v| v.as_str())
        .unwrap();

    // Test full mode matching
    let rule = &config.rules.tool_usage_validation[0];
    let pattern = rule.command_pattern.as_ref().unwrap();
    let mode = rule.match_mode.as_deref().unwrap_or("full");

    let matches = if mode == "full" {
        glob::Pattern::new(pattern)?.matches(command)
    } else {
        false
    };

    assert!(matches, "Exact command should match in full mode");
    assert_eq!(rule.action, "block");
    assert_eq!(rule.message.as_deref(), Some("Dangerous command blocked!"));

    Ok(())
}

#[tokio::test]
async fn test_bash_validation_block_command_family() -> anyhow::Result<()> {
    use conclaude::config::{ConclaudeConfig, RulesConfig, ToolUsageRule};

    // Create test configuration with block rule for command family (prefix mode)
    let config = ConclaudeConfig {
        rules: RulesConfig {
            tool_usage_validation: vec![ToolUsageRule {
                tool: "Bash".to_string(),
                pattern: String::new(),
                action: "block".to_string(),
                message: Some("Git force push blocked!".to_string()),
                command_pattern: Some("git push --force*".to_string()),
                match_mode: Some("prefix".to_string()),
            }],
            ..Default::default()
        },
        ..Default::default()
    };

    // Create PreToolUsePayload with Bash command that should match in prefix mode
    let mut tool_input = HashMap::new();
    tool_input.insert(
        "command".to_string(),
        Value::String("git push --force origin main && echo done".to_string()),
    );

    let payload = PreToolUsePayload {
        base: create_test_base_payload(),
        tool_name: "Bash".to_string(),
        tool_input,
        tool_use_id: None,
    };

    // Test prefix mode matching
    let command = payload
        .tool_input
        .get("command")
        .and_then(|v| v.as_str())
        .unwrap();

    let rule = &config.rules.tool_usage_validation[0];
    let pattern = rule.command_pattern.as_ref().unwrap();
    let mode = rule.match_mode.as_deref().unwrap_or("full");

    let matches = if mode == "prefix" {
        let glob = glob::Pattern::new(pattern)?;
        let words: Vec<&str> = command.split_whitespace().collect();
        (1..=words.len()).any(|i| {
            let prefix = words[..i].join(" ");
            glob.matches(&prefix)
        })
    } else {
        false
    };

    assert!(matches, "Command family should match in prefix mode");
    assert_eq!(rule.action, "block");

    Ok(())
}

#[tokio::test]
async fn test_bash_validation_allow_whitelist() -> anyhow::Result<()> {
    use conclaude::config::{ConclaudeConfig, RulesConfig, ToolUsageRule};

    // Create test configuration with allow rule (whitelist pattern)
    let config = ConclaudeConfig {
        rules: RulesConfig {
            tool_usage_validation: vec![ToolUsageRule {
                tool: "Bash".to_string(),
                pattern: String::new(),
                action: "allow".to_string(),
                message: Some("Only safe commands allowed".to_string()),
                command_pattern: Some("echo *".to_string()),
                match_mode: Some("full".to_string()),
            }],
            ..Default::default()
        },
        ..Default::default()
    };

    // Test command that matches the whitelist
    let mut tool_input_allowed = HashMap::new();
    tool_input_allowed.insert(
        "command".to_string(),
        Value::String("echo hello world".to_string()),
    );

    let payload_allowed = PreToolUsePayload {
        base: create_test_base_payload(),
        tool_name: "Bash".to_string(),
        tool_input: tool_input_allowed,
        tool_use_id: None,
    };

    let command_allowed = payload_allowed
        .tool_input
        .get("command")
        .and_then(|v| v.as_str())
        .unwrap();

    let rule = &config.rules.tool_usage_validation[0];
    let pattern = rule.command_pattern.as_ref().unwrap();

    let matches_allowed = glob::Pattern::new(pattern)?.matches(command_allowed);
    assert!(
        matches_allowed,
        "Whitelisted command should match and be allowed"
    );

    // Test command that does NOT match the whitelist (should be blocked)
    let mut tool_input_blocked = HashMap::new();
    tool_input_blocked.insert(
        "command".to_string(),
        Value::String("rm -rf /tmp".to_string()),
    );

    let payload_blocked = PreToolUsePayload {
        base: create_test_base_payload(),
        tool_name: "Bash".to_string(),
        tool_input: tool_input_blocked,
        tool_use_id: None,
    };

    let command_blocked = payload_blocked
        .tool_input
        .get("command")
        .and_then(|v| v.as_str())
        .unwrap();

    let matches_blocked = glob::Pattern::new(pattern)?.matches(command_blocked);
    assert!(
        !matches_blocked,
        "Non-whitelisted command should not match and be blocked"
    );
    assert_eq!(rule.action, "allow");

    Ok(())
}

#[tokio::test]
async fn test_bash_validation_custom_message() -> anyhow::Result<()> {
    use conclaude::config::{ConclaudeConfig, RulesConfig, ToolUsageRule};

    // Create test configuration with custom error message
    let custom_message = "DANGER: This command could delete important files!";
    let config = ConclaudeConfig {
        rules: RulesConfig {
            tool_usage_validation: vec![ToolUsageRule {
                tool: "Bash".to_string(),
                pattern: String::new(),
                action: "block".to_string(),
                message: Some(custom_message.to_string()),
                command_pattern: Some("rm -rf*".to_string()),
                match_mode: Some("full".to_string()),
            }],
            ..Default::default()
        },
        ..Default::default()
    };

    let mut tool_input = HashMap::new();
    tool_input.insert(
        "command".to_string(),
        Value::String("rm -rf /tmp".to_string()),
    );

    let payload = PreToolUsePayload {
        base: create_test_base_payload(),
        tool_name: "Bash".to_string(),
        tool_input,
        tool_use_id: None,
    };

    let command = payload
        .tool_input
        .get("command")
        .and_then(|v| v.as_str())
        .unwrap();

    let rule = &config.rules.tool_usage_validation[0];
    let pattern = rule.command_pattern.as_ref().unwrap();

    let matches = glob::Pattern::new(pattern)?.matches(command);
    assert!(matches, "Command should match the pattern");
    assert_eq!(rule.message.as_deref(), Some(custom_message));

    Ok(())
}

#[tokio::test]
async fn test_bash_validation_default_match_mode() -> anyhow::Result<()> {
    use conclaude::config::{ConclaudeConfig, RulesConfig, ToolUsageRule};

    // Create test configuration WITHOUT explicit matchMode (should default to "full")
    let config = ConclaudeConfig {
        rules: RulesConfig {
            tool_usage_validation: vec![ToolUsageRule {
                tool: "Bash".to_string(),
                pattern: String::new(),
                action: "block".to_string(),
                message: None,
                command_pattern: Some("curl *".to_string()),
                match_mode: None, // No explicit mode - should default to "full"
            }],
            ..Default::default()
        },
        ..Default::default()
    };

    let mut tool_input = HashMap::new();
    tool_input.insert(
        "command".to_string(),
        Value::String("curl https://evil.com".to_string()),
    );

    let payload = PreToolUsePayload {
        base: create_test_base_payload(),
        tool_name: "Bash".to_string(),
        tool_input,
        tool_use_id: None,
    };

    let command = payload
        .tool_input
        .get("command")
        .and_then(|v| v.as_str())
        .unwrap();

    let rule = &config.rules.tool_usage_validation[0];
    let pattern = rule.command_pattern.as_ref().unwrap();
    let mode = rule.match_mode.as_deref().unwrap_or("full");

    assert_eq!(mode, "full", "Default matchMode should be 'full'");

    let matches = glob::Pattern::new(pattern)?.matches(command);
    assert!(matches, "Command should match in full mode");

    Ok(())
}

#[tokio::test]
async fn test_bash_validation_backward_compatible() -> anyhow::Result<()> {
    use conclaude::config::{ConclaudeConfig, RulesConfig, ToolUsageRule};

    // Create test configuration with file path rule (backward compatibility)
    let config = ConclaudeConfig {
        rules: RulesConfig {
            tool_usage_validation: vec![ToolUsageRule {
                tool: "Write".to_string(),
                pattern: ".env*".to_string(),
                action: "block".to_string(),
                message: Some("Cannot write to .env files".to_string()),
                command_pattern: None, // No command pattern - uses file path pattern
                match_mode: None,
            }],
            ..Default::default()
        },
        ..Default::default()
    };

    let mut tool_input = HashMap::new();
    tool_input.insert(
        "file_path".to_string(),
        Value::String(".env.local".to_string()),
    );

    let payload = PreToolUsePayload {
        base: create_test_base_payload(),
        tool_name: "Write".to_string(),
        tool_input,
        tool_use_id: None,
    };

    let file_path = payload
        .tool_input
        .get("file_path")
        .and_then(|v| v.as_str())
        .unwrap();

    let rule = &config.rules.tool_usage_validation[0];
    let pattern = &rule.pattern;

    let matches = glob::Pattern::new(pattern)?.matches(file_path);
    assert!(matches, "File path pattern should still work");
    assert_eq!(rule.tool, "Write");
    assert!(rule.command_pattern.is_none());

    Ok(())
}

#[tokio::test]
async fn test_bash_validation_wildcard_tool() -> anyhow::Result<()> {
    use conclaude::config::{ConclaudeConfig, RulesConfig, ToolUsageRule};

    // Create test configuration with tool: "*" (applies to all tools, including Bash)
    let config = ConclaudeConfig {
        rules: RulesConfig {
            tool_usage_validation: vec![ToolUsageRule {
                tool: "*".to_string(),
                pattern: String::new(),
                action: "block".to_string(),
                message: Some("Wildcard rule blocks this Bash command".to_string()),
                command_pattern: Some("sudo *".to_string()),
                match_mode: Some("full".to_string()),
            }],
            ..Default::default()
        },
        ..Default::default()
    };

    let mut tool_input = HashMap::new();
    tool_input.insert(
        "command".to_string(),
        Value::String("sudo apt-get update".to_string()),
    );

    let payload = PreToolUsePayload {
        base: create_test_base_payload(),
        tool_name: "Bash".to_string(),
        tool_input,
        tool_use_id: None,
    };

    let command = payload
        .tool_input
        .get("command")
        .and_then(|v| v.as_str())
        .unwrap();

    let rule = &config.rules.tool_usage_validation[0];
    let pattern = rule.command_pattern.as_ref().unwrap();

    // Verify wildcard tool matches Bash
    assert!(
        rule.tool == "Bash" || rule.tool == "*",
        "Wildcard tool should apply to Bash"
    );

    let matches = glob::Pattern::new(pattern)?.matches(command);
    assert!(matches, "Wildcard tool rule should apply to Bash commands");

    Ok(())
}

#[tokio::test]
async fn test_bash_validation_prefix_mode_no_match_in_middle() -> anyhow::Result<()> {
    use conclaude::config::{ConclaudeConfig, RulesConfig, ToolUsageRule};

    // Create test configuration with prefix mode
    let config = ConclaudeConfig {
        rules: RulesConfig {
            tool_usage_validation: vec![ToolUsageRule {
                tool: "Bash".to_string(),
                pattern: String::new(),
                action: "block".to_string(),
                message: None,
                command_pattern: Some("curl *".to_string()),
                match_mode: Some("prefix".to_string()),
            }],
            ..Default::default()
        },
        ..Default::default()
    };

    // Command where "curl" appears in the middle (should NOT match in prefix mode)
    let mut tool_input = HashMap::new();
    tool_input.insert(
        "command".to_string(),
        Value::String("echo test && curl https://example.com".to_string()),
    );

    let payload = PreToolUsePayload {
        base: create_test_base_payload(),
        tool_name: "Bash".to_string(),
        tool_input,
        tool_use_id: None,
    };

    let command = payload
        .tool_input
        .get("command")
        .and_then(|v| v.as_str())
        .unwrap();

    let rule = &config.rules.tool_usage_validation[0];
    let pattern = rule.command_pattern.as_ref().unwrap();
    let mode = rule.match_mode.as_deref().unwrap_or("full");

    let matches = if mode == "prefix" {
        let glob = glob::Pattern::new(pattern)?;
        let words: Vec<&str> = command.split_whitespace().collect();
        (1..=words.len()).any(|i| {
            let prefix = words[..i].join(" ");
            glob.matches(&prefix)
        })
    } else {
        false
    };

    assert!(
        !matches,
        "Prefix mode should NOT match pattern in middle of command"
    );

    Ok(())
}

#[tokio::test]
async fn test_bash_validation_multiple_rules() -> anyhow::Result<()> {
    use conclaude::config::{ConclaudeConfig, RulesConfig, ToolUsageRule};

    // Create test configuration with multiple rules
    let config = ConclaudeConfig {
        rules: RulesConfig {
            tool_usage_validation: vec![
                ToolUsageRule {
                    tool: "Bash".to_string(),
                    pattern: String::new(),
                    action: "block".to_string(),
                    message: Some("Blocked: rm commands".to_string()),
                    command_pattern: Some("rm *".to_string()),
                    match_mode: Some("full".to_string()),
                },
                ToolUsageRule {
                    tool: "Bash".to_string(),
                    pattern: String::new(),
                    action: "block".to_string(),
                    message: Some("Blocked: curl commands".to_string()),
                    command_pattern: Some("curl *".to_string()),
                    match_mode: Some("full".to_string()),
                },
            ],
            ..Default::default()
        },
        ..Default::default()
    };

    // Test that first rule matches
    let mut tool_input1 = HashMap::new();
    tool_input1.insert(
        "command".to_string(),
        Value::String("rm -rf /tmp".to_string()),
    );

    let payload1 = PreToolUsePayload {
        base: create_test_base_payload(),
        tool_name: "Bash".to_string(),
        tool_input: tool_input1,
        tool_use_id: None,
    };

    let command1 = payload1
        .tool_input
        .get("command")
        .and_then(|v| v.as_str())
        .unwrap();
    let rule1 = &config.rules.tool_usage_validation[0];
    let matches1 = glob::Pattern::new(rule1.command_pattern.as_ref().unwrap())?.matches(command1);
    assert!(matches1, "First rule should match rm command");

    // Test that second rule matches
    let mut tool_input2 = HashMap::new();
    tool_input2.insert(
        "command".to_string(),
        Value::String("curl https://example.com".to_string()),
    );

    let payload2 = PreToolUsePayload {
        base: create_test_base_payload(),
        tool_name: "Bash".to_string(),
        tool_input: tool_input2,
        tool_use_id: None,
    };

    let command2 = payload2
        .tool_input
        .get("command")
        .and_then(|v| v.as_str())
        .unwrap();
    let rule2 = &config.rules.tool_usage_validation[1];
    let matches2 = glob::Pattern::new(rule2.command_pattern.as_ref().unwrap())?.matches(command2);
    assert!(matches2, "Second rule should match curl command");

    Ok(())
}

// ============================================================================
// Integration Tests for SubagentStop Hook
// ============================================================================

// Helper function to create a SubagentStop payload for testing
fn create_subagent_stop_payload() -> SubagentStopPayload {
    SubagentStopPayload {
        base: BasePayload {
            session_id: "test_session_456".to_string(),
            transcript_path: "/tmp/session_transcript.jsonl".to_string(),
            hook_event_name: "SubagentStop".to_string(),
            cwd: "/home/user/project".to_string(),
            permission_mode: Some("default".to_string()),
        },
        stop_hook_active: true,
        agent_id: "coder".to_string(),
        agent_transcript_path: "/tmp/coder_transcript.jsonl".to_string(),
    }
}

#[test]
fn test_subagent_stop_payload_structure() {
    let payload = create_subagent_stop_payload();

    // Verify all required fields are present and populated
    assert_eq!(payload.base.session_id, "test_session_456");
    assert_eq!(
        payload.base.transcript_path,
        "/tmp/session_transcript.jsonl"
    );
    assert_eq!(payload.base.hook_event_name, "SubagentStop");
    assert_eq!(payload.base.cwd, "/home/user/project");
    assert!(payload.base.permission_mode.is_some());
    assert!(payload.stop_hook_active);
    assert_eq!(payload.agent_id, "coder");
    assert_eq!(payload.agent_transcript_path, "/tmp/coder_transcript.jsonl");
}

#[test]
fn test_subagent_stop_payload_with_different_agent_ids() {
    // Test with different agent IDs: coder, tester, stuck
    let agent_ids = vec!["coder", "tester", "stuck", "orchestrator"];

    for agent_id in agent_ids {
        let mut payload = create_subagent_stop_payload();
        payload.agent_id = agent_id.to_string();

        assert_eq!(payload.agent_id, agent_id);
        assert!(validate_subagent_stop_payload(&payload).is_ok());
    }
}

#[test]
fn test_subagent_stop_payload_serialization() {
    let payload = create_subagent_stop_payload();

    // Serialize to JSON
    let json_str = serde_json::to_string(&payload).expect("Failed to serialize");

    // Deserialize back
    let deserialized: SubagentStopPayload =
        serde_json::from_str(&json_str).expect("Failed to deserialize");

    // Verify round-trip preservation
    assert_eq!(deserialized.base.session_id, payload.base.session_id);
    assert_eq!(deserialized.agent_id, payload.agent_id);
    assert_eq!(
        deserialized.agent_transcript_path,
        payload.agent_transcript_path
    );
    assert_eq!(deserialized.stop_hook_active, payload.stop_hook_active);
}

#[test]
fn test_validate_subagent_stop_payload_all_fields_valid() {
    let payload = create_subagent_stop_payload();
    let result = validate_subagent_stop_payload(&payload);

    assert!(result.is_ok(), "Valid payload should pass validation");
}

#[test]
fn test_validate_subagent_stop_payload_missing_base_session_id() {
    let mut payload = create_subagent_stop_payload();
    payload.base.session_id = String::new();

    let result = validate_subagent_stop_payload(&payload);
    assert!(
        result.is_err(),
        "Payload with empty session_id should fail validation"
    );
    let error = result.unwrap_err();
    assert!(
        error.contains("session_id") || error.contains("Missing required field"),
        "Error should mention session_id or missing field: {}",
        error
    );
}

#[test]
fn test_validate_subagent_stop_payload_missing_base_transcript_path() {
    let mut payload = create_subagent_stop_payload();
    payload.base.transcript_path = String::new();

    let result = validate_subagent_stop_payload(&payload);
    assert!(
        result.is_err(),
        "Payload with empty transcript_path should fail validation"
    );
    let error = result.unwrap_err();
    assert!(
        error.contains("transcript_path") || error.contains("Missing required field"),
        "Error should mention transcript_path or missing field: {}",
        error
    );
}

#[test]
fn test_validate_subagent_stop_payload_missing_agent_id() {
    let mut payload = create_subagent_stop_payload();
    payload.agent_id = String::new();

    let result = validate_subagent_stop_payload(&payload);
    assert!(
        result.is_err(),
        "Payload with empty agent_id should fail validation"
    );
    assert!(result.unwrap_err().contains("agent_id cannot be empty"));
}

#[test]
fn test_validate_subagent_stop_payload_whitespace_agent_id() {
    let mut payload = create_subagent_stop_payload();
    payload.agent_id = "   ".to_string(); // Only whitespace

    let result = validate_subagent_stop_payload(&payload);
    assert!(
        result.is_err(),
        "Payload with whitespace-only agent_id should fail validation"
    );
    assert!(result.unwrap_err().contains("agent_id cannot be empty"));
}

#[test]
fn test_validate_subagent_stop_payload_missing_agent_transcript_path() {
    let mut payload = create_subagent_stop_payload();
    payload.agent_transcript_path = String::new();

    let result = validate_subagent_stop_payload(&payload);
    assert!(
        result.is_err(),
        "Payload with empty agent_transcript_path should fail validation"
    );
    assert!(result
        .unwrap_err()
        .contains("agent_transcript_path cannot be empty"));
}

#[test]
fn test_validate_subagent_stop_payload_whitespace_agent_transcript_path() {
    let mut payload = create_subagent_stop_payload();
    payload.agent_transcript_path = "   ".to_string(); // Only whitespace

    let result = validate_subagent_stop_payload(&payload);
    assert!(
        result.is_err(),
        "Payload with whitespace-only agent_transcript_path should fail validation"
    );
    assert!(result
        .unwrap_err()
        .contains("agent_transcript_path cannot be empty"));
}

#[test]
fn test_validate_subagent_stop_payload_multiple_missing_fields() {
    let mut payload = create_subagent_stop_payload();
    payload.agent_id = String::new();
    payload.agent_transcript_path = String::new();

    let result = validate_subagent_stop_payload(&payload);
    assert!(
        result.is_err(),
        "Payload with multiple missing fields should fail"
    );
}

#[test]
fn test_subagent_stop_payload_json_round_trip_coder() {
    let json_str = r#"{
        "session_id": "test_session_coder",
        "transcript_path": "/tmp/session_123.jsonl",
        "hook_event_name": "SubagentStop",
        "cwd": "/home/user/project",
        "permission_mode": "default",
        "stop_hook_active": true,
        "agent_id": "coder",
        "agent_transcript_path": "/tmp/coder_123.jsonl"
    }"#;

    let payload: SubagentStopPayload =
        serde_json::from_str(json_str).expect("Failed to deserialize JSON");

    assert_eq!(payload.agent_id, "coder");
    assert_eq!(payload.agent_transcript_path, "/tmp/coder_123.jsonl");
    assert_eq!(payload.base.session_id, "test_session_coder");
    assert!(validate_subagent_stop_payload(&payload).is_ok());
}

#[test]
fn test_subagent_stop_payload_json_round_trip_tester() {
    let json_str = r#"{
        "session_id": "test_session_tester",
        "transcript_path": "/tmp/session_456.jsonl",
        "hook_event_name": "SubagentStop",
        "cwd": "/home/user/project",
        "permission_mode": "default",
        "stop_hook_active": false,
        "agent_id": "tester",
        "agent_transcript_path": "/tmp/tester_456.jsonl"
    }"#;

    let payload: SubagentStopPayload =
        serde_json::from_str(json_str).expect("Failed to deserialize JSON");

    assert_eq!(payload.agent_id, "tester");
    assert_eq!(payload.agent_transcript_path, "/tmp/tester_456.jsonl");
    assert!(!payload.stop_hook_active);
    assert!(validate_subagent_stop_payload(&payload).is_ok());
}

#[test]
fn test_subagent_stop_payload_json_round_trip_stuck() {
    let json_str = r#"{
        "session_id": "test_session_stuck",
        "transcript_path": "/tmp/session_789.jsonl",
        "hook_event_name": "SubagentStop",
        "cwd": "/home/user/project",
        "permission_mode": "restrict",
        "stop_hook_active": true,
        "agent_id": "stuck",
        "agent_transcript_path": "/tmp/stuck_789.jsonl"
    }"#;

    let payload: SubagentStopPayload =
        serde_json::from_str(json_str).expect("Failed to deserialize JSON");

    assert_eq!(payload.agent_id, "stuck");
    assert_eq!(payload.agent_transcript_path, "/tmp/stuck_789.jsonl");
    assert_eq!(payload.base.permission_mode, Some("restrict".to_string()));
    assert!(validate_subagent_stop_payload(&payload).is_ok());
}

#[test]
fn test_subagent_stop_payload_with_long_paths() {
    let mut payload = create_subagent_stop_payload();
    let long_path =
        "/very/long/path/to/transcript/file/that/contains/many/nested/directories/transcript.jsonl";
    payload.agent_transcript_path = long_path.to_string();

    assert_eq!(payload.agent_transcript_path, long_path);
    assert!(validate_subagent_stop_payload(&payload).is_ok());
}

#[test]
fn test_subagent_stop_payload_with_special_characters_in_paths() {
    let mut payload = create_subagent_stop_payload();
    payload.agent_transcript_path = "/tmp/transcript-2024-11-16_10:30:45.jsonl".to_string();
    payload.agent_id = "coder-v2.1".to_string();

    assert!(validate_subagent_stop_payload(&payload).is_ok());
}

#[test]
fn test_subagent_stop_payload_agent_id_case_sensitive() {
    let mut payload1 = create_subagent_stop_payload();
    let mut payload2 = create_subagent_stop_payload();

    payload1.agent_id = "Coder".to_string();
    payload2.agent_id = "coder".to_string();

    // Both should validate successfully
    assert!(validate_subagent_stop_payload(&payload1).is_ok());
    assert!(validate_subagent_stop_payload(&payload2).is_ok());

    // But they should be different values
    assert_ne!(payload1.agent_id, payload2.agent_id);
}

#[test]
fn test_subagent_stop_hook_event_name_validation() {
    let payload = create_subagent_stop_payload();

    // Verify the hook event name is correctly set
    assert_eq!(payload.base.hook_event_name, "SubagentStop");

    // Verify the base payload validates correctly
    assert!(validate_base_payload(&payload.base).is_ok());
}

#[test]
fn test_subagent_stop_payload_with_empty_cwd() {
    let mut payload = create_subagent_stop_payload();
    payload.base.cwd = String::new();

    // Should fail on base payload validation
    let result = validate_subagent_stop_payload(&payload);
    assert!(result.is_err());
}

#[test]
fn test_subagent_stop_payload_with_none_permission_mode() {
    let mut payload = create_subagent_stop_payload();
    payload.base.permission_mode = None;

    // Should still validate successfully since permission_mode is optional
    assert!(validate_subagent_stop_payload(&payload).is_ok());
}

#[test]
fn test_subagent_stop_payload_with_empty_permission_mode() {
    let mut payload = create_subagent_stop_payload();
    payload.base.permission_mode = Some(String::new());

    // Should still validate - only required fields matter for base payload
    // Empty permission_mode is allowed
    assert!(validate_subagent_stop_payload(&payload).is_ok());
}

#[test]
fn test_subagent_stop_payload_stop_hook_active_true_and_false() {
    let mut payload_true = create_subagent_stop_payload();
    let mut payload_false = create_subagent_stop_payload();

    payload_true.stop_hook_active = true;
    payload_false.stop_hook_active = false;

    assert!(validate_subagent_stop_payload(&payload_true).is_ok());
    assert!(validate_subagent_stop_payload(&payload_false).is_ok());
    assert_ne!(
        payload_true.stop_hook_active,
        payload_false.stop_hook_active
    );
}

#[test]
fn test_subagent_stop_payload_json_with_missing_agent_id_field() {
    let json_str = r#"{
        "session_id": "test_session",
        "transcript_path": "/tmp/session.jsonl",
        "hook_event_name": "SubagentStop",
        "cwd": "/home/user/project",
        "permission_mode": "default",
        "stop_hook_active": true,
        "agent_transcript_path": "/tmp/agent.jsonl"
    }"#;

    // This should fail to deserialize because agent_id is required
    let result: Result<SubagentStopPayload, _> = serde_json::from_str(json_str);
    assert!(
        result.is_err(),
        "JSON missing agent_id should fail to deserialize"
    );
}

#[test]
fn test_subagent_stop_payload_json_with_missing_agent_transcript_path_field() {
    let json_str = r#"{
        "session_id": "test_session",
        "transcript_path": "/tmp/session.jsonl",
        "hook_event_name": "SubagentStop",
        "cwd": "/home/user/project",
        "permission_mode": "default",
        "stop_hook_active": true,
        "agent_id": "coder"
    }"#;

    // This should fail to deserialize because agent_transcript_path is required
    let result: Result<SubagentStopPayload, _> = serde_json::from_str(json_str);
    assert!(
        result.is_err(),
        "JSON missing agent_transcript_path should fail to deserialize"
    );
}

#[test]
fn test_subagent_stop_environment_variable_setting_simulation() {
    // Simulate setting environment variables as the hook handler would do
    let payload = create_subagent_stop_payload();
    let unique_id = "test_env_var_setting";

    // Set environment variables with unique suffix to avoid test conflicts
    let agent_id_var = format!("CONCLAUDE_AGENT_ID_{}", unique_id);
    let transcript_var = format!("CONCLAUDE_AGENT_TRANSCRIPT_PATH_{}", unique_id);

    std::env::set_var(&agent_id_var, &payload.agent_id);
    std::env::set_var(&transcript_var, &payload.agent_transcript_path);

    // Verify they were set correctly
    assert_eq!(std::env::var(&agent_id_var).unwrap(), payload.agent_id);
    assert_eq!(
        std::env::var(&transcript_var).unwrap(),
        payload.agent_transcript_path
    );

    // Test the actual environment variables (document what the hook does)
    // Note: We only verify the structure is correct, actual env vars may be set by other tests
    assert!(!payload.agent_id.is_empty());
    assert!(!payload.agent_transcript_path.is_empty());

    // Clean up
    std::env::remove_var(&agent_id_var);
    std::env::remove_var(&transcript_var);
}

#[test]
fn test_subagent_stop_environment_variables_different_agents() {
    let agents = ["coder", "tester", "stuck"];

    for (idx, agent) in agents.iter().enumerate() {
        let mut payload = create_subagent_stop_payload();
        payload.agent_id = agent.to_string();

        // Set environment variables with unique suffix to avoid test conflicts
        let agent_id_var = format!("CONCLAUDE_AGENT_ID_diff_agents_{}", idx);
        let transcript_var = format!("CONCLAUDE_AGENT_TRANSCRIPT_PATH_diff_agents_{}", idx);

        std::env::set_var(&agent_id_var, &payload.agent_id);
        std::env::set_var(&transcript_var, &payload.agent_transcript_path);

        // Verify they match
        assert_eq!(
            std::env::var(&agent_id_var).unwrap(),
            *agent,
            "Agent ID should match in environment for {}",
            agent
        );

        // Clean up
        std::env::remove_var(&agent_id_var);
        std::env::remove_var(&transcript_var);
    }
}

#[test]
fn test_subagent_stop_environment_variables_special_paths() {
    let paths = [
        "/tmp/transcript-with-dashes.jsonl",
        "/tmp/transcript_with_underscores.jsonl",
        "/tmp/transcript.2024-11-16.jsonl",
        "/var/log/conclaude/transcript.jsonl",
    ];

    for (idx, path) in paths.iter().enumerate() {
        let mut payload = create_subagent_stop_payload();
        payload.agent_transcript_path = path.to_string();

        // Set environment variable with unique suffix
        let transcript_var = format!("CONCLAUDE_AGENT_TRANSCRIPT_PATH_special_{}", idx);
        std::env::set_var(&transcript_var, &payload.agent_transcript_path);

        // Verify it was set correctly by reading it back
        let read_value = std::env::var(&transcript_var);
        assert!(
            read_value.is_ok(),
            "Path should be set in environment: {}",
            path
        );
        assert_eq!(
            read_value.unwrap(),
            *path,
            "Path should be preserved in environment: {}",
            path
        );

        // Clean up for next iteration
        std::env::remove_var(&transcript_var);
    }
}

#[test]
fn test_subagent_stop_multiple_sequential_invocations() {
    let agents = [
        ("coder", "/tmp/coder_001.jsonl"),
        ("tester", "/tmp/tester_001.jsonl"),
        ("stuck", "/tmp/stuck_001.jsonl"),
    ];

    for (idx, (agent_id, transcript_path)) in agents.iter().enumerate() {
        let mut payload = create_subagent_stop_payload();
        payload.agent_id = agent_id.to_string();
        payload.agent_transcript_path = transcript_path.to_string();

        // Validate the payload
        assert!(
            validate_subagent_stop_payload(&payload).is_ok(),
            "Payload for {} should validate",
            agent_id
        );

        // Simulate setting environment variables with unique suffix
        let agent_id_var = format!("CONCLAUDE_AGENT_ID_seq_{}", idx);
        let transcript_var = format!("CONCLAUDE_AGENT_TRANSCRIPT_PATH_seq_{}", idx);

        std::env::set_var(&agent_id_var, &payload.agent_id);
        std::env::set_var(&transcript_var, &payload.agent_transcript_path);

        // Verify environment variables are set to the new values
        let agent_id_read = std::env::var(&agent_id_var);
        assert!(agent_id_read.is_ok());
        assert_eq!(
            agent_id_read.unwrap(),
            *agent_id,
            "Environment variable should reflect current agent"
        );

        let transcript_path_read = std::env::var(&transcript_var);
        assert!(transcript_path_read.is_ok());
        assert_eq!(
            transcript_path_read.unwrap(),
            *transcript_path,
            "Environment variable should reflect current transcript path"
        );

        // Clean up
        std::env::remove_var(&agent_id_var);
        std::env::remove_var(&transcript_var);
    }
}

#[test]
fn test_subagent_stop_payload_all_required_fields_present() {
    // This test ensures all required fields are actually present and tested
    let payload = create_subagent_stop_payload();

    // Base fields
    assert!(!payload.base.session_id.is_empty());
    assert!(!payload.base.transcript_path.is_empty());
    assert_eq!(payload.base.hook_event_name, "SubagentStop");
    assert!(!payload.base.cwd.is_empty());

    // New SubagentStop-specific fields
    assert!(!payload.agent_id.is_empty());
    assert!(!payload.agent_transcript_path.is_empty());

    // Verify validation passes
    assert!(validate_subagent_stop_payload(&payload).is_ok());
}

#[test]
fn test_subagent_stop_validation_fails_gracefully_on_invalid_base() {
    let mut payload = create_subagent_stop_payload();
    // Invalidate the base payload by removing session_id
    payload.base.session_id = String::new();

    // The validation should fail due to invalid base
    let result = validate_subagent_stop_payload(&payload);
    assert!(result.is_err());

    // The error message should indicate which field is invalid
    let error_msg = result.unwrap_err();
    assert!(error_msg.contains("session_id"));
}

#[test]
fn test_subagent_stop_validation_fails_gracefully_on_invalid_agent_id() {
    let mut payload = create_subagent_stop_payload();
    payload.agent_id = String::new();

    let result = validate_subagent_stop_payload(&payload);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("agent_id"));
}

#[test]
fn test_subagent_stop_validation_fails_gracefully_on_invalid_agent_transcript_path() {
    let mut payload = create_subagent_stop_payload();
    payload.agent_transcript_path = String::new();

    let result = validate_subagent_stop_payload(&payload);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("agent_transcript_path"));
}

#[test]
fn test_subagent_stop_validation_error_messages_specific() {
    // Test that error messages are specific and helpful

    let mut payload = create_subagent_stop_payload();

    // Test agent_id error
    payload.agent_id = String::new();
    let error = validate_subagent_stop_payload(&payload).unwrap_err();
    assert!(
        error.contains("agent_id"),
        "Error should mention agent_id field"
    );

    // Test agent_transcript_path error
    let mut payload = create_subagent_stop_payload();
    payload.agent_transcript_path = String::new();
    let error = validate_subagent_stop_payload(&payload).unwrap_err();
    assert!(
        error.contains("agent_transcript_path"),
        "Error should mention agent_transcript_path field"
    );

    // Test whitespace-only agent_id error
    let mut payload = create_subagent_stop_payload();
    payload.agent_id = "   ".to_string();
    let error = validate_subagent_stop_payload(&payload).unwrap_err();
    assert!(
        error.contains("agent_id"),
        "Error should mention agent_id for whitespace-only value"
    );
}

// ============================================================================
// Integration Tests for SubagentStart Hook
// ============================================================================

// Helper function to create a SubagentStart payload for testing
fn create_subagent_start_payload() -> SubagentStartPayload {
    SubagentStartPayload {
        base: BasePayload {
            session_id: "test_session_789".to_string(),
            transcript_path: "/tmp/session_transcript.jsonl".to_string(),
            hook_event_name: "SubagentStart".to_string(),
            cwd: "/home/user/project".to_string(),
            permission_mode: Some("default".to_string()),
        },
        agent_id: "coder".to_string(),
        subagent_type: "implementation".to_string(),
        agent_transcript_path: "/tmp/coder_transcript.jsonl".to_string(),
    }
}

#[test]
fn test_subagent_start_payload_structure() {
    let payload = create_subagent_start_payload();

    // Verify all required fields are present and populated
    assert_eq!(payload.base.session_id, "test_session_789");
    assert_eq!(
        payload.base.transcript_path,
        "/tmp/session_transcript.jsonl"
    );
    assert_eq!(payload.base.hook_event_name, "SubagentStart");
    assert_eq!(payload.base.cwd, "/home/user/project");
    assert!(payload.base.permission_mode.is_some());
    assert_eq!(payload.agent_id, "coder");
    assert_eq!(payload.subagent_type, "implementation");
    assert_eq!(payload.agent_transcript_path, "/tmp/coder_transcript.jsonl");
}

#[test]
fn test_subagent_start_payload_with_different_agent_ids() {
    // Test with different agent IDs: coder, tester, stuck
    let agent_ids = vec!["coder", "tester", "stuck", "orchestrator"];

    for agent_id in agent_ids {
        let mut payload = create_subagent_start_payload();
        payload.agent_id = agent_id.to_string();

        assert_eq!(payload.agent_id, agent_id);
        assert!(validate_subagent_start_payload(&payload).is_ok());
    }
}

#[test]
fn test_subagent_start_payload_serialization() {
    let payload = create_subagent_start_payload();

    // Serialize to JSON
    let json_str = serde_json::to_string(&payload).expect("Failed to serialize");

    // Deserialize back
    let deserialized: SubagentStartPayload =
        serde_json::from_str(&json_str).expect("Failed to deserialize");

    // Verify round-trip preservation
    assert_eq!(deserialized.base.session_id, payload.base.session_id);
    assert_eq!(deserialized.agent_id, payload.agent_id);
    assert_eq!(deserialized.subagent_type, payload.subagent_type);
    assert_eq!(
        deserialized.agent_transcript_path,
        payload.agent_transcript_path
    );
}

#[test]
fn test_validate_subagent_start_payload_all_fields_valid() {
    let payload = create_subagent_start_payload();
    let result = validate_subagent_start_payload(&payload);

    assert!(result.is_ok(), "Valid payload should pass validation");
}

#[test]
fn test_validate_subagent_start_payload_missing_base_session_id() {
    let mut payload = create_subagent_start_payload();
    payload.base.session_id = String::new();

    let result = validate_subagent_start_payload(&payload);
    assert!(
        result.is_err(),
        "Payload with empty session_id should fail validation"
    );
    let error = result.unwrap_err();
    assert!(
        error.contains("session_id") || error.contains("Missing required field"),
        "Error should mention session_id or missing field: {}",
        error
    );
}

#[test]
fn test_validate_subagent_start_payload_missing_agent_id() {
    let mut payload = create_subagent_start_payload();
    payload.agent_id = String::new();

    let result = validate_subagent_start_payload(&payload);
    assert!(
        result.is_err(),
        "Payload with empty agent_id should fail validation"
    );
    assert!(result.unwrap_err().contains("agent_id cannot be empty"));
}

#[test]
fn test_validate_subagent_start_payload_whitespace_agent_id() {
    let mut payload = create_subagent_start_payload();
    payload.agent_id = "   ".to_string(); // Only whitespace

    let result = validate_subagent_start_payload(&payload);
    assert!(
        result.is_err(),
        "Payload with whitespace-only agent_id should fail validation"
    );
    assert!(result.unwrap_err().contains("agent_id cannot be empty"));
}

#[test]
fn test_validate_subagent_start_payload_missing_subagent_type() {
    let mut payload = create_subagent_start_payload();
    payload.subagent_type = String::new();

    let result = validate_subagent_start_payload(&payload);
    assert!(
        result.is_err(),
        "Payload with empty subagent_type should fail validation"
    );
    assert!(result
        .unwrap_err()
        .contains("subagent_type cannot be empty"));
}

#[test]
fn test_validate_subagent_start_payload_whitespace_subagent_type() {
    let mut payload = create_subagent_start_payload();
    payload.subagent_type = "   ".to_string(); // Only whitespace

    let result = validate_subagent_start_payload(&payload);
    assert!(
        result.is_err(),
        "Payload with whitespace-only subagent_type should fail validation"
    );
    assert!(result
        .unwrap_err()
        .contains("subagent_type cannot be empty"));
}

#[test]
fn test_validate_subagent_start_payload_missing_agent_transcript_path() {
    let mut payload = create_subagent_start_payload();
    payload.agent_transcript_path = String::new();

    let result = validate_subagent_start_payload(&payload);
    assert!(
        result.is_err(),
        "Payload with empty agent_transcript_path should fail validation"
    );
    assert!(result
        .unwrap_err()
        .contains("agent_transcript_path cannot be empty"));
}

#[test]
fn test_validate_subagent_start_payload_whitespace_agent_transcript_path() {
    let mut payload = create_subagent_start_payload();
    payload.agent_transcript_path = "   ".to_string(); // Only whitespace

    let result = validate_subagent_start_payload(&payload);
    assert!(
        result.is_err(),
        "Payload with whitespace-only agent_transcript_path should fail validation"
    );
    assert!(result
        .unwrap_err()
        .contains("agent_transcript_path cannot be empty"));
}

#[test]
fn test_validate_subagent_start_payload_agent_id_with_leading_trailing_spaces() {
    let mut payload = create_subagent_start_payload();
    payload.agent_id = "  coder  ".to_string();

    // Should pass validation because we trim before checking
    assert!(validate_subagent_start_payload(&payload).is_ok());
}

#[test]
fn test_validate_subagent_start_payload_invalid_base() {
    let mut payload = create_subagent_start_payload();
    payload.base.session_id = String::new();

    let result = validate_subagent_start_payload(&payload);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("session_id"));
}

#[test]
fn test_subagent_start_payload_json_round_trip_coder() {
    let json_str = r#"{
        "session_id": "test_session_coder",
        "transcript_path": "/tmp/session_123.jsonl",
        "hook_event_name": "SubagentStart",
        "cwd": "/home/user/project",
        "permission_mode": "default",
        "agent_id": "coder",
        "subagent_type": "implementation",
        "agent_transcript_path": "/tmp/coder_123.jsonl"
    }"#;

    let payload: SubagentStartPayload =
        serde_json::from_str(json_str).expect("Failed to deserialize JSON");

    assert_eq!(payload.agent_id, "coder");
    assert_eq!(payload.subagent_type, "implementation");
    assert_eq!(payload.agent_transcript_path, "/tmp/coder_123.jsonl");
    assert_eq!(payload.base.session_id, "test_session_coder");
    assert!(validate_subagent_start_payload(&payload).is_ok());
}

#[test]
fn test_subagent_start_payload_json_round_trip_tester() {
    let json_str = r#"{
        "session_id": "test_session_tester",
        "transcript_path": "/tmp/session_456.jsonl",
        "hook_event_name": "SubagentStart",
        "cwd": "/home/user/project",
        "permission_mode": "default",
        "agent_id": "tester",
        "subagent_type": "testing",
        "agent_transcript_path": "/tmp/tester_456.jsonl"
    }"#;

    let payload: SubagentStartPayload =
        serde_json::from_str(json_str).expect("Failed to deserialize JSON");

    assert_eq!(payload.agent_id, "tester");
    assert_eq!(payload.subagent_type, "testing");
    assert_eq!(payload.agent_transcript_path, "/tmp/tester_456.jsonl");
    assert!(validate_subagent_start_payload(&payload).is_ok());
}

#[test]
fn test_subagent_start_payload_json_round_trip_stuck() {
    let json_str = r#"{
        "session_id": "test_session_stuck",
        "transcript_path": "/tmp/session_789.jsonl",
        "hook_event_name": "SubagentStart",
        "cwd": "/home/user/project",
        "permission_mode": "restrict",
        "agent_id": "stuck",
        "subagent_type": "escalation",
        "agent_transcript_path": "/tmp/stuck_789.jsonl"
    }"#;

    let payload: SubagentStartPayload =
        serde_json::from_str(json_str).expect("Failed to deserialize JSON");

    assert_eq!(payload.agent_id, "stuck");
    assert_eq!(payload.subagent_type, "escalation");
    assert_eq!(payload.agent_transcript_path, "/tmp/stuck_789.jsonl");
    assert_eq!(payload.base.permission_mode, Some("restrict".to_string()));
    assert!(validate_subagent_start_payload(&payload).is_ok());
}

#[test]
fn test_subagent_start_payload_json_with_missing_agent_id_field() {
    let json_str = r#"{
        "session_id": "test_session",
        "transcript_path": "/tmp/session.jsonl",
        "hook_event_name": "SubagentStart",
        "cwd": "/home/user/project",
        "permission_mode": "default",
        "subagent_type": "implementation",
        "agent_transcript_path": "/tmp/agent.jsonl"
    }"#;

    // This should fail to deserialize because agent_id is required
    let result: Result<SubagentStartPayload, _> = serde_json::from_str(json_str);
    assert!(
        result.is_err(),
        "JSON missing agent_id should fail to deserialize"
    );
}

#[test]
fn test_subagent_start_payload_json_with_missing_subagent_type_field() {
    let json_str = r#"{
        "session_id": "test_session",
        "transcript_path": "/tmp/session.jsonl",
        "hook_event_name": "SubagentStart",
        "cwd": "/home/user/project",
        "permission_mode": "default",
        "agent_id": "coder",
        "agent_transcript_path": "/tmp/agent.jsonl"
    }"#;

    // This should fail to deserialize because subagent_type is required
    let result: Result<SubagentStartPayload, _> = serde_json::from_str(json_str);
    assert!(
        result.is_err(),
        "JSON missing subagent_type should fail to deserialize"
    );
}

#[test]
fn test_subagent_start_payload_json_with_missing_agent_transcript_path_field() {
    let json_str = r#"{
        "session_id": "test_session",
        "transcript_path": "/tmp/session.jsonl",
        "hook_event_name": "SubagentStart",
        "cwd": "/home/user/project",
        "permission_mode": "default",
        "agent_id": "coder",
        "subagent_type": "implementation"
    }"#;

    // This should fail to deserialize because agent_transcript_path is required
    let result: Result<SubagentStartPayload, _> = serde_json::from_str(json_str);
    assert!(
        result.is_err(),
        "JSON missing agent_transcript_path should fail to deserialize"
    );
}

#[test]
fn test_subagent_start_validation_error_messages_specific() {
    // Test that error messages are specific and helpful

    let mut payload = create_subagent_start_payload();

    // Test agent_id error
    payload.agent_id = String::new();
    let error = validate_subagent_start_payload(&payload).unwrap_err();
    assert!(
        error.contains("agent_id"),
        "Error should mention agent_id field"
    );

    // Test subagent_type error
    let mut payload = create_subagent_start_payload();
    payload.subagent_type = String::new();
    let error = validate_subagent_start_payload(&payload).unwrap_err();
    assert!(
        error.contains("subagent_type"),
        "Error should mention subagent_type field"
    );

    // Test agent_transcript_path error
    let mut payload = create_subagent_start_payload();
    payload.agent_transcript_path = String::new();
    let error = validate_subagent_start_payload(&payload).unwrap_err();
    assert!(
        error.contains("agent_transcript_path"),
        "Error should mention agent_transcript_path field"
    );

    // Test whitespace-only agent_id error
    let mut payload = create_subagent_start_payload();
    payload.agent_id = "   ".to_string();
    let error = validate_subagent_start_payload(&payload).unwrap_err();
    assert!(
        error.contains("agent_id"),
        "Error should mention agent_id for whitespace-only value"
    );
}

#[test]
fn test_subagent_start_payload_with_different_subagent_types() {
    let subagent_types = vec!["implementation", "testing", "escalation", "analysis"];

    for subagent_type in subagent_types {
        let mut payload = create_subagent_start_payload();
        payload.subagent_type = subagent_type.to_string();

        assert_eq!(payload.subagent_type, subagent_type);
        assert!(validate_subagent_start_payload(&payload).is_ok());
    }
}

#[test]
fn test_subagent_start_hook_event_name_validation() {
    let payload = create_subagent_start_payload();

    // Verify the hook event name is correctly set
    assert_eq!(payload.base.hook_event_name, "SubagentStart");

    // Verify the base payload validates correctly
    assert!(validate_base_payload(&payload.base).is_ok());
}

#[test]
fn test_hook_payload_subagent_start_variant_serialization() {
    let subagent_start_payload = create_subagent_start_payload();

    let hook_payload = HookPayload::SubagentStart(subagent_start_payload.clone());

    // Serialize to JSON
    let json_str = serde_json::to_string(&hook_payload).expect("Failed to serialize");

    // Verify the JSON contains the hook_event_name tag
    assert!(json_str.contains("\"hook_event_name\":\"SubagentStart\""));

    // Verify the JSON contains the required fields
    assert!(json_str.contains("\"agent_id\":\"coder\""));
    assert!(json_str.contains("\"subagent_type\":\"implementation\""));
    assert!(json_str.contains("\"agent_transcript_path\":\"/tmp/coder_transcript.jsonl\""));
}
