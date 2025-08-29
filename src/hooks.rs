use crate::config::{extract_bash_commands, load_conclaude_config, ConclaudeConfig};
use crate::logger::create_session_logger;
use crate::types::*;
use anyhow::{Context, Result};
use glob::Pattern;
use serde_json::Value;
use std::io::{self, Read};
use std::path::Path;
use std::process::Stdio;
use std::sync::OnceLock;
use tokio::process::Command as TokioCommand;

/// Cached configuration instance to avoid repeated loads
static CACHED_CONFIG: OnceLock<ConclaudeConfig> = OnceLock::new();

/// Load configuration with caching to avoid repeated file system operations
async fn get_config() -> Result<&'static ConclaudeConfig> {
    if let Some(config) = CACHED_CONFIG.get() {
        Ok(config)
    } else {
        let config = load_conclaude_config().await?;
        Ok(CACHED_CONFIG.get_or_init(|| config))
    }
}

/// Reads and validates hook payload from stdin, creating a session-specific logger.
pub async fn read_payload_from_stdin<T>() -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .context("Failed to read from stdin")?;

    let payload: T = serde_json::from_str(&buffer)
        .context("Failed to parse JSON payload from stdin")?;

    Ok(payload)
}

/// Wrapper function that standardizes hook result processing and process exit codes.
pub async fn handle_hook_result<F, Fut>(handler: F) -> Result<()>
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = Result<HookResult>>,
{
    match handler().await {
        Ok(result) => {
            if result.blocked.unwrap_or(false) && result.message.is_some() {
                eprintln!("❌ Hook blocked: {}", result.message.unwrap());
                std::process::exit(2);
            }
            std::process::exit(0);
        }
        Err(error) => {
            eprintln!("❌ Hook failed: {}", error);
            std::process::exit(1);
        }
    }
}

/// Handles PreToolUse hook events fired before Claude executes any tool.
pub async fn handle_pre_tool_use() -> Result<HookResult> {
    let payload: PreToolUsePayload = read_payload_from_stdin().await?;
    
    validate_base_payload(&payload.base)
        .map_err(|e| anyhow::anyhow!(e))?;

    if payload.tool_name.is_empty() {
        return Err(anyhow::anyhow!("Missing required field: tool_name"));
    }

    // Initialize logger
    let logging_config = LoggingConfig::default();
    create_session_logger(&payload.base.session_id, Some(&logging_config))
        .context("Failed to create session logger")?;

    log::info!(
        "Processing PreToolUse hook: session_id={}, tool_name={}",
        payload.base.session_id,
        payload.tool_name
    );

    let file_modifying_tools = vec!["Write", "Edit", "MultiEdit", "NotebookEdit"];

    if file_modifying_tools.contains(&payload.tool_name.as_str()) {
        if let Some(result) = check_file_validation_rules(&payload).await? {
            return Ok(result);
        }
    }

    Ok(HookResult::success())
}

/// Check file validation rules for file-modifying tools
async fn check_file_validation_rules(payload: &PreToolUsePayload) -> Result<Option<HookResult>> {
    let config = get_config().await?;
    
    // Extract file path from tool input
    let file_path = extract_file_path(&payload.tool_input);
    let Some(file_path) = file_path else {
        return Ok(None);
    };

    let cwd = std::env::current_dir().context("Failed to get current working directory")?;
    let resolved_path = cwd.join(&file_path);
    let relative_path = resolved_path.strip_prefix(&cwd)
        .unwrap_or(resolved_path.as_path())
        .to_string_lossy()
        .to_string();

    // Check preventRootAdditions rule - only applies to Write tool
    if config.rules.prevent_root_additions && payload.tool_name == "Write" {
        if is_root_addition(&file_path, &relative_path) {
            let error_message = format!(
                "Blocked {} operation: preventRootAdditions rule prevents creating files at repository root. File: {}",
                payload.tool_name, file_path
            );

            log::warn!(
                "PreToolUse blocked by preventRootAdditions rule: tool_name={}, file_path={}",
                payload.tool_name,
                file_path
            );

            return Ok(Some(HookResult::blocked(error_message)));
        }
    }

    // Check uneditableFiles rule
    for pattern in &config.rules.uneditable_files {
        if matches_uneditable_pattern(&file_path, &relative_path, &resolved_path.to_string_lossy(), pattern)? {
            let error_message = format!(
                "Blocked {} operation: file matches uneditable pattern '{}'. File: {}",
                payload.tool_name, pattern, file_path
            );

            log::warn!(
                "PreToolUse blocked by uneditableFiles rule: tool_name={}, file_path={}, pattern={}",
                payload.tool_name,
                file_path,
                pattern
            );

            return Ok(Some(HookResult::blocked(error_message)));
        }
    }

    Ok(None)
}

