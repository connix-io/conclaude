use anyhow::{Context, Result};
use std::path::PathBuf;

fn main() -> Result<()> {
    println!("🔧 Generating JSON Schema for conclaude configuration...");

    // Generate the schema
    let schema = conclaude::schema::generate_config_schema()
        .context("Failed to generate JSON schema")?;

    // Write schema to file in the workspace root
    let output_path = PathBuf::from("conclaude-schema.json");
    conclaude::schema::write_schema_to_file(&schema, &output_path)
        .context("Failed to write schema to file")?;

    println!(
        "✅ Schema generated successfully: {}",
        output_path.display()
    );

    // Display schema URL info
    let schema_url = conclaude::schema::get_schema_url();
    println!("📋 Schema URL for YAML language server: {}", schema_url);

    Ok(())
}
