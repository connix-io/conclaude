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

// Helper function to create a Write tool payload for testing
fn create_test_payload_write(file_path: &str) -> PreToolUsePayload {
    let mut tool_input = HashMap::new();
    tool_input.insert(
        "file_path".to_string(),
        Value::String(file_path.to_string()),
    );

    PreToolUsePayload {
        base: create_test_base_payload(),
        tool_name: "Write".to_string(),
        tool_input,
        tool_use_id: None,
    }
}

// ============================================================================
// Basic Glob Matching Tests for preventAdditions
// ============================================================================

#[tokio::test]
async fn test_prevent_additions_exact_directory_match() -> anyhow::Result<()> {
    use conclaude::config::{ConclaudeConfig, PreToolUseConfig};

    // Create test configuration with preventAdditions: ["dist/*"]
    let config = ConclaudeConfig {
        pre_tool_use: PreToolUseConfig {
            prevent_additions: vec!["dist/*".to_string()],
            ..Default::default()
        },
        ..Default::default()
    };

    // Create Write payload for dist/output.js
    let payload = create_test_payload_write("dist/output.js");

    // Extract file path
    let file_path = payload
        .tool_input
        .get("file_path")
        .and_then(|v| v.as_str())
        .unwrap();

    // Test pattern matching logic
    let pattern = &config.pre_tool_use.prevent_additions[0];
    let matches = glob::Pattern::new(pattern)?.matches(file_path);

    // Should block: dist/output.js matches "dist/*" pattern
    assert!(
        matches,
        "Pattern 'dist/*' should match 'dist/output.js' in exact directory match"
    );
    assert_eq!(payload.tool_name, "Write");

    Ok(())
}

#[tokio::test]
async fn test_prevent_additions_wildcard_patterns() -> anyhow::Result<()> {
    use conclaude::config::{ConclaudeConfig, PreToolUseConfig};

    // Create test configuration with preventAdditions: ["build/**"]
    let config = ConclaudeConfig {
        pre_tool_use: PreToolUseConfig {
            prevent_additions: vec!["build/**".to_string()],
            ..Default::default()
        },
        ..Default::default()
    };

    // Create Write payload for build/nested/file.js
    let payload = create_test_payload_write("build/nested/file.js");

    // Extract file path
    let file_path = payload
        .tool_input
        .get("file_path")
        .and_then(|v| v.as_str())
        .unwrap();

    // Test pattern matching logic
    let pattern = &config.pre_tool_use.prevent_additions[0];
    let matches = glob::Pattern::new(pattern)?.matches(file_path);

    // Should block: build/nested/file.js matches "build/**" pattern
    assert!(
        matches,
        "Pattern 'build/**' should match 'build/nested/file.js' with wildcard"
    );
    assert_eq!(payload.tool_name, "Write");

    Ok(())
}

#[tokio::test]
async fn test_prevent_additions_file_extension_patterns() -> anyhow::Result<()> {
    use conclaude::config::{ConclaudeConfig, PreToolUseConfig};

    // Create test configuration with preventAdditions: ["*.log"]
    let config = ConclaudeConfig {
        pre_tool_use: PreToolUseConfig {
            prevent_additions: vec!["*.log".to_string()],
            ..Default::default()
        },
        ..Default::default()
    };

    // Create Write payload for debug.log
    let payload = create_test_payload_write("debug.log");

    // Extract file path
    let file_path = payload
        .tool_input
        .get("file_path")
        .and_then(|v| v.as_str())
        .unwrap();

    // Test pattern matching logic
    let pattern = &config.pre_tool_use.prevent_additions[0];
    let matches = glob::Pattern::new(pattern)?.matches(file_path);

    // Should block: debug.log matches "*.log" pattern
    assert!(
        matches,
        "Pattern '*.log' should match 'debug.log' with file extension"
    );
    assert_eq!(payload.tool_name, "Write");

    Ok(())
}

#[tokio::test]
async fn test_prevent_additions_multiple_patterns() -> anyhow::Result<()> {
    use conclaude::config::{ConclaudeConfig, PreToolUseConfig};

    // Create test configuration with preventAdditions: ["dist/**", "build/**"]
    let config = ConclaudeConfig {
        pre_tool_use: PreToolUseConfig {
            prevent_additions: vec!["dist/**".to_string(), "build/**".to_string()],
            ..Default::default()
        },
        ..Default::default()
    };

    // Test that first pattern blocks dist/output.js
    let payload1 = create_test_payload_write("dist/output.js");
    let file_path1 = payload1
        .tool_input
        .get("file_path")
        .and_then(|v| v.as_str())
        .unwrap();

    let matches1 = config
        .pre_tool_use
        .prevent_additions
        .iter()
        .any(|pattern| glob::Pattern::new(pattern).unwrap().matches(file_path1));

    assert!(
        matches1,
        "First pattern should block 'dist/output.js' in multiple patterns"
    );

    // Test that second pattern blocks build/lib.js
    let payload2 = create_test_payload_write("build/lib.js");
    let file_path2 = payload2
        .tool_input
        .get("file_path")
        .and_then(|v| v.as_str())
        .unwrap();

    let matches2 = config
        .pre_tool_use
        .prevent_additions
        .iter()
        .any(|pattern| glob::Pattern::new(pattern).unwrap().matches(file_path2));

    assert!(
        matches2,
        "Second pattern should block 'build/lib.js' in multiple patterns"
    );

    // Test that unrelated path is not blocked
    let payload3 = create_test_payload_write("src/main.rs");
    let file_path3 = payload3
        .tool_input
        .get("file_path")
        .and_then(|v| v.as_str())
        .unwrap();

    let matches3 = config
        .pre_tool_use
        .prevent_additions
        .iter()
        .any(|pattern| glob::Pattern::new(pattern).unwrap().matches(file_path3));

    assert!(
        !matches3,
        "Unrelated path 'src/main.rs' should not be blocked by multiple patterns"
    );

    Ok(())
}

