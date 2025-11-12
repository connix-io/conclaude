use std::fs::{self, File};
use std::io::Write;
use std::process::Command;
use tempfile::tempdir;

#[test]
fn test_cli_help_command() {
    let output = Command::new("cargo")
        .args(["run", "--", "--help"])
        .output()
        .expect("Failed to run CLI help command");

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");
    assert!(stdout.contains("Claude Code Hook Handler"));
    assert!(stdout.contains("PreToolUse"));
    assert!(stdout.contains("PostToolUse"));
    assert!(stdout.contains("init"));
}

#[test]
fn test_cli_init_command() {
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let temp_path = temp_dir.path();

    let output = Command::new("cargo")
        .args([
            "run",
            "--",
            "init",
            "--config-path",
            &temp_path.join(".conclaude.yaml").to_string_lossy(),
            "--claude-path",
            &temp_path.join(".claude").to_string_lossy(),
        ])
        .output()
        .expect("Failed to run CLI init command");

    // Command should succeed (all output goes to log files now)
    assert!(output.status.success(), "Init command should succeed");

    // Verify files were created
    assert!(temp_path.join(".conclaude.yaml").exists());
    assert!(temp_path.join(".claude").exists());
    assert!(temp_path.join(".claude/settings.json").exists());

    // Verify config file content
    let config_content =
        fs::read_to_string(temp_path.join(".conclaude.yaml")).expect("Failed to read config file");
    assert!(config_content.contains("# yaml-language-server:"));
    assert!(config_content.contains("stop:"));
    assert!(config_content.contains("rules:"));
    assert!(config_content.contains("preventRootAdditions: true"));

    // Verify Claude settings file content
    let settings_content = fs::read_to_string(temp_path.join(".claude/settings.json"))
        .expect("Failed to read settings file");
    assert!(settings_content.contains("PreToolUse"));
    assert!(settings_content.contains("PostToolUse"));
    assert!(settings_content.contains("conclaude PreToolUse"));
}

#[test]
fn test_cli_init_command_force_overwrite() {
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let temp_path = temp_dir.path();
    let config_path = temp_path.join(".conclaude.yaml");

    // Create existing config file
    let mut file = File::create(&config_path).expect("Failed to create config file");
    file.write_all(b"existing content")
        .expect("Failed to write existing content");

    // First init without force should fail (exit code 1)
    let output = Command::new("cargo")
        .args([
            "run",
            "--",
            "init",
            "--config-path",
            &config_path.to_string_lossy(),
            "--claude-path",
            &temp_path.join(".claude").to_string_lossy(),
        ])
        .output()
        .expect("Failed to run CLI init command");

    // Should fail because config already exists
    assert!(
        !output.status.success(),
        "Init without force should fail when config exists"
    );

    // Second init with force should succeed
    let output = Command::new("cargo")
        .args([
            "run",
            "--",
            "init",
            "--force",
            "--config-path",
            &config_path.to_string_lossy(),
            "--claude-path",
            &temp_path.join(".claude").to_string_lossy(),
        ])
        .output()
        .expect("Failed to run CLI init command");

    // Should succeed with --force flag
    assert!(output.status.success(), "Init with --force should succeed");

    // Verify the config was overwritten
    let config_content = fs::read_to_string(&config_path).expect("Failed to read config file");
    assert!(config_content.contains("stop:"));
    assert!(!config_content.contains("existing content"));
}

#[test]
fn test_cli_invalid_command() {
    let output = Command::new("cargo")
        .args(["run", "--", "invalid-command"])
        .output()
        .expect("Failed to run CLI with invalid command");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("Invalid UTF-8");
    // Should show help or error message
    assert!(!stderr.is_empty());
}

// Note: Testing the actual hook handlers with stdin would require more complex setup
// with mock JSON payloads. These tests verify the CLI structure and basic functionality.

// ========== Validate Subcommand Integration Tests ==========

