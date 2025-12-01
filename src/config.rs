// Final test - expecting both workflows to succeed
use anyhow::{Context, Result};
use conclaude_field_derive::FieldList;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// Configuration for individual stop commands with optional messages
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, FieldList)]
#[serde(deny_unknown_fields)]
pub struct StopCommand {
    pub run: String,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default, rename = "showStdout")]
    pub show_stdout: Option<bool>,
    #[serde(default, rename = "showStderr")]
    pub show_stderr: Option<bool>,
    #[serde(default, rename = "maxOutputLines")]
    #[schemars(range(min = 1, max = 10000))]
    pub max_output_lines: Option<u32>,
}

/// Configuration for individual subagent stop commands with optional messages
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, FieldList)]
#[serde(deny_unknown_fields)]
pub struct SubagentStopCommand {
    pub run: String,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default, rename = "showStdout")]
    pub show_stdout: Option<bool>,
    #[serde(default, rename = "showStderr")]
    pub show_stderr: Option<bool>,
    #[serde(default, rename = "maxOutputLines")]
    #[schemars(range(min = 1, max = 10000))]
    pub max_output_lines: Option<u32>,
}

/// Configuration for subagent stop hooks with pattern-based command execution
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default, FieldList)]
#[serde(deny_unknown_fields)]
pub struct SubagentStopConfig {
    /// Map of agent ID patterns to lists of commands to execute when matching subagents stop.
    /// Patterns support glob syntax: "*" matches all, "coder" exact match, "test*" prefix, "*coder" suffix.
    #[serde(default)]
    pub commands: std::collections::HashMap<String, Vec<SubagentStopCommand>>,
}

/// Configuration interface for stop hook commands
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default, FieldList)]
#[serde(deny_unknown_fields)]
pub struct StopConfig {
    #[serde(default)]
    pub commands: Vec<StopCommand>,
    #[serde(default)]
    pub infinite: bool,
    #[serde(default, rename = "infiniteMessage")]
    pub infinite_message: Option<String>,
    #[serde(default)]
    pub rounds: Option<u32>,
}

/// Tool usage validation rule
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ToolUsageRule {
    pub tool: String,
    pub pattern: String,
    pub action: String, // "block" or "allow"
    pub message: Option<String>,
    #[serde(rename = "commandPattern")]
    pub command_pattern: Option<String>,
    #[serde(rename = "matchMode")]
    pub match_mode: Option<String>,
}

/// Configuration for an uneditable file rule.
///
/// Supports two formats:
/// - Simple: `"*.lock"` - Matches files with generic error message
/// - Detailed: `{pattern: "*.lock", message: "..."}` - Custom error message
///
/// The `#[serde(untagged)]` attribute allows serde to automatically handle both
/// plain string patterns and detailed object configurations.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum UnEditableFileRule {
    /// Detailed format with pattern and optional custom message
    #[serde(rename_all = "camelCase")]
    Detailed {
        pattern: String,
        #[serde(default)]
        message: Option<String>,
    },
    /// Simple format: just a glob pattern string
    Simple(String),
}

impl UnEditableFileRule {
    /// Extract the pattern from either variant
    #[must_use]
    pub fn pattern(&self) -> &str {
        match self {
            UnEditableFileRule::Simple(pattern) => pattern,
            UnEditableFileRule::Detailed { pattern, .. } => pattern,
        }
    }

    /// Get the custom message if present (only from Detailed variant)
    #[must_use]
    pub fn message(&self) -> Option<&str> {
        match self {
            UnEditableFileRule::Detailed {
                message: Some(msg), ..
            } => Some(msg),
            _ => None,
        }
    }
}

/// Default function that returns true for serde defaults
fn default_true() -> bool {
    true
}

/// Default function for database enabled field
fn default_database_enabled() -> bool {
    true
}

/// Configuration for database logging
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, FieldList)]
#[serde(deny_unknown_fields)]
pub struct DatabaseConfig {
    /// Whether database logging is enabled
    #[serde(default = "default_database_enabled")]
    pub enabled: bool,
    /// Optional custom path to the database file (overrides platform defaults)
    #[serde(default)]
    pub path: Option<String>,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            path: None,
        }
    }
}

/// Configuration for pre tool use hooks
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, FieldList)]
#[serde(deny_unknown_fields)]
pub struct PreToolUseConfig {
    #[serde(default, rename = "preventAdditions")]
    pub prevent_additions: Vec<String>,
    #[serde(default = "default_true", rename = "preventGeneratedFileEdits")]
    pub prevent_generated_file_edits: bool,
    #[serde(default, rename = "generatedFileMessage")]
    pub generated_file_message: Option<String>,
    #[serde(default = "default_true", rename = "preventRootAdditions")]
    pub prevent_root_additions: bool,
    #[serde(default, rename = "uneditableFiles")]
    pub uneditable_files: Vec<UnEditableFileRule>,
    /// Block Claude from modifying or creating files that match .gitignore patterns
    #[serde(default, rename = "preventUpdateGitIgnored")]
    pub prevent_update_git_ignored: bool,
    #[serde(default, rename = "toolUsageValidation")]
    pub tool_usage_validation: Vec<ToolUsageRule>,
}

impl Default for PreToolUseConfig {
    fn default() -> Self {
        Self {
            prevent_additions: Vec::new(),
            prevent_generated_file_edits: true,
            generated_file_message: None,
            prevent_root_additions: true,
            uneditable_files: Vec::new(),
            prevent_update_git_ignored: false,
            tool_usage_validation: Vec::new(),
        }
    }
}

