import type { ConclaudeConfig } from "./src/config.ts";

/**
 * Project-level conclaude configuration
 * This configuration can be extended or overridden by:
 * - ./.conclaude (local RC file)
 * - ~/.conclaude (global RC file)
 */
export default {
	/**
	 * Commands to run during Stop hook
	 * Each line is executed as a separate bash command
	 */
	stop: {
		run: `
nix develop -c lint
nix develop -c tests
`,
	},

	/**
	 * Validation rules for hook processing
	 */
	rules: {
		/**
		 * Prevent Claude from creating or modifying files at the repository root
		 * Helps maintain clean project structure
		 */
		preventRootAdditions: true,
	},
} as const satisfies ConclaudeConfig;
