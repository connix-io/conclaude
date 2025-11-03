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

#[test]
fn test_cli_validate_with_valid_config() {
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let temp_path = temp_dir.path();
    let config_path = temp_path.join(".conclaude.yaml");

    // Create a valid configuration file
    let valid_config = r#"# yaml-language-server: $schema=https://github.com/conneroisu/conclaude/releases/latest/download/conclaude-schema.json
stop:
  run: "echo test"
  infinite: false
rules:
  preventRootAdditions: true
  uneditableFiles: []
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

    assert!(
        output.status.success(),
        "Validate command should succeed with valid config"
    );

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");
    assert!(stdout.contains("🔍 Validating conclaude configuration"));
    assert!(stdout.contains("✅ Configuration is valid"));
}

#[test]
fn test_cli_validate_with_missing_config() {
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let temp_path = temp_dir.path();
    let config_path = temp_path.join("nonexistent.yaml");

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

    assert!(
        !output.status.success(),
        "Validate command should fail with missing config"
    );

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");
    let stderr = String::from_utf8(output.stderr).expect("Invalid UTF-8");
    assert!(stdout.contains("❌ Configuration validation failed"));
    assert!(stderr.contains("Configuration file not found"));
}

#[test]
fn test_cli_validate_with_invalid_yaml_syntax() {
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let temp_path = temp_dir.path();
    let config_path = temp_path.join(".conclaude.yaml");

    // Create a configuration with invalid YAML syntax
    let invalid_config = r#"
stop:
  run: "echo test
  infinite: false
"#;

    fs::write(&config_path, invalid_config).expect("Failed to write config file");

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

    assert!(
        !output.status.success(),
        "Validate command should fail with invalid YAML syntax"
    );

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");
    let stderr = String::from_utf8(output.stderr).expect("Invalid UTF-8");
    assert!(stdout.contains("❌ Configuration validation failed"));
    assert!(stderr.contains("Failed to parse"));
}

#[test]
fn test_cli_validate_with_unknown_field() {
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let temp_path = temp_dir.path();
    let config_path = temp_path.join(".conclaude.yaml");

    // Create a configuration with an unknown field
    let invalid_config = r#"
stop:
  run: "echo test"
  infinite: false
rules:
  preventRootAdditions: true
  unknownField: "this should fail"
"#;

    fs::write(&config_path, invalid_config).expect("Failed to write config file");

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

    assert!(
        !output.status.success(),
        "Validate command should fail with unknown field"
    );

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");
    let stderr = String::from_utf8(output.stderr).expect("Invalid UTF-8");
    assert!(stdout.contains("❌ Configuration validation failed"));
    assert!(stderr.contains("unknown field"));
}

#[test]
fn test_cli_validate_with_invalid_type() {
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let temp_path = temp_dir.path();
    let config_path = temp_path.join(".conclaude.yaml");

    // Create a configuration with an invalid type (string instead of boolean)
    let invalid_config = r#"
stop:
  run: "echo test"
  infinite: "not a boolean"
rules:
  preventRootAdditions: true
"#;

    fs::write(&config_path, invalid_config).expect("Failed to write config file");

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

    assert!(
        !output.status.success(),
        "Validate command should fail with invalid type"
    );

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");
    let stderr = String::from_utf8(output.stderr).expect("Invalid UTF-8");
    assert!(stdout.contains("❌ Configuration validation failed"));
    assert!(stderr.contains("invalid type"));
}

#[test]
fn test_cli_validate_with_default_search() {
    let temp_dir = tempdir().expect("Failed to create temp directory");

    // Create a valid configuration file in the temp directory
    let config_path = temp_dir.path().join(".conclaude.yaml");
    let valid_config = r#"
stop:
  run: "echo test"
  infinite: false
rules:
  preventRootAdditions: true
"#;

    fs::write(&config_path, valid_config).expect("Failed to write config file");

    // Build the binary first to get its path
    let output = Command::new("cargo")
        .args(["build"])
        .output()
        .expect("Failed to build");
    assert!(output.status.success(), "Failed to build project");

    // Run the built binary from the temp directory
    let binary_path = std::env::current_dir()
        .expect("Failed to get current directory")
        .join("target/debug/conclaude");
    
    let output = Command::new(binary_path)
        .arg("validate")
        .current_dir(temp_dir.path())
        .output()
        .expect("Failed to run validate command");

    let stdout = String::from_utf8(output.stdout).unwrap_or_default();
    let stderr = String::from_utf8(output.stderr).unwrap_or_default();

    assert!(
        output.status.success(),
        "Validate command should succeed with default search. Stdout: {}, Stderr: {}",
        stdout,
        stderr
    );

    assert!(stdout.contains("✅ Configuration is valid"));
}

#[test]
fn test_cli_validate_exit_code_on_success() {
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let temp_path = temp_dir.path();
    let config_path = temp_path.join(".conclaude.yaml");

    // Create a valid configuration file
    let valid_config = r#"
stop:
  run: "echo test"
  infinite: false
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
            &config_path.to_string_lossy(),
        ])
        .output()
        .expect("Failed to run validate command");

    // Exit code should be 0 for success
    assert_eq!(
        output.status.code(),
        Some(0),
        "Exit code should be 0 for valid configuration"
    );
}

#[test]
fn test_cli_validate_exit_code_on_failure() {
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let temp_path = temp_dir.path();
    let config_path = temp_path.join("nonexistent.yaml");

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

    // Exit code should be non-zero for failure
    assert_ne!(
        output.status.code(),
        Some(0),
        "Exit code should be non-zero for invalid/missing configuration"
    );
}
