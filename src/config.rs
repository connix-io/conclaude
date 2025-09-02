use anyhow::{Context, Result};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};

/// Configuration interface for grep rules
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GrepRule {
    #[serde(rename = "filePattern")]
    pub file_pattern: String,
    #[serde(rename = "forbiddenPattern")]
    pub forbidden_pattern: String,
    pub description: String,
}

/// Configuration for individual stop commands with optional messages
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct StopCommand {
    pub run: String,
    #[serde(default)]
    pub message: Option<String>,
}

/// Configuration interface for stop hook commands
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct StopConfig {
    #[serde(default)]
    pub run: String,
    #[serde(default)]
    pub commands: Vec<StopCommand>,
    #[serde(default)]
    pub infinite: bool,
    #[serde(default, rename = "infiniteMessage")]
    pub infinite_message: Option<String>,
    #[serde(default)]
    pub rounds: Option<u32>,
    #[serde(default, rename = "grepRules")]
    pub grep_rules: Vec<GrepRule>,
}

impl Default for StopConfig {
    fn default() -> Self {
        Self {
            run: String::new(),
            commands: Vec::new(),
            infinite: false,
            infinite_message: None,
            rounds: None,
            grep_rules: Vec::new(),
        }
    }
}

/// Configuration interface for validation rules
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RulesConfig {
    #[serde(default, rename = "preventRootAdditions")]
    pub prevent_root_additions: bool,
    #[serde(default, rename = "uneditableFiles")]
    pub uneditable_files: Vec<String>,
    #[serde(default, rename = "toolUsageValidation")]
    pub tool_usage_validation: Vec<ToolUsageRule>,
}

/// Tool usage validation rule
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ToolUsageRule {
    pub tool: String,
    pub pattern: String,
    pub action: String, // "block" or "allow"
    pub message: Option<String>,
}

impl Default for RulesConfig {
    fn default() -> Self {
        Self {
            prevent_root_additions: true,
            uneditable_files: Vec::new(),
            tool_usage_validation: Vec::new(),
        }
    }
}

/// Configuration for pre tool use hooks
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PreToolUseConfig {
    #[serde(default, rename = "grepRules")]
    pub grep_rules: Vec<GrepRule>,
    #[serde(default, rename = "preventAdditions")]
    pub prevent_additions: Vec<String>,
}

impl Default for PreToolUseConfig {
    fn default() -> Self {
        Self {
            grep_rules: Vec::new(),
            prevent_additions: Vec::new(),
        }
    }
}

/// Configuration for git worktree auto finish
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitWorktreeConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default, rename = "autoCreatePR")]
    pub auto_create_pr: bool,
}

impl Default for GitWorktreeConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            auto_create_pr: false,
        }
    }
}

/// Main configuration interface matching the TypeScript version
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ConclaudeConfig {
    pub stop: StopConfig,
    pub rules: RulesConfig,
    #[serde(default, rename = "preToolUse")]
    pub pre_tool_use: PreToolUseConfig,
    #[serde(default, rename = "gitWorktree")]
    pub git_worktree: GitWorktreeConfig,
}

impl Default for ConclaudeConfig {
    fn default() -> Self {
        Self {
            stop: StopConfig::default(),
            rules: RulesConfig::default(),
            pre_tool_use: PreToolUseConfig::default(),
            git_worktree: GitWorktreeConfig::default(),
        }
    }
}

/// Load YAML configuration using native search strategies
/// Search strategy: searches up directory tree until a config file is found
pub async fn load_conclaude_config() -> Result<ConclaudeConfig> {
    let search_paths = get_config_search_paths()?;

    for path in &search_paths {
        if path.exists() {
            let content = fs::read_to_string(path)
                .with_context(|| format!("Failed to read config file: {}", path.display()))?;

            let config: ConclaudeConfig = serde_yaml::from_str(&content)
                .with_context(|| format!("Failed to parse config file: {}", path.display()))?;

            return Ok(config);
        }
    }

    // If no config file is found, show search locations
    let search_locations: Vec<String> = search_paths
        .iter()
        .map(|p| format!("  • {}", p.display()))
        .collect();

    let error_message = format!(
        "Configuration file not found.\n\nSearched the following locations:\n{}\n\nCreate a .conclaude.yaml or .conclaude.yml file with stop and rules sections.\nRun 'conclaude init' to generate a template configuration.",
        search_locations.join("\n")
    );

    Err(anyhow::anyhow!(error_message))
}

