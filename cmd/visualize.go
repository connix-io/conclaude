package cmd

import (
	"fmt"
	"io/fs"
	"path/filepath"

	"github.com/connix-io/conclaude/internal/config"
	"github.com/fatih/color"
	"github.com/gobwas/glob"
	"github.com/spf13/cobra"
)

var (
	visualizeRule        string
	visualizeShowMatches bool
)

// visualizeCmd represents the visualize command
var visualizeCmd = &cobra.Command{
	Use:   "visualize",
	Short: "Visualize file/directory settings from configuration",
	Long: `Visualize file/directory settings from configuration.

Shows configuration rules and optionally displays which files match
the configured patterns. Useful for understanding and debugging
configuration settings.`,
	RunE: runVisualize,
}

func init() {
	visualizeCmd.Flags().
		StringVarP(&visualizeRule, "rule", "r", "", "The specific rule to visualize (e.g., \"uneditableFiles\", \"preventRootAdditions\")")
	visualizeCmd.Flags().
		BoolVar(&visualizeShowMatches, "show-matches", false, "Show files that match the rule")
}

func runVisualize(cmd *cobra.Command, args []string) error {
	fmt.Printf("%s Visualizing configuration rules...\n\n", color.New(color.FgBlue).Sprint("🔍"))

	cfg, err := config.LoadConfig()
	if err != nil {
		return fmt.Errorf("failed to load configuration: %w", err)
	}

	if visualizeRule != "" {
		return visualizeSpecificRule(cfg, visualizeRule)
	}

	return visualizeAllRules(cfg)
}

func visualizeSpecificRule(cfg *config.ConclaudeConfig, ruleName string) error {
	switch ruleName {
	case "uneditableFiles":
		return visualizeUneditableFiles(cfg)
	case "preventRootAdditions":
		return visualizePreventRootAdditions(cfg)
	case "toolUsageValidation":
		return visualizeToolUsageValidation(cfg)
	default:
		fmt.Printf("%s Unknown rule: %s\n", color.New(color.FgRed).Sprint("❌"), ruleName)
		fmt.Println("\nAvailable rules:")
		fmt.Println("   - uneditableFiles")
		fmt.Println("   - preventRootAdditions")
		fmt.Println("   - toolUsageValidation")
		return nil
	}
}

func visualizeUneditableFiles(cfg *config.ConclaudeConfig) error {
	fmt.Printf("%s Uneditable Files:\n", color.New(color.FgBlue).Sprint("📁"))

	if len(cfg.Rules.UneditableFiles) == 0 {
		fmt.Println("   No uneditable files configured")
		return nil
	}

	for _, pattern := range cfg.Rules.UneditableFiles {
		fmt.Printf("   Pattern: %s\n", pattern)

		if visualizeShowMatches {
			globPattern, err := glob.Compile(pattern)
			if err != nil {
				fmt.Printf("      Error: Invalid pattern - %v\n", err)

				continue
			}

			fmt.Println("   Matching files:")
			found := false

			err = filepath.WalkDir(".", func(path string, d fs.DirEntry, err error) error {
				if err != nil {
					return nil // Continue walking
				}

				if !d.IsDir() && globPattern.Match(path) {
					fmt.Printf("      - %s\n", path)
					found = true
				}
				return nil
			})

			if err != nil {
				return fmt.Errorf("failed to walk directory: %w", err)
			}

			if !found {
				fmt.Println("      (no matching files found)")
			}
		}
	}

	return nil
}

func visualizePreventRootAdditions(cfg *config.ConclaudeConfig) error {
	fmt.Printf("%s Prevent Root Additions: %v\n",
		color.New(color.FgRed).Sprint("🚫"),
		cfg.Rules.PreventRootAdditions)

	if cfg.Rules.PreventRootAdditions && visualizeShowMatches {
		fmt.Println("\n   Root directory contents:")

		entries, err := filepath.Glob("*")
		if err != nil {
			return fmt.Errorf("failed to list root directory: %w", err)
		}

		for _, entry := range entries {
			fmt.Printf("      - %s\n", entry)
		}
	}

	return nil
}

func visualizeToolUsageValidation(cfg *config.ConclaudeConfig) error {
	fmt.Printf("%s Tool Usage Validation Rules:\n", color.New(color.FgYellow).Sprint("🔧"))

	if len(cfg.Rules.ToolUsageValidation) == 0 {
		fmt.Println("   No tool usage validation rules configured")
		return nil
	}

	for _, rule := range cfg.Rules.ToolUsageValidation {
		fmt.Printf("   Tool: %s | Pattern: %s | Action: %s\n",
			rule.Tool, rule.Pattern, rule.Action)
		if rule.Message != nil {
			fmt.Printf("      Message: %s\n", *rule.Message)
		}
	}

	return nil
}

func visualizeAllRules(cfg *config.ConclaudeConfig) error {
	fmt.Printf("%s Configuration Overview:\n\n", color.New(color.FgGreen).Sprint("📋"))

	fmt.Printf("%s Prevent Root Additions: %v\n",
		color.New(color.FgRed).Sprint("🚫"),
		cfg.Rules.PreventRootAdditions)

	fmt.Printf("%s Uneditable Files: %d patterns\n",
		color.New(color.FgBlue).Sprint("📁"),
		len(cfg.Rules.UneditableFiles))

	fmt.Printf("%s Tool Usage Validation: %d rules\n",
		color.New(color.FgYellow).Sprint("🔧"),
		len(cfg.Rules.ToolUsageValidation))

	fmt.Printf("%s Infinite Mode: %v\n",
		color.New(color.FgMagenta).Sprint("♾️"),
		cfg.Stop.Infinite)

	if cfg.Stop.Rounds != nil {
		fmt.Printf("%s Rounds Mode: %d rounds\n",
			color.New(color.FgCyan).Sprint("🔄"),
			*cfg.Stop.Rounds)
	}

	fmt.Println("\nUse --rule <rule-name> to see details for a specific rule")
	fmt.Println("Use --show-matches to see which files match the patterns")

	return nil
}
