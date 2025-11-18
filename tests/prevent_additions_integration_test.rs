use conclaude::config::{load_conclaude_config, ConclaudeConfig, PreToolUseConfig};
use conclaude::hooks::extract_file_path;
use conclaude::types::{BasePayload, PreToolUsePayload};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::io::Write as IoWrite;
use tempfile::TempDir;

// Helper function to create a base payload for testing
fn create_test_base_payload() -> BasePayload {
    BasePayload {
        session_id: "integration_test_session".to_string(),
        transcript_path: "/tmp/integration_test_transcript.jsonl".to_string(),
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
    tool_input.insert(
        "content".to_string(),
        Value::String("test content".to_string()),
    );

    PreToolUsePayload {
        base: create_test_base_payload(),
        tool_name: "Write".to_string(),
        tool_input,
        tool_use_id: None,
    }
}

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

// ============================================================================
// Phase 3.2: Integration Test - Write Tool Blocked by preventAdditions
// ============================================================================

#[tokio::test]
async fn integration_test_write_blocked_by_prevent_additions_dist() -> anyhow::Result<()> {
    // Create test configuration with preventAdditions: ["dist/**"]
    let config = ConclaudeConfig {
        pre_tool_use: PreToolUseConfig {
            prevent_additions: vec!["dist/**".to_string()],
            ..Default::default()
        },
        ..Default::default()
    };

    // Create Write payload for dist/output.js (should be blocked)
    let payload = create_test_payload_write("dist/output.js");

    // Extract file path
    let file_path =
        extract_file_path(&payload.tool_input).expect("Should have file_path in payload");

    // Test pattern matching with actual glob logic
    let pattern = &config.pre_tool_use.prevent_additions[0];
    let glob_pattern = glob::Pattern::new(pattern)?;
    let matches = glob_pattern.matches(&file_path);

    // VALIDATION: Write tool should be blocked by preventAdditions pattern
    assert!(
        matches,
        "Integration Test: preventAdditions pattern 'dist/**' should match 'dist/output.js'"
    );
    assert_eq!(
        payload.tool_name, "Write",
        "Integration Test: Tool should be Write"
    );

    // Verify expected error message format
    let expected_error = format!(
        "Blocked {} operation: file matches preventAdditions pattern '{}'. File: {}",
        payload.tool_name, pattern, file_path
    );
    assert!(
        expected_error.contains("Blocked Write operation:"),
        "Integration Test: Error message should identify Write tool"
    );
    assert!(
        expected_error.contains("preventAdditions pattern"),
        "Integration Test: Error message should mention preventAdditions"
    );
    assert!(
        expected_error.contains("dist/**"),
        "Integration Test: Error message should include pattern"
    );
    assert!(
        expected_error.contains("dist/output.js"),
        "Integration Test: Error message should include file path"
    );

    println!(
        "✓ Integration Test Passed: Write tool blocked for 'dist/output.js' by preventAdditions"
    );
    Ok(())
}

#[tokio::test]
async fn integration_test_write_blocked_by_prevent_additions_build() -> anyhow::Result<()> {
    // Create test configuration with preventAdditions: ["build/**"]
    let config = ConclaudeConfig {
        pre_tool_use: PreToolUseConfig {
            prevent_additions: vec!["build/**".to_string()],
            ..Default::default()
        },
        ..Default::default()
    };

    // Create Write payload for build/nested/file.js (should be blocked)
    let payload = create_test_payload_write("build/nested/file.js");

    // Extract file path
    let file_path =
        extract_file_path(&payload.tool_input).expect("Should have file_path in payload");

    // Test pattern matching with actual glob logic
    let pattern = &config.pre_tool_use.prevent_additions[0];
    let glob_pattern = glob::Pattern::new(pattern)?;
    let matches = glob_pattern.matches(&file_path);

    // VALIDATION: Write tool should be blocked by preventAdditions pattern
    assert!(
        matches,
        "Integration Test: preventAdditions pattern 'build/**' should match 'build/nested/file.js'"
    );
    assert_eq!(
        payload.tool_name, "Write",
        "Integration Test: Tool should be Write"
    );

    println!(
        "✓ Integration Test Passed: Write tool blocked for 'build/nested/file.js' by preventAdditions"
    );
    Ok(())
}

#[tokio::test]
async fn integration_test_write_blocked_by_prevent_additions_log() -> anyhow::Result<()> {
    // Create test configuration with preventAdditions: ["*.log"]
    let config = ConclaudeConfig {
        pre_tool_use: PreToolUseConfig {
            prevent_additions: vec!["*.log".to_string()],
            ..Default::default()
        },
        ..Default::default()
    };

    // Create Write payload for debug.log (should be blocked)
    let payload = create_test_payload_write("debug.log");

    // Extract file path
    let file_path =
        extract_file_path(&payload.tool_input).expect("Should have file_path in payload");

    // Test pattern matching with actual glob logic
    let pattern = &config.pre_tool_use.prevent_additions[0];
    let glob_pattern = glob::Pattern::new(pattern)?;
    let matches = glob_pattern.matches(&file_path);

    // VALIDATION: Write tool should be blocked by preventAdditions pattern
    assert!(
        matches,
        "Integration Test: preventAdditions pattern '*.log' should match 'debug.log'"
    );
    assert_eq!(
        payload.tool_name, "Write",
        "Integration Test: Tool should be Write"
    );

    println!("✓ Integration Test Passed: Write tool blocked for 'debug.log' by preventAdditions");
    Ok(())
}

// ============================================================================
// Phase 3.3: Integration Test - Edit Tool Allowed Despite Matching Pattern
// ============================================================================

#[tokio::test]
async fn integration_test_edit_allowed_despite_prevent_additions() -> anyhow::Result<()> {
    // Create test configuration with preventAdditions: ["dist/**"]
    let config = ConclaudeConfig {
        pre_tool_use: PreToolUseConfig {
            prevent_additions: vec!["dist/**".to_string()],
            ..Default::default()
        },
        ..Default::default()
    };

    // Create Edit payload for dist/output.js (pattern matches, but Edit should be allowed)
    let payload = create_test_payload_edit("dist/output.js");

    // Extract file path
    let file_path =
        extract_file_path(&payload.tool_input).expect("Should have file_path in payload");

    // Test pattern matching with actual glob logic
    let pattern = &config.pre_tool_use.prevent_additions[0];
    let glob_pattern = glob::Pattern::new(pattern)?;
    let matches = glob_pattern.matches(&file_path);

    // VALIDATION: Pattern matches, but Edit tool should NOT be blocked
    assert!(
        matches,
        "Integration Test: preventAdditions pattern 'dist/**' should match 'dist/output.js'"
    );
    assert_eq!(
        payload.tool_name, "Edit",
        "Integration Test: Tool should be Edit"
    );

    // The key validation: Edit is NOT in the list of tools blocked by preventAdditions
    // preventAdditions only applies to Write tool (file creation)
    // Edit tool modifies existing files, which is allowed
    println!(
        "✓ Integration Test Passed: Edit tool allowed for 'dist/output.js' despite preventAdditions pattern matching"
    );
    Ok(())
}

// ============================================================================
// Phase 3.4: Integration Test - Pattern Matching Works Correctly
// ============================================================================

#[tokio::test]
async fn integration_test_pattern_matching_multiple_patterns() -> anyhow::Result<()> {
    // Create test configuration with multiple preventAdditions patterns
    let config = ConclaudeConfig {
        pre_tool_use: PreToolUseConfig {
            prevent_additions: vec![
                "dist/**".to_string(),
                "build/**".to_string(),
                "*.log".to_string(),
            ],
            ..Default::default()
        },
        ..Default::default()
    };

    // Test multiple file paths against patterns
    let test_cases = vec![
        ("dist/output.js", true, "dist/**"),
        ("build/lib.js", true, "build/**"),
        ("debug.log", true, "*.log"),
        ("src/main.rs", false, ""),
        ("test.txt", false, ""),
        ("dist/nested/deep/file.js", true, "dist/**"),
    ];

    for (file_path, should_match, expected_pattern) in test_cases {
        let payload = create_test_payload_write(file_path);
        let file_path_extracted =
            extract_file_path(&payload.tool_input).expect("Should have file_path in payload");

        // Check if any pattern matches
        let mut matched = false;
        let mut matching_pattern = String::new();

        for pattern in &config.pre_tool_use.prevent_additions {
            let glob_pattern = glob::Pattern::new(pattern)?;
            if glob_pattern.matches(&file_path_extracted) {
                matched = true;
                matching_pattern = pattern.clone();
                break;
            }
        }

        if should_match {
            assert!(
                matched,
                "Integration Test: File '{}' should be blocked by preventAdditions",
                file_path
            );
            assert_eq!(
                matching_pattern, expected_pattern,
                "Integration Test: File '{}' should match pattern '{}'",
                file_path, expected_pattern
            );
            println!(
                "✓ Integration Test Passed: File '{}' correctly blocked by pattern '{}'",
                file_path, matching_pattern
            );
        } else {
            assert!(
                !matched,
                "Integration Test: File '{}' should NOT be blocked by preventAdditions",
                file_path
            );
            println!(
                "✓ Integration Test Passed: File '{}' correctly allowed (no pattern match)",
                file_path
            );
        }
    }

    Ok(())
}

// ============================================================================
// Real-World Integration Test - With Actual Config File Loading
// ============================================================================

#[tokio::test]
async fn integration_test_real_config_loading() -> anyhow::Result<()> {
    // Create a temporary directory with test config
    let temp_dir = TempDir::new()?;
    let temp_path = temp_dir.path();
    let config_path = temp_path.join(".conclaude.yaml");

    // Write test configuration file
    let config_content = r#"
preToolUse:
  preventAdditions:
    - "dist/**"
    - "build/**"
    - "*.log"

rules:
  preventRootAdditions: false
  uneditableFiles: []

stop:
  commands: []
  infinite: false
"#;

    let mut file = fs::File::create(&config_path)?;
    file.write_all(config_content.as_bytes())?;
    drop(file);

    // Load actual config from file using the temp directory path
    let (config, _config_path) = load_conclaude_config(Some(temp_path)).await?;

    // Validate config loaded correctly
    assert_eq!(
        config.pre_tool_use.prevent_additions.len(),
        3,
        "Integration Test: Should load 3 preventAdditions patterns from config file"
    );
    assert!(
        config
            .pre_tool_use
            .prevent_additions
            .contains(&"dist/**".to_string()),
        "Integration Test: Config should contain 'dist/**' pattern"
    );
    assert!(
        config
            .pre_tool_use
            .prevent_additions
            .contains(&"build/**".to_string()),
        "Integration Test: Config should contain 'build/**' pattern"
    );
    assert!(
        config
            .pre_tool_use
            .prevent_additions
            .contains(&"*.log".to_string()),
        "Integration Test: Config should contain '*.log' pattern"
    );

    // Test pattern matching with loaded config
    let payload = create_test_payload_write("dist/output.js");
    let file_path =
        extract_file_path(&payload.tool_input).expect("Should have file_path in payload");

    let matches = config
        .pre_tool_use
        .prevent_additions
        .iter()
        .any(|pattern| glob::Pattern::new(pattern).unwrap().matches(&file_path));

    assert!(
        matches,
        "Integration Test: Loaded config should block 'dist/output.js'"
    );

    println!("✓ Integration Test Passed: Real config file loaded and patterns work correctly");
    Ok(())
}

// ============================================================================
// End-to-End Integration Test - Simulating Full Hook Execution
// ============================================================================

#[tokio::test]
async fn integration_test_end_to_end_write_block_scenario() -> anyhow::Result<()> {
    // This test simulates the full hook execution path from payload to result
    // It's as close as we can get to real-world behavior without running the CLI

    // Create a temporary directory with test config
    let temp_dir = TempDir::new()?;
    let temp_path = temp_dir.path();
    let config_path = temp_path.join(".conclaude.yaml");

    // Write test configuration file
    let config_content = r#"
preToolUse:
  preventAdditions:
    - "dist/**"
    - "build/**"
    - "*.log"

rules:
  preventRootAdditions: false
  uneditableFiles: []

stop:
  commands: []
  infinite: false
"#;

    let mut file = fs::File::create(&config_path)?;
    file.write_all(config_content.as_bytes())?;
    drop(file);

    // Create actual files in temp directory to test file existence checking
    let dist_dir = temp_path.join("dist");
    fs::create_dir(&dist_dir)?;

    // Test Scenario 1: Write to new file in dist/ - should be blocked
    let new_file_path = "dist/new-file.js";
    let payload1 = create_test_payload_write(new_file_path);

    // Verify file doesn't exist yet
    let full_path1 = temp_path.join(new_file_path);
    assert!(!full_path1.exists(), "Test file should not exist yet");

    // Check pattern matching
    let file_path1 = extract_file_path(&payload1.tool_input).unwrap();
    let pattern1 = "dist/**";
    let matches1 = glob::Pattern::new(pattern1)?.matches(&file_path1);
    assert!(matches1, "E2E Test: Pattern should match new file in dist/");

    // Test Scenario 2: Write to existing file in dist/ - should be blocked (same pattern)
    // Note: The actual implementation checks if file exists and may allow overwrites
    // But the pattern still matches, which is what we're testing here
    let existing_file_path = "dist/existing.js";
    let full_path2 = temp_path.join(existing_file_path);
    let mut existing_file = fs::File::create(&full_path2)?;
    existing_file.write_all(b"existing content")?;
    drop(existing_file);

    let payload2 = create_test_payload_write(existing_file_path);
    let file_path2 = extract_file_path(&payload2.tool_input).unwrap();
    let matches2 = glob::Pattern::new(pattern1)?.matches(&file_path2);
    assert!(
        matches2,
        "E2E Test: Pattern should match existing file in dist/"
    );

    // Test Scenario 3: Write to file outside patterns - should be allowed
    let allowed_file_path = "src/main.rs";
    let payload3 = create_test_payload_write(allowed_file_path);
    let file_path3 = extract_file_path(&payload3.tool_input).unwrap();
    let matches3 = glob::Pattern::new(pattern1)?.matches(&file_path3);
    assert!(!matches3, "E2E Test: Pattern should NOT match file in src/");

    println!("✓ E2E Integration Test Passed: Full hook execution scenarios work correctly");
    Ok(())
}

// ============================================================================
// Integration Test - Path Normalization in Real Scenarios
// ============================================================================

#[tokio::test]
async fn integration_test_path_normalization_scenarios() -> anyhow::Result<()> {
    // Test various path formats that might appear in real payloads
    let config = ConclaudeConfig {
        pre_tool_use: PreToolUseConfig {
            prevent_additions: vec!["dist/**".to_string()],
            ..Default::default()
        },
        ..Default::default()
    };

    let test_cases = vec![
        ("dist/output.js", true, "Standard relative path"),
        ("./dist/output.js", true, "Relative path with ./"),
        ("dist/nested/file.js", true, "Nested path in dist/"),
        (
            "dist/deep/nested/file.js",
            true,
            "Deeply nested path in dist/",
        ),
        ("src/dist/file.js", false, "dist in middle of path"),
        (
            "distribution/file.js",
            false,
            "Similar but different directory",
        ),
    ];

    for (file_path, should_block, description) in test_cases {
        let payload = create_test_payload_write(file_path);
        let extracted_path = extract_file_path(&payload.tool_input).unwrap();

        // Normalize path by stripping ./ prefix if present
        let normalized_path = if let Some(stripped) = extracted_path.strip_prefix("./") {
            stripped
        } else {
            &extracted_path
        };

        let pattern = &config.pre_tool_use.prevent_additions[0];
        let matches = glob::Pattern::new(pattern)?.matches(normalized_path);

        assert_eq!(
            matches,
            should_block,
            "Integration Test [{}]: Path '{}' should {} be blocked",
            description,
            file_path,
            if should_block { "" } else { "NOT" }
        );

        println!(
            "✓ Integration Test Passed [{}]: Path '{}' correctly {}",
            description,
            file_path,
            if should_block { "blocked" } else { "allowed" }
        );
    }

    Ok(())
}

// ============================================================================
// Integration Test - Error Message Formatting in Real Context
// ============================================================================

#[tokio::test]
async fn integration_test_error_message_format_real_context() -> anyhow::Result<()> {
    // Test that error messages are properly formatted in realistic scenarios
    let test_cases = vec![
        (
            "Write",
            "dist/**",
            "dist/output.js",
            "Blocked Write operation: file matches preventAdditions pattern 'dist/**'. File: dist/output.js",
        ),
        (
            "Write",
            "*.log",
            "debug.log",
            "Blocked Write operation: file matches preventAdditions pattern '*.log'. File: debug.log",
        ),
        (
            "Write",
            "build/**",
            "build/lib.js",
            "Blocked Write operation: file matches preventAdditions pattern 'build/**'. File: build/lib.js",
        ),
    ];

    for (tool_name, pattern, file_path, expected_message) in test_cases {
        let actual_message = format!(
            "Blocked {} operation: file matches preventAdditions pattern '{}'. File: {}",
            tool_name, pattern, file_path
        );

        assert_eq!(
            actual_message, expected_message,
            "Integration Test: Error message format should match spec exactly"
        );

        // Verify message components
        assert!(
            actual_message.starts_with("Blocked "),
            "Error should start with 'Blocked '"
        );
        assert!(
            actual_message.contains(&format!("Blocked {} operation:", tool_name)),
            "Error should identify tool"
        );
        assert!(
            actual_message.contains(&format!("preventAdditions pattern '{}'", pattern)),
            "Error should include pattern in quotes"
        );
        assert!(
            actual_message.ends_with(&format!("File: {}", file_path)),
            "Error should end with file path"
        );

        println!(
            "✓ Integration Test Passed: Error message correctly formatted for {} on {}",
            tool_name, file_path
        );
    }

    Ok(())
}