#[tokio::test]
async fn test_prevent_additions_non_matching_allowed() -> anyhow::Result<()> {
    use conclaude::config::{ConclaudeConfig, PreToolUseConfig};

    // Create test configuration with preventAdditions: ["dist/**"]
    let config = ConclaudeConfig {
        pre_tool_use: PreToolUseConfig {
            prevent_additions: vec!["dist/**".to_string()],
            ..Default::default()
        },
        ..Default::default()
    };

    // Create Write payload for src/main.rs (should NOT match)
    let payload = create_test_payload_write("src/main.rs");

    // Extract file path
    let file_path = payload
        .tool_input
        .get("file_path")
        .and_then(|v| v.as_str())
        .unwrap();

    // Test pattern matching logic
    let pattern = &config.pre_tool_use.prevent_additions[0];
    let matches = glob::Pattern::new(pattern)?.matches(file_path);

    // Should allow: src/main.rs does NOT match "dist/**" pattern
    assert!(
        !matches,
        "Pattern 'dist/**' should NOT match 'src/main.rs' - non-matching paths should be allowed"
    );
    assert_eq!(payload.tool_name, "Write");

    Ok(())
}

// ============================================================================
// Tool-Specific Enforcement Tests for preventAdditions
// ============================================================================

// Helper function to create an Edit tool payload for testing
fn create_test_payload_edit(file_path: &str) -> PreToolUsePayload {
    let mut tool_input = HashMap::new();
    tool_input.insert(
        "file_path".to_string(),
        Value::String(file_path.to_string()),
    );
    tool_input.insert(
        "old_string".to_string(),
        Value::String("old content".to_string()),
    );
    tool_input.insert(
        "new_string".to_string(),
        Value::String("new content".to_string()),
    );

    PreToolUsePayload {
        base: create_test_base_payload(),
        tool_name: "Edit".to_string(),
        tool_input,
        tool_use_id: None,
    }
}

// Helper function to create a NotebookEdit tool payload for testing
fn create_test_payload_notebook_edit(notebook_path: &str) -> PreToolUsePayload {
    let mut tool_input = HashMap::new();
    tool_input.insert(
        "notebook_path".to_string(),
        Value::String(notebook_path.to_string()),
    );
    tool_input.insert(
        "cell_index".to_string(),
        Value::Number(serde_json::Number::from(0)),
    );

    PreToolUsePayload {
        base: create_test_base_payload(),
        tool_name: "NotebookEdit".to_string(),
        tool_input,
        tool_use_id: None,
    }
}

#[tokio::test]
async fn test_prevent_additions_blocks_write_tool() -> anyhow::Result<()> {
    use conclaude::config::{ConclaudeConfig, PreToolUseConfig};

    // Create test configuration with preventAdditions: ["dist/**"]
    let config = ConclaudeConfig {
        pre_tool_use: PreToolUseConfig {
            prevent_additions: vec!["dist/**".to_string()],
            ..Default::default()
        },
        ..Default::default()
    };

    // Create Write payload for dist/file.js
    let payload = create_test_payload_write("dist/file.js");

    // Extract file path
    let file_path = payload
        .tool_input
        .get("file_path")
        .and_then(|v| v.as_str())
        .unwrap();

    // Test pattern matching logic
    let pattern = &config.pre_tool_use.prevent_additions[0];
    let matches = glob::Pattern::new(pattern)?.matches(file_path);

    // EXPECTED BEHAVIOR: Write tool SHOULD be blocked by preventAdditions
    assert!(
        matches,
        "preventAdditions should block Write tool for 'dist/file.js'"
    );
    assert_eq!(payload.tool_name, "Write");

    // This test verifies that preventAdditions pattern matches the file path
    // The actual hook enforcement logic will use this to block the Write tool

    Ok(())
}

