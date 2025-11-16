use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Response structure returned by hook handlers to control execution flow.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookResult {
    /// Custom message to display to the user
    pub message: Option<String>,
    /// Whether to block the current operation from proceeding
    pub blocked: Option<bool>,
}

impl HookResult {
    #[must_use]
    pub fn success() -> Self {
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
    /// Current working directory
    pub cwd: String,
    /// Current permission mode (e.g., "default", "acceptEdits", "bypassPermissions", "plan")
    pub permission_mode: Option<String>,
}

/// Payload for `PreToolUse` hook - fired before Claude executes a tool.
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

/// Payload for `PostToolUse` hook - fired after Claude executes a tool.
/// Contains both the input and response data for analysis or logging.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostToolUsePayload {
    #[serde(flatten)]
    pub base: BasePayload,
    /// Name of the tool that was executed
    pub tool_name: String,
    /// Input parameters that were passed to the tool
    pub tool_input: HashMap<String, serde_json::Value>,
    /// Response data returned by the tool execution (can be any JSON value)
    pub tool_response: serde_json::Value,
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

/// Payload for `SubagentStop` hook - fired when a Claude subagent terminates.
/// Subagents are spawned for complex tasks and this fires when they complete.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubagentStopPayload {
    #[serde(flatten)]
    pub base: BasePayload,
    /// Whether stop hooks are currently active for this session
    pub stop_hook_active: bool,
    /// Unique identifier for the subagent that completed (e.g., "coder", "tester", "stuck")
    pub agent_id: String,
    /// Path to the subagent's specific transcript file containing conversation history
    pub agent_transcript_path: String,
}

/// Payload for `UserPromptSubmit` hook - fired when user submits input to Claude.
/// Allows processing or validation of user input before Claude processes it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPromptSubmitPayload {
    #[serde(flatten)]
    pub base: BasePayload,
    /// The user's input prompt text
    pub prompt: String,
}

/// Payload for `PreCompact` hook - fired before transcript compaction occurs.
/// Transcript compaction reduces conversation history size to manage context limits.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreCompactPayload {
    #[serde(flatten)]
    pub base: BasePayload,
    /// Whether compaction was triggered manually by user or automatically by system
    pub trigger: CompactTrigger,
    /// Custom instructions provided for compaction (if any)
    pub custom_instructions: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CompactTrigger {
    Manual,
    Auto,
}

/// Payload for `SessionStart` hook - fired when a new Claude session begins.
/// Allows initialization or setup operations at the start of a conversation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionStartPayload {
    #[serde(flatten)]
    pub base: BasePayload,
    /// Source that initiated the session (e.g., CLI, IDE integration)
    pub source: String,
}

/// Payload for `SessionEnd` hook - fired when a Claude session terminates.
/// Allows cleanup operations or final logging at the end of a conversation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionEndPayload {
    #[serde(flatten)]
    pub base: BasePayload,
    /// Reason for session termination (e.g., "user_exit", "error", "completion")
    pub reason: String,
}

/// Union type of all possible hook event payloads.
#[allow(dead_code)]
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
    #[serde(rename = "SessionEnd")]
    SessionEnd(SessionEndPayload),
}

impl HookPayload {
    #[allow(dead_code)]
    #[must_use]
    pub fn session_id(&self) -> &str {
        match self {
            HookPayload::PreToolUse(p) => &p.base.session_id,
            HookPayload::PostToolUse(p) => &p.base.session_id,
            HookPayload::Notification(p) => &p.base.session_id,
            HookPayload::Stop(p) => &p.base.session_id,
            HookPayload::SubagentStop(p) => &p.base.session_id,
            HookPayload::UserPromptSubmit(p) => &p.base.session_id,
            HookPayload::PreCompact(p) => &p.base.session_id,
            HookPayload::SessionStart(p) => &p.base.session_id,
            HookPayload::SessionEnd(p) => &p.base.session_id,
        }
    }

    #[allow(dead_code)]
    #[must_use]
    pub fn transcript_path(&self) -> &str {
        match self {
            HookPayload::PreToolUse(p) => &p.base.transcript_path,
            HookPayload::PostToolUse(p) => &p.base.transcript_path,
            HookPayload::Notification(p) => &p.base.transcript_path,
            HookPayload::Stop(p) => &p.base.transcript_path,
            HookPayload::SubagentStop(p) => &p.base.transcript_path,
            HookPayload::UserPromptSubmit(p) => &p.base.transcript_path,
            HookPayload::PreCompact(p) => &p.base.transcript_path,
            HookPayload::SessionStart(p) => &p.base.transcript_path,
            HookPayload::SessionEnd(p) => &p.base.transcript_path,
        }
    }

    #[allow(dead_code)]
    #[must_use]
    pub fn hook_event_name(&self) -> &str {
        match self {
            HookPayload::PreToolUse(p) => &p.base.hook_event_name,
            HookPayload::PostToolUse(p) => &p.base.hook_event_name,
            HookPayload::Notification(p) => &p.base.hook_event_name,
            HookPayload::Stop(p) => &p.base.hook_event_name,
            HookPayload::SubagentStop(p) => &p.base.hook_event_name,
            HookPayload::UserPromptSubmit(p) => &p.base.hook_event_name,
            HookPayload::PreCompact(p) => &p.base.hook_event_name,
            HookPayload::SessionStart(p) => &p.base.hook_event_name,
            HookPayload::SessionEnd(p) => &p.base.hook_event_name,
        }
    }
}

