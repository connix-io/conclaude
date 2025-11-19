use crate::config::{extract_bash_commands, load_conclaude_config, ConclaudeConfig};
use crate::types::{
    validate_base_payload, validate_subagent_start_payload, validate_subagent_stop_payload,
    HookResult, NotificationPayload, PostToolUsePayload, PreCompactPayload, PreToolUsePayload,
    SessionEndPayload, SessionStartPayload, StopPayload, SubagentStartPayload, SubagentStopPayload,
    UserPromptSubmitPayload,
};
use anyhow::{Context, Result};
use glob::Pattern;
use notify_rust::{Notification, Urgency};
use serde_json::Value;
use std::collections::HashSet;
use std::fs;
use std::io::{self, Read};
use std::path::Path;
use std::process::Stdio;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::OnceLock;
use tokio::process::Command as TokioCommand;

/// Represents a stop command with its configuration
struct StopCommandConfig {
    command: String,
    message: Option<String>,
    show_stdout: bool,
    show_stderr: bool,
    max_output_lines: Option<u32>,
}

/// Cached configuration instance to avoid repeated loads
static CACHED_CONFIG: OnceLock<(ConclaudeConfig, std::path::PathBuf)> = OnceLock::new();

/// Determine if a hook is a system event hook
///
/// System event hooks are hooks that track session lifecycle and user interactions,
/// as opposed to tool execution or validation hooks.
///
/// # Arguments
///
/// * `hook_name` - The name of the hook to check
///
/// # Returns
///
/// `true` if the hook is a system event hook, `false` otherwise
#[must_use]
fn is_system_event_hook(hook_name: &str) -> bool {
    matches!(
        hook_name,
        "SessionStart"
            | "SessionEnd"
            | "UserPromptSubmit"
            | "SubagentStart"
            | "SubagentStop"
            | "PreCompact"
    )
}

/// Load configuration with caching to avoid repeated file system operations
///
/// # Errors
///
/// Returns an error if the configuration file cannot be loaded or parsed.
async fn get_config() -> Result<&'static (ConclaudeConfig, std::path::PathBuf)> {
    if let Some(config) = CACHED_CONFIG.get() {
        Ok(config)
    } else {
        let config = load_conclaude_config(None).await?;
        Ok(CACHED_CONFIG.get_or_init(|| config))
    }
}

/// Send a system notification for hook execution
///
/// This function sends a system notification when a hook is executed.
/// It gracefully handles errors and logs failures without blocking hook execution.
///
/// # Arguments
///
/// * `hook_name` - The name of the hook being executed
/// * `status` - The execution status ("success" or "failure")
/// * `context` - Optional additional context about the execution
fn send_notification(hook_name: &str, status: &str, context: Option<&str>) {
    // Get configuration to check if notifications are enabled for this hook
    let config_future = get_config();

    // Use tokio::task::block_in_place to safely block in async context
    let config_result =
        tokio::task::block_in_place(|| tokio::runtime::Handle::current().block_on(config_future));

    let (config, _) = match config_result {
        Ok(config) => config,
        Err(e) => {
            // Silently continue if config can't be loaded - notifications are not critical
            eprintln!("Failed to load config for notification: {e}");
            return;
        }
    };

    // Check if notifications are enabled for this hook
    if !config.notifications.is_enabled_for(hook_name) {
        return;
    }

    // Check notification flags based on hook type and status
    let notifications_config = &config.notifications;

    // Determine if this hook should show based on the appropriate flag
    let should_show = match status {
        "failure" => notifications_config.show_errors,
        "success" => notifications_config.show_success,
        _ => {
            // For other statuses, determine if this is a system event hook
            is_system_event_hook(hook_name) && notifications_config.show_system_events
        }
    };

    // Short-circuit if the appropriate flag is false
    if !should_show {
        return;
    }

    // Format notification title and body
    let title = format!("Conclaude - {}", hook_name);
    let body = match context {
        Some(ctx) => format!("{}: {}", status, ctx),
        None => match status {
            "success" => "All checks passed".to_string(),
            "failure" => "Command failed".to_string(),
            _ => format!("Hook completed with status: {}", status),
        },
    };

    // Set urgency based on status (Linux only)
    #[cfg(target_os = "linux")]
    let urgency = if status == "failure" {
        Urgency::Critical
    } else {
        Urgency::Normal
    };

    // Send notification with error handling
    #[cfg(target_os = "linux")]
    let result = Notification::new()
        .summary(&title)
        .body(&body)
        .urgency(urgency)
        .show();

    #[cfg(not(target_os = "linux"))]
    let result = Notification::new().summary(&title).body(&body).show();

    match result {
        Ok(_) => {
            // Notification sent successfully
        }
        Err(e) => {
            // Log the error but don't fail the hook
            eprintln!("Failed to send system notification for hook '{hook_name}': {e}");
        }
    }
}

/// Reads and deserializes the hook payload from stdin.
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
                eprintln!("{}", result.message.unwrap());
                std::process::exit(2);
            }
            std::process::exit(0);
        }
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}

/// Handles `PreToolUse` hook events fired before Claude executes any tool.
///
/// # Errors
///
/// Returns an error if payload validation fails or configuration loading fails.
pub async fn handle_pre_tool_use() -> Result<HookResult> {
    let payload: PreToolUsePayload = read_payload_from_stdin()?;

    validate_base_payload(&payload.base).map_err(|e| anyhow::anyhow!(e))?;

    if payload.tool_name.is_empty() {
        return Err(anyhow::anyhow!("Missing required field: tool_name"));
    }

    println!(
        "Processing PreToolUse hook: session_id={}, tool_name={}",
        payload.base.session_id, payload.tool_name
    );

    // Check tool usage validation rules
    if let Some(result) = check_tool_usage_rules(&payload).await? {
        send_notification(
            "PreToolUse",
            "failure",
            Some(&format!(
                "Tool '{}' blocked by validation rules",
                payload.tool_name
            )),
        );
        return Ok(result);
    }

    let file_modifying_tools = ["Write", "Edit", "MultiEdit", "NotebookEdit"];

    if file_modifying_tools.contains(&payload.tool_name.as_str()) {
        if let Some(result) = check_file_validation_rules(&payload).await? {
            send_notification(
                "PreToolUse",
                "failure",
                Some(&format!(
                    "File validation failed for tool '{}'",
                    payload.tool_name
                )),
            );
            return Ok(result);
        }

        // Check if file is auto-generated and should not be edited
        if let Some(result) = check_auto_generated_file(&payload).await? {
            send_notification(
                "PreToolUse",
                "failure",
                Some(&format!(
                    "Auto-generated file protection blocked tool '{}'",
                    payload.tool_name
                )),
            );
            return Ok(result);
        }
    }

    // Send notification for successful pre-tool-use validation
    send_notification(
        "PreToolUse",
        "success",
        Some(&format!("Tool '{}' approved", payload.tool_name)),
    );
    Ok(HookResult::success())
}