/// Configuration for system notifications
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default, FieldList)]
#[serde(deny_unknown_fields)]
pub struct NotificationsConfig {
    /// Whether notifications are enabled
    #[serde(default)]
    pub enabled: bool,
    /// List of hook names that should trigger notifications. Use ["*"] for all hooks
    #[serde(default)]
    pub hooks: Vec<String>,
    /// Whether to show error notifications
    #[serde(default, rename = "showErrors")]
    pub show_errors: bool,
    /// Whether to show success notifications
    #[serde(default, rename = "showSuccess")]
    pub show_success: bool,
    /// Whether to show system event notifications
    #[serde(default = "default_show_system_events", rename = "showSystemEvents")]
    pub show_system_events: bool,
}

/// Configuration for permission request hooks
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default, FieldList)]
#[serde(deny_unknown_fields)]
pub struct PermissionRequestConfig {
    /// Default action when a tool is requested: "allow" or "deny"
    pub default: String,
    /// Tools to explicitly allow (supports glob patterns)
    #[serde(default)]
    pub allow: Option<Vec<String>>,
    /// Tools to explicitly deny (supports glob patterns)
    #[serde(default)]
    pub deny: Option<Vec<String>>,
}

fn default_show_system_events() -> bool {
    true
}

impl NotificationsConfig {
    /// Check if notifications are enabled for a specific hook
    #[must_use]
    pub fn is_enabled_for(&self, hook_name: &str) -> bool {
        if !self.enabled {
            return false;
        }

        // Check for wildcard
        if self.hooks.iter().any(|hook| hook == "*") {
            return true;
        }

        // Check for specific hook name
        self.hooks.iter().any(|hook| hook == hook_name)
    }
}

/// Main configuration interface matching the TypeScript version
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
#[serde(deny_unknown_fields)]
pub struct ConclaudeConfig {
    #[serde(default)]
    pub stop: StopConfig,
    #[serde(default, rename = "subagentStop")]
    pub subagent_stop: SubagentStopConfig,
    #[serde(default, rename = "preToolUse")]
    pub pre_tool_use: PreToolUseConfig,
    #[serde(default)]
    pub notifications: NotificationsConfig,
    #[serde(default, rename = "permissionRequest")]
    pub permission_request: Option<PermissionRequestConfig>,
    #[serde(default)]
    pub database: DatabaseConfig,
}

/// Extract the field name from an unknown field error message
fn extract_unknown_field(error_msg: &str) -> Option<String> {
    // Try to extract the field name from "unknown field `fieldName`"
    if let Some(start) = error_msg.find("unknown field `") {
        let start_idx = start + "unknown field `".len();
        if let Some(end_idx) = error_msg[start_idx..].find('`') {
            return Some(error_msg[start_idx..start_idx + end_idx].to_string());
        }
    }
    None
}

/// Suggest similar field names based on the unknown field
fn suggest_similar_fields(unknown_field: &str, section: &str) -> Vec<String> {
    let all_fields: Vec<(&str, Vec<&str>)> = vec![
        ("stop", StopConfig::field_names()),
        ("subagentStop", SubagentStopConfig::field_names()),
        ("preToolUse", PreToolUseConfig::field_names()),
        ("notifications", NotificationsConfig::field_names()),
        ("permissionRequest", PermissionRequestConfig::field_names()),
        ("database", DatabaseConfig::field_names()),
        ("commands", StopCommand::field_names()),
        ("subagentStopCommands", SubagentStopCommand::field_names()),
    ];

    // Find the section's valid fields
    let empty_fields: Vec<&str> = vec![];
    let valid_fields = all_fields
        .iter()
        .find(|(s, _)| *s == section)
        .map(|(_, fields)| fields)
        .unwrap_or(&empty_fields);

    // Calculate Levenshtein distance and suggest close matches
    let mut suggestions: Vec<(usize, &str)> = valid_fields
        .iter()
        .map(|field| {
            let distance = levenshtein_distance(unknown_field, field);
            (distance, *field)
        })
        .filter(|(dist, _)| *dist <= 3) // Only suggest if distance is 3 or less
        .collect();

    suggestions.sort_by_key(|(dist, _)| *dist);
    suggestions
        .into_iter()
        .map(|(_, field)| field.to_string())
        .take(3)
        .collect()
}

/// Calculate Levenshtein distance between two strings
fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.chars().count();
    let len2 = s2.chars().count();
    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

    for (i, row) in matrix.iter_mut().enumerate().take(len1 + 1) {
        row[0] = i;
    }
    for (j, cell) in matrix[0].iter_mut().enumerate().take(len2 + 1) {
        *cell = j;
    }

    for (i, c1) in s1.chars().enumerate() {
        for (j, c2) in s2.chars().enumerate() {
            let cost = if c1.eq_ignore_ascii_case(&c2) { 0 } else { 1 };
            matrix[i + 1][j + 1] = std::cmp::min(
                std::cmp::min(matrix[i][j + 1] + 1, matrix[i + 1][j] + 1),
                matrix[i][j] + cost,
            );
        }
    }

    matrix[len1][len2]
}

/// Extract section name from error message (e.g., "stop.infinite" -> "stop")
fn extract_section_from_error(error_msg: &str) -> Option<String> {
    // Look for patterns like "stop:", "rules.", "notifications:"
    if let Some(colon_idx) = error_msg.find(':') {
        let before_colon = &error_msg[..colon_idx];
        if let Some(last_word) = before_colon.split_whitespace().last() {
            if let Some(section) = last_word.split('.').next() {
                return Some(section.to_string());
            }
        }
    }
    None
}

