use std::fs::{self, File};
use std::io::Write;
use std::process::Command;
use tempfile::tempdir;

#[test]
fn test_cli_help_command() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--help"])
        .output()
        .expect("Failed to run CLI help command");

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");
    assert!(stdout.contains("Claude Code Hook Handler"));
    assert!(stdout.contains("PreToolUse"));
    assert!(stdout.contains("PostToolUse"));
    assert!(stdout.contains("init"));
}

#[test]
fn test_cli_version_command() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--version"])
        .output()
        .expect("Failed to run CLI version command");

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");
    assert!(stdout.contains("conclaude"));
    assert!(stdout.contains("0.1.1"));
}

#[test]
fn test_cli_init_command() {
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let temp_path = temp_dir.path();

    let output = Command::new("cargo")
        .args(&[
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

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");
    assert!(stdout.contains("Initializing conclaude configuration"));
    assert!(stdout.contains("Created configuration file"));
    assert!(stdout.contains("Conclaude initialization complete"));

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

    // First init without force should fail
    let output = Command::new("cargo")
        .args(&[
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

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");
    assert!(stdout.contains("Configuration file already exists"));
    assert!(!output.status.success());

    // Second init with force should succeed
    let output = Command::new("cargo")
        .args(&[
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

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");
    assert!(stdout.contains("Created configuration file"));
    assert!(output.status.success());

    // Verify the config was overwritten
    let config_content = fs::read_to_string(&config_path).expect("Failed to read config file");
    assert!(config_content.contains("stop:"));
    assert!(!config_content.contains("existing content"));
}

#[test]
fn test_cli_verbose_flag() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--verbose", "--help"])
        .output()
        .expect("Failed to run CLI with verbose flag");

    // Just verify it doesn't crash with verbose flag
    assert!(output.status.success());
}

#[test]
fn test_cli_disable_file_logging_flag() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--disable-file-logging", "--help"])
        .output()
        .expect("Failed to run CLI with disable-file-logging flag");

    // Just verify it doesn't crash with disable-file-logging flag
    assert!(output.status.success());
}

#[test]
fn test_cli_invalid_command() {
    let output = Command::new("cargo")
        .args(&["run", "--", "invalid-command"])
        .output()
        .expect("Failed to run CLI with invalid command");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("Invalid UTF-8");
    // Should show help or error message
    assert!(!stderr.is_empty());
}

// Note: Testing the actual hook handlers with stdin would require more complex setup
// with mock JSON payloads. These tests verify the CLI structure and basic functionality.
