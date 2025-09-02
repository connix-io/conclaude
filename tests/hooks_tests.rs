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
    // File directly in root directory
    assert!(is_root_addition("", "test.txt"));
    assert!(is_root_addition("", "script.sh"));
    assert!(is_root_addition("", "data.json"));
}

#[test]
fn test_is_root_addition_false_cases() {
    // Dotfiles should be allowed
    assert!(!is_root_addition("", ".gitignore"));
    assert!(!is_root_addition("", ".env"));

    // Config files should be allowed
    assert!(!is_root_addition("", "package.json"));
    assert!(!is_root_addition("", "tsconfig.json"));
    assert!(!is_root_addition("", "config.yaml"));
    assert!(!is_root_addition("", "settings.ini"));
    assert!(!is_root_addition("", "bun.lockb"));
    assert!(!is_root_addition("", "bun.lock"));

    // Files in subdirectories should be allowed
    assert!(!is_root_addition("", "src/test.txt"));
    assert!(!is_root_addition("", "docs/readme.md"));
    assert!(!is_root_addition("", "tests/unit.rs"));

    // Edge cases
    assert!(!is_root_addition("", ""));
    assert!(!is_root_addition("", ".."));
}

#[test]
fn test_matches_uneditable_pattern() {
    // Exact file matches
    assert!(
        matches_uneditable_pattern(
            "package.json",
            "package.json",
            "/path/package.json",
            "package.json"
        )
        .unwrap()
    );

    // Wildcard matches
    assert!(matches_uneditable_pattern("test.md", "test.md", "/path/test.md", "*.md").unwrap());
    assert!(
        matches_uneditable_pattern("README.md", "README.md", "/path/README.md", "*.md").unwrap()
    );

    // Directory pattern matches
    assert!(
        matches_uneditable_pattern(
            "src/index.ts",
            "src/index.ts",
            "/path/src/index.ts",
            "src/**/*.ts"
        )
        .unwrap()
    );
    assert!(
        matches_uneditable_pattern(
            "src/lib/utils.ts",
            "src/lib/utils.ts",
            "/path/src/lib/utils.ts",
            "src/**/*.ts"
        )
        .unwrap()
    );

    // Negative matches
    assert!(
        !matches_uneditable_pattern("other.txt", "other.txt", "/path/other.txt", "*.md").unwrap()
    );
    assert!(
        !matches_uneditable_pattern(
            "lib/index.ts",
            "lib/index.ts",
            "/path/lib/index.ts",
            "src/**/*.ts"
        )
        .unwrap()
    );
}

#[test]
fn test_matches_uneditable_pattern_invalid_glob() {
    let result = matches_uneditable_pattern("test.txt", "test.txt", "/path/test.txt", "[invalid");
    assert!(result.is_err());
}

#[test]
fn test_matches_uneditable_pattern_multiple_patterns() {
    // Test multiple patterns separately (since the glob crate doesn't support brace expansion)
    assert!(
        matches_uneditable_pattern(
            "package.json",
            "package.json",
            "/path/package.json",
            "package.json"
        )
        .unwrap()
    );
    assert!(
        matches_uneditable_pattern(
            "tsconfig.json",
            "tsconfig.json",
            "/path/tsconfig.json",
            "tsconfig.json"
        )
        .unwrap()
    );
    assert!(
        !matches_uneditable_pattern(
            "other.json",
            "other.json",
            "/path/other.json",
            "package.json"
        )
        .unwrap()
    );
}

#[test]
fn test_matches_uneditable_pattern_environment_files() {
    assert!(matches_uneditable_pattern(".env", ".env", "/path/.env", ".env*").unwrap());
    assert!(
        matches_uneditable_pattern(".env.local", ".env.local", "/path/.env.local", ".env*")
            .unwrap()
    );
    assert!(
        matches_uneditable_pattern(
            ".env.production",
            ".env.production",
            "/path/.env.production",
            ".env*"
        )
        .unwrap()
    );
    assert!(
        !matches_uneditable_pattern(
            "environment.txt",
            "environment.txt",
            "/path/environment.txt",
            ".env*"
        )
        .unwrap()
    );
}

#[test]
fn test_matches_uneditable_pattern_directory_patterns() {
    // Match entire directories
    assert!(
        matches_uneditable_pattern(
            "docs/README.md",
            "docs/README.md",
            "/path/docs/README.md",
            "docs/**"
        )
        .unwrap()
    );
    assert!(
        matches_uneditable_pattern(
            "docs/api/index.md",
            "docs/api/index.md",
            "/path/docs/api/index.md",
            "docs/**"
        )
        .unwrap()
    );
    assert!(
        !matches_uneditable_pattern("src/docs.ts", "src/docs.ts", "/path/src/docs.ts", "docs/**")
            .unwrap()
    );
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
    };
    assert!(validate_base_payload(&invalid_base).is_err());
}
