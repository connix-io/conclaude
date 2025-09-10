// Package claude provides Claude Code settings structure and related types.
package claude

// Settings represents Claude Code settings structure for init command.
type Settings struct {
	CoAuthor    *bool                    `json:"includeCoAuthoredBy,omitempty"`
	Permissions *Permissions             `json:"permissions,omitempty"`
	Hooks       map[string][]HookMatcher `json:"hooks,omitempty"`
}

// Permissions represents permissions configuration.
type Permissions struct {
	Allow []string `json:"allow"`
	Deny  []string `json:"deny"`
}

// HookMatcher represents hook matcher configuration.
type HookMatcher struct {
	Matcher string       `json:"matcher"`
	Hooks   []HookConfig `json:"hooks"`
}

// HookConfig represents individual hook configuration.
type HookConfig struct {
	Type    string `json:"type"`
	Command string `json:"command"`
}
