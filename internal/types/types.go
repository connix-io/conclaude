// Package types provides core data structures and configuration.
package types

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
