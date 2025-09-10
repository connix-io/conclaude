// Package claude provides Claude Code settings structure and related types.
package claude

// ClaudeSettings represents Claude Code settings structure for init command.
type ClaudeSettings struct {
	IncludeCoAuthoredBy *bool                          `json:"includeCoAuthoredBy,omitempty"`
	Permissions         *ClaudePermissions             `json:"permissions,omitempty"`
	Hooks               map[string][]ClaudeHookMatcher `json:"hooks,omitempty"`
}

// ClaudePermissions represents permissions configuration.
type ClaudePermissions struct {
	Allow []string `json:"allow"`
	Deny  []string `json:"deny"`
}

// ClaudeHookMatcher represents hook matcher configuration.
type ClaudeHookMatcher struct {
	Matcher string             `json:"matcher"`
	Hooks   []ClaudeHookConfig `json:"hooks"`
}

// ClaudeHookConfig represents individual hook configuration.
type ClaudeHookConfig struct {
	Type    string `json:"type"`
	Command string `json:"command"`
}
