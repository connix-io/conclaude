// Package types provides data structures and constants for hook payloads and configuration.
package types

import (
	"encoding/json"
	"errors"
)

// LoggingConfig represents configuration options for controlling logger
// behavior.
type LoggingConfig struct {
	FileLogging bool `json:"file_logging" yaml:"file_logging"`
}

// HookResult represents response structure returned by hook handlers to
// control execution flow.
type HookResult struct {
	Message *string `json:"message,omitempty"`
	Blocked *bool   `json:"blocked,omitempty"`
}

// NewSuccessResult creates a successful hook result.
func NewSuccessResult() HookResult {
	blocked := false

	return HookResult{
		Blocked: &blocked,
	}
}

// NewBlockedResult creates a blocked hook result with a message.
func NewBlockedResult(message string) HookResult {
	blocked := true

	return HookResult{
		Message: &message,
		Blocked: &blocked,
	}
}

// BasePayload represents base fields present in all hook payloads.
type BasePayload struct {
	SessionID      string `json:"session_id"`
	TranscriptPath string `json:"transcript_path"`
	HookEventName  string `json:"hook_event_name"`
}

// Validate validates the base payload fields.
func (b *BasePayload) Validate() error {
	if b.SessionID == "" {
		return errors.New("missing required field: session_id")
	}
	if b.TranscriptPath == "" {
		return errors.New("missing required field: transcript_path")
	}
	if b.HookEventName == "" {
		return errors.New("missing required field: hook_event_name")
	}

	return nil
}

// PreToolUsePayload represents payload for PreToolUse hook.
type PreToolUsePayload struct {
	BasePayload
	ToolName  string         `json:"tool_name"`
	ToolInput map[string]any `json:"tool_input"`
}

// PostToolUsePayload represents payload for PostToolUse hook.
type PostToolUsePayload struct {
	BasePayload
	ToolName     string         `json:"tool_name"`
	ToolInput    map[string]any `json:"tool_input"`
	ToolResponse ToolResponse   `json:"tool_response"`
}

// ToolResponse represents the response from a tool execution.
type ToolResponse struct {
	Success *bool          `json:"success,omitempty"`
	Data    map[string]any `json:"-"`
}

// UnmarshalJSON custom unmarshaler for ToolResponse to handle flattened data.
func (t *ToolResponse) UnmarshalJSON(data []byte) error {
	type Alias ToolResponse
	aux := &struct {
		*Alias
	}{
		Alias: (*Alias)(t),
	}

	if err := json.Unmarshal(data, &aux); err != nil {
		return err
	}

	// Unmarshal the entire JSON into a map
	var fullData map[string]any
	if err := json.Unmarshal(data, &fullData); err != nil {
		return err
	}

	// Remove known fields and keep the rest as Data
	delete(fullData, "success")
	t.Data = fullData

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
		result["success"] = t.Success
	}

	return json.Marshal(result)
}

// NotificationPayload represents payload for Notification hook.
type NotificationPayload struct {
	BasePayload
	Message string  `json:"message"`
	Title   *string `json:"title,omitempty"`
}

// StopPayload represents payload for Stop hook.
type StopPayload struct {
	BasePayload
	StopHookActive bool `json:"stop_hook_active"`
}

// SubagentStopPayload represents payload for SubagentStop hook.
type SubagentStopPayload struct {
	BasePayload
	StopHookActive bool `json:"stop_hook_active"`
}

// UserPromptSubmitPayload represents payload for UserPromptSubmit hook.
type UserPromptSubmitPayload struct {
	BasePayload
	Prompt string `json:"prompt"`
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

// SessionStartPayload represents payload for SessionStart hook.
type SessionStartPayload struct {
	BasePayload
	Source string `json:"source"`
}