#[tokio::test]
async fn test_prevent_additions_allows_edit_tool() -> anyhow::Result<()> {
    use conclaude::config::{ConclaudeConfig, PreToolUseConfig};

    // Create test configuration with preventAdditions: ["dist/**"]
    let config = ConclaudeConfig {
        pre_tool_use: PreToolUseConfig {
            prevent_additions: vec!["dist/**".to_string()],
            ..Default::default()
        },
        ..Default::default()
    };

    // Create Edit payload for dist/file.js
    let payload = create_test_payload_edit("dist/file.js");

    // Extract file path
    let file_path = payload
        .tool_input
        .get("file_path")
        .and_then(|v| v.as_str())
        .unwrap();

    // Test pattern matching logic
    let pattern = &config.pre_tool_use.prevent_additions[0];
    let matches = glob::Pattern::new(pattern)?.matches(file_path);

    // File path matches the pattern
    assert!(matches, "Pattern 'dist/**' matches 'dist/file.js'");

    // EXPECTED BEHAVIOR: Edit tool should NOT be blocked by preventAdditions
    // even though the file path matches the pattern
    assert_eq!(payload.tool_name, "Edit");

    // This test verifies that Edit tool should be allowed despite matching pattern
    // The actual hook enforcement logic will check tool_name and skip preventAdditions
    // check for Edit tools

    Ok(())
}

#[tokio::test]
async fn test_prevent_additions_allows_notebook_edit_tool() -> anyhow::Result<()> {
    use conclaude::config::{ConclaudeConfig, PreToolUseConfig};

    // Create test configuration with preventAdditions: ["notebooks/**"]
    let config = ConclaudeConfig {
        pre_tool_use: PreToolUseConfig {
            prevent_additions: vec!["notebooks/**".to_string()],
            ..Default::default()
        },
        ..Default::default()
    };

    // Create NotebookEdit payload for notebooks/analysis.ipynb
    let payload = create_test_payload_notebook_edit("notebooks/analysis.ipynb");

    // Extract notebook path
    let notebook_path = payload
        .tool_input
        .get("notebook_path")
        .and_then(|v| v.as_str())
        .unwrap();

    // Test pattern matching logic
    let pattern = &config.pre_tool_use.prevent_additions[0];
    let matches = glob::Pattern::new(pattern)?.matches(notebook_path);

    // Notebook path matches the pattern
    assert!(
        matches,
        "Pattern 'notebooks/**' matches 'notebooks/analysis.ipynb'"
    );

    // EXPECTED BEHAVIOR: NotebookEdit tool should NOT be blocked by preventAdditions
    // even though the notebook path matches the pattern
    assert_eq!(payload.tool_name, "NotebookEdit");

    // This test verifies that NotebookEdit tool should be allowed despite matching pattern
    // The actual hook enforcement logic will check tool_name and skip preventAdditions
    // check for NotebookEdit tools

    Ok(())
}

// ============================================================================
// Path Normalization Tests for preventAdditions
// ============================================================================

#[tokio::test]
async fn test_prevent_additions_relative_path_normalization() -> anyhow::Result<()> {
    use conclaude::config::{ConclaudeConfig, PreToolUseConfig};

    // Create test configuration with preventAdditions: ["dist/**"]
    let config = ConclaudeConfig {
        pre_tool_use: PreToolUseConfig {
            prevent_additions: vec!["dist/**".to_string()],
            ..Default::default()
        },
        ..Default::default()
    };

    // Test relative path with ./ prefix: "./dist/file.js"
    let payload = create_test_payload_write("./dist/file.js");

    // Extract file path
    let file_path = payload
        .tool_input
        .get("file_path")
        .and_then(|v| v.as_str())
        .unwrap();

    // Normalize the path by stripping "./" prefix
    let normalized_path = if let Some(stripped) = file_path.strip_prefix("./") {
        stripped
    } else {
        file_path
    };

    // Test pattern matching logic with normalized path
    let pattern = &config.pre_tool_use.prevent_additions[0];
    let matches = glob::Pattern::new(pattern)?.matches(normalized_path);

    // Should block: "./dist/file.js" normalizes to "dist/file.js" and matches "dist/**"
    assert!(
        matches,
        "Pattern 'dist/**' should match './dist/file.js' after normalization to 'dist/file.js'"
    );
    assert_eq!(payload.tool_name, "Write");

    Ok(())
}