/// Format a descriptive error message for YAML parsing failures
fn format_parse_error(error: &serde_yaml::Error, config_path: &Path) -> String {
    let base_error = error.to_string();
    let mut parts = vec![
        format!(
            "Failed to parse configuration file: {}",
            config_path.display()
        ),
        String::new(),
        format!("Error: {}", base_error),
    ];

    // Extract line number if present
    let has_line_number = base_error.contains("at line");

    // Add specific guidance based on error type
    if base_error.contains("unknown field") {
        parts.push(String::new());

        // Try to extract the unknown field and suggest alternatives
        let unknown_field = extract_unknown_field(&base_error);
        let section = extract_section_from_error(&base_error);

        if let (Some(field), Some(sec)) = (unknown_field, section) {
            let suggestions = suggest_similar_fields(&field, &sec);
            if !suggestions.is_empty() {
                parts.push("💡 Did you mean one of these?".to_string());
                for suggestion in &suggestions {
                    parts.push(format!("   • {suggestion}"));
                }
                parts.push(String::new());
            }
        }

        parts.push("Common causes:".to_string());
        parts.push("  • Typo in field name (check spelling and capitalization)".to_string());
        parts.push("  • Using a field that doesn't exist in this section".to_string());
        parts.push("  • Using camelCase vs snake_case incorrectly (use camelCase)".to_string());
        parts.push(String::new());
        parts.push("Valid field names by section:".to_string());
        parts.push("  stop: commands, infinite, infiniteMessage, rounds".to_string());
        parts.push("  subagentStop: commands".to_string());
        parts.push(
            "  preToolUse: preventAdditions, preventGeneratedFileEdits, generatedFileMessage, preventRootAdditions, uneditableFiles, preventUpdateGitIgnored, toolUsageValidation"
                .to_string(),
        );
        parts.push(
            "  notifications: enabled, hooks, showErrors, showSuccess, showSystemEvents"
                .to_string(),
        );
        parts.push("  permissionRequest: default, allow, deny".to_string());
        parts.push("  database: enabled, path".to_string());
        parts.push("  commands (stop): run, message, showStdout, showStderr, maxOutputLines".to_string());
        parts.push("  commands (subagentStop): run, message, showStdout, showStderr, maxOutputLines".to_string());
    } else if base_error.contains("invalid type") {
        parts.push(String::new());
        parts.push("Type mismatch detected. Common causes:".to_string());
        parts.push(
            "  • Using quotes around a boolean value (use true/false without quotes)".to_string(),
        );
        parts.push("  • Using a string where a number is expected (remove quotes)".to_string());
        parts.push("  • Using a single value where an array is expected (wrap in [])".to_string());
        parts.push(String::new());
        parts.push("✅ Examples of correct formatting:".to_string());
        parts.push("   Boolean:  infinite: true             # no quotes".to_string());
        parts.push("   Number:   rounds: 3                  # no quotes".to_string());
        parts.push("   Number:   maxOutputLines: 100        # no quotes".to_string());
        parts.push("   String:   run: \"cargo test\"          # with quotes".to_string());
        parts.push("   Array:    hooks: [\"Stop\"]            # square brackets".to_string());
        parts.push("   Array:    uneditableFiles: []        # empty array".to_string());
    } else if base_error.contains("expected") || base_error.contains("while parsing") {
        parts.push(String::new());
        parts.push("YAML syntax error detected. Common causes:".to_string());
        parts.push(
            "  • Incorrect indentation (YAML requires consistent spaces, not tabs)".to_string(),
        );
        parts.push("  • Missing colon (:) after a field name".to_string());
        parts.push("  • Unmatched quotes or brackets".to_string());
        parts.push("  • Using tabs instead of spaces for indentation".to_string());

        if has_line_number {
            parts.push(String::new());
            parts.push("💡 Check the line number above and the lines around it.".to_string());
        }

        parts.push(String::new());
        parts.push("✅ YAML formatting tips:".to_string());
        parts.push("   • Use 2 spaces for each indentation level".to_string());
        parts.push("   • Always put a space after the colon: 'key: value'".to_string());
        parts.push("   • Use quotes for strings with special characters".to_string());
        parts.push("   • Arrays can be: [item1, item2] or on separate lines with -".to_string());
    } else if base_error.contains("missing field") {
        parts.push(String::new());
        parts.push("A required field is missing from the configuration.".to_string());
        parts.push("Check the default configuration with: conclaude init".to_string());
    }

    parts.push(String::new());
    parts.push("For a valid configuration template, run:".to_string());
    parts.push("  conclaude init".to_string());

    parts.join("\n")
}

/// Parse and validate configuration content from a string
///
/// # Errors
///
/// Returns an error if YAML parsing fails or validation constraints are violated.
pub fn parse_and_validate_config(content: &str, config_path: &Path) -> Result<ConclaudeConfig> {
    let config: ConclaudeConfig = serde_yaml::from_str(content).map_err(|e| {
        let error_msg = format_parse_error(&e, config_path);
        anyhow::anyhow!(error_msg)
    })?;

    validate_config_constraints(&config)?;

    Ok(config)
}

