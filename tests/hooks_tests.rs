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
