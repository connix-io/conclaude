use anyhow::{Context, Result};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};

/// Configuration interface for stop hook commands
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct StopConfig {
    pub run: String,
    #[serde(default)]
    pub infinite: bool,
    #[serde(default, rename = "infiniteMessage")]
    pub infinite_message: Option<String>,
}

impl Default for StopConfig {
    fn default() -> Self {
        Self {
            run: String::new(),
            infinite: false,
            infinite_message: None,
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
}

impl Default for RulesConfig {
    fn default() -> Self {
        Self {
            prevent_root_additions: true,
            uneditable_files: Vec::new(),
        }
    }
}

/// Main configuration interface matching the TypeScript version
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ConclaudeConfig {
    pub stop: StopConfig,
    pub rules: RulesConfig,
}

impl Default for ConclaudeConfig {
    fn default() -> Self {
        Self {
            stop: StopConfig::default(),
            rules: RulesConfig::default(),
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
    let search_locations: Vec<String> = search_paths.iter()
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