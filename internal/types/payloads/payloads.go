// Package payloads provides data structures for various hook payloads.
package payloads

import (
	"encoding/json"
	"errors"
)

const (
	// EmptyString represents an empty string value for validation checks.
	EmptyString = ""
	// SuccessField represents the JSON field name for success status.
	SuccessField = "success"
)

// BasePayload represents base fields present in all hook payloads.
type BasePayload struct {
	SessionID      string `json:"session_id"`
	TranscriptPath string `json:"transcript_path"`
	HookEventName  string `json:"hook_event_name"`
}

// Validate validates the base payload fields.
func (b *BasePayload) Validate() error {
	if b.SessionID == EmptyString {
		return errors.New("missing required field: session_id")
	}
	if b.TranscriptPath == EmptyString {
		return errors.New("missing required field: transcript_path")
	}
	if b.HookEventName == EmptyString {
		return errors.New("missing required field: hook_event_name")
	}

	return nil
}

// GetSessionID returns the session ID from the base payload.
func (b *BasePayload) GetSessionID() string {
	return b.SessionID
}

// PreToolUsePayload represents payload for PreToolUse hook.
type PreToolUsePayload struct {
	BasePayload
	ToolName  string         `json:"tool_name"`
	ToolInput map[string]any `json:"tool_input"`
}

// GetSessionID returns the session ID for PreToolUsePayload.
func (p PreToolUsePayload) GetSessionID() string {
	return p.SessionID
}

// Validate validates the PreToolUsePayload.
func (p PreToolUsePayload) Validate() error {
	return p.BasePayload.Validate()
}

// PostToolUsePayload represents payload for PostToolUse hook.
type PostToolUsePayload struct {
	BasePayload
	ToolName     string         `json:"tool_name"`
	ToolInput    map[string]any `json:"tool_input"`
	ToolResponse ToolResponse   `json:"tool_response"`
}

// GetSessionID returns the session ID for PostToolUsePayload.
func (p PostToolUsePayload) GetSessionID() string {
	return p.SessionID
}

// Validate validates the PostToolUsePayload.
func (p PostToolUsePayload) Validate() error {
	return p.BasePayload.Validate()
}

// ToolResponse represents the response from a tool execution.
type ToolResponse struct {
	Success *bool          `json:"success,omitempty"`
	Data    map[string]any `json:"-"`
}

// UnmarshalJSON custom unmarshaler for ToolResponse to handle both
// objects and arrays.
func (t *ToolResponse) UnmarshalJSON(data []byte) error {
	// First, try to unmarshal as an object
	var objData map[string]any
	if err := json.Unmarshal(data, &objData); err == nil {
		// It's an object, handle the success field
		if success, ok := objData[SuccessField].(bool); ok {
			t.Success = &success
			delete(objData, SuccessField)
		}
		t.Data = objData

		return nil
	}

	// If that fails, try to unmarshal as an array or other type
	var anyData any
	if err := json.Unmarshal(data, &anyData); err != nil {
		return err
	}

	// Store the raw data
	t.Data = map[string]any{
		"data": anyData,
	}

	return nil
}

// MarshalJSON custom marshaler for ToolResponse to flatten data.
func (t ToolResponse) MarshalJSON() ([]byte, error) {
	result := make(map[string]any)

	// Add all data fields
	for k, v := range t.Data {
		result[k] = v
	}

	// Add success field if present
	if t.Success != nil {
		result[SuccessField] = t.Success
	}

	return json.Marshal(result)
}

// NotificationPayload represents payload for Notification hook.
type NotificationPayload struct {
	BasePayload
	Message string  `json:"message"`
	Title   *string `json:"title,omitempty"`
}

// GetSessionID returns the session ID for NotificationPayload.
func (n NotificationPayload) GetSessionID() string {
	return n.SessionID
}

// Validate validates the NotificationPayload.
func (n NotificationPayload) Validate() error {
	return n.BasePayload.Validate()
}

// StopPayload represents payload for Stop hook.
type StopPayload struct {
	BasePayload
	StopHookActive bool `json:"stop_hook_active"`
}

// GetSessionID returns the session ID for StopPayload.
func (s StopPayload) GetSessionID() string {
	return s.SessionID
}

// Validate validates the StopPayload.
func (s StopPayload) Validate() error {
	return s.BasePayload.Validate()
}

// SubagentStopPayload represents payload for SubagentStop hook.
type SubagentStopPayload struct {
	BasePayload
	StopHookActive bool `json:"stop_hook_active"`
}

// GetSessionID returns the session ID for SubagentStopPayload.
func (s SubagentStopPayload) GetSessionID() string {
	return s.SessionID
}

// Validate validates the SubagentStopPayload.
func (s SubagentStopPayload) Validate() error {
	return s.BasePayload.Validate()
}

// UserPromptSubmitPayload represents payload for UserPromptSubmit hook.
type UserPromptSubmitPayload struct {
	BasePayload
	Prompt string `json:"prompt"`
}

// GetSessionID returns the session ID for UserPromptSubmitPayload.
func (u UserPromptSubmitPayload) GetSessionID() string {
	return u.SessionID
}

// Validate validates the UserPromptSubmitPayload.
func (u UserPromptSubmitPayload) Validate() error {
	return u.BasePayload.Validate()
}

// CompactTrigger represents the trigger type for compaction.
type CompactTrigger string

const (
	// CompactTriggerManual represents manual compaction trigger.
	CompactTriggerManual CompactTrigger = "manual"
	// CompactTriggerAuto represents automatic compaction trigger.
	CompactTriggerAuto CompactTrigger = "auto"
)

// PreCompactPayload represents payload for PreCompact hook.
type PreCompactPayload struct {
	BasePayload
	Trigger CompactTrigger `json:"trigger"`
}

// GetSessionID returns the session ID for PreCompactPayload.
func (p PreCompactPayload) GetSessionID() string {
	return p.SessionID
}

// Validate validates the PreCompactPayload.
func (p PreCompactPayload) Validate() error {
	return p.BasePayload.Validate()
}

// SessionStartPayload represents payload for SessionStart hook.
type SessionStartPayload struct {
	BasePayload
	Source string `json:"source"`
}

// GetSessionID returns the session ID for SessionStartPayload.
func (s SessionStartPayload) GetSessionID() string {
	return s.SessionID
}

// Validate validates the SessionStartPayload.
func (s SessionStartPayload) Validate() error {
	return s.BasePayload.Validate()
}
