import { type SpawnSyncReturns, spawnSync } from "child_process";
import { cosmiconfig } from "cosmiconfig";
import * as fs from "fs";
import { minimatch } from "minimatch";
import * as path from "path";
import { parse as parseYaml } from "yaml";

/**
 * Configuration interface for grep rules
 */
export interface GrepRule {
	filePattern: string;
	forbiddenPattern: string;
	description: string;
}

/**
 * Configuration interface for stop hook commands
 */
export interface StopConfig {
	run: string;
	infinite?: boolean;
	infiniteMessage?: string;
	grepRules?: GrepRule[];
}

/**
 * Configuration interface for PreToolUse hook validation
 */
export interface PreToolUseConfig {
	grepRules?: GrepRule[];
}

/**
 * Configuration interface for validation rules
 */
export interface RulesConfig {
	preventRootAdditions: boolean;
	uneditableFiles: string[];
}

/**
 * Main configuration interface matching conclaude.schema.json
 */
export interface ConclaudeConfig {
	stop: StopConfig;
	preToolUse?: PreToolUseConfig;
	rules: RulesConfig;
}

/**
 * Load YAML configuration using cosmiconfig's native search strategies
 * Only supports YAML configuration files
 * Search strategy: project - searches up directory tree until package.json is found
 */
export async function loadConclaudeConfig(): Promise<ConclaudeConfig> {
	const explorer = cosmiconfig("conclaude", {
		searchStrategy: "project",
		searchPlaces: [
			".conclaude.yaml",
			".conclaude.yml",
		],
		loaders: {
			".yaml": (filepath, content) => parseYaml(content),
			".yml": (filepath, content) => parseYaml(content),
		},
	});

	try {
		const result = await explorer.search();

		if (!result) {
			// Show common locations that would be searched
			const commonLocations = [
				process.cwd(),
				require("path").dirname(process.cwd()),
			]
				.map((dir) => [
					require("path").join(dir, ".conclaude.yaml"),
					require("path").join(dir, ".conclaude.yml"),
				])
				.flat();

			const errorMessage = [
				"Configuration file not found.",
				"",
				"Searched the following locations (and parent directories up to project root):",
				...commonLocations.map((location) => `  • ${location}`),
				"",
				"Search strategy: Current directory up to project root (directory containing package.json)",
				"",
				"Create a .conclaude.yaml or .conclaude.yml file with stop and rules sections.",
				"Run 'conclaude init' to generate a template configuration.",
			].join("\n");

			throw new Error(errorMessage);
		}

		return result.config as ConclaudeConfig;
	} catch (error) {
		// If it's already our formatted error, re-throw it
		if (
			error instanceof Error &&
			error.message.includes("Configuration file not found")
		) {
			throw error;
		}

		// Otherwise, it's a parsing error
		throw new Error(
			`Failed to parse configuration file: ${error instanceof Error ? error.message : String(error)}`,
		);
	}
}

/**
 * @param bashScript - the bash script to analyze
 * @returns an array of commands found in the bash script
 */
export function extractBashCommands(bashScript: string): string[] {
	// Create a bash script that outputs each command in a parseable format
	const analyzerScript = `#!/bin/bash
# This script outputs plain text lines, NOT JSON

# Process each line of the input script
while IFS= read -r line; do
  # Skip empty lines and comments
  if [[ -z "\${line// }" ]] || [[ "$line" =~ ^[[:space:]]*# ]]; then
    continue
  fi
  
  # Output in a simple delimited format (NOT JSON)
  echo "CMD:$line"
done << 'EOF'
${bashScript}
EOF
`;

	const result: SpawnSyncReturns<string> = spawnSync(
		"bash",
		[
			"-c",
			analyzerScript,
		],
		{
			encoding: "utf8", // This makes stdout/stderr be strings instead of Buffers
			shell: false,
		},
	);

	// result contains plain text like:
	// ```txt
	// CMD:echo "hello"
	// CMD:npm install
	// CMD:npm test
	// ```

	const commands: string[] = [];

	if (result.stdout) {
		// Split the PLAIN TEXT output by newlines
		const lines: string[] = result.stdout.split("\n");

		// Parse each PLAIN TEXT line
		for (const line of lines) {
			if (line.startsWith("CMD:")) {
				// Extract command from the plain text format
				const command = line.substring(4); // Remove 'CMD:' prefix
				if (command) {
					commands.push(command);
				}
			}
		}
	}

	// Check for errors (result.stderr is also a plain string)
	if (result.stderr) {
		console.error("Bash reported errors:", result.stderr);
	}

	return commands;
}

/**
 * Result of grep rule validation
 */
