// Package rules provides rule-related configuration structures.
package rules

// ToolUsageRule represents tool usage validation rule.
type ToolUsageRule struct {
	Tool    string  `json:"tool" yaml:"tool" jsonschema:"title=Tool name,description=Name of the tool to validate"`
	Pattern string  `json:"pattern" yaml:"pattern" jsonschema:"title=Pattern to match,description=Regular expression or glob pattern to match against tool usage"`
	Action  string  `json:"action" yaml:"action" jsonschema:"title=Action to take,description=Action to take when pattern matches (block or allow)"`
	Message *string `json:"message,omitempty" yaml:"message,omitempty" jsonschema:"title=Custom message,description=Custom message to display when rule triggers"`
}

// RulesConfig represents configuration interface for validation rules.
type RulesConfig struct {
	PreventRootAdditions bool            `json:"preventRoot" yaml:"preventRoot"`
	UneditableFiles      []string        `json:"uneditableFiles" yaml:"uneditableFiles"`
	ToolUsageValidation  []ToolUsageRule `json:"toolUsageValidation" yaml:"toolUsageValidation" jsonschema:"title=Tool usage validation,description=Rules for validating tool usage"`
}

// PreToolUseConfig represents configuration for pre tool use hooks.
type PreToolUseConfig struct {
	PreventAdditions          []string `json:"preventAdditions" yaml:"preventAdditions" jsonschema:"title=Prevent additions,description=Patterns for files that cannot be created"`
	PreventGeneratedFileEdits bool     `json:"preventGeneratedFileEdits" yaml:"preventGeneratedFileEdits" jsonschema:"title=Prevent generated file edits,description=Prevent editing of auto-generated files,default=true"`
	GeneratedFileMessage      *string  `json:"generatedFileMessage,omitempty" yaml:"generatedFileMessage,omitempty" jsonschema:"title=Generated file message,description=Custom message for generated file blocking"`
}
