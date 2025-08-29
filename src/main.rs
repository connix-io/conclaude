mod config;
mod hooks;
mod logger;
mod schema;
mod types;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use hooks::*;
use std::fs;
use std::path::PathBuf;

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Claude Code hook handler CLI tool that processes hook events and manages lifecycle hooks
#[derive(Parser)]
#[command(
    name = "conclaude",
    version = VERSION,
    about = "Claude Code Hook Handler - Processes hook events via JSON payloads from stdin",
    long_about = None
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose logging output
    #[arg(short, long, global = true)]
    verbose: bool,

    /// Disable logging to temporary files (overrides CONCLAUDE_DISABLE_FILE_LOGGING)
    #[arg(long, global = true)]
    disable_file_logging: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize conclaude configuration and Claude Code hooks
    Init {
        /// Path for .conclaude.yaml file
        #[arg(long)]
        config_path: Option<String>,

        /// Path for .claude directory
        #[arg(long)]
        claude_path: Option<String>,

        /// Overwrite existing configuration files
        #[arg(short = 'f', long)]
        force: bool,

        /// Custom schema URL for YAML language server header
        #[arg(long)]
        schema_url: Option<String>,
    },
    /// Generate JSON Schema for conclaude configuration
    GenerateSchema {
        /// Output file path for the schema
        #[arg(short, long, default_value = "conclaude-schema.json")]
        output: String,

        /// Validate the generated schema
        #[arg(long)]
        validate: bool,
    },
    /// Process PreToolUse hook - fired before tool execution
    #[clap(name = "PreToolUse")]
    PreToolUse,
    /// Process PostToolUse hook - fired after tool execution
    #[clap(name = "PostToolUse")]
    PostToolUse,
    /// Process Notification hook - fired for system notifications
    #[clap(name = "Notification")]
    Notification,
    /// Process UserPromptSubmit hook - fired when user submits input
    #[clap(name = "UserPromptSubmit")]
    UserPromptSubmit,
    /// Process SessionStart hook - fired when session begins
    #[clap(name = "SessionStart")]
    SessionStart,
    /// Process Stop hook - fired when session terminates
    #[clap(name = "Stop")]
    Stop,
    /// Process SubagentStop hook - fired when subagent completes
    #[clap(name = "SubagentStop")]
    SubagentStop,
    /// Process PreCompact hook - fired before transcript compaction
    #[clap(name = "PreCompact")]
    PreCompact,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Set logging level environment variable
    if cli.verbose {
        std::env::set_var("CONCLAUDE_LOG_LEVEL", "debug");
    }

    // Set file logging environment variable based on CLI flag
    if cli.disable_file_logging {
        std::env::set_var("CONCLAUDE_DISABLE_FILE_LOGGING", "true");
    }

    match cli.command {
        Commands::Init {
            config_path,
            claude_path,
            force,
            schema_url,
        } => handle_init(config_path, claude_path, force, schema_url).await,
        Commands::GenerateSchema { output, validate } => {
            handle_generate_schema(output, validate).await
        }
        Commands::PreToolUse => handle_hook_result(handle_pre_tool_use).await,
        Commands::PostToolUse => handle_hook_result(handle_post_tool_use).await,
        Commands::Notification => handle_hook_result(handle_notification).await,
        Commands::UserPromptSubmit => handle_hook_result(handle_user_prompt_submit).await,
        Commands::SessionStart => handle_hook_result(handle_session_start).await,
        Commands::Stop => handle_hook_result(handle_stop).await,
        Commands::SubagentStop => handle_hook_result(handle_subagent_stop).await,
        Commands::PreCompact => handle_hook_result(handle_pre_compact).await,
    }
}