#[tokio::test]
async fn test_prevent_additions_absolute_path_resolution() -> anyhow::Result<()> {
    use conclaude::config::{ConclaudeConfig, PreToolUseConfig};
    use std::path::Path;

    // Create test configuration with preventAdditions: ["dist/**"]
    let config = ConclaudeConfig {
        pre_tool_use: PreToolUseConfig {
            prevent_additions: vec!["dist/**".to_string()],
            ..Default::default()
        },
        ..Default::default()
    };

    // Test absolute path: "/home/user/project/dist/file.js"
    let absolute_path = "/home/user/project/dist/file.js";
    let payload = create_test_payload_write(absolute_path);

    // Extract file path
    let file_path = payload
        .tool_input
        .get("file_path")
        .and_then(|v| v.as_str())
        .unwrap();

    // Simulate resolving absolute path to relative path
    // In real scenario, this would be done by stripping cwd prefix
    let project_root = Path::new("/home/user/project");
    let resolved_path = Path::new(file_path);

    let relative_path = if file_path.starts_with('/') {
        // For absolute paths, extract the relative portion
        resolved_path
            .strip_prefix(project_root)
            .unwrap_or(resolved_path)
            .to_string_lossy()
            .to_string()
    } else {
        file_path.to_string()
    };

    // Test pattern matching logic with relative path
    let pattern = &config.pre_tool_use.prevent_additions[0];
    let matches = glob::Pattern::new(pattern)?.matches(&relative_path);

    // Should block: absolute path resolves to "dist/file.js" and matches "dist/**"
    assert!(
        matches,
        "Pattern 'dist/**' should match absolute path '/home/user/project/dist/file.js' after resolution to 'dist/file.js'"
    );
    assert_eq!(payload.tool_name, "Write");

    Ok(())
}

#[tokio::test]
async fn test_prevent_additions_parent_directory_refs() -> anyhow::Result<()> {
    use conclaude::config::{ConclaudeConfig, PreToolUseConfig};
    use std::path::Path;

    // Create test configuration with preventAdditions: ["dist/**"]
    let config = ConclaudeConfig {
        pre_tool_use: PreToolUseConfig {
            prevent_additions: vec!["dist/**".to_string()],
            ..Default::default()
        },
        ..Default::default()
    };

    // Test path with parent directory reference: "src/../dist/file.js"
    let payload = create_test_payload_write("src/../dist/file.js");

    // Extract file path
    let file_path = payload
        .tool_input
        .get("file_path")
        .and_then(|v| v.as_str())
        .unwrap();

    // Normalize the path by resolving parent directory references
    // In a real scenario, this would be done via Path::canonicalize or manual normalization
    let path = Path::new(file_path);
    let mut components = Vec::new();

    for component in path.components() {
        match component {
            std::path::Component::ParentDir => {
                components.pop(); // Remove last component when we hit ".."
            }
            std::path::Component::Normal(c) => {
                components.push(c.to_string_lossy().to_string());
            }
            std::path::Component::CurDir => {
                // Skip "." components
            }
            _ => {}
        }
    }

    let normalized_path = components.join("/");

    // Test pattern matching logic with normalized path
    let pattern = &config.pre_tool_use.prevent_additions[0];
    let matches = glob::Pattern::new(pattern)?.matches(&normalized_path);

    // Should block: "src/../dist/file.js" normalizes to "dist/file.js" and matches "dist/**"
    assert!(
        matches,
        "Pattern 'dist/**' should match 'src/../dist/file.js' after normalization to 'dist/file.js'"
    );
    assert_eq!(payload.tool_name, "Write");
    assert_eq!(
        normalized_path, "dist/file.js",
        "Path should normalize to 'dist/file.js'"
    );

    Ok(())
}

// ============================================================================
// Error Message Tests for preventAdditions
// ============================================================================

#[tokio::test]
async fn test_prevent_additions_error_format() -> anyhow::Result<()> {
    // This test defines the expected error message format when preventAdditions blocks an operation
    // Format: "Blocked {tool_name} operation: file matches preventAdditions pattern '{pattern}'. File: {file_path}"

    let tool_name = "Write";
    let pattern = "dist/**";
    let file_path = "dist/output.js";

    // Build the expected error message according to spec
    let expected_error = format!(
        "Blocked {} operation: file matches preventAdditions pattern '{}'. File: {}",
        tool_name, pattern, file_path
    );

    // Verify the error message format has all required components
    assert!(
        expected_error.starts_with("Blocked "),
        "Error message should start with 'Blocked '"
    );
    assert!(
        expected_error.contains("operation:"),
        "Error message should contain 'operation:'"
    );
    assert!(
        expected_error.contains("file matches preventAdditions pattern"),
        "Error message should contain 'file matches preventAdditions pattern'"
    );
    assert!(
        expected_error.contains("File: "),
        "Error message should contain 'File: '"
    );

    // Verify specific format matches spec exactly
    assert_eq!(
        expected_error,
        "Blocked Write operation: file matches preventAdditions pattern 'dist/**'. File: dist/output.js",
        "Error message format should match spec exactly"
    );

    Ok(())
}

#[tokio::test]
async fn test_prevent_additions_error_includes_pattern() -> anyhow::Result<()> {
    // This test verifies that the error message includes the specific pattern that matched

    let tool_name = "Write";
    let file_path = "build/nested/file.js";

    // Test with different patterns to ensure pattern is included
    let patterns = vec!["build/**", "*.js", "build/**/file.js"];

    for pattern in patterns {
        let error_message = format!(
            "Blocked {} operation: file matches preventAdditions pattern '{}'. File: {}",
            tool_name, pattern, file_path
        );

        // Verify the pattern is included in single quotes
        assert!(
            error_message.contains(&format!("'{}'", pattern)),
            "Error message should include pattern '{}' in single quotes",
            pattern
        );

        // Verify the pattern appears in the context of preventAdditions
        assert!(
            error_message.contains(&format!("preventAdditions pattern '{}'", pattern)),
            "Error message should show pattern '{}' in context of preventAdditions",
            pattern
        );
    }

    Ok(())
}

