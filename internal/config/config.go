// Package config provides configuration management for conclaude hooks and rules.
package config

import (
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/connix-io/conclaude/internal/config/rules"
	"github.com/gobwas/glob"
	"gopkg.in/yaml.v3"
)

// StopCommand represents configuration for individual stop commands with optional messages.
type StopCommand struct {
	Run     string  `json:"run" yaml:"run" jsonschema:"title=Run,description=Run"`
	Message *string `json:"message,omitempty" yaml:"message,omitempty" jsonschema:"title=Optional message,description=Message to display when command runs"`
}

// StopConfig represents configuration interface for stop hook commands.
type StopConfig struct {
	Run             string        `json:"run" yaml:"run" jsonschema:"title=Cmd,description=Stop cmd"`
	Commands        []StopCommand `json:"commands" yaml:"commands" jsonschema:"title=Stop commands,description=List of commands to run on stop"`
	Infinite        bool          `json:"infinite" yaml:"infinite" jsonschema:"title=Infinite mode,description=Keep session running indefinitely"`
	InfiniteMessage *string       `json:"infiniteMessage,omitempty" yaml:"infiniteMessage,omitempty"`
	Rounds          *uint32       `json:"rounds,omitempty" yaml:"rounds,omitempty" jsonschema:"title=Number of rounds,description=Number of rounds to run before stopping"`
}

// GitWorktreeConfig represents configuration for git worktree auto finish.
type GitWorktreeConfig struct {
	Enabled              bool    `json:"enabled" yaml:"enabled" jsonschema:"title=Enable git worktree,description=Enable git worktree functionality"`
	AutoCreatePR         bool    `json:"autoCreatePR" yaml:"autoCreatePR" jsonschema:"title=Auto create PR,description=Automatically create pull request"`
	AutoCreatePRCommand  *string `json:"autoCreatePRCommand,omitempty" yaml:"autoCreatePRCommand,omitempty" jsonschema:"title=PR creation command,description=Command to run for creating PR"`
	AutoCreatePRTemplate *string `json:"autoCreatePRTemplate,omitempty" yaml:"autoCreatePRTemplate,omitempty" jsonschema:"title=PR template,description=Template for PR creation"`
}

// ConclaudeConfig represents main configuration interface matching the TypeScript version.
type ConclaudeConfig struct {
	Stop        StopConfig             `json:"stop" yaml:"stop" jsonschema:"title=Stop configuration,description=Configuration for stop hooks"`
	Rules       rules.Config           `json:"rules" yaml:"rules" jsonschema:"title=Rules configuration,description=Configuration for validation rules"`
	PreToolUse  rules.PreToolUseConfig `json:"preToolUse" yaml:"preToolUse" jsonschema:"title=PreToolUse configuration,description=Configuration for pre-tool-use hooks"`
	GitWorktree GitWorktreeConfig      `json:"gitWorktree" yaml:"gitWorktree" jsonschema:"title=Git worktree configuration,description=Configuration for git worktree functionality"`
}

// DefaultConfig returns the default configuration.
func DefaultConfig() ConclaudeConfig {
	return ConclaudeConfig{
		Stop: StopConfig{
			Run:      "",
			Commands: []StopCommand{},
			Infinite: false,
			Rounds:   nil,
		},
		Rules: rules.Config{
			PreventRootAdditions: true,
			UneditableFiles:      []string{},
			ToolUsageValidation:  []rules.ToolUsageRule{},
		},
		PreToolUse: rules.PreToolUseConfig{
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

// LoadConfig loads the conclaude configuration from file.
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

// GenerateDefaultConfigYAML generates the default configuration as YAML string.
func GenerateDefaultConfigYAML() string {
	config := DefaultConfig()
	data, _ := yaml.Marshal(config)

	return strings.TrimSpace(string(data))
}

// ExtractBashCommands extracts bash commands from tool input for validation.
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

// ExtractFilePath extracts file path from tool input.
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

// IsAutoGeneratedFile checks if a file is auto-generated based on common patterns.
func IsAutoGeneratedFile(filePath string) bool {
	// Simpler patterns that work with gobwas/glob
	autoGenPatterns := []string{
		"*/node_modules/*",
		"node_modules/*",
		"*/.next/*",
		".next/*",
		"*/build/*",
		"build/*",
		"*/dist/*",
		"dist/*",
		"*/target/*",
		"target/*",
		"*.generated.*",
		"*.gen.*",
		"*/generated/*",
		"generated/*",
		"*/__generated__/*",
		"__generated__/*",
		"*/coverage/*",
		"coverage/*",
		"*/.coverage/*",
		".coverage/*",
		// Specific lock files only
		"go.sum",
		"*/go.sum",
		"Cargo.lock",
		"*/Cargo.lock",
		"package-lock.json",
		"*/package-lock.json",
		"yarn.lock",
		"*/yarn.lock",
		"pnpm-lock.yaml",
		"*/pnpm-lock.yaml",
	}

	for _, pattern := range autoGenPatterns {
		g, err := glob.Compile(pattern)
		if err != nil {
			continue
		}
		if g.Match(filePath) {
			return true
		}
	}

	return false
}
