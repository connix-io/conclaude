package config

import (
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"gopkg.in/yaml.v3"
)

// StopCommand represents configuration for individual stop commands with optional messages
type StopCommand struct {
	Run     string  `json:"run" yaml:"run" jsonschema:"title=Command to run,description=Shell command to execute"`
	Message *string `json:"message,omitempty" yaml:"message,omitempty" jsonschema:"title=Optional message,description=Message to display when command runs"`
}

// StopConfig represents configuration interface for stop hook commands
type StopConfig struct {
	Run             string        `json:"run" yaml:"run" jsonschema:"title=Simple command,description=Single command to run on stop"`
	Commands        []StopCommand `json:"commands" yaml:"commands" jsonschema:"title=Stop commands,description=List of commands to run on stop"`
	Infinite        bool          `json:"infinite" yaml:"infinite" jsonschema:"title=Infinite mode,description=Keep session running indefinitely"`
	InfiniteMessage *string       `json:"infiniteMessage,omitempty" yaml:"infiniteMessage,omitempty" jsonschema:"title=Infinite message,description=Custom message for infinite mode"`
	Rounds          *uint32       `json:"rounds,omitempty" yaml:"rounds,omitempty" jsonschema:"title=Number of rounds,description=Number of rounds to run before stopping"`
}

// ToolUsageRule represents tool usage validation rule
type ToolUsageRule struct {
	Tool    string  `json:"tool" yaml:"tool" jsonschema:"title=Tool name,description=Name of the tool to validate"`
	Pattern string  `json:"pattern" yaml:"pattern" jsonschema:"title=Pattern to match,description=Regular expression or glob pattern to match against tool usage"`
	Action  string  `json:"action" yaml:"action" jsonschema:"title=Action to take,description=Action to take when pattern matches (block or allow)"`
	Message *string `json:"message,omitempty" yaml:"message,omitempty" jsonschema:"title=Custom message,description=Custom message to display when rule triggers"`
}

// RulesConfig represents configuration interface for validation rules
type RulesConfig struct {
	PreventRootAdditions bool            `json:"preventRootAdditions" yaml:"preventRootAdditions" jsonschema:"title=Prevent root additions,description=Prevent adding files to repository root,default=true"`
	UneditableFiles      []string        `json:"uneditableFiles" yaml:"uneditableFiles" jsonschema:"title=Uneditable files,description=List of file patterns that cannot be edited"`
	ToolUsageValidation  []ToolUsageRule `json:"toolUsageValidation" yaml:"toolUsageValidation" jsonschema:"title=Tool usage validation,description=Rules for validating tool usage"`
}

// PreToolUseConfig represents configuration for pre tool use hooks
type PreToolUseConfig struct {
	PreventAdditions          []string `json:"preventAdditions" yaml:"preventAdditions" jsonschema:"title=Prevent additions,description=Patterns for files that cannot be created"`
	PreventGeneratedFileEdits bool     `json:"preventGeneratedFileEdits" yaml:"preventGeneratedFileEdits" jsonschema:"title=Prevent generated file edits,description=Prevent editing of auto-generated files,default=true"`
	GeneratedFileMessage      *string  `json:"generatedFileMessage,omitempty" yaml:"generatedFileMessage,omitempty" jsonschema:"title=Generated file message,description=Custom message for generated file blocking"`
}

// GitWorktreeConfig represents configuration for git worktree auto finish
type GitWorktreeConfig struct {
	Enabled              bool    `json:"enabled" yaml:"enabled" jsonschema:"title=Enable git worktree,description=Enable git worktree functionality"`
	AutoCreatePR         bool    `json:"autoCreatePR" yaml:"autoCreatePR" jsonschema:"title=Auto create PR,description=Automatically create pull request"`
	AutoCreatePRCommand  *string `json:"autoCreatePRCommand,omitempty" yaml:"autoCreatePRCommand,omitempty" jsonschema:"title=PR creation command,description=Command to run for creating PR"`
	AutoCreatePRTemplate *string `json:"autoCreatePRTemplate,omitempty" yaml:"autoCreatePRTemplate,omitempty" jsonschema:"title=PR template,description=Template for PR creation"`
}

// ConclaudeConfig represents main configuration interface matching the TypeScript version
type ConclaudeConfig struct {
	Stop        StopConfig        `json:"stop" yaml:"stop" jsonschema:"title=Stop configuration,description=Configuration for stop hooks"`
	Rules       RulesConfig       `json:"rules" yaml:"rules" jsonschema:"title=Rules configuration,description=Configuration for validation rules"`
	PreToolUse  PreToolUseConfig  `json:"preToolUse" yaml:"preToolUse" jsonschema:"title=PreToolUse configuration,description=Configuration for pre-tool-use hooks"`
	GitWorktree GitWorktreeConfig `json:"gitWorktree" yaml:"gitWorktree" jsonschema:"title=Git worktree configuration,description=Configuration for git worktree functionality"`
}