/// Extract file path from tool input
pub fn extract_file_path(tool_input: &std::collections::HashMap<String, Value>) -> Option<String> {
    tool_input
        .get("file_path")
        .or_else(|| tool_input.get("notebook_path"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

/// Check if a file path represents a root addition
pub fn is_root_addition(_file_path: &str, relative_path: &str) -> bool {
    let path = Path::new(relative_path);
    let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
    
    // Check if the file is directly in the root directory (no subdirectories)
    let is_in_root = !relative_path.contains(std::path::MAIN_SEPARATOR) 
        && !relative_path.is_empty() 
        && relative_path != "..";

    // Allow dotfiles and configuration files
    let is_config_file = file_name.contains("config") 
        || file_name.contains("settings")
        || file_name == "package.json"
        || file_name == "tsconfig.json"
        || file_name == "bun.lockb"
        || file_name == "bun.lock";

    let is_dotfile = file_name.starts_with('.');

    is_in_root && !is_dotfile && !is_config_file
}

/// Check if a file matches an uneditable pattern
pub fn matches_uneditable_pattern(
    file_path: &str,
    relative_path: &str,
    resolved_path: &str,
    pattern: &str,
) -> Result<bool> {
    let glob_pattern = Pattern::new(pattern)
        .with_context(|| format!("Invalid glob pattern: {}", pattern))?;

    Ok(glob_pattern.matches(file_path)
        || glob_pattern.matches(relative_path)
        || glob_pattern.matches(resolved_path))
}

/// Handles PostToolUse hook events fired after Claude executes a tool.
pub async fn handle_post_tool_use() -> Result<HookResult> {
    let payload: PostToolUsePayload = read_payload_from_stdin().await?;
    
    validate_base_payload(&payload.base)
        .map_err(|e| anyhow::anyhow!(e))?;

    if payload.tool_name.is_empty() {
        return Err(anyhow::anyhow!("Missing required field: tool_name"));
    }

    // Initialize logger
    let logging_config = LoggingConfig::default();
    create_session_logger(&payload.base.session_id, Some(&logging_config))
        .context("Failed to create session logger")?;

    log::info!(
        "Processing PostToolUse hook: session_id={}, tool_name={}",
        payload.base.session_id,
        payload.tool_name
    );

    Ok(HookResult::success())
}

/// Handles Notification hook events when Claude sends system notifications.
pub async fn handle_notification() -> Result<HookResult> {
    let payload: NotificationPayload = read_payload_from_stdin().await?;
    
    validate_base_payload(&payload.base)
        .map_err(|e| anyhow::anyhow!(e))?;

    if payload.message.is_empty() {
        return Err(anyhow::anyhow!("Missing required field: message"));
    }

    // Initialize logger
    let logging_config = LoggingConfig::default();
    create_session_logger(&payload.base.session_id, Some(&logging_config))
        .context("Failed to create session logger")?;

    log::info!(
        "Processing Notification hook: session_id={}, message={}",
        payload.base.session_id,
        payload.message
    );

    Ok(HookResult::success())
}

/// Handles UserPromptSubmit hook events when users submit input to Claude.
pub async fn handle_user_prompt_submit() -> Result<HookResult> {
    let payload: UserPromptSubmitPayload = read_payload_from_stdin().await?;
    
    validate_base_payload(&payload.base)
        .map_err(|e| anyhow::anyhow!(e))?;

    if payload.prompt.is_empty() {
        return Err(anyhow::anyhow!("Missing required field: prompt"));
    }

    // Initialize logger
    let logging_config = LoggingConfig::default();
    create_session_logger(&payload.base.session_id, Some(&logging_config))
        .context("Failed to create session logger")?;

    log::info!(
        "Processing UserPromptSubmit hook: session_id={}",
        payload.base.session_id
    );

    Ok(HookResult::success())
}

/// Handles SessionStart hook events when a new Claude session begins.
pub async fn handle_session_start() -> Result<HookResult> {
    let payload: SessionStartPayload = read_payload_from_stdin().await?;
    
    validate_base_payload(&payload.base)
        .map_err(|e| anyhow::anyhow!(e))?;

    if payload.source.is_empty() {
        return Err(anyhow::anyhow!("Missing required field: source"));
    }

    // Initialize logger
    let logging_config = LoggingConfig::default();
    create_session_logger(&payload.base.session_id, Some(&logging_config))
        .context("Failed to create session logger")?;

    log::info!(
        "Processing SessionStart hook: session_id={}, source={}",
        payload.base.session_id,
        payload.source
    );

    Ok(HookResult::success())
}

/// Handles Stop hook events when a Claude session is terminating.
pub async fn handle_stop() -> Result<HookResult> {
    let payload: StopPayload = read_payload_from_stdin().await?;
    
    validate_base_payload(&payload.base)
        .map_err(|e| anyhow::anyhow!(e))?;

    // Initialize logger
    let logging_config = LoggingConfig::default();
    create_session_logger(&payload.base.session_id, Some(&logging_config))
        .context("Failed to create session logger")?;

    log::info!(
        "Processing Stop hook: session_id={}",
        payload.base.session_id
    );

    // Extract and execute commands from config.stop.run
    let config = get_config().await?;
    let commands = extract_bash_commands(&config.stop.run)?;

    log::info!("Executing {} stop hook commands", commands.len());

    for (index, command) in commands.iter().enumerate() {
        log::info!("Executing command {}/{}: {}", index + 1, commands.len(), command);

        let child = TokioCommand::new("bash")
            .arg("-c")
            .arg(command)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .with_context(|| format!("Failed to spawn command: {}", command))?;

        let output = child.wait_with_output().await
            .with_context(|| format!("Failed to wait for command: {}", command))?;

        if !output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            let exit_code = output.status.code().unwrap_or(1);
            
            let stdout_section = if !stdout.is_empty() {
                format!("\nStdout: {}", stdout)
            } else {
                String::new()
            };
            
            let stderr_section = if !stderr.is_empty() {
                format!("\nStderr: {}", stderr)
            } else {
                String::new()
            };
            
            let error_message = format!(
                "Command failed with exit code {}: {}{}{}",
                exit_code, command, stdout_section, stderr_section
            );

            log::error!("Stop hook command failed: {}", error_message);
            return Ok(HookResult::blocked(error_message));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        log::info!("Command completed successfully: {}", command);
        if !stdout.trim().is_empty() {
            log::info!("Command output: {}", stdout.trim());
        }
    }

    log::info!("All stop hook commands completed successfully");

    // Check if infinite mode is enabled
    if config.stop.infinite {
        let infinite_message = config.stop.infinite_message
            .as_deref()
            .unwrap_or("continue working on the task");
        
        log::info!("Infinite mode enabled, sending continuation message: {}", infinite_message);
        return Ok(HookResult::blocked(infinite_message.to_string()));
    }

    Ok(HookResult::success())
}

/// Handles SubagentStop hook events when Claude subagents complete their tasks.
pub async fn handle_subagent_stop() -> Result<HookResult> {
    let payload: SubagentStopPayload = read_payload_from_stdin().await?;
    
    validate_base_payload(&payload.base)
        .map_err(|e| anyhow::anyhow!(e))?;

    // Initialize logger
    let logging_config = LoggingConfig::default();
    create_session_logger(&payload.base.session_id, Some(&logging_config))
        .context("Failed to create session logger")?;

    log::info!(
        "Processing SubagentStop hook: session_id={}",
        payload.base.session_id
    );

    Ok(HookResult::success())
}

/// Handles PreCompact hook events before transcript compaction occurs.
pub async fn handle_pre_compact() -> Result<HookResult> {
    let payload: PreCompactPayload = read_payload_from_stdin().await?;
    
    validate_base_payload(&payload.base)
        .map_err(|e| anyhow::anyhow!(e))?;

    // Initialize logger
    let logging_config = LoggingConfig::default();
    create_session_logger(&payload.base.session_id, Some(&logging_config))
        .context("Failed to create session logger")?;

    log::info!(
        "Processing PreCompact hook: session_id={}, trigger={:?}",
        payload.base.session_id,
        payload.trigger
    );

    Ok(HookResult::success())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_root_addition() {
        assert!(is_root_addition("test.txt", "test.txt"));
        assert!(!is_root_addition(".gitignore", ".gitignore"));
        assert!(!is_root_addition("package.json", "package.json"));
        assert!(!is_root_addition("src/test.txt", "src/test.txt"));
        assert!(!is_root_addition("config.yaml", "config.yaml"));
    }

    #[test]
    fn test_matches_uneditable_pattern() {
        assert!(matches_uneditable_pattern("package.json", "package.json", "/path/package.json", "package.json").unwrap());
        assert!(matches_uneditable_pattern("test.md", "test.md", "/path/test.md", "*.md").unwrap());
        assert!(matches_uneditable_pattern("src/index.ts", "src/index.ts", "/path/src/index.ts", "src/**/*.ts").unwrap());
        assert!(!matches_uneditable_pattern("other.txt", "other.txt", "/path/other.txt", "*.md").unwrap());
    }

    #[test]
    fn test_extract_file_path() {
        let mut tool_input = std::collections::HashMap::new();
        tool_input.insert("file_path".to_string(), Value::String("test.txt".to_string()));
        assert_eq!(extract_file_path(&tool_input), Some("test.txt".to_string()));

        tool_input.clear();
        tool_input.insert("notebook_path".to_string(), Value::String("notebook.ipynb".to_string()));
        assert_eq!(extract_file_path(&tool_input), Some("notebook.ipynb".to_string()));

        tool_input.clear();
        assert_eq!(extract_file_path(&tool_input), None);
    }
}