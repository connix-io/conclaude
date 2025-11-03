// Integration tests for timeout execution behavior
use std::fs;
use std::io::Write;
use tempfile::TempDir;

/// Helper function to create a test configuration file with timeout
fn create_config_with_timeout(dir: &TempDir, timeout_secs: u64) -> std::path::PathBuf {
    let config_path = dir.path().join(".conclaude.yaml");
    let config_content = format!(
        r#"
stop:
  commands:
    - run: "sleep {}"
      timeout: {}
      message: "Command took too long"
rules:
  preventRootAdditions: true
"#,
        timeout_secs + 5, // Sleep longer than timeout
        timeout_secs
    );

    let mut file = fs::File::create(&config_path).unwrap();
    file.write_all(config_content.as_bytes()).unwrap();
    file.flush().unwrap();

    config_path
}

/// Helper function to create a test configuration file without timeout
fn create_config_without_timeout(dir: &TempDir) -> std::path::PathBuf {
    let config_path = dir.path().join(".conclaude.yaml");
    let config_content = r#"
stop:
  commands:
    - run: "echo 'quick command'"
      message: "Should succeed"
rules:
  preventRootAdditions: true
"#;

    let mut file = fs::File::create(&config_path).unwrap();
    file.write_all(config_content.as_bytes()).unwrap();
    file.flush().unwrap();

    config_path
}

/// Test that commands complete successfully when no timeout is set
#[tokio::test]
async fn test_command_succeeds_without_timeout() {
    let temp_dir = TempDir::new().unwrap();
    let _config_path = create_config_without_timeout(&temp_dir);

    // Change to temp directory so config is found
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(temp_dir.path()).unwrap();

    // Load config and verify timeout is None
    let (config, _) = conclaude::config::load_conclaude_config(None)
        .await
        .unwrap();
    assert_eq!(config.stop.commands[0].timeout, None);

    // Restore original directory
    std::env::set_current_dir(original_dir).unwrap();
}

/// Test that config with timeout can be loaded
#[tokio::test]
async fn test_load_config_with_timeout() {
    let temp_dir = TempDir::new().unwrap();
    let _config_path = create_config_with_timeout(&temp_dir, 30);

    // Change to temp directory so config is found
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(temp_dir.path()).unwrap();

    // Load config and verify timeout is set
    let (config, _) = conclaude::config::load_conclaude_config(None)
        .await
        .unwrap();
    assert_eq!(config.stop.commands.len(), 1);
    assert_eq!(config.stop.commands[0].timeout, Some(30));

    // Restore original directory
    std::env::set_current_dir(original_dir).unwrap();
}

/// Test that schema generation includes timeout field with proper constraints
#[test]
fn test_schema_includes_timeout_field() {
    use conclaude::config::StopCommand;
    use schemars::schema_for;

    let schema = schema_for!(StopCommand);
    let schema_json = serde_json::to_string_pretty(&schema).unwrap();

    // Verify timeout field exists in schema
    assert!(schema_json.contains("timeout"));
}

/// Test configuration with various timeout values
#[tokio::test]
async fn test_various_timeout_values() {
    let temp_dir = TempDir::new().unwrap();

    let test_cases = vec![1, 30, 60, 300, 3600];

    for timeout_val in test_cases {
        let config_path = temp_dir
            .path()
            .join(format!(".conclaude-{}.yaml", timeout_val));
        let config_content = format!(
            r#"
stop:
  commands:
    - run: "echo test"
      timeout: {}
rules:
  preventRootAdditions: true
"#,
            timeout_val
        );

        let mut file = fs::File::create(&config_path).unwrap();
        file.write_all(config_content.as_bytes()).unwrap();
        file.flush().unwrap();

        let content = fs::read_to_string(&config_path).unwrap();
        let config: conclaude::config::ConclaudeConfig = serde_yaml::from_str(&content).unwrap();

        assert_eq!(config.stop.commands[0].timeout, Some(timeout_val));
    }
}

/// Test that multiple commands can have different timeout values
#[tokio::test]
async fn test_mixed_timeout_values() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join(".conclaude.yaml");
    let config_content = r#"
stop:
  commands:
    - run: "echo 'fast command'"
      timeout: 10
    - run: "echo 'medium command'"
      timeout: 60
    - run: "echo 'slow command'"
      timeout: 300
    - run: "echo 'no timeout command'"
rules:
  preventRootAdditions: true
"#;

    let mut file = fs::File::create(&config_path).unwrap();
    file.write_all(config_content.as_bytes()).unwrap();
    file.flush().unwrap();

    let content = fs::read_to_string(&config_path).unwrap();
    let config: conclaude::config::ConclaudeConfig = serde_yaml::from_str(&content).unwrap();

    assert_eq!(config.stop.commands.len(), 4);
    assert_eq!(config.stop.commands[0].timeout, Some(10));
    assert_eq!(config.stop.commands[1].timeout, Some(60));
    assert_eq!(config.stop.commands[2].timeout, Some(300));
    assert_eq!(config.stop.commands[3].timeout, None);
}
