use crate::types::LoggingConfig;
use log::LevelFilter;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Initialize the logger based on configuration
///
/// # Errors
///
/// Returns an error if logger initialization fails or file logging setup fails.
pub fn init_logger(session_id: Option<&str>, config: Option<&LoggingConfig>) -> anyhow::Result<()> {
    let logging_config = resolve_logging_config(config);

    let mut builder = env_logger::Builder::from_default_env();

    // Set default log level
    let log_level = std::env::var("CONCLAUDE_LOG_LEVEL").unwrap_or_else(|_| "info".to_string());

    let level_filter = match log_level.to_lowercase().as_str() {
        "error" => LevelFilter::Error,
        "warn" => LevelFilter::Warn,
        "debug" => LevelFilter::Debug,
        "trace" => LevelFilter::Trace,
        _ => LevelFilter::Info, // Default includes "info" and any unrecognized values
    };

    builder.filter_level(level_filter);

    // Configure format
    builder.format(|buf, record| {
        let timestamp = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ");
        writeln!(
            buf,
            "{} [conclaude] {}: {}",
            timestamp,
            record.level(),
            record.args()
        )
    });

    // If file logging is enabled, set up file output
    if logging_config.file_logging {
        if let Some(session_id) = session_id {
            let log_file_path = get_log_file_path(session_id);
            setup_file_logging(&mut builder, &log_file_path)?;
        }
    }

    builder.try_init()?;

    Ok(())
}

/// Resolves logging configuration from environment variables and optional overrides.
#[must_use]
pub fn resolve_logging_config(config: Option<&LoggingConfig>) -> LoggingConfig {
    let env_var = std::env::var("CONCLAUDE_DISABLE_FILE_LOGGING").ok();
    resolve_logging_config_with_env(config, env_var.as_deref())
}

/// Internal function that resolves logging configuration with explicit environment variable value.
/// This allows for deterministic testing without global environment variable manipulation.
#[must_use]
pub fn resolve_logging_config_with_env(
    config: Option<&LoggingConfig>,
    env_var: Option<&str>,
) -> LoggingConfig {
    // Check environment variable CONCLAUDE_DISABLE_FILE_LOGGING
    // - If "true", disable file logging
    // - If "false", enable file logging
    // - If unset, default to disabled
    let default_file_logging = match env_var {
        Some("false") => true, // Enable if explicitly set to "false"
        _ => false,            // Default to disabled if unset, "true", or invalid values
    };

    LoggingConfig {
        file_logging: config.map_or(default_file_logging, |c| c.file_logging),
    }
}

/// Generate a log file path for the given session ID
#[must_use]
pub fn get_log_file_path(session_id: &str) -> PathBuf {
    let project_name = get_project_name();
    let sanitized_project = sanitize_project_name(&project_name);
    let filename = format!("conclaude-{sanitized_project}-sess-{session_id}.jsonl");

    let temp_dir = std::env::temp_dir();
    temp_dir.join(filename)
}

/// Get the current project name from the working directory
fn get_project_name() -> String {
    match std::env::current_dir() {
        Ok(cwd) => cwd
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown")
            .to_string(),
        Err(_) => "unknown".to_string(),
    }
}

/// Sanitize project name for use in filenames
#[must_use]
pub fn sanitize_project_name(name: &str) -> String {
    name.to_lowercase()
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' {
                c
            } else {
                '-'
            }
        })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
        .trim_matches('-')
        .to_string()
}

/// Setup file logging (this is a placeholder - in a real implementation
/// you might use a more sophisticated logging framework like tracing)
fn setup_file_logging(
    _builder: &mut env_logger::Builder,
    log_file_path: &Path,
) -> anyhow::Result<()> {
    // Note: env_logger doesn't support file output directly
    // In a production implementation, you might want to use tracing + tracing-appender
    // or implement a custom logger that writes to both console and file

    // For now, we'll just ensure the log directory exists
    if let Some(parent) = log_file_path.parent() {
        fs::create_dir_all(parent)?;
    }

    Ok(())
}

/// Create a session-specific logger instance
///
/// # Errors
///
/// Returns an error if logger initialization fails.
pub fn create_session_logger(
    session_id: &str,
    config: Option<&LoggingConfig>,
) -> anyhow::Result<()> {
    let log_path = get_log_file_path(session_id);
    let temp_dir = std::env::temp_dir();
    let resolved_config = resolve_logging_config(config);

    init_logger(Some(session_id), config).map_err(|e| {
        let temp_readonly = std::fs::metadata(&temp_dir)
            .map(|m| m.permissions().readonly())
            .unwrap_or(true);

        eprintln!("Failed to initialize session logger");
        eprintln!("  Session ID: {}", session_id);
        eprintln!("  Error: {}", e);
        eprintln!("  Attempted log path: {}", log_path.display());
        eprintln!("  Temp directory: {}", temp_dir.display());
        eprintln!("  Temp dir exists: {}", temp_dir.exists());
        eprintln!("  File logging enabled: {}", resolved_config.file_logging);
        eprintln!("  Temp dir readonly: {}", temp_readonly);

        anyhow::anyhow!(
            "Failed to initialize session logger: {} (session_id={}, log_path={})",
            e,
            session_id,
            log_path.display()
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_project_name() {
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
    fn test_resolve_logging_config_default() {
        let config = resolve_logging_config(None);
        assert!(!config.file_logging);
    }

    #[test]
    fn test_resolve_logging_config_with_override() {
        let override_config = LoggingConfig { file_logging: true };
        let config = resolve_logging_config(Some(&override_config));
        assert!(config.file_logging);
    }

    #[test]
    fn test_get_log_file_path() {
        let path = get_log_file_path("test-session-123");
        let filename = path.file_name().unwrap().to_str().unwrap();
        assert!(filename.starts_with("conclaude-"));
        assert!(filename.contains("sess-test-session-123"));
        assert!(filename.to_lowercase().ends_with(".jsonl"));
    }
}