/// Validate configuration values against constraints
fn validate_config_constraints(config: &ConclaudeConfig) -> Result<()> {
    // Validate maxOutputLines range (1-10000)
    for (idx, command) in config.stop.commands.iter().enumerate() {
        if let Some(max_lines) = command.max_output_lines {
            if !(1..=10000).contains(&max_lines) {
                let error_msg = format!(
                    "Range validation failed for stop.commands[{idx}].maxOutputLines\n\n\
                     Error: Value {max_lines} is out of valid range\n\n\
                     ✅ Valid range: 1 to 10000\n\n\
                     Common causes:\n\
                       • Value is too large (maximum is 10000)\n\
                       • Value is too small (minimum is 1)\n\
                       • Using a negative number\n\n\
                     Example valid configurations:\n\
                       maxOutputLines: 100      # default, good for most cases\n\
                       maxOutputLines: 1000     # for verbose output\n\
                       maxOutputLines: 10000    # maximum allowed\n\n\
                     For a valid configuration template, run:\n\
                       conclaude init"
                );
                return Err(anyhow::anyhow!(error_msg));
            }
        }
    }

    // Validate rounds if specified
    if let Some(rounds) = config.stop.rounds {
        if rounds == 0 {
            let error_msg = "Range validation failed for stop.rounds\n\n\
                 Error: Value must be at least 1\n\n\
                 ✅ Valid range: 1 or greater (or omit for no limit)\n\n\
                 Common causes:\n\
                   • Using 0 (use infinite: true instead for unlimited rounds)\n\
                   • Negative values are not allowed\n\n\
                 Example valid configurations:\n\
                   rounds: 1        # run once\n\
                   rounds: 3        # run three times\n\
                   infinite: true   # unlimited (don't use rounds)\n\n\
                 For a valid configuration template, run:\n\
                   conclaude init"
                .to_string();
            return Err(anyhow::anyhow!(error_msg));
        }
    }

    // Validate permissionRequest.default if specified
    if let Some(permission_request) = &config.permission_request {
        let default_value = permission_request.default.to_lowercase();
        if default_value != "allow" && default_value != "deny" {
            let error_msg = format!(
                "Validation failed for permissionRequest.default\n\n\
                 Error: Invalid value '{}'\n\n\
                 ✅ Valid values: \"allow\" or \"deny\"\n\n\
                 Common causes:\n\
                   • Typo in value (check spelling)\n\
                   • Using a value other than allow or deny\n\n\
                 Example valid configurations:\n\
                   permissionRequest:\n\
                     default: allow    # allow all tools by default\n\
                   \n\
                   permissionRequest:\n\
                     default: deny     # deny all tools by default\n\n\
                 For a valid configuration template, run:\n\
                   conclaude init",
                permission_request.default
            );
            return Err(anyhow::anyhow!(error_msg));
        }
    }

    // Validate database configuration
    if let Some(ref path) = config.database.path {
        if path.is_empty() {
            let error_msg = "Validation failed for database.path\n\n\
                 Error: Path cannot be empty\n\n\
                 Common causes:\n\
                   • Using an empty string for the path\n\
                   • Remove the path field to use platform defaults\n\n\
                 Example valid configurations:\n\
                   database:\n\
                     enabled: true\n\
                     path: \"/custom/path/conclaude.db\"    # custom path\n\
                   \n\
                   database:\n\
                     enabled: true                          # uses platform defaults\n\n\
                 For a valid configuration template, run:\n\
                   conclaude init"
                .to_string();
            return Err(anyhow::anyhow!(error_msg));
        }

        // Check if parent directory exists (if path is absolute)
        let path_buf = PathBuf::from(path);
        if path_buf.is_absolute() {
            if let Some(parent) = path_buf.parent() {
                if !parent.exists() {
                    let error_msg = format!(
                        "Validation failed for database.path\n\n\
                         Error: Parent directory does not exist: {}\n\n\
                         Common causes:\n\
                           • Directory path does not exist\n\
                           • Typo in directory path\n\
                           • Need to create the directory first\n\n\
                         Example valid configurations:\n\
                           database:\n\
                             enabled: true\n\
                             path: \"/custom/path/conclaude.db\"    # ensure /custom/path exists\n\
                           \n\
                           database:\n\
                             enabled: true                          # uses platform defaults\n\n\
                         For a valid configuration template, run:\n\
                           conclaude init",
                        parent.display()
                    );
                    return Err(anyhow::anyhow!(error_msg));
                }
            }
        }
    }

    // Validate subagentStop configuration
    for (pattern, commands) in &config.subagent_stop.commands {
        // Validate pattern is not empty
        if pattern.trim().is_empty() {
            let error_msg = "Validation failed for subagentStop.commands\n\n\
                 Error: Pattern key cannot be empty\n\n\
                 ✅ Valid patterns: \"*\" (all), \"coder\" (exact), \"test*\" (prefix), \"*coder\" (suffix)\n\n\
                 Example valid configurations:\n\
                   subagentStop:\n\
                     commands:\n\
                       \"*\":\n\
                         - run: \"echo all subagents\"\n\
                       \"coder\":\n\
                         - run: \"npm run lint\"\n\n\
                 For a valid configuration template, run:\n\
                   conclaude init"
                .to_string();
            return Err(anyhow::anyhow!(error_msg));
        }

        // Validate maxOutputLines range for each command
        for (idx, command) in commands.iter().enumerate() {
            if let Some(max_lines) = command.max_output_lines {
                if !(1..=10000).contains(&max_lines) {
                    let error_msg = format!(
                        "Range validation failed for subagentStop.commands[\"{pattern}\"][{idx}].maxOutputLines\n\n\
                         Error: Value {max_lines} is out of valid range\n\n\
                         ✅ Valid range: 1 to 10000\n\n\
                         Common causes:\n\
                           • Value is too large (maximum is 10000)\n\
                           • Value is too small (minimum is 1)\n\
                           • Using a negative number\n\n\
                         Example valid configurations:\n\
                           maxOutputLines: 100      # default, good for most cases\n\
                           maxOutputLines: 1000     # for verbose output\n\
                           maxOutputLines: 10000    # maximum allowed\n\n\
                         For a valid configuration template, run:\n\
                           conclaude init"
                    );
                    return Err(anyhow::anyhow!(error_msg));
                }
            }
        }
    }

    Ok(())
}