/// Validates that a payload contains all required base fields.
///
/// # Errors
///
/// Returns an error if any required base field is missing or empty.
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
    if base.cwd.is_empty() {
        return Err("Missing required field: cwd".to_string());
    }
    Ok(())
}

/// Validates that a SubagentStopPayload contains all required fields.
///
/// # Errors
///
/// Returns an error if any required field is missing or empty (after trimming whitespace).
#[allow(dead_code)]
pub fn validate_subagent_stop_payload(payload: &SubagentStopPayload) -> Result<(), String> {
    // First validate the base payload
    validate_base_payload(&payload.base)?;

    // Validate agent_id
    if payload.agent_id.trim().is_empty() {
        return Err("agent_id cannot be empty".to_string());
    }

    // Validate agent_transcript_path
    if payload.agent_transcript_path.trim().is_empty() {
        return Err("agent_transcript_path cannot be empty".to_string());
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
            cwd: "/current/dir".to_string(),
            permission_mode: Some("default".to_string()),
        };
        assert!(validate_base_payload(&valid_base).is_ok());

        let invalid_base = BasePayload {
            session_id: String::new(),
            transcript_path: "/path/to/transcript".to_string(),
            hook_event_name: "PreToolUse".to_string(),
            cwd: "/current/dir".to_string(),
            permission_mode: Some("default".to_string()),
        };
        assert!(validate_base_payload(&invalid_base).is_err());
    }

    #[test]
    fn test_validate_subagent_stop_payload_valid() {
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
        assert!(validate_subagent_stop_payload(&payload).is_ok());
    }

    #[test]
    fn test_validate_subagent_stop_payload_empty_agent_id() {
        let payload = SubagentStopPayload {
            base: BasePayload {
                session_id: "test_session".to_string(),
                transcript_path: "/path/to/transcript".to_string(),
                hook_event_name: "SubagentStop".to_string(),
                cwd: "/current/dir".to_string(),
                permission_mode: Some("default".to_string()),
            },
            stop_hook_active: true,
            agent_id: String::new(),
            agent_transcript_path: "/path/to/agent/transcript".to_string(),
        };
        let result = validate_subagent_stop_payload(&payload);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "agent_id cannot be empty");
    }

    #[test]
    fn test_validate_subagent_stop_payload_whitespace_agent_id() {
        let payload = SubagentStopPayload {
            base: BasePayload {
                session_id: "test_session".to_string(),
                transcript_path: "/path/to/transcript".to_string(),
                hook_event_name: "SubagentStop".to_string(),
                cwd: "/current/dir".to_string(),
                permission_mode: Some("default".to_string()),
            },
            stop_hook_active: true,
            agent_id: "   ".to_string(),
            agent_transcript_path: "/path/to/agent/transcript".to_string(),
        };
        let result = validate_subagent_stop_payload(&payload);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "agent_id cannot be empty");
    }

    #[test]
    fn test_validate_subagent_stop_payload_empty_agent_transcript_path() {
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
            agent_transcript_path: String::new(),
        };
        let result = validate_subagent_stop_payload(&payload);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "agent_transcript_path cannot be empty");
    }

    #[test]
    fn test_validate_subagent_stop_payload_whitespace_agent_transcript_path() {
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
            agent_transcript_path: "   ".to_string(),
        };
        let result = validate_subagent_stop_payload(&payload);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "agent_transcript_path cannot be empty");
    }

    #[test]
    fn test_validate_subagent_stop_payload_invalid_base() {
        let payload = SubagentStopPayload {
            base: BasePayload {
                session_id: String::new(),
                transcript_path: "/path/to/transcript".to_string(),
                hook_event_name: "SubagentStop".to_string(),
                cwd: "/current/dir".to_string(),
                permission_mode: Some("default".to_string()),
            },
            stop_hook_active: true,
            agent_id: "coder".to_string(),
            agent_transcript_path: "/path/to/agent/transcript".to_string(),
        };
        let result = validate_subagent_stop_payload(&payload);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("session_id"));
    }

    #[test]
    fn test_validate_subagent_stop_payload_agent_id_with_leading_trailing_spaces() {
        let payload = SubagentStopPayload {
            base: BasePayload {
                session_id: "test_session".to_string(),
                transcript_path: "/path/to/transcript".to_string(),
                hook_event_name: "SubagentStop".to_string(),
                cwd: "/current/dir".to_string(),
                permission_mode: Some("default".to_string()),
            },
            stop_hook_active: true,
            agent_id: "  coder  ".to_string(),
            agent_transcript_path: "/path/to/agent/transcript".to_string(),
        };
        assert!(validate_subagent_stop_payload(&payload).is_ok());
    }

    #[test]
    fn test_validate_subagent_stop_payload_different_agent_types() {
        let agent_types = vec!["coder", "tester", "stuck"];

        for agent_type in agent_types {
            let payload = SubagentStopPayload {
                base: BasePayload {
                    session_id: "test_session".to_string(),
                    transcript_path: "/path/to/transcript".to_string(),
                    hook_event_name: "SubagentStop".to_string(),
                    cwd: "/current/dir".to_string(),
                    permission_mode: Some("default".to_string()),
                },
                stop_hook_active: true,
                agent_id: agent_type.to_string(),
                agent_transcript_path: "/path/to/agent/transcript".to_string(),
            };
            assert!(validate_subagent_stop_payload(&payload).is_ok());
        }
    }
}
