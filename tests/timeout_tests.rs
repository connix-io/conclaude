use conclaude::config::{ConclaudeConfig, StopCommand, StopConfig};
use serde_yaml;
use std::io::Write;
use tempfile::NamedTempFile;

/// Test that timeout field can be parsed from YAML configuration
#[test]
fn test_timeout_field_parsing() {
    let yaml = r#"
stop:
  commands:
    - run: "sleep 5"
      timeout: 30
"#;

    let config: ConclaudeConfig = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(config.stop.commands.len(), 1);
    assert_eq!(config.stop.commands[0].timeout, Some(30));
}

/// Test that timeout field is optional and defaults to None
#[test]
fn test_timeout_field_optional() {
    let yaml = r#"
stop:
  commands:
    - run: "echo hello"
"#;

    let config: ConclaudeConfig = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(config.stop.commands.len(), 1);
    assert_eq!(config.stop.commands[0].timeout, None);
}

/// Test backward compatibility - configs without timeout should still work
#[test]
fn test_backward_compatibility_no_timeout() {
    let yaml = r#"
stop:
  commands:
    - run: "echo test"
      message: "Test failed"
      showStdout: true
      showStderr: true
      maxOutputLines: 100
"#;

    let config: ConclaudeConfig = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(config.stop.commands.len(), 1);
    assert_eq!(config.stop.commands[0].timeout, None);
    assert_eq!(
        config.stop.commands[0].message,
        Some("Test failed".to_string())
    );
    assert_eq!(config.stop.commands[0].show_stdout, Some(true));
}

/// Test that timeout field serializes and deserializes correctly
#[test]
fn test_timeout_serialization_roundtrip() {
    let cmd = StopCommand {
        run: "echo test".to_string(),
        message: Some("Test message".to_string()),
        show_stdout: Some(true),
        show_stderr: Some(false),
        max_output_lines: Some(50),
        timeout: Some(120),
    };

    let yaml = serde_yaml::to_string(&cmd).unwrap();
    let deserialized: StopCommand = serde_yaml::from_str(&yaml).unwrap();

    assert_eq!(deserialized.run, "echo test");
    assert_eq!(deserialized.timeout, Some(120));
}

/// Test multiple commands with different timeout values
#[test]
fn test_multiple_commands_with_different_timeouts() {
    let yaml = r#"
stop:
  commands:
    - run: "quick command"
      timeout: 10
    - run: "slow command"
      timeout: 300
    - run: "no timeout command"
"#;

    let config: ConclaudeConfig = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(config.stop.commands.len(), 3);
    assert_eq!(config.stop.commands[0].timeout, Some(10));
    assert_eq!(config.stop.commands[1].timeout, Some(300));
    assert_eq!(config.stop.commands[2].timeout, None);
}

/// Test that timeout field with null value is accepted
#[test]
fn test_timeout_explicit_null() {
    let yaml = r#"
stop:
  commands:
    - run: "echo test"
      timeout: null
"#;

    let config: ConclaudeConfig = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(config.stop.commands.len(), 1);
    assert_eq!(config.stop.commands[0].timeout, None);
}

/// Test comprehensive configuration with timeout field
#[test]
fn test_comprehensive_config_with_timeout() {
    let yaml = r#"
stop:
  run: "echo legacy"
  commands:
    - run: "npm test"
      message: "Tests failed"
      showStdout: true
      showStderr: true
      maxOutputLines: 100
      timeout: 600
    - run: "npm run build"
      message: "Build failed"
      showStderr: true
      timeout: 300
  infinite: false
  infiniteMessage: "continue working"
  rounds: null
rules:
  preventRootAdditions: true
  uneditableFiles:
    - "package.json"
preToolUse:
  preventAdditions: []
  preventGeneratedFileEdits: true
notifications:
  enabled: false
  hooks: []
"#;

    let config: ConclaudeConfig = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(config.stop.commands.len(), 2);
    assert_eq!(config.stop.commands[0].timeout, Some(600));
    assert_eq!(config.stop.commands[1].timeout, Some(300));
}

/// Test that timeout field can be loaded from file
#[test]
fn test_load_config_with_timeout_from_file() {
    let yaml_content = r#"
stop:
  commands:
    - run: "cargo test"
      timeout: 180
rules:
  preventRootAdditions: true
"#;

    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(yaml_content.as_bytes()).unwrap();
    temp_file.flush().unwrap();

    let content = std::fs::read_to_string(temp_file.path()).unwrap();
    let config: ConclaudeConfig = serde_yaml::from_str(&content).unwrap();

    assert_eq!(config.stop.commands.len(), 1);
    assert_eq!(config.stop.commands[0].timeout, Some(180));
}

/// Test validation of positive timeout values
#[test]
fn test_timeout_positive_values() {
    let test_cases = vec![
        ("timeout: 1", Some(1)),
        ("timeout: 60", Some(60)),
        ("timeout: 3600", Some(3600)),
        ("timeout: 86400", Some(86400)),
    ];

    for (timeout_yaml, expected) in test_cases {
        let yaml = format!(
            r#"
stop:
  commands:
    - run: "echo test"
      {}
"#,
            timeout_yaml
        );

        let config: ConclaudeConfig = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(
            config.stop.commands[0].timeout, expected,
            "Failed for: {}",
            timeout_yaml
        );
    }
}

/// Test that timeout field works with legacy run field
#[test]
fn test_timeout_with_legacy_run_field() {
    let yaml = r#"
stop:
  run: |
    echo "legacy command 1"
    echo "legacy command 2"
  commands:
    - run: "echo new command"
      timeout: 60
"#;

    let config: ConclaudeConfig = serde_yaml::from_str(yaml).unwrap();
    // The legacy run field should not have timeout
    // Only structured commands should have timeout
    assert_eq!(config.stop.commands.len(), 1);
    assert_eq!(config.stop.commands[0].timeout, Some(60));
}

/// Test interaction between timeout and other command options
#[test]
fn test_timeout_interaction_with_other_options() {
    let yaml = r#"
stop:
  commands:
    - run: "npm test"
      message: "Tests failed"
      showStdout: true
      showStderr: true
      maxOutputLines: 50
      timeout: 300
"#;

    let config: ConclaudeConfig = serde_yaml::from_str(yaml).unwrap();
    let cmd = &config.stop.commands[0];

    assert_eq!(cmd.timeout, Some(300));
    assert_eq!(cmd.message, Some("Tests failed".to_string()));
    assert_eq!(cmd.show_stdout, Some(true));
    assert_eq!(cmd.show_stderr, Some(true));
    assert_eq!(cmd.max_output_lines, Some(50));
}