/// Load YAML configuration using native search strategies
///
/// Search strategy: searches up directory tree from the starting directory,
/// checking for `.conclaude.yaml` or `.conclaude.yml` in each parent directory.
/// The search stops when either:
/// - A config file is found, OR
/// - The filesystem root is reached, OR
/// - 12 directory levels have been searched
///
/// # Arguments
///
/// * `start_dir` - Optional starting directory for config search. If None, uses current directory.
///
/// # Errors
///
/// Returns an error if no configuration file is found, file reading fails, or YAML parsing fails.
pub async fn load_conclaude_config(start_dir: Option<&Path>) -> Result<(ConclaudeConfig, PathBuf)> {
    let search_paths = get_config_search_paths(start_dir)?;

    for path in &search_paths {
        if path.exists() {
            let content = fs::read_to_string(path)
                .with_context(|| format!("Failed to read config file: {}", path.display()))?;

            let config = parse_and_validate_config(&content, path)?;

            return Ok((config, path.clone()));
        }
    }

    // If no config file is found, show search locations
    let search_locations: Vec<String> = search_paths
        .iter()
        .map(|p| format!("  • {}", p.display()))
        .collect();

    let error_message = format!(
        "Configuration file not found.\n\nSearched the following locations:\n{}\n\nCreate a .conclaude.yaml or .conclaude.yml file with stop and preToolUse sections.\nRun 'conclaude init' to generate a template configuration.",
        search_locations.join("\n")
    );

    Err(anyhow::anyhow!(error_message))
}

fn get_config_search_paths(start_dir: Option<&Path>) -> Result<Vec<PathBuf>> {
    let mut paths = Vec::new();
    let mut current_dir = match start_dir {
        Some(dir) => dir.to_path_buf(),
        None => std::env::current_dir()?,
    };
    let mut levels_searched = 0;
    const MAX_SEARCH_LEVELS: u32 = 12;

    loop {
        // Add .conclaude.yaml and .conclaude.yml to search paths
        paths.push(current_dir.join(".conclaude.yaml"));
        paths.push(current_dir.join(".conclaude.yml"));

        // Move to parent directory first, then increment level count
        match current_dir.parent() {
            Some(parent) => {
                current_dir = parent.to_path_buf();
                levels_searched += 1;

                // Check if we've reached the maximum search level limit
                if levels_searched >= MAX_SEARCH_LEVELS {
                    break;
                }
            }
            None => break, // Reached filesystem root
        }
    }

    Ok(paths)
}

/// Extracts individual commands from a bash script string
///
/// # Errors
///
/// Returns an error if the bash command execution fails or UTF-8 parsing fails.
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
{bash_script}
EOF"#
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
        eprintln!("Bash reported errors: {stderr}");
    }

    Ok(commands)
}