export interface GrepRuleViolation {
	rule: GrepRule;
	file: string;
	lineNumber: number;
	lineContent: string;
}

/**
 * Execute grep rules across the entire codebase for Stop hook validation
 *
 * @param grepRules - Array of grep rules to validate
 * @param baseDir - Base directory to search (defaults to current working directory)
 * @returns Array of violations found
 */
export function executeGrepRules(
	grepRules: GrepRule[],
	baseDir: string = process.cwd(),
): GrepRuleViolation[] {
	const violations: GrepRuleViolation[] = [];

	for (const rule of grepRules) {
		try {
			// Use ripgrep if available, otherwise fall back to find + grep pipeline
			const useRipgrep = isRipgrepAvailable();
			
			if (useRipgrep) {
				const args = buildRipgrepArgs(rule, baseDir);
				const result = spawnSync("rg", args, {
					encoding: "utf8",
					shell: false,
					cwd: baseDir,
				});

				if (result.status === 0 && result.stdout) {
					const matches = parseGrepOutput(result.stdout, rule);
					violations.push(...matches);
				}
			} else {
				// Use find + grep pipeline for complex patterns
				const matches = executeTraditionalGrepRule(rule, baseDir);
				violations.push(...matches);
			}
		} catch (error) {
			// If grep command fails, log the error but don't throw
			console.error(
				`Failed to execute grep rule for pattern '${rule.forbiddenPattern}':`,
				error,
			);
		}
	}

	return violations;
}

/**
 * Check a specific file against grep rules for PreToolUse hook validation
 *
 * @param filePath - Path to the file to check
 * @param grepRules - Array of grep rules to validate
 * @param baseDir - Base directory for relative path resolution (defaults to current working directory)
 * @returns Array of violations found in the file
 */
export function checkFileGrepRules(
	filePath: string,
	grepRules: GrepRule[],
	baseDir: string = process.cwd(),
): GrepRuleViolation[] {
	const violations: GrepRuleViolation[] = [];

	// Resolve the full path
	const resolvedPath = path.resolve(baseDir, filePath);
	const relativePath = path.relative(baseDir, resolvedPath);

	// Check if file exists
	if (!fs.existsSync(resolvedPath)) {
		return violations; // File doesn't exist yet (new file), no violations
	}

	for (const rule of grepRules) {
		try {
			// Check if file matches the pattern
			const matchesPattern =
				minimatch(filePath, rule.filePattern) ||
				minimatch(relativePath, rule.filePattern) ||
				minimatch(resolvedPath, rule.filePattern);

			if (!matchesPattern) {
				continue; // Skip this rule if file doesn't match the pattern
			}

			// Use ripgrep if available, otherwise fall back to grep
			const grepCommand = isRipgrepAvailable() ? "rg" : "grep";
			const args = buildGrepArgsForFile(grepCommand, rule, resolvedPath);

			const result = spawnSync(grepCommand, args, {
				encoding: "utf8",
				shell: false,
			});

			if (result.status === 0 && result.stdout) {
				const matches = parseGrepOutput(result.stdout, rule, filePath);
				violations.push(...matches);
			}
		} catch (error) {
			// If grep command fails, log the error but don't throw
			console.error(
				`Failed to check file '${filePath}' against rule '${rule.forbiddenPattern}':`,
				error,
			);
		}
	}

	return violations;
}

/**
 * Check if ripgrep (rg) is available on the system
 *
 * @returns true if ripgrep is available, false otherwise
 */
function isRipgrepAvailable(): boolean {
	try {
		const result = spawnSync(
			"rg",
			[
				"--version",
			],
			{
				stdio: "pipe",
				encoding: "utf8",
			},
		);
		return result.status === 0;
	} catch (error) {
		return false;
	}
}

/**
 * Build ripgrep command arguments for codebase-wide search
 *
 * @param rule - The grep rule to apply
 * @param baseDir - Base directory to search
 * @returns Array of command arguments
 */
function buildRipgrepArgs(
	rule: GrepRule,
	baseDir: string,
): string[] {
	return [
		"--line-number", // Show line numbers
		"--no-heading", // Don't show filename headers
		"--color=never", // No color output
		"--glob",
		rule.filePattern, // File pattern matching
		rule.forbiddenPattern, // Search pattern
		".", // Search current directory
	];
}

/**
 * Execute a grep rule using traditional grep with find for complex patterns
 *
 * @param rule - The grep rule to apply
 * @param baseDir - Base directory to search
 * @returns Array of violations found
 */
