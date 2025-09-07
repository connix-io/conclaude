use conclaude::logger::*;
use conclaude::types::LoggingConfig;

#[test]
fn test_sanitize_project_name() {
    // Test basic functionality
    assert_eq!(sanitize_project_name("my-project"), "my-project");
    assert_eq!(sanitize_project_name("My Project!"), "my-project");
    assert_eq!(
        sanitize_project_name("test_project_123"),
        "test-project-123"
    );
    assert_eq!(sanitize_project_name("---test---"), "test");
    assert_eq!(sanitize_project_name(""), "");
}

#[test]
fn test_sanitize_project_name_special_chars() {
    assert_eq!(sanitize_project_name("hello@world"), "hello-world");
    assert_eq!(sanitize_project_name("test.project"), "test-project");
    assert_eq!(
        sanitize_project_name("my project with spaces"),
        "my-project-with-spaces"
    );
    assert_eq!(sanitize_project_name("project#$%^&*()"), "project");
}

#[test]
fn test_get_log_file_path() {
    let path = get_log_file_path("test-session-123");
    let filename = path.file_name().unwrap().to_str().unwrap();
    assert!(filename.starts_with("conclaude-"));
    assert!(filename.contains("sess-test-session-123"));
    assert!(
        std::path::Path::new(filename)
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("jsonl"))
    );
}

#[test]
fn test_resolve_logging_config_default() {
    // Test with no environment variable set (None)
    let config = resolve_logging_config_with_env(None, None);
    assert!(!config.file_logging);
}

#[test]
fn test_resolve_logging_config_with_env_var_true() {
    // Test with environment variable explicitly set to "true" (disable file logging)
    let config = resolve_logging_config_with_env(None, Some("true"));
    assert!(!config.file_logging);
}

#[test]
fn test_resolve_logging_config_with_env_var_false() {
    // Test with environment variable explicitly set to "false" (enable file logging)
    let config = resolve_logging_config_with_env(None, Some("false"));
    assert!(config.file_logging);
}

#[test]
fn test_resolve_logging_config_with_override() {
    // Test that config override takes precedence (no env var)
    let override_config = LoggingConfig { file_logging: true };
    let config = resolve_logging_config_with_env(Some(&override_config), None);
    assert!(config.file_logging);
}

#[test]
fn test_resolve_logging_config_override_takes_precedence() {
    // Test that config override takes precedence over environment variable
    let override_config = LoggingConfig { file_logging: true };
    let config = resolve_logging_config_with_env(Some(&override_config), Some("true"));
    assert!(config.file_logging);
}

#[test]
fn test_init_logger_basic() {
    // This test just ensures the logger can be initialized without panicking
    let result = init_logger(Some("test-session"), None);
    // Logger might already be initialized by previous tests, so we accept either Ok or the specific "already initialized" error
    assert!(result.is_ok() || result.unwrap_err().to_string().contains("already"));
}

#[test]
fn test_create_session_logger() {
    let config = LoggingConfig {
        file_logging: false,
    };
    let result = create_session_logger("test-session", Some(&config));
    // Logger might already be initialized by previous tests, so we accept either Ok or the specific "already initialized" error
    assert!(result.is_ok() || result.unwrap_err().to_string().contains("already"));
}
