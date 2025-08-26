import { cosmiconfig } from "cosmiconfig";
import { parse as parseYaml } from "yaml";
import { type SpawnSyncReturns, spawnSync } from "child_process";
import * as path from "path";
import * as fs from "fs";

/**
 * Configuration interface for stop hook commands
 */
export interface StopConfig {
	run: string;
	infinite?: boolean;
	infiniteMessage?: string;
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
	rules: RulesConfig;
}

/**
 * Find the git root directory starting from a given directory
 * @param startDir - Directory to start searching from
 * @returns Path to git root or null if not found
 */
function findGitRoot(startDir: string): string | null {
	let currentDir = path.resolve(startDir);
	
	while (true) {
		const gitDir = path.join(currentDir, ".git");
		if (fs.existsSync(gitDir)) {
			return currentDir;
		}
		
		const parentDir = path.dirname(currentDir);
		if (parentDir === currentDir) {
			// Reached filesystem root
			return null;
		}
		currentDir = parentDir;
	}
}

/**
 * Search for config file in directories, starting from startDir and going up
 * @param startDir - Directory to start searching from
 * @param maxLevels - Maximum number of parent directories to search
 * @returns Object with config path and content, or null if not found
 */
function searchConfigFile(startDir: string, maxLevels: number = 2): { filePath: string; content: string; searchedPaths: string[] } | { filePath: null; content: null; searchedPaths: string[] } {
	const configFilenames = [".conclaude.yaml", ".conclaude.yml"];
	const searchedPaths: string[] = [];
	let currentDir = path.resolve(startDir);
	
	// Find git root to limit search scope
	const gitRoot = findGitRoot(startDir);
	
	for (let level = 0; level <= maxLevels; level++) {
		for (const filename of configFilenames) {
			const configPath = path.join(currentDir, filename);
			searchedPaths.push(configPath);
			
			if (fs.existsSync(configPath)) {
				try {
					const content = fs.readFileSync(configPath, 'utf8');
					return { filePath: configPath, content, searchedPaths };
				} catch (error) {
					// Continue searching if file exists but can't be read
					console.warn(`Warning: Found config file at ${configPath} but couldn't read it: ${error}`);
				}
			}
		}
		
		// Stop if we've reached the git root
		if (gitRoot && currentDir === gitRoot) {
			break;
		}
		
		// Move to parent directory
		const parentDir = path.dirname(currentDir);
		if (parentDir === currentDir) {
			// Reached filesystem root
			break;
		}
		currentDir = parentDir;
	}
	
	return { filePath: null, content: null, searchedPaths };
}

/**
 * Load YAML configuration using enhanced search strategy
 * Searches for .conclaude.yaml or .conclaude.yml files starting from current directory
 * and going up to git root (maximum 2 parent directories)
 */
export async function loadConclaudeConfig(startDir: string = process.cwd()): Promise<ConclaudeConfig> {
	// Search for config file using our enhanced search
	const searchResult = searchConfigFile(startDir);
	
	if (!searchResult.filePath) {
		// Create helpful error message showing where we searched
		const searchedPathsFormatted = searchResult.searchedPaths
			.map(p => `  - ${p}`)
			.join('\n');
		
		const gitRoot = findGitRoot(startDir);
		const gitRootMessage = gitRoot 
			? `\n\nSearch was limited to git repository root: ${gitRoot}`
			: '\n\nNo git repository found - searched up to filesystem root or maximum 2 parent directories.';
		
		throw new Error(
			`No .conclaude.yaml or .conclaude.yml configuration file found.\n\n` +
			`Searched locations:\n${searchedPathsFormatted}${gitRootMessage}\n\n` +
			`Please create a configuration file with stop and rules sections in one of the searched locations.`
		);
	}

	// Parse the YAML content
	try {
		const config = parseYaml(searchResult.content);
		return config as ConclaudeConfig;
	} catch (error) {
		throw new Error(
			`Found configuration file at ${searchResult.filePath} but failed to parse YAML content: ${error}`
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
