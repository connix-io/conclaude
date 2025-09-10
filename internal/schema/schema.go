package schema

import (
	"encoding/json"
	"fmt"
	"os"

	"github.com/connix-io/conclaude/internal/config"
	"github.com/invopop/jsonschema"
	jsonschemavalidator "github.com/santhosh-tekuri/jsonschema/v6"
	"gopkg.in/yaml.v3"
)

const (
	DefaultSchemaURL = "https://raw.githubusercontent.com/connix-io/conclaude/" +
		"main/conclaude-schema.json"

	// File permission constants
	SchemaFilePermission = 0600

	// String constants
	EmptyString = ""
)

// GenerateSchema generates JSON schema for the ConclaudeConfig.
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
	// Remove the Version field - let the library handle it

	schemaBytes, err := json.MarshalIndent(schema, "", "  ")
	if err != nil {
		return "", fmt.Errorf("failed to marshal schema to JSON: %w", err)
	}

	return string(schemaBytes), nil
}

// WriteSchemaToFile writes the JSON schema to a file.
func WriteSchemaToFile(schema string, filePath string) error {
	err := os.WriteFile(filePath, []byte(schema), SchemaFilePermission)
	if err != nil {
		return fmt.Errorf(
			"failed to write schema to file %s: %w",
			filePath,
			err,
		)
	}

	return nil
}

// ValidateConfigAgainstSchema validates a YAML config string against the
// JSON schema.
func ValidateConfigAgainstSchema(configYAML string) error {
	// First, convert YAML to JSON for validation
	var yamlData interface{}
	err := yaml.Unmarshal([]byte(configYAML), &yamlData)
	if err != nil {
		return fmt.Errorf("failed to parse YAML: %w", err)
	}

	jsonData, err := json.Marshal(yamlData)
	if err != nil {
		return fmt.Errorf("failed to convert YAML to JSON: %w", err)
	}

	// Generate the schema
	schemaStr, err := GenerateSchema()
	if err != nil {
		return fmt.Errorf("failed to generate schema: %w", err)
	}

	// Parse schema to add as resource
	var schemaDoc interface{}
	if err := json.Unmarshal([]byte(schemaStr), &schemaDoc); err != nil {
		return fmt.Errorf("failed to parse schema: %w", err)
	}
	
	// Compile the schema using the v6 API
	compiler := jsonschemavalidator.NewCompiler()
	compiler.DefaultDraft(jsonschemavalidator.Draft7)
	
	// Add the schema as a resource
	if err := compiler.AddResource("schema.json", schemaDoc); err != nil {
		return fmt.Errorf("failed to add schema resource: %w", err)
	}
	
	// Compile the schema
	schema, err := compiler.Compile("schema.json")
	if err != nil {
		return fmt.Errorf("failed to compile schema: %w", err)
	}

	// Parse the JSON data for validation
	var instance interface{}
	if err := json.Unmarshal(jsonData, &instance); err != nil {
		return fmt.Errorf("failed to parse JSON for validation: %w", err)
	}

	// Validate against the schema
	if err := schema.Validate(instance); err != nil {
		return fmt.Errorf("validation failed: %w", err)
	}

	return nil
}

// GenerateYAMLLanguageServerHeader generates the YAML language server header.
func GenerateYAMLLanguageServerHeader(customSchemaURL *string) string {
	schemaURL := DefaultSchemaURL
	if customSchemaURL != nil && *customSchemaURL != EmptyString {
		schemaURL = *customSchemaURL
	}

	return fmt.Sprintf("# yaml-language-server: $schema=%s\n\n", schemaURL)
}

// GetSchemaURL returns the default schema URL.
func GetSchemaURL() string {
	return DefaultSchemaURL
}