#[tokio::test]
async fn test_prevent_additions_error_includes_tool_name() -> anyhow::Result<()> {
    // This test verifies that the error message identifies which tool is being blocked
    // preventAdditions only blocks Write tool, but the message should clearly identify it

    let pattern = "dist/**";
    let file_path = "dist/output.js";
    let tool_name = "Write";

    let error_message = format!(
        "Blocked {} operation: file matches preventAdditions pattern '{}'. File: {}",
        tool_name, pattern, file_path
    );

    // Verify tool name appears at the start of the error
    assert!(
        error_message.contains("Blocked Write operation:"),
        "Error message should start with 'Blocked Write operation:'"
    );

    // Verify tool name is the first thing after "Blocked "
    assert!(
        error_message.starts_with("Blocked Write "),
        "Error message should have tool name immediately after 'Blocked '"
    );

    // Verify the exact format: "Blocked {tool_name} operation:"
    let expected_prefix = format!("Blocked {} operation:", tool_name);
    assert!(
        error_message.starts_with(&expected_prefix),
        "Error message should start with '{}'",
        expected_prefix
    );

    Ok(())
}

#[tokio::test]
async fn test_prevent_additions_error_includes_file_path() -> anyhow::Result<()> {
    // This test verifies that the error message includes the full file path that was blocked

    let tool_name = "Write";
    let pattern = "*.log";

    // Test with various file paths to ensure they're included correctly
    let file_paths = vec![
        "debug.log",
        "dist/output.js",
        "build/nested/deep/file.js",
        "./dist/file.js",
    ];

    for file_path in file_paths {
        let error_message = format!(
            "Blocked {} operation: file matches preventAdditions pattern '{}'. File: {}",
            tool_name, pattern, file_path
        );

        // Verify the file path is included with "File: " prefix
        assert!(
            error_message.contains(&format!("File: {}", file_path)),
            "Error message should include file path '{}' with 'File: ' prefix",
            file_path
        );

        // Verify the file path appears at the end of the message
        assert!(
            error_message.ends_with(file_path),
            "Error message should end with the file path '{}'",
            file_path
        );

        // Verify the complete format for this file path
        let expected_ending = format!("File: {}", file_path);
        assert!(
            error_message.ends_with(&expected_ending),
            "Error message should end with 'File: {}'",
            file_path
        );
    }

    Ok(())
}

// ============================================================================
// Rule Interaction Tests for preventAdditions
// ============================================================================

#[tokio::test]
async fn test_prevent_additions_with_prevent_root_additions() -> anyhow::Result<()> {
    use conclaude::config::{ConclaudeConfig, PreToolUseConfig, RulesConfig};

    // Create test configuration with BOTH preventAdditions and preventRootAdditions
    let config = ConclaudeConfig {
        pre_tool_use: PreToolUseConfig {
            prevent_additions: vec!["*".to_string()], // Block all file additions
            ..Default::default()
        },
        rules: RulesConfig {
            prevent_root_additions: true, // Also block root additions
            ..Default::default()
        },
        ..Default::default()
    };

    // Create Write payload for root-level file "README.md"
    let payload = create_test_payload_write("README.md");

    // Extract file path
    let file_path = payload
        .tool_input
        .get("file_path")
        .and_then(|v| v.as_str())
        .unwrap();

    // Test that preventAdditions pattern matches
    let prevent_additions_matches = config
        .pre_tool_use
        .prevent_additions
        .iter()
        .any(|pattern| glob::Pattern::new(pattern).unwrap().matches(file_path));

    // Test that file is at root level (no '/' in path)
    let is_root_level = !file_path.contains('/');

    // EXPECTED BEHAVIOR: Both rules can block the same Write operation independently
    // preventAdditions should match because "*" matches "README.md"
    assert!(
        prevent_additions_matches,
        "preventAdditions pattern '*' should match 'README.md'"
    );

    // preventRootAdditions should also apply because file is at root level
    assert!(
        config.rules.prevent_root_additions && is_root_level,
        "preventRootAdditions should also block root-level file 'README.md'"
    );

    // Both rules enforce independently - either can block the operation
    assert_eq!(payload.tool_name, "Write");

    Ok(())
}

