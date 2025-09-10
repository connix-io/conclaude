package cmd

import (
	"os"

	"github.com/spf13/cobra"
)

var (
	verbose            bool
	disableFileLogging bool
	Version            = "0.1.2" // matches Rust version
)

var rootCmd = &cobra.Command{
	Use:     "conclaude",
	Short:   "Claude Code Hook Handler",
	Long:    `Claude Code Hook Handler - Processes hook events via JSON payloads from stdin`,
	Version: Version,
	PersistentPreRun: func(cmd *cobra.Command, args []string) {
		// Set logging level environment variable
		if verbose {
			os.Setenv("CONCLAUDE_LOG_LEVEL", "debug")
		}

		// Set file logging environment variable based on CLI flag
		if disableFileLogging {
			os.Setenv("CONCLAUDE_DISABLE_FILE_LOGGING", "true")
		}
	},
}

func Execute() error {
	return rootCmd.Execute()
}

func init() {
	// Global flags
	rootCmd.PersistentFlags().
		BoolVarP(&verbose, "verbose", "v", false, "Enable verbose logging output")
	rootCmd.PersistentFlags().BoolVar(&disableFileLogging, "disable-file-logging", false,
		"Disable logging to temporary files (overrides CONCLAUDE_DISABLE_FILE_LOGGING)")

	// Add subcommands
	rootCmd.AddCommand(initCmd)
	rootCmd.AddCommand(generateSchemaCmd)
	rootCmd.AddCommand(preToolUseCmd)
	rootCmd.AddCommand(postToolUseCmd)
	rootCmd.AddCommand(notificationCmd)
	rootCmd.AddCommand(userPromptSubmitCmd)
	rootCmd.AddCommand(sessionStartCmd)
	rootCmd.AddCommand(stopCmd)
	rootCmd.AddCommand(subagentStopCmd)
	rootCmd.AddCommand(preCompactCmd)
	rootCmd.AddCommand(visualizeCmd)
}
