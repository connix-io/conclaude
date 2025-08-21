import { loadConfig } from "c12";
import { type SpawnSyncReturns, spawnSync } from "child_process";

/**
 * Configuration interface for stop hook commands
 */
export interface StopConfig {
	run: string;
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
 * Load layered configuration using c12
 * Priority (high to low): runtime overrides → project config → local RC → global RC → package.json → defaults
 */
export async function loadConclaudeConfig(): Promise<ConclaudeConfig> {
	const { config } = await loadConfig<ConclaudeConfig>({
		name: "conclaude",
		configFile: "conclaude.config",
		rcFile: ".conclaude",
		globalRc: true,
		packageJson: "conclaude",
		defaults: {
			stop: {
				run: 'nix develop -c "lint"\nbun test',
			},
			rules: {
				preventRootAdditions: true,
				uneditableFiles: [],
			},
		},
	});

	return config as ConclaudeConfig;
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
