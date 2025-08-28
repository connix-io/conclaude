import { type SpawnSyncReturns, spawnSync } from "child_process";
import { cosmiconfig } from "cosmiconfig";
import { parse as parseYaml } from "yaml";

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