#[test]
fn test_validate_with_valid_configuration() {
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let temp_path = temp_dir.path();
    let config_path = temp_path.join(".conclaude.yaml");

    // Create a valid configuration
    let valid_config = r#"
stop:
  commands:
    - run: "echo test"
  infinite: false
  rounds: 5
rules:
  preventRootAdditions: true
  uneditableFiles:
    - "*.lock"
    - "package-lock.json"
  toolUsageValidation: []
preToolUse:
  preventAdditions: []
  preventGeneratedFileEdits: true
notifications:
  enabled: false
  hooks: []
  showErrors: false
  showSuccess: false
  showSystemEvents: true
"#;

    fs::write(&config_path, valid_config).expect("Failed to write config file");

    let output = Command::new("cargo")
        .args([
            "run",
            "--",
            "validate",
            "--config-path",
            &config_path.to_string_lossy(),
        ])
        .output()
        .expect("Failed to run validate command");

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");

    // Should succeed with exit code 0
    assert!(
        output.status.success(),
        "Validate should succeed with valid config. stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Verify success message in output
    assert!(stdout.contains("Configuration is valid"));
    assert!(stdout.contains("Config file:"));
    assert!(stdout.contains("Configuration summary:"));
}

#[test]
fn test_validate_with_missing_configuration() {
    let temp_dir = tempdir().expect("Failed to create temp directory");

    // Create a deep directory structure to ensure no config is found
    let mut current_path = temp_dir.path().to_path_buf();
    for i in 0..15 {
        current_path = current_path.join(format!("level_{i}"));
        fs::create_dir(&current_path).unwrap();
    }

    let output = Command::new("cargo")
        .args([
            "run",
            "--",
            "validate",
            "--config-path",
            &current_path.to_string_lossy(),
        ])
        .output()
        .expect("Failed to run validate command");

    let stderr = String::from_utf8(output.stderr).expect("Invalid UTF-8");

    // Should fail with non-zero exit code
    assert!(
        !output.status.success(),
        "Validate should fail when config is missing"
    );

    // Exit code should be 1 (from std::process::exit(1))
    // Note: May be 101 in test environment due to how cargo run handles exits
    let exit_code = output.status.code();
    assert!(
        exit_code == Some(1) || exit_code == Some(101),
        "Exit code should be 1 or 101 for missing config, got: {:?}",
        exit_code
    );

    // Verify error message mentions "Configuration file not found"
    assert!(
        stderr.contains("Configuration file not found")
            || stderr.contains("Configuration validation failed"),
        "Error message should mention config not found. stderr: {stderr}"
    );
}

#[test]
fn test_validate_with_invalid_yaml_syntax() {
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let temp_path = temp_dir.path();
    let config_path = temp_path.join(".conclaude.yaml");

    // Create a config with invalid YAML syntax (bad indentation)
    let invalid_yaml = r#"
stop:
  commands:
    - run: "echo test"
rules:
preventRootAdditions: true
"#;

    fs::write(&config_path, invalid_yaml).expect("Failed to write config file");

    let output = Command::new("cargo")
        .args([
            "run",
            "--",
            "validate",
            "--config-path",
            &config_path.to_string_lossy(),
        ])
        .output()
        .expect("Failed to run validate command");

    let stderr = String::from_utf8(output.stderr).expect("Invalid UTF-8");

    // Should fail with non-zero exit code
    assert!(
        !output.status.success(),
        "Validate should fail with invalid YAML syntax"
    );
    let exit_code = output.status.code();
    assert!(
        exit_code == Some(1) || exit_code == Some(101),
        "Exit code should be 1 or 101 for invalid YAML, got: {:?}",
        exit_code
    );

    // Verify error message mentions parsing/syntax error
    assert!(
        stderr.contains("validation failed")
            || stderr.contains("parse")
            || stderr.contains("syntax"),
        "Error message should mention parsing/syntax issue. stderr: {stderr}"
    );
}

#[test]
fn test_validate_with_unknown_fields() {
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let temp_path = temp_dir.path();
    let config_path = temp_path.join(".conclaude.yaml");

    // Create a config with unknown fields
    let config_with_unknown_fields = r#"
stop:
  commands:
    - run: "echo test"
  unknownField: "should fail"
  anotherBadField: 123
rules:
  preventRootAdditions: true
"#;

    fs::write(&config_path, config_with_unknown_fields).expect("Failed to write config file");

    let output = Command::new("cargo")
        .args([
            "run",
            "--",
            "validate",
            "--config-path",
            &config_path.to_string_lossy(),
        ])
        .output()
        .expect("Failed to run validate command");

    let stderr = String::from_utf8(output.stderr).expect("Invalid UTF-8");

    // Should fail with non-zero exit code
    assert!(
        !output.status.success(),
        "Validate should fail with unknown fields"
    );
    let exit_code = output.status.code();
    assert!(
        exit_code == Some(1) || exit_code == Some(101),
        "Exit code should be 1 or 101 for unknown fields, got: {:?}",
        exit_code
    );

    // Verify error message mentions unknown field
    assert!(
        stderr.contains("unknown field"),
        "Error message should mention unknown field. stderr: {stderr}"
    );

    // Verify error message provides helpful suggestions
    assert!(
        stderr.contains("Valid field names") || stderr.contains("Common causes"),
        "Error message should provide helpful suggestions. stderr: {stderr}"
    );
}

#[test]
fn test_validate_with_invalid_types() {
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let temp_path = temp_dir.path();
    let config_path = temp_path.join(".conclaude.yaml");

    // Create a config with invalid types (string instead of boolean)
    let config_with_invalid_types = r#"
stop:
  commands:
    - run: "echo test"
  infinite: "true"
rules:
  preventRootAdditions: "yes"
"#;

    fs::write(&config_path, config_with_invalid_types).expect("Failed to write config file");

    let output = Command::new("cargo")
        .args([
            "run",
            "--",
            "validate",
            "--config-path",
            &config_path.to_string_lossy(),
        ])
        .output()
        .expect("Failed to run validate command");

    let stderr = String::from_utf8(output.stderr).expect("Invalid UTF-8");

    // Should fail with non-zero exit code
    assert!(
        !output.status.success(),
        "Validate should fail with invalid types"
    );
    let exit_code = output.status.code();
    assert!(
        exit_code == Some(1) || exit_code == Some(101),
        "Exit code should be 1 or 101 for invalid types, got: {:?}",
        exit_code
    );

    // Verify error message mentions type error
    assert!(
        stderr.contains("invalid type") || stderr.contains("expected"),
        "Error message should mention type error. stderr: {stderr}"
    );

    // Verify error message provides helpful information
    assert!(
        stderr.contains("Common causes") || stderr.contains("Example"),
        "Error message should provide helpful examples. stderr: {stderr}"
    );
}

#[test]
fn test_validate_with_out_of_range_values() {
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let temp_path = temp_dir.path();
    let config_path = temp_path.join(".conclaude.yaml");

    // Create a config with out-of-range values (maxOutputLines > 10000)
    let config_with_out_of_range = r#"
stop:
  commands:
    - run: "test"
      maxOutputLines: 50000
rules:
  preventRootAdditions: true
"#;

    fs::write(&config_path, config_with_out_of_range).expect("Failed to write config file");

    let output = Command::new("cargo")
        .args([
            "run",
            "--",
            "validate",
            "--config-path",
            &config_path.to_string_lossy(),
        ])
        .output()
        .expect("Failed to run validate command");

    let stderr = String::from_utf8(output.stderr).expect("Invalid UTF-8");

    // Should fail with non-zero exit code
    assert!(
        !output.status.success(),
        "Validate should fail with out-of-range values"
    );
    let exit_code = output.status.code();
    assert!(
        exit_code == Some(1) || exit_code == Some(101),
        "Exit code should be 1 or 101 for out-of-range values, got: {:?}",
        exit_code
    );

    // Verify error message mentions range validation
    assert!(
        stderr.contains("Range validation") || stderr.contains("out of valid range"),
        "Error message should mention range validation. stderr: {stderr}"
    );

    // Verify error message provides valid range
    assert!(
        stderr.contains("Valid range"),
        "Error message should provide valid range. stderr: {stderr}"
    );
}

#[test]
fn test_validate_with_out_of_range_rounds() {
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let temp_path = temp_dir.path();
    let config_path = temp_path.join(".conclaude.yaml");

    // Create a config with out-of-range rounds value (0)
    let config_with_invalid_rounds = r#"
stop:
  commands:
    - run: "echo test"
  rounds: 0
rules:
  preventRootAdditions: true
"#;

    fs::write(&config_path, config_with_invalid_rounds).expect("Failed to write config file");

    let output = Command::new("cargo")
        .args([
            "run",
            "--",
            "validate",
            "--config-path",
            &config_path.to_string_lossy(),
        ])
        .output()
        .expect("Failed to run validate command");

    let stderr = String::from_utf8(output.stderr).expect("Invalid UTF-8");

    // Should fail with non-zero exit code
    assert!(
        !output.status.success(),
        "Validate should fail with rounds value of 0"
    );
    let exit_code = output.status.code();
    assert!(
        exit_code == Some(1) || exit_code == Some(101),
        "Exit code should be 1 or 101 for invalid rounds, got: {:?}",
        exit_code
    );

    // Verify error message mentions range validation for rounds
    assert!(
        stderr.contains("Range validation") && stderr.contains("rounds"),
        "Error message should mention range validation for rounds. stderr: {stderr}"
    );
}

#[test]
fn test_validate_with_custom_config_path_file() {
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let temp_path = temp_dir.path();
    let custom_config = temp_path.join("custom-config.yaml");

    // Create a valid configuration with a custom filename
    let valid_config = r#"
stop:
  commands:
    - run: "echo custom"
rules:
  preventRootAdditions: false
"#;

    fs::write(&custom_config, valid_config).expect("Failed to write custom config file");

    let output = Command::new("cargo")
        .args([
            "run",
            "--",
            "validate",
            "--config-path",
            &custom_config.to_string_lossy(),
        ])
        .output()
        .expect("Failed to run validate command");

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");

    // Should succeed with exit code 0
    assert!(
        output.status.success(),
        "Validate should succeed with custom config path. stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Verify success message references the custom config path
    assert!(stdout.contains("Configuration is valid"));
    assert!(stdout.contains("custom-config.yaml"));
}

#[test]
fn test_validate_with_custom_config_path_directory() {
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let temp_path = temp_dir.path();
    let config_path = temp_path.join(".conclaude.yaml");

    // Create a valid configuration in a custom directory
    let valid_config = r#"
stop:
  commands:
    - run: "echo directory test"
rules:
  preventRootAdditions: true
"#;

    fs::write(&config_path, valid_config).expect("Failed to write config file");

    let output = Command::new("cargo")
        .args([
            "run",
            "--",
            "validate",
            "--config-path",
            &temp_path.to_string_lossy(),
        ])
        .output()
        .expect("Failed to run validate command");

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");

    // Should succeed with exit code 0
    assert!(
        output.status.success(),
        "Validate should succeed with custom directory path. stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Verify success message
    assert!(stdout.contains("Configuration is valid"));
}

#[test]
fn test_validate_with_nonexistent_custom_config_file() {
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let temp_path = temp_dir.path();
    let nonexistent_config = temp_path.join("does-not-exist.yaml");

    let output = Command::new("cargo")
        .args([
            "run",
            "--",
            "validate",
            "--config-path",
            &nonexistent_config.to_string_lossy(),
        ])
        .output()
        .expect("Failed to run validate command");

    let stderr = String::from_utf8(output.stderr).expect("Invalid UTF-8");

    // Should fail with non-zero exit code
    assert!(
        !output.status.success(),
        "Validate should fail with nonexistent config file"
    );
    let exit_code = output.status.code();
    assert!(
        exit_code == Some(1) || exit_code == Some(101),
        "Exit code should be 1 or 101 for nonexistent file, got: {:?}",
        exit_code
    );

    // Verify error message mentions file error
    assert!(
        stderr.contains("No such file or directory") || stderr.contains("not found"),
        "Error message should mention file error. stderr: {stderr}"
    );
}

#[test]
fn test_validate_displays_configuration_summary() {
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let temp_path = temp_dir.path();
    let config_path = temp_path.join(".conclaude.yaml");

    // Create a configuration with specific values to verify in summary
    let config_with_features = r#"
stop:
  commands:
    - run: "echo test"
  infinite: true
  rounds: 10
rules:
  preventRootAdditions: false
  uneditableFiles:
    - "*.lock"
    - "*.json"
  toolUsageValidation:
    - tool: "Write"
      pattern: ".*"
      action: "block"
notifications:
  enabled: true
  hooks: ["*"]
"#;

    fs::write(&config_path, config_with_features).expect("Failed to write config file");

    let output = Command::new("cargo")
        .args([
            "run",
            "--",
            "validate",
            "--config-path",
            &config_path.to_string_lossy(),
        ])
        .output()
        .expect("Failed to run validate command");

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");

    // Should succeed
    assert!(
        output.status.success(),
        "Validate should succeed. stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Verify configuration summary includes key details
    assert!(stdout.contains("Configuration summary:"));
    assert!(stdout.contains("Prevent root additions: false"));
    assert!(stdout.contains("Uneditable files: 2 pattern(s)"));
    assert!(stdout.contains("Tool usage validation: 1 rule(s)"));
    assert!(stdout.contains("Infinite mode: true"));
    assert!(stdout.contains("Rounds: 10"));
    assert!(stdout.contains("Notifications enabled: true"));
}