function executeTraditionalGrepRule(
	rule: GrepRule,
	baseDir: string,
): GrepRuleViolation[] {
	const violations: GrepRuleViolation[] = [];

	try {
		// First, find all files matching the pattern using glob matching
		const matchingFiles = findMatchingFiles(rule.filePattern, baseDir);

		// Then grep each matching file
		for (const file of matchingFiles) {
			const result = spawnSync("grep", [
				"-H", // Always show filename
				"-n", // Show line numbers
				rule.forbiddenPattern,
				file,
			], {
				encoding: "utf8",
				shell: false,
			});

			if (result.status === 0 && result.stdout) {
				const matches = parseGrepOutput(result.stdout, rule);
				violations.push(...matches);
			}
		}
	} catch (error) {
		console.error(`Failed to execute traditional grep rule: ${error}`);
	}

	return violations;
}

/**
 * Find files matching a glob pattern
 *
 * @param pattern - Glob pattern to match
 * @param baseDir - Base directory to search
 * @returns Array of matching file paths
 */
function findMatchingFiles(pattern: string, baseDir: string): string[] {
	const matchingFiles: string[] = [];

	function scanDirectory(dir: string): void {
		try {
			const items = fs.readdirSync(dir);
			
			for (const item of items) {
				const itemPath = path.join(dir, item);
				const relativePath = path.relative(baseDir, itemPath);
				
				try {
					const stat = fs.statSync(itemPath);
					
					if (stat.isDirectory()) {
						// Recursively scan subdirectories
						scanDirectory(itemPath);
					} else if (stat.isFile()) {
						// Check if file matches the pattern
						if (minimatch(relativePath, pattern) || minimatch(item, pattern)) {
							matchingFiles.push(itemPath);
						}
					}
				} catch {
					// Skip files/directories we can't read
				}
			}
		} catch {
			// Skip directories we can't read
		}
	}

	scanDirectory(baseDir);
	return matchingFiles;
}

/**
 * Build grep command arguments for single file search
 *
 * @param grepCommand - The grep command to use ("rg" or "grep")  
 * @param rule - The grep rule to apply
 * @param filePath - Path to the specific file to check
 * @returns Array of command arguments
 */
function buildGrepArgsForFile(
	grepCommand: string,
	rule: GrepRule,
	filePath: string,
): string[] {
	if (grepCommand === "rg") {
		return [
			"--line-number", // Show line numbers
			"--no-heading", // Don't show filename headers
			"--color=never", // No color output
			rule.forbiddenPattern, // Search pattern
			filePath, // Specific file
		];
	} else {
		return [
			"-H", // Always show filename
			"-n", // Show line numbers
			rule.forbiddenPattern, // Search pattern
			filePath, // Specific file
		];
	}
}

/**
 * Build ripgrep command arguments for single file search
 *
 * @param rule - The grep rule to apply
 * @param filePath - Path to the specific file to check
 * @returns Array of command arguments
 */
function buildRipgrepArgsForFile(
	rule: GrepRule,
	filePath: string,
): string[] {
	return [
		"--line-number", // Show line numbers
		"--no-heading", // Don't show filename headers
		"--color=never", // No color output
		rule.forbiddenPattern, // Search pattern
		filePath, // Specific file
	];
}

/**
 * Parse grep output and convert to GrepRuleViolation objects
 *
 * @param grepOutput - Raw output from grep/rg command
 * @param rule - The rule that was being checked
 * @param specificFile - If checking a specific file, use this path instead of parsing from output
 * @returns Array of parsed violations
 */
function parseGrepOutput(
	grepOutput: string,
	rule: GrepRule,
	specificFile?: string,
): GrepRuleViolation[] {
	const violations: GrepRuleViolation[] = [];
	const lines = grepOutput.trim().split("\n");

	for (const line of lines) {
		if (!line.trim()) continue;

		let file: string;
		let lineNumber: number;
		let lineContent: string;

		if (specificFile) {
			// For single file checks, parse line number and content only
			const match = line.match(/^(\d+):(.*)$/);
			if (match?.[1] && match[2] !== undefined) {
				file = specificFile;
				lineNumber = Number.parseInt(match[1]);
				lineContent = match[2];
			} else {
				continue; // Skip malformed lines
			}
		} else {
			// For recursive searches, parse file:line:content format
			const match = line.match(/^([^:]+):(\d+):(.*)$/);
			if (match?.[1] && match[2] && match[3] !== undefined) {
				file = match[1];
				lineNumber = Number.parseInt(match[2]);
				lineContent = match[3];
			} else {
				continue; // Skip malformed lines
			}
		}

		violations.push({
			rule,
			file,
			lineNumber,
			lineContent: lineContent.trim(),
		});
	}

	return violations;
}
