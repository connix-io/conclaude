use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration options for controlling logger behavior.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoggingConfig {
    /// Whether to enable file logging to temporary directory
    pub file_logging: bool,
}

/// Response structure returned by hook handlers to control execution flow.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookResult {
    /// Custom message to display to the user
    pub message: Option<String>,
    /// Whether to block the current operation from proceeding
    pub blocked: Option<bool>,
}

impl HookResult {
    #[must_use] pub fn success() -> Self {
        Self {
            message: None,
            blocked: Some(false),
        }
    }

    pub fn blocked(message: impl Into<String>) -> Self {
        Self {
            message: Some(message.into()),
            blocked: Some(true),
        }
    }
}

/// Base fields present in all hook payloads
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasePayload {
    /// Unique identifier for the current Claude session
    pub session_id: String,
    /// Path to the JSONL transcript file containing conversation history
    pub transcript_path: String,
    /// Hook event type identifier
    pub hook_event_name: String,
}

/// Payload for PreToolUse hook - fired before Claude executes a tool.
/// Allows blocking or modifying tool execution before it occurs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreToolUsePayload {
    #[serde(flatten)]
    pub base: BasePayload,
    /// Name of the tool about to be executed (e.g., "Edit", "Bash", "Read")
    pub tool_name: String,
    /// Input parameters that will be passed to the tool
    pub tool_input: HashMap<String, serde_json::Value>,
}

/// Payload for PostToolUse hook - fired after Claude executes a tool.
/// Contains both the input and response data for analysis or logging.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostToolUsePayload {
    #[serde(flatten)]
    pub base: BasePayload,
    /// Name of the tool that was executed
    pub tool_name: String,
    /// Input parameters that were passed to the tool
    pub tool_input: HashMap<String, serde_json::Value>,
    /// Response data returned by the tool execution
    pub tool_response: ToolResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResponse {
    #[serde(flatten)]
    pub data: HashMap<String, serde_json::Value>,
    /// Whether the tool execution completed successfully
    pub success: Option<bool>,
}

/// Payload for Notification hook - fired when Claude sends system notifications.
/// Used for displaying messages or alerts to the user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationPayload {
    #[serde(flatten)]
    pub base: BasePayload,
    /// The notification message content
    pub message: String,
    /// Optional title for the notification
    pub title: Option<String>,
}

/// Payload for Stop hook - fired when a Claude session is terminating.
/// Allows for cleanup operations or final processing before session ends.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StopPayload {
    #[serde(flatten)]
    pub base: BasePayload,
    /// Whether stop hooks are currently active for this session
    pub stop_hook_active: bool,
}

/// Payload for SubagentStop hook - fired when a Claude subagent terminates.
/// Subagents are spawned for complex tasks and this fires when they complete.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubagentStopPayload {
    #[serde(flatten)]
    pub base: BasePayload,
    /// Whether stop hooks are currently active for this session
    pub stop_hook_active: bool,
}

/// Payload for UserPromptSubmit hook - fired when user submits input to Claude.
/// Allows processing or validation of user input before Claude processes it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPromptSubmitPayload {
    #[serde(flatten)]
    pub base: BasePayload,
    /// The user's input prompt text
    pub prompt: String,
}

/// Payload for PreCompact hook - fired before transcript compaction occurs.
/// Transcript compaction reduces conversation history size to manage context limits.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreCompactPayload {
    #[serde(flatten)]
    pub base: BasePayload,
    /// Whether compaction was triggered manually by user or automatically by system
    pub trigger: CompactTrigger,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CompactTrigger {
    Manual,
    Auto,
}

/// Payload for SessionStart hook - fired when a new Claude session begins.
/// Allows initialization or setup operations at the start of a conversation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionStartPayload {
    #[serde(flatten)]
    pub base: BasePayload,
    /// Source that initiated the session (e.g., CLI, IDE integration)
    pub source: String,
}