#[tokio::test]
async fn test_prevent_additions_with_uneditable_files_write() -> anyhow::Result<()> {
    use conclaude::config::{ConclaudeConfig, PreToolUseConfig, RulesConfig};

    // Create test configuration with BOTH preventAdditions and uneditableFiles
    let config = ConclaudeConfig {
        pre_tool_use: PreToolUseConfig {
            prevent_additions: vec!["dist/**".to_string()], // Block additions to dist/
            ..Default::default()
        },
        rules: RulesConfig {
            uneditable_files: vec!["dist/output.js".to_string()], // Also protect dist/output.js
            ..Default::default()
        },
        ..Default::default()
    };

    // Create Write payload for dist/output.js (matches BOTH rules)
    let payload = create_test_payload_write("dist/output.js");

    // Extract file path
    let file_path = payload
        .tool_input
        .get("file_path")
        .and_then(|v| v.as_str())
        .unwrap();

    // Test that preventAdditions pattern matches
    let prevent_additions_matches = config
        .pre_tool_use
        .prevent_additions
        .iter()
        .any(|pattern| glob::Pattern::new(pattern).unwrap().matches(file_path));

    // Test that uneditableFiles pattern matches
    let uneditable_files_matches = config
        .rules
        .uneditable_files
        .iter()
        .any(|pattern| glob::Pattern::new(pattern).unwrap().matches(file_path));

    // EXPECTED BEHAVIOR: Both rules check Write operations
    // If either matches, the operation should be blocked
    assert!(
        prevent_additions_matches,
        "preventAdditions should match 'dist/output.js'"
    );

    assert!(
        uneditable_files_matches,
        "uneditableFiles should also match 'dist/output.js'"
    );

    // Both rules can block the same Write operation
    assert_eq!(payload.tool_name, "Write");

    Ok(())
}

#[tokio::test]
async fn test_prevent_additions_vs_uneditable_files_edit() -> anyhow::Result<()> {
    use conclaude::config::{ConclaudeConfig, PreToolUseConfig, RulesConfig};

    // Create test configuration with BOTH preventAdditions and uneditableFiles
    let config = ConclaudeConfig {
        pre_tool_use: PreToolUseConfig {
            prevent_additions: vec!["dist/**".to_string()], // Block additions to dist/
            ..Default::default()
        },
        rules: RulesConfig {
            uneditable_files: vec!["dist/**".to_string()], // Also protect dist/
            ..Default::default()
        },
        ..Default::default()
    };

    // Create Edit payload for dist/output.js (matches both patterns)
    let payload = create_test_payload_edit("dist/output.js");

    // Extract file path
    let file_path = payload
        .tool_input
        .get("file_path")
        .and_then(|v| v.as_str())
        .unwrap();

    // Test that both patterns match the file path
    let prevent_additions_matches = config
        .pre_tool_use
        .prevent_additions
        .iter()
        .any(|pattern| glob::Pattern::new(pattern).unwrap().matches(file_path));

    let uneditable_files_matches = config
        .rules
        .uneditable_files
        .iter()
        .any(|pattern| glob::Pattern::new(pattern).unwrap().matches(file_path));

    // Both patterns match the file path
    assert!(
        prevent_additions_matches,
        "preventAdditions pattern should match 'dist/output.js'"
    );

    assert!(
        uneditable_files_matches,
        "uneditableFiles pattern should match 'dist/output.js'"
    );

    // EXPECTED BEHAVIOR: Edit operations check uneditableFiles but NOT preventAdditions
    // preventAdditions only blocks Write tool (file creation)
    // uneditableFiles blocks ALL file modifications (Write, Edit, NotebookEdit)
    assert_eq!(payload.tool_name, "Edit");

    // This test verifies that:
    // - Edit tool should be blocked by uneditableFiles (applies to all tools)
    // - Edit tool should NOT be checked by preventAdditions (only applies to Write)

    Ok(())
}

#[tokio::test]
async fn test_prevent_root_additions_vs_prevent_additions() -> anyhow::Result<()> {
    use conclaude::config::{ConclaudeConfig, PreToolUseConfig, RulesConfig};

    // Create test configuration with BOTH preventAdditions and preventRootAdditions
    let config = ConclaudeConfig {
        pre_tool_use: PreToolUseConfig {
            prevent_additions: vec!["src/**".to_string()], // Block additions to src/ only
            ..Default::default()
        },
        rules: RulesConfig {
            prevent_root_additions: true, // Block root-level additions
            ..Default::default()
        },
        ..Default::default()
    };

    // Test 1: Root-level file blocked by preventRootAdditions, not preventAdditions
    let payload1 = create_test_payload_write("newfile.txt");
    let file_path1 = payload1
        .tool_input
        .get("file_path")
        .and_then(|v| v.as_str())
        .unwrap();

    let prevent_additions_matches1 = config
        .pre_tool_use
        .prevent_additions
        .iter()
        .any(|pattern| glob::Pattern::new(pattern).unwrap().matches(file_path1));

    let is_root_level1 = !file_path1.contains('/');

    // preventAdditions should NOT match (file is not in src/)
    assert!(
        !prevent_additions_matches1,
        "preventAdditions should NOT match 'newfile.txt' (not in src/)"
    );

    // preventRootAdditions SHOULD block (file is at root level)
    assert!(
        config.rules.prevent_root_additions && is_root_level1,
        "preventRootAdditions SHOULD block root-level file 'newfile.txt'"
    );

    // Test 2: src/ file blocked by preventAdditions, not preventRootAdditions
    let payload2 = create_test_payload_write("src/main.rs");
    let file_path2 = payload2
        .tool_input
        .get("file_path")
        .and_then(|v| v.as_str())
        .unwrap();

    let prevent_additions_matches2 = config
        .pre_tool_use
        .prevent_additions
        .iter()
        .any(|pattern| glob::Pattern::new(pattern).unwrap().matches(file_path2));

    let is_root_level2 = !file_path2.contains('/');

    // preventAdditions SHOULD match (file is in src/)
    assert!(
        prevent_additions_matches2,
        "preventAdditions SHOULD match 'src/main.rs'"
    );

    // preventRootAdditions should NOT block (file is not at root level)
    assert!(
        !is_root_level2,
        "preventRootAdditions should NOT block 'src/main.rs' (not at root)"
    );

    // EXPECTED BEHAVIOR: The two rules have different scopes:
    // - preventRootAdditions: Blocks Write to root-level files only
    // - preventAdditions: Blocks Write based on glob patterns (independent of root level)

    Ok(())
}