/// TypeScript interfaces for Claude Code settings structure
#[derive(serde::Serialize, serde::Deserialize)]
struct ClaudeHookConfig {
    #[serde(rename = "type")]
    config_type: String,
    command: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct ClaudeHookMatcher {
    matcher: String,
    hooks: Vec<ClaudeHookConfig>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct ClaudePermissions {
    allow: Vec<String>,
    deny: Vec<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct ClaudeSettings {
    #[serde(rename = "includeCoAuthoredBy")]
    include_co_authored_by: bool,
    permissions: ClaudePermissions,
    hooks: std::collections::HashMap<String, Vec<ClaudeHookMatcher>>,
}

/// Handles Init command to set up conclaude configuration and Claude Code hooks.
async fn handle_init(
    config_path: Option<String>,
    claude_path: Option<String>,
    force: bool,
    schema_url: Option<String>,
) -> Result<()> {
    let cwd = std::env::current_dir().context("Failed to get current directory")?;
    let config_path = config_path
        .map(PathBuf::from)
        .unwrap_or_else(|| cwd.join(".conclaude.yaml"));
    let claude_path = claude_path
        .map(PathBuf::from)
        .unwrap_or_else(|| cwd.join(".claude"));
    let settings_path = claude_path.join("settings.json");

    println!("🚀 Initializing conclaude configuration...\n");

    // Check if config file exists
    if config_path.exists() && !force {
        println!("⚠️  Configuration file already exists:");
        println!("   {}", config_path.display());
        println!("\nUse --force to overwrite existing configuration.");
        std::process::exit(1);
    }

    // Create .conclaude.yaml with YAML language server header
    let yaml_header = schema::generate_yaml_language_server_header(schema_url.as_deref());
    let config_content = format!("{}{}", yaml_header, config::generate_default_config());
    fs::write(&config_path, config_content)
        .with_context(|| format!("Failed to write config file: {}", config_path.display()))?;

    println!("✅ Created configuration file with YAML language server support:");
    println!("   {}", config_path.display());
    let default_schema_url = schema::get_schema_url();
    let used_schema_url = schema_url.as_deref().unwrap_or(&default_schema_url);
    println!("   Schema URL: {}", used_schema_url);

    // Create .claude directory if it doesn't exist
    fs::create_dir_all(&claude_path).with_context(|| {
        format!(
            "Failed to create .claude directory: {}",
            claude_path.display()
        )
    })?;

    // Handle settings.json
    let mut settings = if settings_path.exists() {
        let settings_content = fs::read_to_string(&settings_path).with_context(|| {
            format!("Failed to read settings file: {}", settings_path.display())
        })?;
        let settings: ClaudeSettings =
            serde_json::from_str(&settings_content).with_context(|| {
                format!("Failed to parse settings file: {}", settings_path.display())
            })?;
        println!("\n📝 Found existing Claude settings, updating hooks...");
        settings
    } else {
        println!("\n📝 Creating Claude Code settings...");
        ClaudeSettings {
            include_co_authored_by: false,
            permissions: ClaudePermissions {
                allow: Vec::new(),
                deny: Vec::new(),
            },
            hooks: std::collections::HashMap::new(),
        }
    };

    // Define all hook types and their commands
    let hook_types = [
        "UserPromptSubmit",
        "PreToolUse",
        "PostToolUse",
        "Notification",
        "Stop",
        "SubagentStop",
        "PreCompact",
        "SessionStart",
    ];

    // Add hook configurations
    for hook_type in &hook_types {
        settings.hooks.insert(
            hook_type.to_string(),
            vec![ClaudeHookMatcher {
                matcher: String::new(),
                hooks: vec![ClaudeHookConfig {
                    config_type: "command".to_string(),
                    command: format!("conclaude {}", hook_type),
                }],
            }],
        );
    }

    // Write updated settings
    let settings_json =
        serde_json::to_string_pretty(&settings).context("Failed to serialize settings to JSON")?;
    fs::write(&settings_path, settings_json)
        .with_context(|| format!("Failed to write settings file: {}", settings_path.display()))?;

    println!("✅ Updated Claude Code settings:");
    println!("   {}", settings_path.display());

    println!("\n🎉 Conclaude initialization complete!");
    println!("\nConfigured hooks:");
    for hook_type in &hook_types {
        println!("   • {}", hook_type);
    }
    println!("\nYou can now use Claude Code with conclaude hook handling.");

    Ok(())
}

/// Handles GenerateSchema command to generate JSON Schema for conclaude configuration.
async fn handle_generate_schema(output: String, validate: bool) -> Result<()> {
    let output_path = PathBuf::from(output);

    println!("🔧 Generating JSON Schema for conclaude configuration...");

    // Generate the schema
    let schema = schema::generate_config_schema().context("Failed to generate JSON schema")?;

    // Write schema to file
    schema::write_schema_to_file(&schema, &output_path)
        .context("Failed to write schema to file")?;

    println!("✅ Schema generated successfully:");
    println!("   {}", output_path.display());

    // Optionally validate the schema
    if validate {
        println!("\n🔍 Validating generated schema...");

        // Test with the default configuration
        let default_config = config::generate_default_config();
        schema::validate_config_against_schema(&default_config)
            .context("Default configuration failed schema validation")?;

        println!("✅ Schema validation passed!");
        println!("   Default configuration is valid against the generated schema.");
    }

    // Display schema URL info
    let schema_url = schema::get_schema_url();
    println!("\n📋 Schema URL for YAML language server:");
    println!("   {}", schema_url);

    println!("\n💡 Add this header to your .conclaude.yaml files for IDE support:");
    println!(
        "   {}",
        schema::generate_yaml_language_server_header(None).trim()
    );

    Ok(())
}
