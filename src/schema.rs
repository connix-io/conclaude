use anyhow::{Context, Result};
use schemars::schema_for;
use serde_json::{Value, json};
use std::fs;
use std::path::PathBuf;

use crate::config::ConclaudeConfig;

/// Generates a JSON Schema for the `ConclaudeConfig` structure
///
/// # Errors
///
/// Returns an error if JSON schema generation or value serialization fails.
pub fn generate_config_schema() -> Result<Value> {
    let schema = schema_for!(ConclaudeConfig);

    // Enhance the schema with additional metadata
    let mut schema_value = serde_json::to_value(schema)?;

    // Add metadata to the root schema
    if let Value::Object(ref mut schema_obj) = schema_value {
        schema_obj.insert(
            "$schema".to_string(),
            json!("http://json-schema.org/draft-07/schema#"),
        );
        schema_obj.insert("title".to_string(), json!("Conclaude Configuration"));
        schema_obj.insert(
            "description".to_string(),
            json!("Configuration schema for Conclaude - Claude Code hook handler"),
        );
        schema_obj.insert("$id".to_string(), json!("https://github.com/conneroisu/conclaude/releases/latest/download/conclaude-schema.json"));
    }

    Ok(schema_value)
}

/// Writes the generated schema to a file
///
/// # Errors
///
/// Returns an error if JSON serialization fails, directory creation fails, or file writing fails.
pub fn write_schema_to_file(schema: &Value, output_path: &PathBuf) -> Result<()> {
    let schema_json =
        serde_json::to_string_pretty(schema).context("Failed to serialize schema to JSON")?;

    // Ensure parent directory exists
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
    }

    fs::write(output_path, schema_json)
        .with_context(|| format!("Failed to write schema to: {}", output_path.display()))?;

    Ok(())
}

/// Validates that a given YAML content matches the schema
///
/// # Errors
///
/// Returns an error if the YAML content is invalid or does not match the expected structure.
pub fn validate_config_against_schema(config_content: &str) -> Result<()> {
    // Parse the YAML to ensure it's valid
    let _: ConclaudeConfig = serde_yaml::from_str(config_content)
        .context("Configuration file contains invalid YAML or does not match expected structure")?;

    Ok(())
}

/// Gets the default schema URL for YAML language server headers
#[must_use]
pub fn get_schema_url() -> String {
    "https://github.com/conneroisu/conclaude/releases/latest/download/conclaude-schema.json"
        .to_string()
}

/// Generates a YAML language server header comment with schema URL
#[must_use]
pub fn generate_yaml_language_server_header(custom_schema_url: Option<&str>) -> String {
    let default_url = get_schema_url();
    let schema_url = custom_schema_url.unwrap_or(&default_url);
    format!("# yaml-language-server: $schema={schema_url}\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_generate_config_schema() {
        let schema = generate_config_schema().unwrap();

        // Check that the schema is a valid JSON object
        assert!(schema.is_object());

        // Check for required metadata
        let schema_obj = schema.as_object().unwrap();
        assert!(schema_obj.contains_key("$schema"));
        assert!(schema_obj.contains_key("title"));
        assert!(schema_obj.contains_key("description"));
        assert!(schema_obj.contains_key("$id"));
        assert!(schema_obj.contains_key("type"));
        assert!(schema_obj.contains_key("properties"));
    }

    #[test]
    fn test_write_schema_to_file() {
        let schema = generate_config_schema().unwrap();
        let temp_dir = tempdir().unwrap();
        let schema_path = temp_dir.path().join("test-schema.json");

        write_schema_to_file(&schema, &schema_path).unwrap();

        // Verify file exists and contains valid JSON
        assert!(schema_path.exists());
        let content = fs::read_to_string(&schema_path).unwrap();
        let _: Value = serde_json::from_str(&content).unwrap();
    }

    #[test]
    fn test_validate_config_against_schema() {
        let valid_config = r#"
stop:
  run: "echo test"
  infinite: false
rules:
  preventRootAdditions: true
  uneditableFiles: []
"#;

        validate_config_against_schema(valid_config).unwrap();

        let invalid_config = r#"
invalid_field: "should fail"
"#;

        assert!(validate_config_against_schema(invalid_config).is_err());
    }

    #[test]
    fn test_generate_yaml_language_server_header() {
        let default_header = generate_yaml_language_server_header(None);
        assert!(default_header.contains("yaml-language-server:"));
        assert!(default_header.contains("github.com/conneroisu/conclaude"));

        let custom_header =
            generate_yaml_language_server_header(Some("https://example.com/schema.json"));
        assert!(custom_header.contains("https://example.com/schema.json"));
    }

    #[test]
    fn test_get_schema_url() {
        let url = get_schema_url();
        assert!(url.starts_with("https://"));
        assert!(url.contains("github.com"));
        assert!(url.to_lowercase().ends_with(".json"));
    }
}
