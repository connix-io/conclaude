use crate::config::{ConclaudeConfig, extract_bash_commands, load_conclaude_config};
use crate::logger::create_session_logger;
use crate::types::{
    HookResult, LoggingConfig, NotificationPayload, PostToolUsePayload, PreCompactPayload,
    PreToolUsePayload, SessionStartPayload, StopPayload, SubagentStopPayload,
    UserPromptSubmitPayload, validate_base_payload,
};
use anyhow::{Context, Result};
use glob::Pattern;
use serde_json::Value;
use std::collections::HashSet;
use std::fs;
use std::io::{self, Read};
use std::path::Path;
use std::process::Stdio;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicU32, Ordering};
use tokio::process::Command as TokioCommand;

/// Represents a stop command with its configuration
struct StopCommandConfig {
    command: String,
    message: Option<String>,
    show_stdout: bool,
    show_stderr: bool,
}

/// Cached configuration instance to avoid repeated loads
static CACHED_CONFIG: OnceLock<ConclaudeConfig> = OnceLock::new();

/// Load configuration with caching to avoid repeated file system operations
///
/// # Errors
///
/// Returns an error if the configuration file cannot be loaded or parsed.
async fn get_config() -> Result<&'static ConclaudeConfig> {
    if let Some(config) = CACHED_CONFIG.get() {
        Ok(config)
    } else {
        let config = load_conclaude_config().await?;
        Ok(CACHED_CONFIG.get_or_init(|| config))
    }
}

/// Reads and validates hook payload from stdin, creating a session-specific logger.
///
/// # Errors
///
/// Returns an error if reading from stdin fails or if the JSON payload cannot be parsed.
pub fn read_payload_from_stdin<T>() -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .context("Failed to read from stdin")?;

    let payload: T =
        serde_json::from_str(&buffer).context("Failed to parse JSON payload from stdin")?;

    Ok(payload)
}

/// Wrapper function that standardizes hook result processing and process exit codes.
///
/// # Errors
///
/// Returns an error if the hook handler fails to execute.
///
/// # Panics
///
/// This function does not panic - the `unwrap()` call is guarded by `is_some()` check.
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
            eprintln!("❌ Hook failed: {error}");
            std::process::exit(1);
        }
    }
}

/// Handles `PreToolUse` hook events fired before Claude executes any tool.
///
/// # Errors
///
/// Returns an error if payload validation fails, logger creation fails, or configuration loading fails.
pub async fn handle_pre_tool_use() -> Result<HookResult> {
    let payload: PreToolUsePayload = read_payload_from_stdin()?;

    validate_base_payload(&payload.base).map_err(|e| anyhow::anyhow!(e))?;

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

    // Check tool usage validation rules
    if let Some(result) = check_tool_usage_rules(&payload).await? {
        return Ok(result);
    }

    let file_modifying_tools = ["Write", "Edit", "MultiEdit", "NotebookEdit"];

    if file_modifying_tools.contains(&payload.tool_name.as_str()) {
        if let Some(result) = check_file_validation_rules(&payload).await? {
            return Ok(result);
        }

        // Check if file is auto-generated and should not be edited
        if let Some(result) = check_auto_generated_file(&payload).await? {
            return Ok(result);
        }
    }

    Ok(HookResult::success())
}

