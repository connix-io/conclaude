// Package rules provides rule-related configuration structures.
package rules

// ToolUsageRule represents tool usage validation rule.
type ToolUsageRule struct {
	Tool    string  `json:"tool" yaml:"tool" jsonschema:"title=Tool,description=Tool name"`
	Pattern string  `json:"pattern" yaml:"pattern" jsonschema:"title=Pattern"`
	Action  string  `json:"action" yaml:"action" jsonschema:"title=Action,description=Block or allow"`
	Message *string `json:"message,omitempty" yaml:"message,omitempty" jsonschema:"title=Message"`
}

// Config represents configuration interface for validation rules.
type Config struct {
	PreventRootAdditions bool            `json:"preventRoot" yaml:"preventRoot"`
	UneditableFiles      []string        `json:"uneditableFiles" yaml:"uneditableFiles"`
	ToolUsageValidation  []ToolUsageRule `json:"toolUsageValidation" yaml:"toolUsageValidation"`
}

// PreToolUseConfig represents configuration for pre tool use hooks.
type PreToolUseConfig struct {
	PreventAdditions          []string `json:"preventAdditions" yaml:"preventAdditions"`
	PreventGeneratedFileEdits bool     `json:"preventGeneratedFileEdits" yaml:"preventGeneratedFileEdits"`
	GeneratedFileMessage      *string  `json:"generatedFileMessage,omitempty" yaml:"generatedFileMessage,omitempty"`
}