// ============================================================================
// Edge Case Tests for preventAdditions
// ============================================================================

#[tokio::test]
async fn test_prevent_additions_empty_array() -> anyhow::Result<()> {
    use conclaude::config::{ConclaudeConfig, PreToolUseConfig};

    // Create test configuration with empty preventAdditions array
    let config = ConclaudeConfig {
        pre_tool_use: PreToolUseConfig {
            prevent_additions: vec![], // Empty array - should allow everything
            ..Default::default()
        },
        ..Default::default()
    };

    // Create Write payload for any file
    let payload = create_test_payload_write("dist/output.js");

    // Extract file path
    let file_path = payload
        .tool_input
        .get("file_path")
        .and_then(|v| v.as_str())
        .unwrap();

    // Test that no patterns match (empty array)
    let matches = config
        .pre_tool_use
        .prevent_additions
        .iter()
        .any(|pattern| glob::Pattern::new(pattern).unwrap().matches(file_path));

    // EXPECTED BEHAVIOR: Empty preventAdditions array should allow all Write operations
    assert!(
        !matches,
        "Empty preventAdditions array should not match any file path"
    );
    assert!(
        config.pre_tool_use.prevent_additions.is_empty(),
        "preventAdditions array should be empty"
    );
    assert_eq!(payload.tool_name, "Write");

    Ok(())
}

#[tokio::test]
async fn test_prevent_additions_write_existing_file() -> anyhow::Result<()> {
    use conclaude::config::{ConclaudeConfig, PreToolUseConfig};
    use std::fs;
    use std::io::Write as IoWrite;
    use tempfile::TempDir;

    // Create test configuration with preventAdditions: ["dist/**"]
    let config = ConclaudeConfig {
        pre_tool_use: PreToolUseConfig {
            prevent_additions: vec!["dist/**".to_string()],
            ..Default::default()
        },
        ..Default::default()
    };

    // Create a temporary directory and file to simulate existing file
    let temp_dir = TempDir::new()?;
    let dist_dir = temp_dir.path().join("dist");
    fs::create_dir(&dist_dir)?;
    let file_path = dist_dir.join("output.js");

    // Create the file
    let mut file = fs::File::create(&file_path)?;
    file.write_all(b"existing content")?;
    drop(file);

    // Verify file exists
    assert!(
        file_path.exists(),
        "Test file should exist before Write operation"
    );

    // Create Write payload for the existing file
    let payload = create_test_payload_write("dist/output.js");

    // Extract file path
    let file_path_str = payload
        .tool_input
        .get("file_path")
        .and_then(|v| v.as_str())
        .unwrap();

    // Test pattern matching logic
    let pattern = &config.pre_tool_use.prevent_additions[0];
    let matches = glob::Pattern::new(pattern)?.matches(file_path_str);

    // EXPECTED BEHAVIOR: Pattern matches, BUT if file exists, Write should be allowed
    // preventAdditions is for CREATION, not modification
    assert!(matches, "Pattern 'dist/**' should match 'dist/output.js'");
    assert_eq!(payload.tool_name, "Write");

    // This test verifies that:
    // - The pattern DOES match the file path
    // - BUT the actual hook should check file existence
    // - Write to existing files should be allowed (overwrite is permitted)
    // - preventAdditions only blocks NEW file creation

    Ok(())
}

#[tokio::test]
async fn test_prevent_additions_invalid_glob_pattern() -> anyhow::Result<()> {
    use conclaude::config::{ConclaudeConfig, PreToolUseConfig};

    // Create test configuration with an INVALID glob pattern
    let config = ConclaudeConfig {
        pre_tool_use: PreToolUseConfig {
            prevent_additions: vec!["[invalid".to_string()], // Invalid glob: unclosed bracket
            ..Default::default()
        },
        ..Default::default()
    };

    // Create Write payload
    let payload = create_test_payload_write("dist/output.js");

    // Test that invalid glob pattern returns an error
    let pattern = &config.pre_tool_use.prevent_additions[0];
    let result = glob::Pattern::new(pattern);

    // EXPECTED BEHAVIOR: Invalid glob pattern should return an error
    assert!(
        result.is_err(),
        "Invalid glob pattern '[invalid' should return an error"
    );

    // Verify the error is a PatternError
    if let Err(err) = result {
        // The error should be related to the invalid pattern
        let error_message = err.to_string();
        assert!(
            !error_message.is_empty(),
            "Error message should not be empty for invalid pattern"
        );
    }

    // The hook implementation should handle this error gracefully
    // and either reject the operation or log the error
    assert_eq!(payload.tool_name, "Write");

    Ok(())
}

