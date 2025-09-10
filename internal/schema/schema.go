package schema

import (
	"encoding/json"
	"fmt"
	"os"

	"github.com/connix-io/conclaude-go/internal/config"
	"github.com/invopop/jsonschema"
)

const DefaultSchemaURL = "https://raw.githubusercontent.com/connix-io/conclaude/main/conclaude-schema.json"

// GenerateSchema generates JSON schema for the ConclaudeConfig
func GenerateSchema() (string, error) {
	reflector := jsonschema.Reflector{
		AllowAdditionalProperties:  false,
		RequiredFromJSONSchemaTags: true,
		DoNotReference:             true,
	}

	schema := reflector.Reflect(&config.ConclaudeConfig{})

	// Set schema metadata
	schema.Title = "Conclaude Configuration"
	schema.Description = "Configuration schema for conclaude hook handler"
	schema.Version = "http://json-schema.org/draft-07/schema#"

	schemaBytes, err := json.MarshalIndent(schema, "", "  ")
	if err != nil {
		return "", fmt.Errorf("failed to marshal schema to JSON: %w", err)
	}

	return string(schemaBytes), nil
}

// WriteSchemaToFile writes the JSON schema to a file
func WriteSchemaToFile(schema string, filePath string) error {
	if err := os.WriteFile(filePath, []byte(schema), 0644); err != nil {
		return fmt.Errorf("failed to write schema to file %s: %w", filePath, err)
	}
	return nil
}

// ValidateConfigAgainstSchema validates a YAML config string against the JSON schema
func ValidateConfigAgainstSchema(configYAML string) error {
	// For now, we'll do basic validation by attempting to unmarshal
	// In a full implementation, you'd use a proper JSON schema validator
	var config config.ConclaudeConfig
	return json.Unmarshal([]byte(configYAML), &config)
}

// GenerateYAMLLanguageServerHeader generates the YAML language server header
func GenerateYAMLLanguageServerHeader(customSchemaURL *string) string {
	schemaURL := DefaultSchemaURL
	if customSchemaURL != nil && *customSchemaURL != "" {
		schemaURL = *customSchemaURL
	}

	return fmt.Sprintf("# yaml-language-server: $schema=%s\n\n", schemaURL)
}

// GetSchemaURL returns the default schema URL
func GetSchemaURL() string {
	return DefaultSchemaURL
}
