package cmd

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"

	"github.com/connix-io/conclaude-go/internal/config"
	"github.com/connix-io/conclaude-go/internal/schema"
	"github.com/fatih/color"
	"github.com/spf13/cobra"
)

var (
	initConfigPath string
	initClaudePath string
	initForce      bool
	initSchemaURL  string
)

// initCmd represents the init command
var initCmd = &cobra.Command{
	Use:   "init",
	Short: "Initialize conclaude configuration and Claude Code hooks",
	Long: `Initialize conclaude configuration and Claude Code hooks.

Creates .conclaude.yaml configuration file and updates Claude Code settings.json
with hook configurations for all supported hook events.`,
	RunE: runInit,
}

func init() {
	initCmd.Flags().StringVar(&initConfigPath, "config-path", "", "Path for .conclaude.yaml file")
	initCmd.Flags().StringVar(&initClaudePath, "claude-path", "", "Path for .claude directory")
	initCmd.Flags().
		BoolVarP(&initForce, "force", "f", false, "Overwrite existing configuration files")
	initCmd.Flags().
		StringVar(&initSchemaURL, "schema-url", "", "Custom schema URL for YAML language server header")
}

func runInit(cmd *cobra.Command, args []string) error {
	cwd, err := os.Getwd()
	if err != nil {
		return fmt.Errorf("failed to get current directory: %w", err)
	}

	configPath := filepath.Join(cwd, ".conclaude.yaml")
	if initConfigPath != "" {
		configPath = initConfigPath
	}

	claudePath := filepath.Join(cwd, ".claude")
	if initClaudePath != "" {
		claudePath = initClaudePath
	}

	settingsPath := filepath.Join(claudePath, "settings.json")

	fmt.Printf(
		"%s Initializing conclaude configuration...\n\n",
		color.New(color.FgCyan).Sprint("🚀"),
	)

	// Check if config file exists
	if _, err := os.Stat(configPath); err == nil && !initForce {
		fmt.Printf(
			"%s Configuration file already exists:\n",
			color.New(color.FgYellow).Sprint("⚠️"),
		)
		fmt.Printf("   %s\n", configPath)
		fmt.Println("\nUse --force to overwrite existing configuration.")
		os.Exit(1)
	}

	// Create .conclaude.yaml with YAML language server header
	var schemaURLPtr *string
	if initSchemaURL != "" {
		schemaURLPtr = &initSchemaURL
	}
	yamlHeader := schema.GenerateYAMLLanguageServerHeader(schemaURLPtr)
	configContent := yamlHeader + config.GenerateDefaultConfigYAML()

	if err := os.WriteFile(configPath, []byte(configContent), 0644); err != nil {
		return fmt.Errorf("failed to write config file %s: %w", configPath, err)
	}

	fmt.Printf(
		"%s Created configuration file with YAML language server support:\n",
		color.New(color.FgGreen).Sprint("✅"),
	)
	fmt.Printf("   %s\n", configPath)
	defaultSchemaURL := schema.GetSchemaURL()
	usedSchemaURL := defaultSchemaURL
	if initSchemaURL != "" {
		usedSchemaURL = initSchemaURL
	}
	fmt.Printf("   Schema URL: %s\n", usedSchemaURL)

	// Create .claude directory if it doesn't exist
	if err := os.MkdirAll(claudePath, 0755); err != nil {
		return fmt.Errorf("failed to create .claude directory %s: %w", claudePath, err)
	}

	// Handle settings.json
	var settings config.ClaudeSettings
	if data, err := os.ReadFile(settingsPath); err == nil {
		if err := json.Unmarshal(data, &settings); err != nil {
			return fmt.Errorf("failed to parse settings file %s: %w", settingsPath, err)
		}
		fmt.Printf(
			"\n%s Found existing Claude settings, updating hooks...\n",
			color.New(color.FgBlue).Sprint("📝"),
		)
	} else {
		fmt.Printf("\n%s Creating Claude Code settings...\n", color.New(color.FgBlue).Sprint("📝"))
		settings = config.ClaudeSettings{
			Permissions: &config.ClaudePermissions{
				Allow: []string{},
				Deny:  []string{},
			},
			Hooks: make(map[string][]config.ClaudeHookMatcher),
		}
	}

	// Define all hook types and their commands
	hookTypes := []string{
		"UserPromptSubmit",
		"PreToolUse",
		"PostToolUse",
		"Notification",
		"Stop",
		"SubagentStop",
		"PreCompact",
		"SessionStart",
	}

	// Add hook configurations
	if settings.Hooks == nil {
		settings.Hooks = make(map[string][]config.ClaudeHookMatcher)
	}

	for _, hookType := range hookTypes {
		settings.Hooks[hookType] = []config.ClaudeHookMatcher{
			{
				Matcher: "",
				Hooks: []config.ClaudeHookConfig{
					{
						Type:    "command",
						Command: "conclaude " + hookType,
					},
				},
			},
		}
	}

	// Write updated settings
	settingsJSON, err := json.MarshalIndent(settings, "", "  ")
	if err != nil {
		return fmt.Errorf("failed to serialize settings to JSON: %w", err)
	}

	if err := os.WriteFile(settingsPath, settingsJSON, 0644); err != nil {
		return fmt.Errorf("failed to write settings file %s: %w", settingsPath, err)
	}

	fmt.Printf("%s Updated Claude Code settings:\n", color.New(color.FgGreen).Sprint("✅"))
	fmt.Printf("   %s\n", settingsPath)

	fmt.Printf("\n%s Conclaude initialization complete!\n", color.New(color.FgGreen).Sprint("🎉"))
	fmt.Println("\nConfigured hooks:")
	for _, hookType := range hookTypes {
		fmt.Printf("   • %s\n", hookType)
	}
	fmt.Println("\nYou can now use Claude Code with conclaude hook handling.")

	return nil
}