/// Generate a default configuration file content
/// The configuration is embedded at compile time from default-config.yaml
#[must_use]
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

    #[test]
    fn test_field_list_generation() {
        // Verify that the generated field_names() methods return the correct field names
        assert_eq!(
            StopConfig::field_names(),
            vec!["commands", "infinite", "infiniteMessage", "rounds"]
        );

        assert_eq!(
            PreToolUseConfig::field_names(),
            vec![
                "preventAdditions",
                "preventGeneratedFileEdits",
                "generatedFileMessage",
                "preventRootAdditions",
                "uneditableFiles",
                "preventUpdateGitIgnored",
                "toolUsageValidation"
            ]
        );

        assert_eq!(
            NotificationsConfig::field_names(),
            vec![
                "enabled",
                "hooks",
                "showErrors",
                "showSuccess",
                "showSystemEvents"
            ]
        );

        assert_eq!(
            StopCommand::field_names(),
            vec![
                "run",
                "message",
                "showStdout",
                "showStderr",
                "maxOutputLines"
            ]
        );
    }

    #[test]
    fn test_suggest_similar_fields_common_typo() {
        // Test common typo: "showStdOut" should suggest "showStdout"
        let suggestions = suggest_similar_fields("showStdOut", "commands");
        assert!(
            !suggestions.is_empty(),
            "Should suggest fields for common typo"
        );
        assert_eq!(
            suggestions[0], "showStdout",
            "First suggestion should be 'showStdout'"
        );
    }

    #[test]
    fn test_suggest_similar_fields_case_insensitive() {
        // Test case-insensitive matching: "INFINITE" should suggest "infinite"
        let suggestions = suggest_similar_fields("INFINITE", "stop");
        assert!(
            !suggestions.is_empty(),
            "Should suggest fields ignoring case"
        );
        assert!(
            suggestions.contains(&"infinite".to_string()),
            "Should suggest 'infinite' for 'INFINITE'"
        );
    }

    #[test]
    fn test_suggest_similar_fields_distance_threshold() {
        // Test that only suggestions within distance 3 are returned
        // "infinit" (distance 1) should be suggested
        let suggestions = suggest_similar_fields("infinit", "stop");
        assert!(
            suggestions.contains(&"infinite".to_string()),
            "Should suggest 'infinite' for 'infinit' (distance 1)"
        );

        // "infinte" (distance 1, missing 'i') should be suggested
        let suggestions = suggest_similar_fields("infinte", "stop");
        assert!(
            suggestions.contains(&"infinite".to_string()),
            "Should suggest 'infinite' for 'infinte' (distance 1)"
        );

        // "wxyz" has distance > 3 from all stop fields, should not suggest anything
        let suggestions = suggest_similar_fields("wxyz", "stop");
        assert!(
            suggestions.is_empty(),
            "Should not suggest anything for 'wxyz' (distance > 3 from all fields)"
        );
    }

    #[test]
    fn test_suggest_similar_fields_no_close_matches() {
        // Test that empty results are returned when no close matches exist
        let suggestions = suggest_similar_fields("completelywrongfield", "stop");
        assert!(
            suggestions.is_empty(),
            "Should return empty for field with no close matches"
        );

        let suggestions = suggest_similar_fields("abcdefgh", "rules");
        assert!(
            suggestions.is_empty(),
            "Should return empty when distance exceeds threshold"
        );
    }

    #[test]
    fn test_suggest_similar_fields_sorted_by_distance() {
        // Test that suggestions are sorted by distance (closest first)
        // "messag" (distance 1 from "message") should come before anything with higher distance
        let suggestions = suggest_similar_fields("messag", "commands");
        if !suggestions.is_empty() {
            assert_eq!(
                suggestions[0], "message",
                "Closest match should be first in suggestions"
            );
        }
    }

    #[test]
    fn test_suggest_similar_fields_max_three_suggestions() {
        // Test that at most 3 suggestions are returned
        let suggestions = suggest_similar_fields("sho", "commands");
        assert!(
            suggestions.len() <= 3,
            "Should return at most 3 suggestions, got {}",
            suggestions.len()
        );
    }

    #[test]
    fn test_suggest_similar_fields_invalid_section() {
        // Test that empty results are returned for invalid section
        let suggestions = suggest_similar_fields("infinite", "invalid_section");
        assert!(
            suggestions.is_empty(),
            "Should return empty for invalid section"
        );
    }

    #[test]
    fn test_suggest_similar_fields_notifications_section() {
        // Test suggestions for notifications section
        let suggestions = suggest_similar_fields("enable", "notifications");
        assert!(
            suggestions.contains(&"enabled".to_string()),
            "Should suggest 'enabled' for 'enable' in notifications section"
        );
    }

    #[test]
    fn test_suggest_similar_fields_pretooluse_section() {
        // Test suggestions for preToolUse section with camelCase field
        let suggestions = suggest_similar_fields("preventRootAddition", "preToolUse");
        assert!(
            suggestions.contains(&"preventRootAdditions".to_string()),
            "Should suggest 'preventRootAdditions' for 'preventRootAddition'"
        );
    }

    #[test]
    fn test_config_without_rules_section_works() {
        // Test that configuration without rules section works normally
        let valid_config = r#"
preToolUse:
  preventRootAdditions: true
  uneditableFiles: []
  preventAdditions: []
  preventGeneratedFileEdits: true
  toolUsageValidation: []

stop:
  commands: []
  infinite: false

notifications:
  enabled: false
  hooks: []
  showErrors: false
  showSuccess: false
  showSystemEvents: true
"#;

        let result = parse_and_validate_config(valid_config, Path::new("test.yaml"));
        assert!(
            result.is_ok(),
            "Should accept config without rules section: {:?}",
            result.err()
        );
    }

    #[test]
    fn test_permission_request_valid_config() {
        // Test valid permissionRequest configuration
        let config_yaml = r#"
permissionRequest:
  default: allow
  allow:
    - Read
    - Write
  deny:
    - Bash

stop:
  commands: []
  infinite: false

preToolUse:
  preventRootAdditions: true
  uneditableFiles: []
  preventAdditions: []
  preventGeneratedFileEdits: true
  toolUsageValidation: []

notifications:
  enabled: false
  hooks: []
  showErrors: false
  showSuccess: false
  showSystemEvents: true
"#;

        let result = parse_and_validate_config(config_yaml, Path::new("test.yaml"));
        assert!(
            result.is_ok(),
            "Valid permissionRequest config should parse: {:?}",
            result.err()
        );

        let config = result.unwrap();
        assert!(
            config.permission_request.is_some(),
            "permission_request should be populated"
        );
        let pr = config.permission_request.unwrap();
        assert_eq!(pr.default, "allow");
        assert_eq!(pr.allow.as_ref().unwrap().len(), 2);
        assert_eq!(pr.deny.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn test_permission_request_invalid_default() {
        // Test that invalid default value is rejected
        let config_yaml = r#"
permissionRequest:
  default: invalid_value

stop:
  commands: []
  infinite: false

preToolUse:
  preventRootAdditions: true
  uneditableFiles: []
  preventAdditions: []
  preventGeneratedFileEdits: true
  toolUsageValidation: []

notifications:
  enabled: false
  hooks: []
  showErrors: false
  showSuccess: false
  showSystemEvents: true
"#;

        let result = parse_and_validate_config(config_yaml, Path::new("test.yaml"));
        assert!(
            result.is_err(),
            "Invalid default value should fail validation"
        );
        let error = result.err().unwrap().to_string();
        assert!(
            error.contains("allow") && error.contains("deny"),
            "Error message should mention valid values"
        );
    }

    #[test]
    fn test_permission_request_optional() {
        // Test that permissionRequest is optional
        let config_yaml = r#"
stop:
  commands: []
  infinite: false

preToolUse:
  preventRootAdditions: true
  uneditableFiles: []
  preventAdditions: []
  preventGeneratedFileEdits: true
  toolUsageValidation: []

notifications:
  enabled: false
  hooks: []
  showErrors: false
  showSuccess: false
  showSystemEvents: true
"#;

        let result = parse_and_validate_config(config_yaml, Path::new("test.yaml"));
        assert!(
            result.is_ok(),
            "Config without permissionRequest should parse: {:?}",
            result.err()
        );

        let config = result.unwrap();
        assert!(
            config.permission_request.is_none(),
            "permission_request should be None when not specified"
        );
    }

    #[test]
    fn test_permission_request_field_list() {
        // Test that PermissionRequestConfig field names are correct
        assert_eq!(
            PermissionRequestConfig::field_names(),
            vec!["default", "allow", "deny"]
        );
    }

    #[test]
    fn test_uneditable_file_rule_simple_string_format() {
        // Test that simple string patterns deserialize correctly
        let yaml = r#"
preToolUse:
  uneditableFiles:
    - "*.lock"
    - ".env"
  preventAdditions: []
  preventGeneratedFileEdits: true
  preventRootAdditions: true
  toolUsageValidation: []

stop:
  commands: []
  infinite: false

notifications:
  enabled: false
  hooks: []
  showErrors: false
  showSuccess: false
  showSystemEvents: true
"#;
        let config: ConclaudeConfig = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(config.pre_tool_use.uneditable_files.len(), 2);

        // Verify patterns extracted correctly
        assert_eq!(
            config.pre_tool_use.uneditable_files[0].pattern(),
            "*.lock"
        );
        assert_eq!(config.pre_tool_use.uneditable_files[1].pattern(), ".env");

        // Verify no custom messages
        assert!(config.pre_tool_use.uneditable_files[0].message().is_none());
        assert!(config.pre_tool_use.uneditable_files[1].message().is_none());
    }

    #[test]
    fn test_uneditable_file_rule_detailed_object_format() {
        // Test that detailed objects with pattern and message deserialize correctly
        let yaml = r#"
preToolUse:
  uneditableFiles:
    - pattern: "*.lock"
      message: "Lock files are auto-generated. Run 'npm install' to update."
    - pattern: ".env"
      message: "Environment files contain secrets."
  preventAdditions: []
  preventGeneratedFileEdits: true
  preventRootAdditions: true
  toolUsageValidation: []

stop:
  commands: []
  infinite: false

notifications:
  enabled: false
  hooks: []
  showErrors: false
  showSuccess: false
  showSystemEvents: true
"#;
        let config: ConclaudeConfig = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(config.pre_tool_use.uneditable_files.len(), 2);

        // Verify patterns extracted correctly
        assert_eq!(
            config.pre_tool_use.uneditable_files[0].pattern(),
            "*.lock"
        );
        assert_eq!(config.pre_tool_use.uneditable_files[1].pattern(), ".env");

        // Verify custom messages
        assert_eq!(
            config.pre_tool_use.uneditable_files[0].message(),
            Some("Lock files are auto-generated. Run 'npm install' to update.")
        );
        assert_eq!(
            config.pre_tool_use.uneditable_files[1].message(),
            Some("Environment files contain secrets.")
        );
    }

    #[test]
    fn test_uneditable_file_rule_mixed_format() {
        // Test that arrays can mix both simple strings and detailed objects
        let yaml = r#"
preToolUse:
  uneditableFiles:
    - "*.lock"
    - pattern: ".env"
      message: "Secrets must not be committed."
    - "package.json"
  preventAdditions: []
  preventGeneratedFileEdits: true
  preventRootAdditions: true
  toolUsageValidation: []

stop:
  commands: []
  infinite: false

notifications:
  enabled: false
  hooks: []
  showErrors: false
  showSuccess: false
  showSystemEvents: true
"#;
        let config: ConclaudeConfig = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(config.pre_tool_use.uneditable_files.len(), 3);

        // First is simple format
        assert_eq!(
            config.pre_tool_use.uneditable_files[0].pattern(),
            "*.lock"
        );
        assert!(config.pre_tool_use.uneditable_files[0].message().is_none());

        // Second is detailed format with message
        assert_eq!(config.pre_tool_use.uneditable_files[1].pattern(), ".env");
        assert_eq!(
            config.pre_tool_use.uneditable_files[1].message(),
            Some("Secrets must not be committed.")
        );

        // Third is simple format
        assert_eq!(
            config.pre_tool_use.uneditable_files[2].pattern(),
            "package.json"
        );
        assert!(config.pre_tool_use.uneditable_files[2].message().is_none());
    }

    #[test]
    fn test_uneditable_file_rule_detailed_without_message() {
        // Test that detailed format without message field works (message is optional)
        let yaml = r#"
preToolUse:
  uneditableFiles:
    - pattern: "*.lock"
  preventAdditions: []
  preventGeneratedFileEdits: true
  preventRootAdditions: true
  toolUsageValidation: []

stop:
  commands: []
  infinite: false

notifications:
  enabled: false
  hooks: []
  showErrors: false
  showSuccess: false
  showSystemEvents: true
"#;
        let config: ConclaudeConfig = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(config.pre_tool_use.uneditable_files.len(), 1);
        assert_eq!(
            config.pre_tool_use.uneditable_files[0].pattern(),
            "*.lock"
        );
        assert!(config.pre_tool_use.uneditable_files[0].message().is_none());
    }

    #[test]
    fn test_uneditable_file_rule_backward_compatibility() {
        // Test that existing configs with simple string format still work
        let yaml = r#"
preToolUse:
  preventRootAdditions: true
  uneditableFiles:
    - "*.lock"
    - ".env"
    - "package-lock.json"
  preventAdditions: []
  preventGeneratedFileEdits: true
  toolUsageValidation: []

stop:
  commands: []
  infinite: false

notifications:
  enabled: false
  hooks: []
  showErrors: false
  showSuccess: false
  showSystemEvents: true
"#;

        let result = parse_and_validate_config(yaml, Path::new("test.yaml"));
        assert!(
            result.is_ok(),
            "Backward compatible config should parse: {:?}",
            result.err()
        );

        let config = result.unwrap();
        assert_eq!(config.pre_tool_use.uneditable_files.len(), 3);

        // Verify all patterns are extracted correctly
        let patterns: Vec<&str> = config
            .pre_tool_use
            .uneditable_files
            .iter()
            .map(|r| r.pattern())
            .collect();
        assert_eq!(patterns, vec!["*.lock", ".env", "package-lock.json"]);
    }

    #[test]
    fn test_database_config_defaults() {
        // Test that database config has correct defaults
        let config = DatabaseConfig::default();
        assert!(config.enabled, "Database should be enabled by default");
        assert!(config.path.is_none(), "Path should be None by default");
    }

    #[test]
    fn test_database_config_with_custom_path() {
        // Test that database config can be parsed with custom path
        let yaml = r#"
database:
  enabled: true
  path: "/tmp/custom.db"

stop:
  commands: []
  infinite: false

preToolUse:
  preventRootAdditions: true
  uneditableFiles: []
  preventAdditions: []
  preventGeneratedFileEdits: true
  toolUsageValidation: []

notifications:
  enabled: false
  hooks: []
  showErrors: false
  showSuccess: false
  showSystemEvents: true
"#;

        let result = parse_and_validate_config(yaml, Path::new("test.yaml"));
        assert!(
            result.is_ok(),
            "Database config with custom path should parse: {:?}",
            result.err()
        );

        let config = result.unwrap();
        assert!(config.database.enabled);
        assert_eq!(config.database.path, Some("/tmp/custom.db".to_string()));
    }

    #[test]
    fn test_database_config_disabled() {
        // Test that database can be disabled
        let yaml = r#"
database:
  enabled: false

stop:
  commands: []
  infinite: false

preToolUse:
  preventRootAdditions: true
  uneditableFiles: []
  preventAdditions: []
  preventGeneratedFileEdits: true
  toolUsageValidation: []

notifications:
  enabled: false
  hooks: []
  showErrors: false
  showSuccess: false
  showSystemEvents: true
"#;

        let result = parse_and_validate_config(yaml, Path::new("test.yaml"));
        assert!(
            result.is_ok(),
            "Database config disabled should parse: {:?}",
            result.err()
        );

        let config = result.unwrap();
        assert!(!config.database.enabled);
        assert!(config.database.path.is_none());
    }

    #[test]
    fn test_database_config_empty_path_fails() {
        // Test that empty path string fails validation
        let yaml = r#"
database:
  enabled: true
  path: ""

stop:
  commands: []
  infinite: false

preToolUse:
  preventRootAdditions: true
  uneditableFiles: []
  preventAdditions: []
  preventGeneratedFileEdits: true
  toolUsageValidation: []

notifications:
  enabled: false
  hooks: []
  showErrors: false
  showSuccess: false
  showSystemEvents: true
"#;

        let result = parse_and_validate_config(yaml, Path::new("test.yaml"));
        assert!(
            result.is_err(),
            "Database config with empty path should fail validation"
        );
        let error = result.err().unwrap().to_string();
        assert!(
            error.contains("Path cannot be empty"),
            "Error should mention empty path"
        );
    }

    #[test]
    fn test_database_config_nonexistent_parent_fails() {
        // Test that path with non-existent parent directory fails validation
        let yaml = r#"
database:
  enabled: true
  path: "/nonexistent/directory/that/does/not/exist/test.db"

stop:
  commands: []
  infinite: false

preToolUse:
  preventRootAdditions: true
  uneditableFiles: []
  preventAdditions: []
  preventGeneratedFileEdits: true
  toolUsageValidation: []

notifications:
  enabled: false
  hooks: []
  showErrors: false
  showSuccess: false
  showSystemEvents: true
"#;

        let result = parse_and_validate_config(yaml, Path::new("test.yaml"));
        assert!(
            result.is_err(),
            "Database config with non-existent parent should fail validation"
        );
        let error = result.err().unwrap().to_string();
        assert!(
            error.contains("Parent directory does not exist"),
            "Error should mention parent directory"
        );
    }

    #[test]
    fn test_database_config_optional() {
        // Test that database config is optional and uses defaults
        let yaml = r#"
stop:
  commands: []
  infinite: false

preToolUse:
  preventRootAdditions: true
  uneditableFiles: []
  preventAdditions: []
  preventGeneratedFileEdits: true
  toolUsageValidation: []

notifications:
  enabled: false
  hooks: []
  showErrors: false
  showSuccess: false
  showSystemEvents: true
"#;

        let result = parse_and_validate_config(yaml, Path::new("test.yaml"));
        assert!(
            result.is_ok(),
            "Config without database section should parse: {:?}",
            result.err()
        );

        let config = result.unwrap();
        assert!(config.database.enabled, "Should use default enabled=true");
        assert!(config.database.path.is_none(), "Should use default path=None");
    }

    #[test]
    fn test_database_field_list() {
        // Test that DatabaseConfig field names are correct
        assert_eq!(
            DatabaseConfig::field_names(),
            vec!["enabled", "path"]
        );
    }

    #[test]
    fn test_uneditable_file_rule_pattern_extraction() {
        // Test the pattern() method for both variants
        let simple = UnEditableFileRule::Simple("*.txt".to_string());
        assert_eq!(simple.pattern(), "*.txt");

        let detailed = UnEditableFileRule::Detailed {
            pattern: "*.md".to_string(),
            message: Some("Custom message".to_string()),
        };
        assert_eq!(detailed.pattern(), "*.md");
    }

    #[test]
    fn test_uneditable_file_rule_message_extraction() {
        // Test the message() method for all cases
        let simple = UnEditableFileRule::Simple("*.txt".to_string());
        assert!(simple.message().is_none());

        let detailed_with_msg = UnEditableFileRule::Detailed {
            pattern: "*.md".to_string(),
            message: Some("Custom message".to_string()),
        };
        assert_eq!(detailed_with_msg.message(), Some("Custom message"));

        let detailed_without_msg = UnEditableFileRule::Detailed {
            pattern: "*.md".to_string(),
            message: None,
        };
        assert!(detailed_without_msg.message().is_none());
    }
}