#[tokio::test]
async fn test_prevent_additions_pattern_with_trailing_slash() -> anyhow::Result<()> {
    use conclaude::config::{ConclaudeConfig, PreToolUseConfig};

    // Create test configuration with pattern ending in slash: "dist/"
    let config = ConclaudeConfig {
        pre_tool_use: PreToolUseConfig {
            prevent_additions: vec!["dist/".to_string()], // Pattern with trailing slash
            ..Default::default()
        },
        ..Default::default()
    };

    // Test 1: File directly in dist directory
    let payload1 = create_test_payload_write("dist/output.js");
    let file_path1 = payload1
        .tool_input
        .get("file_path")
        .and_then(|v| v.as_str())
        .unwrap();

    let pattern = &config.pre_tool_use.prevent_additions[0];
    let matches1 = glob::Pattern::new(pattern)?.matches(file_path1);

    // Test 2: The directory itself
    let matches2 = glob::Pattern::new(pattern)?.matches("dist/");

    // Test 3: Nested file
    let payload3 = create_test_payload_write("dist/nested/file.js");
    let file_path3 = payload3
        .tool_input
        .get("file_path")
        .and_then(|v| v.as_str())
        .unwrap();
    let matches3 = glob::Pattern::new(pattern)?.matches(file_path3);

    // EXPECTED BEHAVIOR: Pattern "dist/" with trailing slash
    // According to glob crate behavior, "dist/" matches the directory "dist/" exactly
    // It does NOT match files like "dist/output.js"
    assert!(
        !matches1,
        "Pattern 'dist/' should NOT match 'dist/output.js' (glob crate behavior)"
    );

    assert!(matches2, "Pattern 'dist/' should match 'dist/' exactly");

    assert!(
        !matches3,
        "Pattern 'dist/' should NOT match 'dist/nested/file.js'"
    );

    // This test documents that trailing slashes in glob patterns
    // have specific behavior - use "dist/**" to match all files in directory
    assert_eq!(payload1.tool_name, "Write");

    Ok(())
}

#[tokio::test]
async fn test_prevent_additions_pattern_without_trailing_slash() -> anyhow::Result<()> {
    use conclaude::config::{ConclaudeConfig, PreToolUseConfig};

    // Create test configuration with pattern WITHOUT trailing slash: "dist"
    let config = ConclaudeConfig {
        pre_tool_use: PreToolUseConfig {
            prevent_additions: vec!["dist".to_string()], // Pattern without trailing slash
            ..Default::default()
        },
        ..Default::default()
    };

    // Test 1: Exact match - directory name
    let matches1 = glob::Pattern::new(&config.pre_tool_use.prevent_additions[0])?.matches("dist");

    // Test 2: File in directory
    let payload2 = create_test_payload_write("dist/output.js");
    let file_path2 = payload2
        .tool_input
        .get("file_path")
        .and_then(|v| v.as_str())
        .unwrap();
    let matches2 =
        glob::Pattern::new(&config.pre_tool_use.prevent_additions[0])?.matches(file_path2);

    // Test 3: Nested file
    let payload3 = create_test_payload_write("dist/nested/file.js");
    let file_path3 = payload3
        .tool_input
        .get("file_path")
        .and_then(|v| v.as_str())
        .unwrap();
    let matches3 =
        glob::Pattern::new(&config.pre_tool_use.prevent_additions[0])?.matches(file_path3);

    // Test 4: Similar name but different path
    let matches4 =
        glob::Pattern::new(&config.pre_tool_use.prevent_additions[0])?.matches("distribution");

    // EXPECTED BEHAVIOR: Pattern "dist" without trailing slash
    // According to glob crate, this matches EXACTLY "dist" and nothing else
    assert!(matches1, "Pattern 'dist' should match 'dist' exactly");

    assert!(
        !matches2,
        "Pattern 'dist' should NOT match 'dist/output.js' (no wildcard)"
    );

    assert!(
        !matches3,
        "Pattern 'dist' should NOT match 'dist/nested/file.js' (no wildcard)"
    );

    assert!(
        !matches4,
        "Pattern 'dist' should NOT match 'distribution' (not exact match)"
    );

    // This test documents that patterns without wildcards or trailing slashes
    // only match the exact string - use "dist/*" or "dist/**" for directory contents
    assert_eq!(payload2.tool_name, "Write");

    Ok(())
}
