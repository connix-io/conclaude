package cmd

import (
	"fmt"
	"path/filepath"

	"github.com/connix-io/conclaude-go/internal/config"
	"github.com/connix-io/conclaude-go/internal/schema"
	"github.com/fatih/color"
	"github.com/spf13/cobra"
)

var (
	schemaOutput   string
	schemaValidate bool
)

// generateSchemaCmd represents the generate-schema command
var generateSchemaCmd = &cobra.Command{
	Use:   "generate-schema",
	Short: "Generate JSON Schema for conclaude configuration",
	Long: `Generate JSON Schema for conclaude configuration.

Creates a JSON Schema file that can be used for validation and IDE support
when editing .conclaude.yaml configuration files.`,
	RunE: runGenerateSchema,
}

func init() {
	generateSchemaCmd.Flags().
		StringVarP(&schemaOutput, "output", "o", "conclaude-schema.json", "Output file path for the schema")
	generateSchemaCmd.Flags().
		BoolVar(&schemaValidate, "validate", false, "Validate the generated schema")
}

func runGenerateSchema(cmd *cobra.Command, args []string) error {
	outputPath, err := filepath.Abs(schemaOutput)
	if err != nil {
		return fmt.Errorf("failed to resolve output path: %w", err)
	}

	fmt.Printf(
		"%s Generating JSON Schema for conclaude configuration...\n",
		color.New(color.FgBlue).Sprint("🔧"),
	)

	// Generate the schema
	schemaJSON, err := schema.GenerateSchema()
	if err != nil {
		return fmt.Errorf("failed to generate JSON schema: %w", err)
	}

	// Write schema to file
	if err := schema.WriteSchemaToFile(schemaJSON, outputPath); err != nil {
		return fmt.Errorf("failed to write schema to file: %w", err)
	}

	fmt.Printf("%s Schema generated successfully:\n", color.New(color.FgGreen).Sprint("✅"))
	fmt.Printf("   %s\n", outputPath)

	// Optionally validate the schema
	if schemaValidate {
		fmt.Printf("\n%s Validating generated schema...\n", color.New(color.FgBlue).Sprint("🔍"))

		// Test with the default configuration
		defaultConfig := config.GenerateDefaultConfigYAML()
		if err := schema.ValidateConfigAgainstSchema(defaultConfig); err != nil {
			return fmt.Errorf("default configuration failed schema validation: %w", err)
		}

		fmt.Printf("%s Schema validation passed!\n", color.New(color.FgGreen).Sprint("✅"))
		fmt.Println("   Default configuration is valid against the generated schema.")
	}

	// Display schema URL info
	schemaURL := schema.GetSchemaURL()
	fmt.Printf("\n%s Schema URL for YAML language server:\n", color.New(color.FgBlue).Sprint("📋"))
	fmt.Printf("   %s\n", schemaURL)

	fmt.Printf(
		"\n%s Add this header to your .conclaude.yaml files for IDE support:\n",
		color.New(color.FgYellow).Sprint("💡"),
	)
	fmt.Printf("   %s", schema.GenerateYAMLLanguageServerHeader(nil))

	return nil
}
