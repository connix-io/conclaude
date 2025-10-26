use conclaude::types::*;
use std::collections::HashMap;

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
fn test_validate_base_payload_valid() {
    let valid_base = BasePayload {
        session_id: "test_session".to_string(),
        transcript_path: "/path/to/transcript".to_string(),
        hook_event_name: "PreToolUse".to_string(),
        cwd: "/current/dir".to_string(),
        permission_mode: Some("default".to_string()),
    };
    assert!(validate_base_payload(&valid_base).is_ok());
}

#[test]
fn test_validate_base_payload_missing_session_id() {
    let invalid_base = BasePayload {
        session_id: String::new(),
        transcript_path: "/path/to/transcript".to_string(),
        hook_event_name: "PreToolUse".to_string(),
        cwd: "/current/dir".to_string(),
        permission_mode: Some("default".to_string()),
    };
    let result = validate_base_payload(&invalid_base);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("session_id"));
}

#[test]
fn test_validate_base_payload_missing_transcript_path() {
    let invalid_base = BasePayload {
        session_id: "test_session".to_string(),
        transcript_path: String::new(),
        hook_event_name: "PreToolUse".to_string(),
        cwd: "/current/dir".to_string(),
        permission_mode: Some("default".to_string()),
    };
    let result = validate_base_payload(&invalid_base);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("transcript_path"));
}

#[test]
fn test_validate_base_payload_missing_hook_event_name() {
    let invalid_base = BasePayload {
        session_id: "test_session".to_string(),
        transcript_path: "/path/to/transcript".to_string(),
        hook_event_name: String::new(),
        cwd: "/current/dir".to_string(),
        permission_mode: Some("default".to_string()),
    };
    let result = validate_base_payload(&invalid_base);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("hook_event_name"));
}


#[test]
fn test_compact_trigger_serialization() {
    let trigger = CompactTrigger::Manual;
    let json = serde_json::to_string(&trigger).unwrap();
    assert_eq!(json, "\"manual\"");

    let trigger = CompactTrigger::Auto;
    let json = serde_json::to_string(&trigger).unwrap();
    assert_eq!(json, "\"auto\"");
}

#[test]
fn test_compact_trigger_deserialization() {
    let json = "\"manual\"";
    let trigger: CompactTrigger = serde_json::from_str(json).unwrap();
    matches!(trigger, CompactTrigger::Manual);

    let json = "\"auto\"";
    let trigger: CompactTrigger = serde_json::from_str(json).unwrap();
    matches!(trigger, CompactTrigger::Auto);
}

#[test]
fn test_pre_tool_use_payload_serialization() {
    let mut tool_input = HashMap::new();
    tool_input.insert(
        "file_path".to_string(),
        serde_json::Value::String("test.txt".to_string()),
    );

    let payload = PreToolUsePayload {
        base: BasePayload {
            session_id: "test_session".to_string(),
            transcript_path: "/path/to/transcript".to_string(),
            hook_event_name: "PreToolUse".to_string(),
            cwd: "/current/dir".to_string(),
            permission_mode: Some("default".to_string()),
        },
        tool_name: "Edit".to_string(),
        tool_input,
    };

    let json = serde_json::to_string(&payload).unwrap();
    assert!(json.contains("test_session"));
    assert!(json.contains("Edit"));
    assert!(json.contains("test.txt"));
}

#[test]
fn test_notification_payload_deserialization() {
    let json = r#"{
        "session_id": "test_session",
        "transcript_path": "/path/to/transcript",
        "hook_event_name": "Notification",
        "cwd": "/current/dir",
        "permission_mode": "default",
        "message": "Test notification",
        "title": "Test Title"
    }"#;

    let payload: NotificationPayload = serde_json::from_str(json).unwrap();
    assert_eq!(payload.base.session_id, "test_session");
    assert_eq!(payload.message, "Test notification");
    assert_eq!(payload.title, Some("Test Title".to_string()));
}

#[test]
fn test_stop_payload_deserialization() {
    let json = r#"{
        "session_id": "test_session",
        "transcript_path": "/path/to/transcript",
        "hook_event_name": "Stop",
        "cwd": "/current/dir",
        "permission_mode": "default",
        "stop_hook_active": true
    }"#;

    let payload: StopPayload = serde_json::from_str(json).unwrap();
    assert_eq!(payload.base.session_id, "test_session");
    assert!(payload.stop_hook_active);
}

#[test]
fn test_user_prompt_submit_payload_deserialization() {
    let json = r#"{
        "session_id": "test_session",
        "transcript_path": "/path/to/transcript",
        "hook_event_name": "UserPromptSubmit",
        "cwd": "/current/dir",
        "permission_mode": "default",
        "prompt": "Hello Claude"
    }"#;

    let payload: UserPromptSubmitPayload = serde_json::from_str(json).unwrap();
    assert_eq!(payload.base.session_id, "test_session");
    assert_eq!(payload.prompt, "Hello Claude");
}

#[test]
fn test_pre_compact_payload_deserialization() {
    let json = r#"{
        "session_id": "test_session",
        "transcript_path": "/path/to/transcript",
        "hook_event_name": "PreCompact",
        "cwd": "/current/dir",
        "permission_mode": "default",
        "trigger": "auto",
        "custom_instructions": null
    }"#;

    let payload: PreCompactPayload = serde_json::from_str(json).unwrap();
    assert_eq!(payload.base.session_id, "test_session");
    matches!(payload.trigger, CompactTrigger::Auto);
}

#[test]
fn test_session_start_payload_deserialization() {
    let json = r#"{
        "session_id": "test_session",
        "transcript_path": "/path/to/transcript",
        "hook_event_name": "SessionStart",
        "cwd": "/current/dir",
        "permission_mode": "default",
        "source": "CLI"
    }"#;

    let payload: SessionStartPayload = serde_json::from_str(json).unwrap();
    assert_eq!(payload.base.session_id, "test_session");
    assert_eq!(payload.source, "CLI");
}