/// Check file validation rules for file-modifying tools
///
/// # Errors
///
/// Returns an error if configuration loading fails, directory access fails, or glob pattern processing fails.
async fn check_file_validation_rules(payload: &PreToolUsePayload) -> Result<Option<HookResult>> {
    let (config, config_path) = get_config().await?;

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
        && is_root_addition(&file_path, &relative_path, config_path)
    {
        let error_message = format!(
            "Blocked {} operation: preventRootAdditions rule prevents creating files at repository root. File: {}",
            payload.tool_name, file_path
        );

        eprintln!(
            "PreToolUse blocked by preventRootAdditions rule: tool_name={}, file_path={}",
            payload.tool_name, file_path
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

            eprintln!(
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

/// Extracts the Bash command string from tool input payload
/// Returns None if the command is missing, empty, or contains only whitespace
pub fn extract_bash_command<S: std::hash::BuildHasher>(
    tool_input: &std::collections::HashMap<String, Value, S>,
) -> Option<String> {
    tool_input
        .get("command")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(std::string::ToString::to_string)
}

/// Check if a file path represents a root addition
///
/// A file is considered a root addition if it's being created at the same directory
/// level as the .conclaude.yaml config file.
#[must_use]
pub fn is_root_addition(_file_path: &str, relative_path: &str, config_path: &Path) -> bool {
    // Handle edge cases - empty paths and parent directory references
    if relative_path.is_empty() || relative_path == ".." {
        return false;
    }

    // Get the directory containing the config file
    let config_dir = config_path.parent().unwrap_or(Path::new("."));

    // Get the current working directory
    let Ok(cwd) = std::env::current_dir() else {
        return false;
    };

    // Resolve the full path of the file being created
    let resolved_file_path = cwd.join(relative_path);

    // Get the directory that will contain the new file
    let file_parent_dir = resolved_file_path.parent().unwrap_or(&cwd);

    // Compare the canonical paths if possible, otherwise compare as-is
    let config_dir_canonical = config_dir
        .canonicalize()
        .unwrap_or_else(|_| config_dir.to_path_buf());
    let file_dir_canonical = file_parent_dir
        .canonicalize()
        .unwrap_or_else(|_| file_parent_dir.to_path_buf());

    // Block if the file is being created in the same directory as the config
    config_dir_canonical == file_dir_canonical
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
/// Returns an error if payload validation fails or configuration loading fails.
#[allow(clippy::unused_async)]
pub async fn handle_post_tool_use() -> Result<HookResult> {
    let payload: PostToolUsePayload = read_payload_from_stdin()?;

    validate_base_payload(&payload.base).map_err(|e| anyhow::anyhow!(e))?;

    if payload.tool_name.is_empty() {
        return Err(anyhow::anyhow!("Missing required field: tool_name"));
    }

    println!(
        "Processing PostToolUse hook: session_id={}, tool_name={}",
        payload.base.session_id, payload.tool_name
    );

    // Send notification for post tool use completion
    send_notification(
        "PostToolUse",
        "success",
        Some(&format!("Tool '{}' completed", payload.tool_name)),
    );
    Ok(HookResult::success())
}

/// Handles `Notification` hook events when Claude sends system notifications.
///
/// # Errors
///
/// Returns an error if payload validation fails or configuration loading fails.
#[allow(clippy::unused_async)]
pub async fn handle_notification() -> Result<HookResult> {
    let payload: NotificationPayload = read_payload_from_stdin()?;

    validate_base_payload(&payload.base).map_err(|e| anyhow::anyhow!(e))?;

    if payload.message.is_empty() {
        return Err(anyhow::anyhow!("Missing required field: message"));
    }

    println!(
        "Processing Notification hook: session_id={}, message={}",
        payload.base.session_id, payload.message
    );

    // Send notification for notification hook processing
    send_notification(
        "Notification",
        "success",
        Some(&format!("Message: {}", payload.message)),
    );
    Ok(HookResult::success())
}

/// Handles `UserPromptSubmit` hook events when users submit input to Claude.
///
/// # Errors
///
/// Returns an error if payload validation fails or configuration loading fails.
#[allow(clippy::unused_async)]
pub async fn handle_user_prompt_submit() -> Result<HookResult> {
    let payload: UserPromptSubmitPayload = read_payload_from_stdin()?;

    validate_base_payload(&payload.base).map_err(|e| anyhow::anyhow!(e))?;

    if payload.prompt.is_empty() {
        return Err(anyhow::anyhow!("Missing required field: prompt"));
    }

    println!(
        "Processing UserPromptSubmit hook: session_id={}",
        payload.base.session_id
    );

    // Send notification for user prompt submission
    send_notification("UserPromptSubmit", "success", Some("User input received"));
    Ok(HookResult::success())
}

/// Handles `SessionStart` hook events when a new Claude session begins.
///
/// # Errors
///
/// Returns an error if payload validation fails or configuration loading fails.
#[allow(clippy::unused_async)]
pub async fn handle_session_start() -> Result<HookResult> {
    let payload: SessionStartPayload = read_payload_from_stdin()?;

    validate_base_payload(&payload.base).map_err(|e| anyhow::anyhow!(e))?;

    if payload.source.is_empty() {
        return Err(anyhow::anyhow!("Missing required field: source"));
    }

    println!(
        "Processing SessionStart hook: session_id={}, source={}",
        payload.base.session_id, payload.source
    );

    // Send notification for session start
    send_notification(
        "SessionStart",
        "success",
        Some(&format!("Session started from {}", payload.source)),
    );
    Ok(HookResult::success())
}

/// Handles `SessionEnd` hook events when a Claude session terminates.
///
/// # Errors
///
/// Returns an error if payload validation fails or configuration loading fails.
#[allow(clippy::unused_async)]
pub async fn handle_session_end() -> Result<HookResult> {
    let payload: SessionEndPayload = read_payload_from_stdin()?;

    validate_base_payload(&payload.base).map_err(|e| anyhow::anyhow!(e))?;

    if payload.reason.is_empty() {
        return Err(anyhow::anyhow!("Missing required field: reason"));
    }

    println!(
        "Processing SessionEnd hook: session_id={}, reason={}",
        payload.base.session_id, payload.reason
    );

    Ok(HookResult::success())
}

/// Truncate output to a maximum number of lines
///
/// Returns a tuple of (truncated_output, is_truncated, omitted_line_count)
fn truncate_output(output: &str, max_lines: u32) -> (String, bool, usize) {
    let lines: Vec<&str> = output.lines().collect();
    let total_lines = lines.len();
    let max_lines_usize = max_lines as usize;

    if total_lines <= max_lines_usize {
        // No truncation needed
        (output.to_string(), false, 0)
    } else {
        // Take first N lines and calculate omitted count
        let truncated_lines = &lines[..max_lines_usize];
        let omitted_count = total_lines - max_lines_usize;
        let truncated = truncated_lines.join("\n");
        (truncated, true, omitted_count)
    }
}

/// Collect stop commands from configuration
///
/// # Errors
///
/// Returns an error if bash command extraction fails.
fn collect_stop_commands(config: &ConclaudeConfig) -> Result<Vec<StopCommandConfig>> {
    let mut commands = Vec::new();

    // Add structured commands with messages and output control
    for cmd_config in &config.stop.commands {
        let extracted = extract_bash_commands(&cmd_config.run)?;
        let show_stdout = cmd_config.show_stdout.unwrap_or(false);
        let show_stderr = cmd_config.show_stderr.unwrap_or(false);
        let max_output_lines = cmd_config.max_output_lines;
        for cmd in extracted {
            commands.push(StopCommandConfig {
                command: cmd,
                message: cmd_config.message.clone(),
                show_stdout,
                show_stderr,
                max_output_lines,
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
async fn execute_stop_commands(commands: &[StopCommandConfig]) -> Result<Option<HookResult>> {
    println!("Executing {} stop hook commands", commands.len());

    for (index, cmd_config) in commands.iter().enumerate() {
        println!(
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

            // Log detailed failure information with command and outputs appended
            // Respect showStdout and showStderr flags when logging to console
            // Build diagnostic output dynamically to omit sections when flags are false
            let mut diagnostic = format!(
                "Stop command failed:\n  Command: {}\n  Status: Failed (exit code: {})",
                cmd_config.command, exit_code
            );

            // Only include Stdout section if showStdout is true
            if cmd_config.show_stdout {
                let stdout_display = if stdout.trim().is_empty() {
                    "    (no stdout)".to_string()
                } else {
                    stdout
                        .trim()
                        .lines()
                        .map(|line| format!("    {}", line))
                        .collect::<Vec<_>>()
                        .join("\n")
                };
                diagnostic.push_str(&format!("\n  Stdout:\n{}", stdout_display));
            }

            // Only include Stderr section if showStderr is true
            if cmd_config.show_stderr {
                let stderr_display = if stderr.trim().is_empty() {
                    "    (no stderr)".to_string()
                } else {
                    stderr
                        .trim()
                        .lines()
                        .map(|line| format!("    {}", line))
                        .collect::<Vec<_>>()
                        .join("\n")
                };
                diagnostic.push_str(&format!("\n  Stderr:\n{}", stderr_display));
            }

            eprintln!("{}", diagnostic);

            let stdout_section = if cmd_config.show_stdout && !stdout.is_empty() {
                if let Some(max_lines) = cmd_config.max_output_lines {
                    let (truncated, is_truncated, omitted) = truncate_output(&stdout, max_lines);
                    if is_truncated {
                        format!("\nStdout: {}\n... ({} lines omitted)", truncated, omitted)
                    } else {
                        format!("\nStdout: {}", truncated)
                    }
                } else {
                    format!("\nStdout: {}", stdout)
                }
            } else {
                String::new()
            };

            let stderr_section = if cmd_config.show_stderr && !stderr.is_empty() {
                if let Some(max_lines) = cmd_config.max_output_lines {
                    let (truncated, is_truncated, omitted) = truncate_output(&stderr, max_lines);
                    if is_truncated {
                        format!("\nStderr: {}\n... ({} lines omitted)", truncated, omitted)
                    } else {
                        format!("\nStderr: {}", truncated)
                    }
                } else {
                    format!("\nStderr: {}", stderr)
                }
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

        // Successful individual commands produce no output
    }

    println!("All stop hook commands completed successfully");
    Ok(None)
}

/// Handles `Stop` hook events when a Claude session is terminating.
///
/// # Errors
///
/// Returns an error if payload validation fails, configuration loading fails,
/// command execution fails, or directory operations fail.
pub async fn handle_stop() -> Result<HookResult> {
    // Track rounds for infinite alternative using atomic counter
    static ROUND_COUNT: AtomicU32 = AtomicU32::new(0);

    let payload: StopPayload = read_payload_from_stdin()?;

    validate_base_payload(&payload.base).map_err(|e| anyhow::anyhow!(e))?;

    println!(
        "Processing Stop hook: session_id={}",
        payload.base.session_id
    );

    let (config, _config_path) = get_config().await?;

    // Snapshot root directory if preventRootAdditions is enabled
    let root_snapshot = if config.rules.prevent_root_additions {
        Some(snapshot_root_directory()?)
    } else {
        None
    };

    // Extract and execute commands from config.stop.commands
    let commands_with_messages = collect_stop_commands(config)?;

    // Execute commands
    if let Some(result) = execute_stop_commands(&commands_with_messages).await? {
        // Send notification for blocked/failed stop hook
        send_notification(
            "Stop",
            "failure",
            Some(
                &result
                    .message
                    .clone()
                    .unwrap_or_else(|| "Hook blocked".to_string()),
            ),
        );
        return Ok(result);
    }

    // Check root additions if enabled
    if let Some(snapshot) = root_snapshot {
        if let Some(result) = check_root_additions(&snapshot)? {
            // Send notification for blocked root additions
            send_notification(
                "Stop",
                "failure",
                Some(
                    &result
                        .message
                        .clone()
                        .unwrap_or_else(|| "Root additions blocked".to_string()),
                ),
            );
            return Ok(result);
        }
    }

    // Check rounds mode (alternative to infinite)
    if let Some(max_rounds) = config.stop.rounds {
        let current_round = ROUND_COUNT.fetch_add(1, Ordering::SeqCst) + 1;
        if current_round < max_rounds {
            let message = format!("Round {current_round}/{max_rounds} completed, continuing...");
            println!("{message}");
            // Send notification for round completion (not a failure, but continuing)
            send_notification("Stop", "success", Some(&message));
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

        println!("Infinite mode enabled, sending continuation message: {infinite_message}");
        // Send notification for infinite mode continuation
        send_notification(
            "Stop",
            "success",
            Some(&format!("Continuing: {}", infinite_message)),
        );
        return Ok(HookResult::blocked(infinite_message.to_string()));
    }

    // Send notification for successful stop hook completion
    send_notification("Stop", "success", None);
    Ok(HookResult::success())
}

/// Match agent_id against configured subagent stop patterns
///
/// Returns a vector of (pattern, commands) tuples for all matching patterns.
/// Wildcard patterns (*) are returned first, followed by other matching patterns.
fn match_subagent_patterns<'a>(
    agent_id: &str,
    config: &'a crate::config::SubagentStopConfig,
) -> Vec<(&'a str, &'a Vec<crate::config::SubagentStopCommand>)> {
    let mut matches = Vec::new();
    let mut wildcard_match = None;

    for (pattern, commands) in &config.commands {
        if pattern == "*" {
            // Store wildcard match separately to ensure it comes first
            wildcard_match = Some((pattern.as_str(), commands));
        } else if let Ok(glob_pattern) = Pattern::new(pattern) {
            if glob_pattern.matches(agent_id) {
                matches.push((pattern.as_str(), commands));
            }
        }
    }

    // Construct result with wildcard first if present
    let mut result = Vec::new();
    if let Some(wildcard) = wildcard_match {
        result.push(wildcard);
    }
    result.extend(matches);
    result
}

/// Build environment variables for subagent stop commands
///
/// Creates a HashMap of environment variables to pass to command execution,
/// including subagent-specific context from the payload.
fn build_subagent_env_vars(payload: &SubagentStopPayload) -> std::collections::HashMap<String, String> {
    let mut env_vars = std::collections::HashMap::new();

    // Subagent-specific environment variables
    env_vars.insert("CONCLAUDE_AGENT_ID".to_string(), payload.agent_id.clone());
    env_vars.insert("CONCLAUDE_AGENT_TRANSCRIPT_PATH".to_string(), payload.agent_transcript_path.clone());

    // Session-level environment variables
    env_vars.insert("CONCLAUDE_SESSION_ID".to_string(), payload.base.session_id.clone());
    env_vars.insert("CONCLAUDE_TRANSCRIPT_PATH".to_string(), payload.base.transcript_path.clone());
    env_vars.insert("CONCLAUDE_HOOK_EVENT".to_string(), "SubagentStop".to_string());
    env_vars.insert("CONCLAUDE_CWD".to_string(), payload.base.cwd.clone());

    env_vars
}

/// Represents a subagent stop command with its configuration
struct SubagentStopCommandConfig {
    command: String,
    #[allow(dead_code)]
    message: Option<String>,
    show_stdout: bool,
    show_stderr: bool,
    max_output_lines: Option<u32>,
}

/// Collect subagent stop commands from configuration for a specific agent_id
///
/// # Errors
///
/// Returns an error if bash command extraction fails.
fn collect_subagent_stop_commands(
    agent_id: &str,
    config: &crate::config::SubagentStopConfig,
) -> Result<Vec<SubagentStopCommandConfig>> {
    let mut commands = Vec::new();
    let matches = match_subagent_patterns(agent_id, config);

    for (_pattern, pattern_commands) in matches {
        for cmd_config in pattern_commands {
            let extracted = extract_bash_commands(&cmd_config.run)?;
            let show_stdout = cmd_config.show_stdout.unwrap_or(false);
            let show_stderr = cmd_config.show_stderr.unwrap_or(false);
            let max_output_lines = cmd_config.max_output_lines;

            for cmd in extracted {
                commands.push(SubagentStopCommandConfig {
                    command: cmd,
                    message: cmd_config.message.clone(),
                    show_stdout,
                    show_stderr,
                    max_output_lines,
                });
            }
        }
    }

    Ok(commands)
}

/// Execute subagent stop hook commands with environment variables
///
/// # Errors
///
/// Returns an error if command execution fails or process spawning fails.
async fn execute_subagent_stop_commands(
    commands: &[SubagentStopCommandConfig],
    env_vars: &std::collections::HashMap<String, String>,
) -> Result<Option<HookResult>> {
    if commands.is_empty() {
        return Ok(None);
    }

    println!("Executing {} subagent stop hook commands", commands.len());

    for (index, cmd_config) in commands.iter().enumerate() {
        println!(
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
            .envs(env_vars)
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

            // Log detailed failure information
            let mut diagnostic = format!(
                "Subagent stop command failed:\n  Command: {}\n  Status: Failed (exit code: {})",
                cmd_config.command, exit_code
            );

            if cmd_config.show_stdout {
                let stdout_display = if stdout.trim().is_empty() {
                    "    (no stdout)".to_string()
                } else {
                    stdout
                        .trim()
                        .lines()
                        .map(|line| format!("    {}", line))
                        .collect::<Vec<_>>()
                        .join("\n")
                };
                diagnostic.push_str(&format!("\n  Stdout:\n{}", stdout_display));
            }

            if cmd_config.show_stderr {
                let stderr_display = if stderr.trim().is_empty() {
                    "    (no stderr)".to_string()
                } else {
                    stderr
                        .trim()
                        .lines()
                        .map(|line| format!("    {}", line))
                        .collect::<Vec<_>>()
                        .join("\n")
                };
                diagnostic.push_str(&format!("\n  Stderr:\n{}", stderr_display));
            }

            eprintln!("{}", diagnostic);

            // Build error message (not blocking, just logging)
            // Subagent stop commands don't block the hook, they just log errors
            continue;
        }

        // Successful individual commands produce no output unless configured
        if cmd_config.show_stdout && !stdout.is_empty() {
            if let Some(max_lines) = cmd_config.max_output_lines {
                let (truncated, is_truncated, omitted) = truncate_output(&stdout, max_lines);
                if is_truncated {
                    println!("Command stdout:\n{}\n... ({} lines omitted)", truncated, omitted);
                } else {
                    println!("Command stdout:\n{}", truncated);
                }
            } else {
                println!("Command stdout:\n{}", stdout);
            }
        }

        if cmd_config.show_stderr && !stderr.is_empty() {
            if let Some(max_lines) = cmd_config.max_output_lines {
                let (truncated, is_truncated, omitted) = truncate_output(&stderr, max_lines);
                if is_truncated {
                    eprintln!("Command stderr:\n{}\n... ({} lines omitted)", truncated, omitted);
                } else {
                    eprintln!("Command stderr:\n{}", truncated);
                }
            } else {
                eprintln!("Command stderr:\n{}", stderr);
            }
        }
    }

    println!("All subagent stop hook commands completed");
    Ok(None)
}

/// Handles `SubagentStart` hook events when Claude subagents begin execution.
///
/// # Errors
///
/// Returns an error if payload validation fails or configuration loading fails.
#[allow(clippy::unused_async)]
pub async fn handle_subagent_start() -> Result<HookResult> {
    let payload: SubagentStartPayload = read_payload_from_stdin()?;

    // Validate the payload including agent_id, subagent_type, and agent_transcript_path fields
    validate_subagent_start_payload(&payload).map_err(|e| anyhow::anyhow!(e))?;

    println!(
        "Processing SubagentStart hook: session_id={}, agent_id={}",
        payload.base.session_id, payload.agent_id
    );

    // Set environment variables for the subagent's information
    // These allow downstream hooks and processes to access subagent details
    std::env::set_var("CONCLAUDE_AGENT_ID", &payload.agent_id);
    std::env::set_var("CONCLAUDE_SUBAGENT_TYPE", &payload.subagent_type);
    std::env::set_var(
        "CONCLAUDE_AGENT_TRANSCRIPT_PATH",
        &payload.agent_transcript_path,
    );

    // Send notification for subagent start with agent ID included
    send_notification(
        "SubagentStart",
        "success",
        Some(&format!("Subagent '{}' started", payload.agent_id)),
    );
    Ok(HookResult::success())
}

/// Handles `SubagentStop` hook events when Claude subagents complete their tasks.
///
/// # Errors
///
/// Returns an error if payload validation fails, configuration loading fails, or command execution fails.
pub async fn handle_subagent_stop() -> Result<HookResult> {
    let payload: SubagentStopPayload = read_payload_from_stdin()?;

    // Validate the payload including new agent_id and agent_transcript_path fields
    validate_subagent_stop_payload(&payload).map_err(|e| anyhow::anyhow!(e))?;

    println!(
        "Processing SubagentStop hook: session_id={}, agent_id={}",
        payload.base.session_id, payload.agent_id
    );

    // Load configuration to check for subagent stop commands
    let (config, _config_path) = get_config().await?;

    // Build environment variables for command execution
    let env_vars = build_subagent_env_vars(&payload);

    // Set environment variables for the subagent's information in the current process
    // These allow downstream hooks and processes to access subagent details
    std::env::set_var("CONCLAUDE_AGENT_ID", &payload.agent_id);
    std::env::set_var(
        "CONCLAUDE_AGENT_TRANSCRIPT_PATH",
        &payload.agent_transcript_path,
    );

    // Collect commands for this agent_id
    let commands = collect_subagent_stop_commands(&payload.agent_id, &config.subagent_stop)?;

    // Execute commands if any are configured
    if !commands.is_empty() {
        // Execute commands (errors are logged but don't block the hook)
        if let Err(e) = execute_subagent_stop_commands(&commands, &env_vars).await {
            eprintln!("Error executing subagent stop commands: {}", e);
            // Continue to send notification even if commands failed
        }
    }

    // Send notification for subagent stop with agent ID included
    send_notification(
        "SubagentStop",
        "success",
        Some(&format!("Subagent '{}' completed", payload.agent_id)),
    );
    Ok(HookResult::success())
}

/// Handles `PreCompact` hook events before transcript compaction occurs.
///
/// # Errors
///
/// Returns an error if payload validation fails or configuration loading fails.
#[allow(clippy::unused_async)]
pub async fn handle_pre_compact() -> Result<HookResult> {
    let payload: PreCompactPayload = read_payload_from_stdin()?;

    validate_base_payload(&payload.base).map_err(|e| anyhow::anyhow!(e))?;

    println!(
        "Processing PreCompact hook: session_id={}, trigger={:?}",
        payload.base.session_id, payload.trigger
    );

    // Send notification for pre-compact hook
    send_notification(
        "PreCompact",
        "success",
        Some(&format!("Compaction triggered: {:?}", payload.trigger)),
    );
    Ok(HookResult::success())
}

/// Check tool usage validation rules
///
/// # Errors
///
/// Returns an error if configuration loading fails or glob pattern creation fails.
async fn check_tool_usage_rules(payload: &PreToolUsePayload) -> Result<Option<HookResult>> {
    let (config, _config_path) = get_config().await?;

    for rule in &config.rules.tool_usage_validation {
        if rule.tool == payload.tool_name || rule.tool == "*" {
            // Check if this is a Bash command with a commandPattern rule
            if payload.tool_name == "Bash" && rule.command_pattern.is_some() {
                // Extract the command
                if let Some(command) = extract_bash_command(&payload.tool_input) {
                    let pattern = rule.command_pattern.as_ref().unwrap();
                    let mode = rule.match_mode.as_deref().unwrap_or("full");

                    // Perform pattern matching based on mode
                    let matches = if mode == "prefix" {
                        // Prefix mode: test progressively longer prefixes
                        let glob = Pattern::new(pattern)?;
                        let words: Vec<&str> = command.split_whitespace().collect();
                        (1..=words.len()).any(|i| {
                            let prefix = words[..i].join(" ");
                            glob.matches(&prefix)
                        })
                    } else {
                        // Full mode: match entire command
                        Pattern::new(pattern)?.matches(&command)
                    };

                    // Handle actions based on match result
                    if rule.action == "block" && matches {
                        let message = rule.message.clone().unwrap_or_else(|| {
                            format!("Bash command blocked by validation rule: {}", pattern)
                        });
                        return Ok(Some(HookResult::blocked(message)));
                    } else if rule.action == "allow" && !matches {
                        let message = rule.message.clone().unwrap_or_else(|| {
                            format!(
                                "Bash command blocked: does not match allow rule pattern: {}",
                                pattern
                            )
                        });
                        return Ok(Some(HookResult::blocked(message)));
                    } else if rule.action == "allow" && matches {
                        // Allow and stop checking further rules for this command
                        return Ok(None);
                    }
                }
                // Skip file-path validation for Bash command rules
                continue;
            }

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
    let (config, _config_path) = get_config().await?;

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

        eprintln!(
            "PreToolUse blocked auto-generated file edit: tool_name={}, file_path={}, marker={}",
            payload.tool_name, file_path, marker
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
    use crate::config::NotificationsConfig;

    #[test]
    fn test_is_system_event_hook() {
        // Test system event hooks
        assert!(is_system_event_hook("SessionStart"));
        assert!(is_system_event_hook("SessionEnd"));
        assert!(is_system_event_hook("UserPromptSubmit"));
        assert!(is_system_event_hook("SubagentStart"));
        assert!(is_system_event_hook("SubagentStop"));
        assert!(is_system_event_hook("PreCompact"));

        // Test non-system event hooks
        assert!(!is_system_event_hook("PreToolUse"));
        assert!(!is_system_event_hook("PostToolUse"));
        assert!(!is_system_event_hook("Notification"));
        assert!(!is_system_event_hook("Stop"));
    }

    #[test]
    fn test_notification_flag_gating_logic() {
        // Test configuration with all flags enabled (default behavior)
        let config_all_enabled = NotificationsConfig {
            enabled: true,
            hooks: vec!["*".to_string()],
            show_errors: true,
            show_success: true,
            show_system_events: true,
        };

        // All notification types should be allowed
        assert!(config_all_enabled.is_enabled_for("PreToolUse"));
        assert!(config_all_enabled.is_enabled_for("SessionStart"));

        // Test configuration with only errors enabled
        let config_errors_only = NotificationsConfig {
            enabled: true,
            hooks: vec!["*".to_string()],
            show_errors: true,
            show_success: false,
            show_system_events: false,
        };

        // This tests the is_enabled_for method - flags are checked in send_notification
        assert!(config_errors_only.is_enabled_for("PreToolUse"));
        assert!(config_errors_only.is_enabled_for("SessionStart"));

        // Test configuration with only success enabled
        let config_success_only = NotificationsConfig {
            enabled: true,
            hooks: vec!["*".to_string()],
            show_errors: false,
            show_success: true,
            show_system_events: false,
        };

        assert!(config_success_only.is_enabled_for("PreToolUse"));
        assert!(config_success_only.is_enabled_for("SessionStart"));

        // Test configuration with only system events enabled
        let config_system_only = NotificationsConfig {
            enabled: true,
            hooks: vec!["*".to_string()],
            show_errors: false,
            show_success: false,
            show_system_events: true,
        };

        assert!(config_system_only.is_enabled_for("PreToolUse"));
        assert!(config_system_only.is_enabled_for("SessionStart"));
    }

    #[test]
    fn test_is_root_addition() {
        use std::env;

        // Get current working directory for testing
        let cwd = env::current_dir().unwrap();

        // Simulate config file in the current directory
        let config_path = cwd.join(".conclaude.yaml");

        // Files at the same level as config should be blocked
        assert!(is_root_addition("test.txt", "test.txt", &config_path));
        assert!(is_root_addition("newfile.rs", "newfile.rs", &config_path));

        // BREAKING CHANGE: Dotfiles are now also blocked at root level
        assert!(is_root_addition(".gitignore", ".gitignore", &config_path));
        assert!(is_root_addition(".env", ".env", &config_path));

        // BREAKING CHANGE: Config files are now also blocked at root level
        assert!(is_root_addition(
            "package.json",
            "package.json",
            &config_path
        ));
        assert!(is_root_addition("config.yaml", "config.yaml", &config_path));

        // Files in subdirectories should not be blocked
        assert!(!is_root_addition(
            "src/test.txt",
            "src/test.txt",
            &config_path
        ));
        assert!(!is_root_addition(
            "tests/foo.rs",
            "tests/foo.rs",
            &config_path
        ));
        assert!(!is_root_addition(
            "nested/deep/file.txt",
            "nested/deep/file.txt",
            &config_path
        ));
    }

    #[test]
    fn test_matches_uneditable_pattern() {
        assert!(matches_uneditable_pattern(
            "package.json",
            "package.json",
            "/path/package.json",
            "package.json"
        )
        .unwrap());
        assert!(matches_uneditable_pattern("test.md", "test.md", "/path/test.md", "*.md").unwrap());
        assert!(matches_uneditable_pattern(
            "src/index.ts",
            "src/index.ts",
            "/path/src/index.ts",
            "src/**/*.ts"
        )
        .unwrap());
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

    #[test]
    fn test_truncate_output_no_truncation_needed() {
        let output = "line1\nline2\nline3";
        let (truncated, is_truncated, omitted) = truncate_output(output, 10);
        assert_eq!(truncated, "line1\nline2\nline3");
        assert!(!is_truncated);
        assert_eq!(omitted, 0);
    }

    #[test]
    fn test_truncate_output_exact_limit() {
        let output = "line1\nline2\nline3";
        let (truncated, is_truncated, omitted) = truncate_output(output, 3);
        assert_eq!(truncated, "line1\nline2\nline3");
        assert!(!is_truncated);
        assert_eq!(omitted, 0);
    }

    #[test]
    fn test_truncate_output_with_truncation() {
        let output = "line1\nline2\nline3\nline4\nline5";
        let (truncated, is_truncated, omitted) = truncate_output(output, 2);
        assert_eq!(truncated, "line1\nline2");
        assert!(is_truncated);
        assert_eq!(omitted, 3);
    }

    #[test]
    fn test_truncate_output_empty() {
        let output = "";
        let (truncated, is_truncated, omitted) = truncate_output(output, 10);
        assert_eq!(truncated, "");
        assert!(!is_truncated);
        assert_eq!(omitted, 0);
    }

    #[test]
    fn test_truncate_output_single_line() {
        let output = "single line";
        let (truncated, is_truncated, omitted) = truncate_output(output, 1);
        assert_eq!(truncated, "single line");
        assert!(!is_truncated);
        assert_eq!(omitted, 0);
    }

    #[test]
    fn test_truncate_output_large_limit() {
        let output = "line1\nline2";
        let (truncated, is_truncated, omitted) = truncate_output(output, 10000);
        assert_eq!(truncated, "line1\nline2");
        assert!(!is_truncated);
        assert_eq!(omitted, 0);
    }

    #[test]
    fn test_truncate_output_multiple_lines_exact_boundary() {
        let output = "line1\nline2\nline3\nline4\nline5";
        let (truncated, is_truncated, omitted) = truncate_output(output, 5);
        assert_eq!(truncated, "line1\nline2\nline3\nline4\nline5");
        assert!(!is_truncated);
        assert_eq!(omitted, 0);
    }

    #[test]
    fn test_truncate_output_multiple_lines_just_over_limit() {
        let output = "line1\nline2\nline3\nline4\nline5\nline6";
        let (truncated, is_truncated, omitted) = truncate_output(output, 5);
        assert_eq!(truncated, "line1\nline2\nline3\nline4\nline5");
        assert!(is_truncated);
        assert_eq!(omitted, 1);
    }

    #[test]
    fn test_truncate_output_preserves_content() {
        let output = "Line with special chars: !@#$%^&*()\nAnother line\n\nEmpty line above";
        let (truncated, is_truncated, omitted) = truncate_output(output, 2);
        assert_eq!(
            truncated,
            "Line with special chars: !@#$%^&*()\nAnother line"
        );
        assert!(is_truncated);
        assert_eq!(omitted, 2);
    }

    #[test]
    fn test_collect_stop_commands_with_output_config() {
        use crate::config::StopCommand;

        let config = ConclaudeConfig {
            stop: crate::config::StopConfig {
                commands: vec![
                    StopCommand {
                        run: "echo hello".to_string(),
                        message: Some("Custom message".to_string()),
                        show_stdout: Some(true),
                        show_stderr: Some(false),
                        max_output_lines: Some(10),
                    },
                    StopCommand {
                        run: "ls -la".to_string(),
                        message: None,
                        show_stdout: Some(false),
                        show_stderr: Some(true),
                        max_output_lines: Some(5),
                    },
                ],
                infinite: false,
                infinite_message: None,
                rounds: None,
            },
            ..Default::default()
        };

        let commands = collect_stop_commands(&config).unwrap();
        assert_eq!(commands.len(), 2);

        assert_eq!(commands[0].command, "echo hello");
        assert!(commands[0].show_stdout);
        assert!(!commands[0].show_stderr);
        assert_eq!(commands[0].max_output_lines, Some(10));
        assert_eq!(commands[0].message, Some("Custom message".to_string()));

        assert_eq!(commands[1].command, "ls -la");
        assert!(!commands[1].show_stdout);
        assert!(commands[1].show_stderr);
        assert_eq!(commands[1].max_output_lines, Some(5));
        assert_eq!(commands[1].message, None);
    }

    #[test]
    fn test_collect_stop_commands_default_values() {
        use crate::config::StopCommand;

        let config = ConclaudeConfig {
            stop: crate::config::StopConfig {
                commands: vec![StopCommand {
                    run: "echo test".to_string(),
                    message: None,
                    show_stdout: None,
                    show_stderr: None,
                    max_output_lines: None,
                }],
                infinite: false,
                infinite_message: None,
                rounds: None,
            },
            ..Default::default()
        };

        let commands = collect_stop_commands(&config).unwrap();
        assert_eq!(commands.len(), 1);

        // Defaults should be false for show flags and None for max_output_lines
        assert!(!commands[0].show_stdout);
        assert!(!commands[0].show_stderr);
        assert_eq!(commands[0].max_output_lines, None);
    }

    #[test]
    fn test_extract_bash_command_valid() {
        let mut tool_input = std::collections::HashMap::new();
        tool_input.insert(
            "command".to_string(),
            Value::String("echo hello".to_string()),
        );
        assert_eq!(
            extract_bash_command(&tool_input),
            Some("echo hello".to_string())
        );
    }

    #[test]
    fn test_extract_bash_command_missing() {
        let tool_input: std::collections::HashMap<String, Value> = std::collections::HashMap::new();
        assert_eq!(extract_bash_command(&tool_input), None);
    }

    #[test]
    fn test_extract_bash_command_empty() {
        let mut tool_input = std::collections::HashMap::new();
        tool_input.insert("command".to_string(), Value::String("".to_string()));
        assert_eq!(extract_bash_command(&tool_input), None);
    }

    #[test]
    fn test_extract_bash_command_whitespace_only() {
        let mut tool_input = std::collections::HashMap::new();
        tool_input.insert(
            "command".to_string(),
            Value::String("   \n\t   ".to_string()),
        );
        assert_eq!(extract_bash_command(&tool_input), None);
    }

    #[test]
    fn test_extract_bash_command_trims_whitespace() {
        let mut tool_input = std::collections::HashMap::new();
        tool_input.insert(
            "command".to_string(),
            Value::String("  echo test  ".to_string()),
        );
        assert_eq!(
            extract_bash_command(&tool_input),
            Some("echo test".to_string())
        );
    }

    #[test]
    fn test_extract_bash_command_non_string_value() {
        let mut tool_input = std::collections::HashMap::new();
        tool_input.insert("command".to_string(), Value::Number(42.into()));
        assert_eq!(extract_bash_command(&tool_input), None);
    }

    #[test]
    fn test_bash_command_full_match_exact() {
        use glob::Pattern;

        // Exact match
        let pattern = Pattern::new("rm -rf /").unwrap();
        assert!(pattern.matches("rm -rf /"));

        // With wildcard at end
        let pattern2 = Pattern::new("rm -rf /*").unwrap();
        assert!(pattern2.matches("rm -rf /"));
        assert!(pattern2.matches("rm -rf /tmp"));

        // Doesn't match with prefix
        let pattern3 = Pattern::new("rm -rf /*").unwrap();
        assert!(!pattern3.matches("sudo rm -rf /"));
    }

    #[test]
    fn test_bash_command_full_match_wildcard() {
        use glob::Pattern;

        let pattern = Pattern::new("git push --force*").unwrap();
        assert!(pattern.matches("git push --force"));
        assert!(pattern.matches("git push --force origin main"));
        assert!(!pattern.matches("git push origin main"));
    }

    #[test]
    fn test_bash_command_prefix_match_at_start() {
        use glob::Pattern;

        let pattern = Pattern::new("curl *").unwrap();
        let command = "curl https://example.com && echo done";

        // Simulate prefix matching logic
        let words: Vec<&str> = command.split_whitespace().collect();
        let matches = (1..=words.len()).any(|i| {
            let prefix = words[..i].join(" ");
            pattern.matches(&prefix)
        });

        assert!(matches, "Should match 'curl https://example.com' at start");
    }

    #[test]
    fn test_bash_command_prefix_no_match_middle() {
        use glob::Pattern;

        let pattern = Pattern::new("curl *").unwrap();
        let command = "echo test && curl https://example.com";

        let words: Vec<&str> = command.split_whitespace().collect();
        let matches = (1..=words.len()).any(|i| {
            let prefix = words[..i].join(" ");
            pattern.matches(&prefix)
        });

        assert!(!matches, "Should not match 'curl' in middle of command");
    }

    #[test]
    fn test_bash_command_wildcard_variations() {
        use glob::Pattern;

        let test_cases = vec![
            ("rm -rf*", "rm -rf /", true),
            ("rm -rf*", "rm -rf /tmp", true),
            ("git push --force*", "git push --force", true),
            ("git push --force*", "git push --force origin main", true),
            ("docker run*", "docker start", false),
            ("npm install*", "npm install", true),
            ("npm install*", "npm ci", false),
        ];

        for (pattern_str, command, expected) in test_cases {
            let pattern = Pattern::new(pattern_str).unwrap();
            assert_eq!(
                pattern.matches(command),
                expected,
                "Pattern: '{}', Command: '{}', Expected: {}",
                pattern_str,
                command,
                expected
            );
        }
    }

    #[test]
    fn test_match_subagent_patterns_wildcard() {
        use crate::config::{SubagentStopCommand, SubagentStopConfig};
        use std::collections::HashMap;

        let mut commands_map = HashMap::new();
        commands_map.insert(
            "*".to_string(),
            vec![SubagentStopCommand {
                run: "echo 'any agent'".to_string(),
                message: None,
                show_stdout: None,
                show_stderr: None,
                max_output_lines: None,
            }],
        );

        let config = SubagentStopConfig {
            commands: commands_map,
        };

        let matches = match_subagent_patterns("coder", &config);
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].0, "*");
    }

    #[test]
    fn test_match_subagent_patterns_exact_match() {
        use crate::config::{SubagentStopCommand, SubagentStopConfig};
        use std::collections::HashMap;

        let mut commands_map = HashMap::new();
        commands_map.insert(
            "coder".to_string(),
            vec![SubagentStopCommand {
                run: "echo 'coder'".to_string(),
                message: None,
                show_stdout: None,
                show_stderr: None,
                max_output_lines: None,
            }],
        );

        let config = SubagentStopConfig {
            commands: commands_map,
        };

        let matches = match_subagent_patterns("coder", &config);
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].0, "coder");

        // Should not match different agent
        let no_matches = match_subagent_patterns("tester", &config);
        assert_eq!(no_matches.len(), 0);
    }

    #[test]
    fn test_match_subagent_patterns_prefix_glob() {
        use crate::config::{SubagentStopCommand, SubagentStopConfig};
        use std::collections::HashMap;

        let mut commands_map = HashMap::new();
        commands_map.insert(
            "test*".to_string(),
            vec![SubagentStopCommand {
                run: "echo 'test agent'".to_string(),
                message: None,
                show_stdout: None,
                show_stderr: None,
                max_output_lines: None,
            }],
        );

        let config = SubagentStopConfig {
            commands: commands_map,
        };

        let matches1 = match_subagent_patterns("tester", &config);
        assert_eq!(matches1.len(), 1);
        assert_eq!(matches1[0].0, "test*");

        let matches2 = match_subagent_patterns("test-runner", &config);
        assert_eq!(matches2.len(), 1);
        assert_eq!(matches2[0].0, "test*");

        // Should not match
        let no_matches = match_subagent_patterns("runner-test", &config);
        assert_eq!(no_matches.len(), 0);
    }

    #[test]
    fn test_match_subagent_patterns_suffix_glob() {
        use crate::config::{SubagentStopCommand, SubagentStopConfig};
        use std::collections::HashMap;

        let mut commands_map = HashMap::new();
        commands_map.insert(
            "*coder".to_string(),
            vec![SubagentStopCommand {
                run: "echo 'coder agent'".to_string(),
                message: None,
                show_stdout: None,
                show_stderr: None,
                max_output_lines: None,
            }],
        );

        let config = SubagentStopConfig {
            commands: commands_map,
        };

        let matches1 = match_subagent_patterns("coder", &config);
        assert_eq!(matches1.len(), 1);
        assert_eq!(matches1[0].0, "*coder");

        let matches2 = match_subagent_patterns("auto-coder", &config);
        assert_eq!(matches2.len(), 1);
        assert_eq!(matches2[0].0, "*coder");

        // Should not match
        let no_matches = match_subagent_patterns("coder-agent", &config);
        assert_eq!(no_matches.len(), 0);
    }

    #[test]
    fn test_match_subagent_patterns_character_class_glob() {
        use crate::config::{SubagentStopCommand, SubagentStopConfig};
        use std::collections::HashMap;

        let mut commands_map = HashMap::new();
        commands_map.insert(
            "agent_[0-9]*".to_string(),
            vec![SubagentStopCommand {
                run: "echo 'numbered agent'".to_string(),
                message: None,
                show_stdout: None,
                show_stderr: None,
                max_output_lines: None,
            }],
        );

        let config = SubagentStopConfig {
            commands: commands_map,
        };

        let matches1 = match_subagent_patterns("agent_1", &config);
        assert_eq!(matches1.len(), 1);
        assert_eq!(matches1[0].0, "agent_[0-9]*");

        let matches2 = match_subagent_patterns("agent_2x", &config);
        assert_eq!(matches2.len(), 1);
        assert_eq!(matches2[0].0, "agent_[0-9]*");

        // Should not match
        let no_matches = match_subagent_patterns("agent_x", &config);
        assert_eq!(no_matches.len(), 0);
    }

    #[test]
    fn test_match_subagent_patterns_wildcard_first() {
        use crate::config::{SubagentStopCommand, SubagentStopConfig};
        use std::collections::HashMap;

        let mut commands_map = HashMap::new();
        commands_map.insert(
            "*".to_string(),
            vec![SubagentStopCommand {
                run: "echo 'wildcard'".to_string(),
                message: None,
                show_stdout: None,
                show_stderr: None,
                max_output_lines: None,
            }],
        );
        commands_map.insert(
            "coder".to_string(),
            vec![SubagentStopCommand {
                run: "echo 'coder specific'".to_string(),
                message: None,
                show_stdout: None,
                show_stderr: None,
                max_output_lines: None,
            }],
        );
        commands_map.insert(
            "*coder".to_string(),
            vec![SubagentStopCommand {
                run: "echo 'coder suffix'".to_string(),
                message: None,
                show_stdout: None,
                show_stderr: None,
                max_output_lines: None,
            }],
        );

        let config = SubagentStopConfig {
            commands: commands_map,
        };

        let matches = match_subagent_patterns("coder", &config);
        assert_eq!(matches.len(), 3);
        // Wildcard should be first
        assert_eq!(matches[0].0, "*");
        // Other matches can be in any order, but should include both
        let patterns: Vec<&str> = matches.iter().map(|(p, _)| *p).collect();
        assert!(patterns.contains(&"coder"));
        assert!(patterns.contains(&"*coder"));
    }

    #[test]
    fn test_build_subagent_env_vars() {
        use crate::types::{BasePayload, SubagentStopPayload};

        let payload = SubagentStopPayload {
            base: BasePayload {
                session_id: "test_session".to_string(),
                transcript_path: "/path/to/transcript".to_string(),
                hook_event_name: "SubagentStop".to_string(),
                cwd: "/current/dir".to_string(),
                permission_mode: Some("default".to_string()),
            },
            stop_hook_active: true,
            agent_id: "coder".to_string(),
            agent_transcript_path: "/path/to/agent/transcript".to_string(),
        };

        let env_vars = build_subagent_env_vars(&payload);

        assert_eq!(env_vars.get("CONCLAUDE_AGENT_ID"), Some(&"coder".to_string()));
        assert_eq!(
            env_vars.get("CONCLAUDE_AGENT_TRANSCRIPT_PATH"),
            Some(&"/path/to/agent/transcript".to_string())
        );
        assert_eq!(
            env_vars.get("CONCLAUDE_SESSION_ID"),
            Some(&"test_session".to_string())
        );
        assert_eq!(
            env_vars.get("CONCLAUDE_TRANSCRIPT_PATH"),
            Some(&"/path/to/transcript".to_string())
        );
        assert_eq!(
            env_vars.get("CONCLAUDE_HOOK_EVENT"),
            Some(&"SubagentStop".to_string())
        );
        assert_eq!(env_vars.get("CONCLAUDE_CWD"), Some(&"/current/dir".to_string()));
    }

    #[test]
    fn test_collect_subagent_stop_commands_no_matches() {
        use crate::config::SubagentStopConfig;
        use std::collections::HashMap;

        let config = SubagentStopConfig {
            commands: HashMap::new(),
        };

        let commands = collect_subagent_stop_commands("coder", &config).unwrap();
        assert_eq!(commands.len(), 0);
    }

    #[test]
    fn test_collect_subagent_stop_commands_with_wildcard() {
        use crate::config::{SubagentStopCommand, SubagentStopConfig};
        use std::collections::HashMap;

        let mut commands_map = HashMap::new();
        commands_map.insert(
            "*".to_string(),
            vec![SubagentStopCommand {
                run: "echo 'test'".to_string(),
                message: Some("Test message".to_string()),
                show_stdout: Some(true),
                show_stderr: Some(false),
                max_output_lines: Some(100),
            }],
        );

        let config = SubagentStopConfig {
            commands: commands_map,
        };

        let commands = collect_subagent_stop_commands("coder", &config).unwrap();
        assert_eq!(commands.len(), 1);
        assert_eq!(commands[0].command, "echo 'test'");
        assert_eq!(commands[0].message, Some("Test message".to_string()));
        assert!(commands[0].show_stdout);
        assert!(!commands[0].show_stderr);
        assert_eq!(commands[0].max_output_lines, Some(100));
    }

    #[test]
    fn test_collect_subagent_stop_commands_multiple_patterns() {
        use crate::config::{SubagentStopCommand, SubagentStopConfig};
        use std::collections::HashMap;

        let mut commands_map = HashMap::new();
        commands_map.insert(
            "*".to_string(),
            vec![SubagentStopCommand {
                run: "echo 'wildcard'".to_string(),
                message: None,
                show_stdout: None,
                show_stderr: None,
                max_output_lines: None,
            }],
        );
        commands_map.insert(
            "coder".to_string(),
            vec![SubagentStopCommand {
                run: "echo 'coder specific'".to_string(),
                message: None,
                show_stdout: None,
                show_stderr: None,
                max_output_lines: None,
            }],
        );

        let config = SubagentStopConfig {
            commands: commands_map,
        };

        let commands = collect_subagent_stop_commands("coder", &config).unwrap();
        assert_eq!(commands.len(), 2);
        // Wildcard should come first
        assert_eq!(commands[0].command, "echo 'wildcard'");
        assert_eq!(commands[1].command, "echo 'coder specific'");
    }
}