/// Check file validation rules for file-modifying tools
///
/// # Errors
///
/// Returns an error if configuration loading fails, directory access fails, or glob pattern processing fails.
async fn check_file_validation_rules(payload: &PreToolUsePayload) -> Result<Option<HookResult>> {
    let config = get_config().await?;

    // Extract file path from tool input
    let file_path = extract_file_path(&payload.tool_input);
    let Some(file_path) = file_path else {
        return Ok(None);
    };

    let cwd = std::env::current_dir().context("Failed to get current working directory")?;
    let resolved_path = cwd.join(&file_path);
    let relative_path = resolved_path
        .strip_prefix(&cwd)
        .unwrap_or(resolved_path.as_path())
        .to_string_lossy()
        .to_string();

    // Check preventRootAdditions rule - only applies to Write tool
    if config.rules.prevent_root_additions
        && payload.tool_name == "Write"
        && is_root_addition(&file_path, &relative_path)
    {
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

    // Check uneditableFiles rule
    for pattern in &config.rules.uneditable_files {
        if matches_uneditable_pattern(
            &file_path,
            &relative_path,
            &resolved_path.to_string_lossy(),
            pattern,
        )? {
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
pub fn extract_file_path<S: std::hash::BuildHasher>(
    tool_input: &std::collections::HashMap<String, Value, S>,
) -> Option<String> {
    tool_input
        .get("file_path")
        .or_else(|| tool_input.get("notebook_path"))
        .and_then(|v| v.as_str())
        .map(std::string::ToString::to_string)
}

/// Check if a file path represents a root addition
#[must_use]
pub fn is_root_addition(_file_path: &str, relative_path: &str) -> bool {
    let path = Path::new(relative_path);
    let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

    // Check if the file is directly in the root directory (no subdirectories)
    let is_in_root = !relative_path.contains(std::path::MAIN_SEPARATOR)
        && !relative_path.is_empty()
        && relative_path != "..";

    // Allow dotfiles and configuration files
    // TODO: REMOVE THIS HACK
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
///
/// # Errors
///
/// Returns an error if the glob pattern is invalid.
pub fn matches_uneditable_pattern(
    file_path: &str,
    relative_path: &str,
    resolved_path: &str,
    pattern: &str,
) -> Result<bool> {
    let glob_pattern =
        Pattern::new(pattern).with_context(|| format!("Invalid glob pattern: {pattern}"))?;

    Ok(glob_pattern.matches(file_path)
        || glob_pattern.matches(relative_path)
        || glob_pattern.matches(resolved_path))
}

/// Handles `PostToolUse` hook events fired after Claude executes a tool.
///
/// # Errors
///
/// Returns an error if payload validation fails or logger creation fails.
#[allow(clippy::unused_async)]
pub async fn handle_post_tool_use() -> Result<HookResult> {
    let payload: PostToolUsePayload = read_payload_from_stdin()?;

    validate_base_payload(&payload.base).map_err(|e| anyhow::anyhow!(e))?;

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

/// Handles `Notification` hook events when Claude sends system notifications.
///
/// # Errors
///
/// Returns an error if payload validation fails or logger creation fails.
#[allow(clippy::unused_async)]
pub async fn handle_notification() -> Result<HookResult> {
    let payload: NotificationPayload = read_payload_from_stdin()?;

    validate_base_payload(&payload.base).map_err(|e| anyhow::anyhow!(e))?;

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

/// Handles `UserPromptSubmit` hook events when users submit input to Claude.
///
/// # Errors
///
/// Returns an error if payload validation fails or logger creation fails.
#[allow(clippy::unused_async)]
pub async fn handle_user_prompt_submit() -> Result<HookResult> {
    let payload: UserPromptSubmitPayload = read_payload_from_stdin()?;

    validate_base_payload(&payload.base).map_err(|e| anyhow::anyhow!(e))?;

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

/// Handles `SessionStart` hook events when a new Claude session begins.
///
/// # Errors
///
/// Returns an error if payload validation fails or logger creation fails.
#[allow(clippy::unused_async)]
pub async fn handle_session_start() -> Result<HookResult> {
    let payload: SessionStartPayload = read_payload_from_stdin()?;

    validate_base_payload(&payload.base).map_err(|e| anyhow::anyhow!(e))?;

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

/// Collect stop commands from configuration
///
/// # Errors
///
/// Returns an error if bash command extraction fails.
fn collect_stop_commands(config: &ConclaudeConfig) -> Result<Vec<StopCommandConfig>> {
    let mut commands = Vec::new();

    // Add legacy run commands
    if !config.stop.run.is_empty() {
        let extracted = extract_bash_commands(&config.stop.run)?;
        for cmd in extracted {
            commands.push(StopCommandConfig {
                command: cmd,
                message: None,
                show_stdout: false,
                show_stderr: false,
            });
        }
    }

    // Add new structured commands with messages
    for cmd_config in &config.stop.commands {
        let extracted = extract_bash_commands(&cmd_config.run)?;
        let show_stdout = cmd_config.show_stdout.unwrap_or(false);
        let show_stderr = cmd_config.show_stderr.unwrap_or(false);
        for cmd in extracted {
            commands.push(StopCommandConfig {
                command: cmd,
                message: cmd_config.message.clone(),
                show_stdout,
                show_stderr,
            });
        }
    }

    Ok(commands)
}

/// Execute stop hook commands
///
/// # Errors
///
/// Returns an error if command execution fails or process spawning fails.
async fn execute_stop_commands(
    commands: &[StopCommandConfig],
) -> Result<Option<HookResult>> {
    log::info!(
        "Executing {} stop hook commands",
        commands.len()
    );

    for (index, cmd_config) in commands.iter().enumerate() {
        log::info!(
            "Executing command {}/{}: {}",
            index + 1,
            commands.len(),
            cmd_config.command
        );

        let child = TokioCommand::new("bash")
            .arg("-c")
            .arg(&cmd_config.command)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .with_context(|| format!("Failed to spawn command: {}", cmd_config.command))?;

        let output = child
            .wait_with_output()
            .await
            .with_context(|| format!("Failed to wait for command: {}", cmd_config.command))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if !output.status.success() {
            let exit_code = output.status.code().unwrap_or(1);
            let separator = "-".repeat(60);

            // Log detailed failure information with command and outputs appended
            log::error!(
                "Stop command failed:\n{}\n  Command: {}\n  Status: Failed (exit code: {})\n  Stdout:\n{}\n  Stderr:\n{}\n{}",
                separator,
                cmd_config.command,
                exit_code,
                if stdout.trim().is_empty() {
                    "    (no stdout)".to_string()
                } else {
                    stdout
                        .trim()
                        .lines()
                        .map(|line| format!("    {}", line))
                        .collect::<Vec<_>>()
                        .join("\n")
                },
                if stderr.trim().is_empty() {
                    "    (no stderr)".to_string()
                } else {
                    stderr
                        .trim()
                        .lines()
                        .map(|line| format!("    {}", line))
                        .collect::<Vec<_>>()
                        .join("\n")
                },
                separator
            );

            let stdout_section = if cmd_config.show_stdout && !stdout.is_empty() {
                format!("\nStdout: {stdout}")
            } else {
                String::new()
            };

            let stderr_section = if cmd_config.show_stderr && !stderr.is_empty() {
                format!("\nStderr: {stderr}")
            } else {
                String::new()
            };

            let error_message = if let Some(custom_msg) = &cmd_config.message {
                format!("{custom_msg}{stdout_section}{stderr_section}")
            } else {
                format!(
                    "Command failed with exit code {exit_code}: {}{stdout_section}{stderr_section}",
                    cmd_config.command
                )
            };

            return Ok(Some(HookResult::blocked(error_message)));
        }

        // Always log command execution details with stdout appended
        let separator = "-".repeat(60);
        log::info!(
            "Stop command executed:\n{}\n  Command: {}\n  Status: Success\n  Output:\n{}\n{}",
            separator,
            cmd_config.command,
            if stdout.trim().is_empty() {
                "    (no output)".to_string()
            } else {
                stdout
                    .trim()
                    .lines()
                    .map(|line| format!("    {}", line))
                    .collect::<Vec<_>>()
                    .join("\n")
            },
            separator
        );

        // If showStdout or showStderr is true, print to stdout/stderr for user/Claude to see
        if cmd_config.show_stdout && !stdout.is_empty() {
            print!("{stdout}");
        }
        if cmd_config.show_stderr && !stderr.is_empty() {
            eprint!("{stderr}");
        }
    }

    log::info!("All stop hook commands completed successfully");
    Ok(None)
}

/// Handles `Stop` hook events when a Claude session is terminating.
///
/// # Errors
///
/// Returns an error if payload validation fails, logger creation fails, configuration loading fails,
/// command execution fails, or directory operations fail.
pub async fn handle_stop() -> Result<HookResult> {
    // Track rounds for infinite alternative using atomic counter
    static ROUND_COUNT: AtomicU32 = AtomicU32::new(0);

    let payload: StopPayload = read_payload_from_stdin()?;

    validate_base_payload(&payload.base).map_err(|e| anyhow::anyhow!(e))?;

    // Initialize logger
    let logging_config = LoggingConfig::default();
    create_session_logger(&payload.base.session_id, Some(&logging_config))
        .context("Failed to create session logger")?;

    log::info!(
        "Processing Stop hook: session_id={}",
        payload.base.session_id
    );

    let config = get_config().await?;

    // Snapshot root directory if preventRootAdditions is enabled
    let root_snapshot = if config.rules.prevent_root_additions {
        Some(snapshot_root_directory()?)
    } else {
        None
    };

    // Extract and execute commands from config.stop.run and config.stop.commands
    let commands_with_messages = collect_stop_commands(config)?;

    // Execute commands
    if let Some(result) = execute_stop_commands(&commands_with_messages).await? {
        return Ok(result);
    }

    // Check root additions if enabled
    if let Some(snapshot) = root_snapshot {
        if let Some(result) = check_root_additions(&snapshot)? {
            return Ok(result);
        }
    }

    // Check rounds mode (alternative to infinite)
    if let Some(max_rounds) = config.stop.rounds {
        let current_round = ROUND_COUNT.fetch_add(1, Ordering::SeqCst) + 1;
        if current_round < max_rounds {
            let message = format!("Round {current_round}/{max_rounds} completed, continuing...");
            log::info!("{message}");
            return Ok(HookResult::blocked(message));
        }
        ROUND_COUNT.store(0, Ordering::SeqCst); // Reset for next session
    }

    // Check if infinite mode is enabled
    if config.stop.infinite {
        let infinite_message = config
            .stop
            .infinite_message
            .as_deref()
            .unwrap_or("continue working on the task");

        log::info!("Infinite mode enabled, sending continuation message: {infinite_message}");
        return Ok(HookResult::blocked(infinite_message.to_string()));
    }

    Ok(HookResult::success())
}

/// Handles `SubagentStop` hook events when Claude subagents complete their tasks.
///
/// # Errors
///
/// Returns an error if payload validation fails or logger creation fails.
#[allow(clippy::unused_async)]
pub async fn handle_subagent_stop() -> Result<HookResult> {
    let payload: SubagentStopPayload = read_payload_from_stdin()?;

    validate_base_payload(&payload.base).map_err(|e| anyhow::anyhow!(e))?;

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

/// Handles `PreCompact` hook events before transcript compaction occurs.
///
/// # Errors
///
/// Returns an error if payload validation fails or logger creation fails.
#[allow(clippy::unused_async)]
pub async fn handle_pre_compact() -> Result<HookResult> {
    let payload: PreCompactPayload = read_payload_from_stdin()?;

    validate_base_payload(&payload.base).map_err(|e| anyhow::anyhow!(e))?;

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

/// Check tool usage validation rules
///
/// # Errors
///
/// Returns an error if configuration loading fails or glob pattern creation fails.
async fn check_tool_usage_rules(payload: &PreToolUsePayload) -> Result<Option<HookResult>> {
    let config = get_config().await?;

    for rule in &config.rules.tool_usage_validation {
        if rule.tool == payload.tool_name || rule.tool == "*" {
            // Extract file path if available
            if let Some(file_path) = extract_file_path(&payload.tool_input) {
                let matches = Pattern::new(&rule.pattern)?.matches(&file_path);

                if (rule.action == "block" && matches) || (rule.action == "allow" && !matches) {
                    let message = rule.message.clone().unwrap_or_else(|| {
                        format!("Tool usage blocked by validation rule: {}", rule.pattern)
                    });
                    return Ok(Some(HookResult::blocked(message)));
                }
            }
        }
    }

    Ok(None)
}

/// Check if file contains auto-generated markers
///
/// Returns the marker found if file contains generation markers, None otherwise
#[must_use]
pub fn check_generated_file_markers(content: &str) -> Option<String> {
    // Check first 100 lines for markers
    let lines_to_check: Vec<&str> = content.lines().take(100).collect();
    let content_to_check = lines_to_check.join("\n");

    // List of markers to check (case-insensitive)
    let markers = [
        "DO NOT EDIT",
        "do not edit",
        "Code generated by",
        "Auto-generated",
        "Autogenerated",
        "Generated code",
        "@generated",
        "This file is generated",
        "This file was generated",
    ];

    for marker in &markers {
        if content_to_check
            .to_lowercase()
            .contains(&marker.to_lowercase())
        {
            // Find the actual marker text in the file for accurate reporting
            for line in &lines_to_check {
                if line.to_lowercase().contains(&marker.to_lowercase()) {
                    // Extract the actual marker phrase from the line
                    let lower_line = line.to_lowercase();
                    let lower_marker = marker.to_lowercase();
                    if let Some(pos) = lower_line.find(&lower_marker) {
                        // Get the actual text from the original line
                        let actual_marker = &line[pos..pos + marker.len()];
                        return Some(actual_marker.to_string());
                    }
                }
            }
            // Fallback to the marker as-is if we can't extract the exact text
            return Some((*marker).to_string());
        }
    }

    None
}

/// Check if file is auto-generated and should not be edited
///
/// # Errors
///
/// Returns an error if configuration loading fails or file cannot be read.
pub async fn check_auto_generated_file(payload: &PreToolUsePayload) -> Result<Option<HookResult>> {
    let config = get_config().await?;

    // Only check if the feature is enabled
    if !config.pre_tool_use.prevent_generated_file_edits {
        return Ok(None);
    }

    // Extract file path from tool input
    let file_path = extract_file_path(&payload.tool_input);
    let Some(file_path) = file_path else {
        return Ok(None);
    };

    // For Write tool, check if file exists first (new files can't be auto-generated)
    if payload.tool_name == "Write" && !Path::new(&file_path).exists() {
        return Ok(None);
    }

    // Read file content
    let Ok(content) = fs::read_to_string(&file_path) else {
        return Ok(None); // File doesn't exist or can't be read, allow operation
    };

    // Check for auto-generated markers
    if let Some(marker) = check_generated_file_markers(&content) {
        // Use custom message or default
        let message = if let Some(custom_msg) = &config.pre_tool_use.generated_file_message {
            custom_msg
                .replace("{file_path}", &file_path)
                .replace("{marker}", &marker)
        } else {
            format!(
                "BLOCKED: File '{file_path}' is auto-generated (contains '{marker}'). This file should NEVER be edited directly. Modifications should be made to the source/template that generates this file."
            )
        };

        log::warn!(
            "PreToolUse blocked auto-generated file edit: tool_name={}, file_path={}, marker={}",
            payload.tool_name,
            file_path,
            marker
        );

        return Ok(Some(HookResult::blocked(message)));
    }

    Ok(None)
}

/// Snapshot the root directory
///
/// # Errors
///
/// Returns an error if the current directory cannot be read.
fn snapshot_root_directory() -> Result<HashSet<String>> {
    let mut snapshot = HashSet::new();

    for entry in (fs::read_dir(".")?).flatten() {
        if let Ok(file_name) = entry.file_name().into_string() {
            snapshot.insert(file_name);
        }
    }

    Ok(snapshot)
}

/// Check for new additions to the root directory
///
/// # Errors
///
/// Returns an error if the current directory cannot be read.
fn check_root_additions(snapshot: &HashSet<String>) -> Result<Option<HookResult>> {
    let mut new_files = Vec::new();

    for entry in (fs::read_dir(".")?).flatten() {
        if let Ok(file_name) = entry.file_name().into_string() {
            if !snapshot.contains(&file_name) && !file_name.starts_with('.') {
                new_files.push(file_name);
            }
        }
    }

    if !new_files.is_empty() {
        let message = format!(
            "Unauthorized root additions detected: {}",
            new_files.join(", ")
        );
        return Ok(Some(HookResult::blocked(message)));
    }

    Ok(None)
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
        assert!(
            matches_uneditable_pattern(
                "package.json",
                "package.json",
                "/path/package.json",
                "package.json"
            )
            .unwrap()
        );
        assert!(matches_uneditable_pattern("test.md", "test.md", "/path/test.md", "*.md").unwrap());
        assert!(
            matches_uneditable_pattern(
                "src/index.ts",
                "src/index.ts",
                "/path/src/index.ts",
                "src/**/*.ts"
            )
            .unwrap()
        );
        assert!(
            !matches_uneditable_pattern("other.txt", "other.txt", "/path/other.txt", "*.md")
                .unwrap()
        );
    }

    #[test]
    fn test_extract_file_path() {
        let mut tool_input = std::collections::HashMap::new();
        tool_input.insert(
            "file_path".to_string(),
            Value::String("test.txt".to_string()),
        );
        assert_eq!(extract_file_path(&tool_input), Some("test.txt".to_string()));

        tool_input.clear();
        tool_input.insert(
            "notebook_path".to_string(),
            Value::String("notebook.ipynb".to_string()),
        );
        assert_eq!(
            extract_file_path(&tool_input),
            Some("notebook.ipynb".to_string())
        );

        tool_input.clear();
        assert_eq!(extract_file_path(&tool_input), None);
    }
}