fn get_config_search_paths() -> Result<Vec<PathBuf>> {
    let mut paths = Vec::new();
    let mut current_dir = std::env::current_dir()?;

    loop {
        // Add .conclaude.yaml and .conclaude.yml to search paths
        paths.push(current_dir.join(".conclaude.yaml"));
        paths.push(current_dir.join(".conclaude.yml"));

        // Check if we've reached the project root (directory containing package.json)
        if current_dir.join("package.json").exists() {
            break;
        }

        // Move to parent directory
        match current_dir.parent() {
            Some(parent) => current_dir = parent.to_path_buf(),
            None => break, // Reached filesystem root
        }
    }

    Ok(paths)
}

/// Extracts individual commands from a bash script string
pub fn extract_bash_commands(bash_script: &str) -> Result<Vec<String>> {
    let analyzer_script = format!(
        r#"#!/bin/bash
# This script outputs plain text lines, NOT JSON

# Process each line of the input script
while IFS= read -r line; do
  # Skip empty lines and comments
  if [[ -z "${{line// }}" ]] || [[ "$line" =~ ^[[:space:]]*# ]]; then
    continue
  fi
  
  # Output in a simple delimited format (NOT JSON)
  echo "CMD:$line"
done << 'EOF'
{}
EOF"#,
        bash_script
    );

    let output = Command::new("bash")
        .arg("-c")
        .arg(&analyzer_script)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .context("Failed to execute bash command analyzer")?;

    let mut commands = Vec::new();

    if !output.stdout.is_empty() {
        let stdout = String::from_utf8(output.stdout)
            .context("Failed to parse bash analyzer stdout as UTF-8")?;

        for line in stdout.lines() {
            if let Some(command) = line.strip_prefix("CMD:") {
                if !command.is_empty() {
                    commands.push(command.to_string());
                }
            }
        }
    }

    // Check for errors
    if !output.stderr.is_empty() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        log::warn!("Bash reported errors: {}", stderr);
    }

    Ok(commands)
}

/// Generate a default configuration file content
/// The configuration is embedded at compile time from default-config.yaml
pub fn generate_default_config() -> String {
    include_str!("default-config.yaml").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_bash_commands_single() {
        let script = "echo hello";
        let commands = extract_bash_commands(script).unwrap();
        assert_eq!(commands, vec!["echo hello"]);
    }

    #[test]
    fn test_extract_bash_commands_multiple() {
        let script = "echo hello\nnpm install\nnpm test";
        let commands = extract_bash_commands(script).unwrap();
        assert_eq!(commands, vec!["echo hello", "npm install", "npm test"]);
    }

    #[test]
    fn test_extract_bash_commands_ignores_comments() {
        let script = "# This is a comment\necho hello\n# Another comment\nnpm test";
        let commands = extract_bash_commands(script).unwrap();
        assert_eq!(commands, vec!["echo hello", "npm test"]);
    }

    #[test]
    fn test_extract_bash_commands_ignores_empty_lines() {
        let script = "echo hello\n\nnpm test\n";
        let commands = extract_bash_commands(script).unwrap();
        assert_eq!(commands, vec!["echo hello", "npm test"]);
    }

    #[test]
    fn test_extract_bash_commands_complex() {
        let script = r#"nix develop -c "lint"
bun x tsc --noEmit
cd /tmp && echo "test""#;
        let commands = extract_bash_commands(script).unwrap();
        assert_eq!(
            commands,
            vec![
                r#"nix develop -c "lint""#,
                "bun x tsc --noEmit",
                r#"cd /tmp && echo "test""#
            ]
        );
    }
}