/// Union type of all possible hook event payloads.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "hook_event_name")]
pub enum HookPayload {
    #[serde(rename = "PreToolUse")]
    PreToolUse(PreToolUsePayload),
    #[serde(rename = "PostToolUse")]
    PostToolUse(PostToolUsePayload),
    #[serde(rename = "Notification")]
    Notification(NotificationPayload),
    #[serde(rename = "Stop")]
    Stop(StopPayload),
    #[serde(rename = "SubagentStop")]
    SubagentStop(SubagentStopPayload),
    #[serde(rename = "UserPromptSubmit")]
    UserPromptSubmit(UserPromptSubmitPayload),
    #[serde(rename = "PreCompact")]
    PreCompact(PreCompactPayload),
    #[serde(rename = "SessionStart")]
    SessionStart(SessionStartPayload),
}

impl HookPayload {
    #[must_use] pub fn session_id(&self) -> &str {
        match self {
            HookPayload::PreToolUse(p) => &p.base.session_id,
            HookPayload::PostToolUse(p) => &p.base.session_id,
            HookPayload::Notification(p) => &p.base.session_id,
            HookPayload::Stop(p) => &p.base.session_id,
            HookPayload::SubagentStop(p) => &p.base.session_id,
            HookPayload::UserPromptSubmit(p) => &p.base.session_id,
            HookPayload::PreCompact(p) => &p.base.session_id,
            HookPayload::SessionStart(p) => &p.base.session_id,
        }
    }

    #[must_use] pub fn transcript_path(&self) -> &str {
        match self {
            HookPayload::PreToolUse(p) => &p.base.transcript_path,
            HookPayload::PostToolUse(p) => &p.base.transcript_path,
            HookPayload::Notification(p) => &p.base.transcript_path,
            HookPayload::Stop(p) => &p.base.transcript_path,
            HookPayload::SubagentStop(p) => &p.base.transcript_path,
            HookPayload::UserPromptSubmit(p) => &p.base.transcript_path,
            HookPayload::PreCompact(p) => &p.base.transcript_path,
            HookPayload::SessionStart(p) => &p.base.transcript_path,
        }
    }

    #[must_use] pub fn hook_event_name(&self) -> &str {
        match self {
            HookPayload::PreToolUse(p) => &p.base.hook_event_name,
            HookPayload::PostToolUse(p) => &p.base.hook_event_name,
            HookPayload::Notification(p) => &p.base.hook_event_name,
            HookPayload::Stop(p) => &p.base.hook_event_name,
            HookPayload::SubagentStop(p) => &p.base.hook_event_name,
            HookPayload::UserPromptSubmit(p) => &p.base.hook_event_name,
            HookPayload::PreCompact(p) => &p.base.hook_event_name,
            HookPayload::SessionStart(p) => &p.base.hook_event_name,
        }
    }
}

/// Validates that a payload contains all required base fields.
pub fn validate_base_payload(base: &BasePayload) -> Result<(), String> {
    if base.session_id.is_empty() {
        return Err("Missing required field: session_id".to_string());
    }
    if base.transcript_path.is_empty() {
        return Err("Missing required field: transcript_path".to_string());
    }
    if base.hook_event_name.is_empty() {
        return Err("Missing required field: hook_event_name".to_string());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hook_result_success() {
        let result = HookResult::success();
        assert_eq!(result.blocked, Some(false));
        assert_eq!(result.message, None);
    }

    #[test]
    fn test_hook_result_blocked() {
        let result = HookResult::blocked("Test blocking message");
        assert_eq!(result.blocked, Some(true));
        assert_eq!(result.message, Some("Test blocking message".to_string()));
    }

    #[test]
    fn test_validate_base_payload() {
        let valid_base = BasePayload {
            session_id: "test_session".to_string(),
            transcript_path: "/path/to/transcript".to_string(),
            hook_event_name: "PreToolUse".to_string(),
        };
        assert!(validate_base_payload(&valid_base).is_ok());

        let invalid_base = BasePayload {
            session_id: "".to_string(),
            transcript_path: "/path/to/transcript".to_string(),
            hook_event_name: "PreToolUse".to_string(),
        };
        assert!(validate_base_payload(&invalid_base).is_err());
    }
}