// DefaultConfig returns the default configuration
func DefaultConfig() ConclaudeConfig {
	return ConclaudeConfig{
		Stop: StopConfig{
			Run:      "",
			Commands: []StopCommand{},
			Infinite: false,
			Rounds:   nil,
		},
		Rules: RulesConfig{
			PreventRootAdditions: true,
			UneditableFiles:      []string{},
			ToolUsageValidation:  []ToolUsageRule{},
		},
		PreToolUse: PreToolUseConfig{
			PreventAdditions:          []string{},
			PreventGeneratedFileEdits: true,
			GeneratedFileMessage:      nil,
		},
		GitWorktree: GitWorktreeConfig{
			Enabled:      false,
			AutoCreatePR: false,
		},
	}
}

// LoadConfig loads the conclaude configuration from file
func LoadConfig() (*ConclaudeConfig, error) {
	cwd, err := os.Getwd()
	if err != nil {
		return nil, fmt.Errorf("failed to get current working directory: %w", err)
	}

	// Look for .conclaude.yaml starting from current directory and walking up
	configPath := ""
	currentDir := cwd

	for {
		candidatePath := filepath.Join(currentDir, ".conclaude.yaml")
		if _, err := os.Stat(candidatePath); err == nil {
			configPath = candidatePath
			break
		}

		parent := filepath.Dir(currentDir)
		if parent == currentDir {
			// Reached root directory
			break
		}
		currentDir = parent
	}

	if configPath == "" {
		// No config file found, return default config
		config := DefaultConfig()
		return &config, nil
	}

	// Read and parse the config file
	data, err := os.ReadFile(configPath)
	if err != nil {
		return nil, fmt.Errorf("failed to read config file %s: %w", configPath, err)
	}

	var config ConclaudeConfig
	if err := yaml.Unmarshal(data, &config); err != nil {
		return nil, fmt.Errorf("failed to parse config file %s: %w", configPath, err)
	}

	return &config, nil
}

// GenerateDefaultConfigYAML generates the default configuration as YAML string
func GenerateDefaultConfigYAML() string {
	config := DefaultConfig()
	data, _ := yaml.Marshal(config)

	return strings.TrimSpace(string(data))
}

// ExtractBashCommands extracts bash commands from tool input for validation
func ExtractBashCommands(toolInput map[string]interface{}) []string {
	var commands []string

	if cmd, exists := toolInput["command"]; exists {
		if cmdStr, ok := cmd.(string); ok {
			// Split on ; and && to get individual commands
			parts := strings.FieldsFunc(cmdStr, func(c rune) bool {
				return c == ';' || strings.Contains(string(c), "&&")
			})
			for _, part := range parts {
				commands = append(commands, strings.TrimSpace(part))
			}
		}
	}

	return commands
}

// ExtractFilePath extracts file path from tool input
func ExtractFilePath(toolInput map[string]interface{}) *string {
	// Check various possible file path keys
	pathKeys := []string{"file_path", "path", "filename", "file"}

	for _, key := range pathKeys {
		if value, exists := toolInput[key]; exists {
			if pathStr, ok := value.(string); ok && pathStr != "" {
				return &pathStr
			}
		}
	}

	return nil
}

// IsAutoGeneratedFile checks if a file is auto-generated based on common patterns
func IsAutoGeneratedFile(filePath string) bool {
	autoGenPatterns := []string{
		"**/node_modules/**",
		"**/.next/**",
		"**/build/**",
		"**/dist/**",
		"**/target/**",
		"**/*.generated.*",
		"**/*.gen.*",
		"**/generated/**",
		"**/__generated__/**",
		"**/coverage/**",
		"**/.coverage/**",
		"**/*.lock",
		"**/go.sum",
		"**/Cargo.lock",
		"**/package-lock.json",
		"**/yarn.lock",
		"**/pnpm-lock.yaml",
	}

	for _, pattern := range autoGenPatterns {
		if matched, _ := filepath.Match(pattern, filePath); matched {
			return true
		}
	}

	return false
}

// ClaudeSettings represents Claude Code settings structure for init command
type ClaudeSettings struct {
	IncludeCoAuthoredBy *bool                          `json:"includeCoAuthoredBy,omitempty"`
	Permissions         *ClaudePermissions             `json:"permissions,omitempty"`
	Hooks               map[string][]ClaudeHookMatcher `json:"hooks,omitempty"`
}

// ClaudePermissions represents permissions configuration
type ClaudePermissions struct {
	Allow []string `json:"allow"`
	Deny  []string `json:"deny"`
}

// ClaudeHookMatcher represents hook matcher configuration
type ClaudeHookMatcher struct {
	Matcher string             `json:"matcher"`
	Hooks   []ClaudeHookConfig `json:"hooks"`
}

// ClaudeHookConfig represents individual hook configuration
type ClaudeHookConfig struct {
	Type    string `json:"type"`
	Command string `json:"command"`
}
