#!/usr/bin/env node
/**
 * Claude Code Hook Handler CLI
 *
 * This CLI tool processes hook events from Claude Code by reading JSON payloads from stdin
 * and executing appropriate handlers. Each hook type corresponds to a different event in
 * the Claude Code execution lifecycle (tool usage, notifications, session events, etc.).
 *
 * Usage: echo '{"hook_event_name":"PreToolUse",...}' | conclaude PreToolUse
 */

/** biome-ignore-all lint/suspicious/noConsole: CLI requires console output for user feedback */

import { minimatch } from "minimatch";
import pkg from "../package.json";
import type { Logger } from "winston";
import yargs, { type Arguments, type CommandModule } from "yargs";
import { hideBin } from "yargs/helpers";
import {
	type ConclaudeConfig,
	extractBashCommands,
	loadConclaudeConfig,
} from "./config.ts";
import { createLogger } from "./logger.ts";
import type {
	BasePayloadType,
	HookResult,
	NotificationPayload,
	PostToolUsePayload,
	PreCompactPayload,
	PreToolUsePayload,
	SessionStartPayload,
	StopPayload,
	SubagentStopPayload,
	UserPromptSubmitPayload,
} from "./types.ts";

/**
 * Cached configuration instance to avoid repeated loads
 */
let cachedConfig: ConclaudeConfig | null = null;

/**
 * Load configuration with caching to avoid repeated file system operations
 */
async function getConfig(): Promise<ConclaudeConfig> {
	if (!cachedConfig) {
		cachedConfig = await loadConclaudeConfig();
	}
	return cachedConfig;
}

/**
 * Wrapper function that standardizes hook result processing and process exit codes.
 *
 * This function ensures consistent behavior across all hook handlers by:
 * - Converting HookResult to appropriate exit codes (0=success, 1=error, 2=blocked)
 * - Formatting error messages with consistent emoji indicators
 * - Handling both thrown errors and returned error states
 *
 * @param handler - Function that processes hook payload and returns HookResult
 * @returns Function compatible with yargs CommandModule handler signature
 */
export function handleHookResult(
	handler: (argv: Arguments) => Promise<HookResult>,
): (argv: Arguments) => Promise<void> {
	return async (argv: Arguments): Promise<void> => {
		try {
			const result = await handler(argv);

			if (result.blocked && result.message) {
				process.stderr.write(`❌ Hook blocked: ${result.message}\n`);
				process.exit(2);
			}

			process.exit(0);
		} catch (error) {
			process.stderr.write(
				`❌ Hook failed: ${error instanceof Error ? error.message : String(error)}\n`,
			);
			process.exit(1);
		}
	};
}

/**
 * Reads and validates hook payload from stdin, creating a session-specific logger.
 *
 * This function:
 * - Reads all data from stdin as JSON
 * - Validates required base fields (session_id, transcript_path, hook_event_name)
 * - Creates a Winston logger tagged with the session ID
 * - Returns both the typed payload and logger for use in handlers
 *
 * @template T - The specific hook payload type expected
 * @returns Promise containing the validated payload and session logger
 * @throws {Error} When required fields are missing or JSON is invalid
 */
async function readPayload<T extends BasePayloadType>(): Promise<{
	payload: T;
	logger: Logger;
}> {
	const chunks: Buffer[] = [];
	for await (const chunk of process.stdin) {
		chunks.push(chunk);
	}
	const rawData = Buffer.concat(chunks).toString();
	const payload = JSON.parse(rawData) as T;

	// Validate base payload fields
	if (!payload.session_id)
		throw new Error("Missing required field: session_id");
	if (!payload.transcript_path)
		throw new Error("Missing required field: transcript_path");
	if (!payload.hook_event_name)
		throw new Error("Missing required field: hook_event_name");

	// Create logger with session ID
	const logger = createLogger(payload.session_id);

	return {
		payload,
		logger,
	};
}

/**
 * Handles PreToolUse hook events fired before Claude executes any tool.
 *
 * This hook allows intercepting and potentially blocking tool execution before it occurs.
 * Useful for:
 * - Logging tool usage for audit purposes
 * - Implementing security policies (e.g., blocking dangerous commands)
 * - Modifying tool inputs or validating parameters
 * - Rate limiting or resource management
 *
 * @param _argv - CLI arguments (unused, hook data comes from stdin)
 * @returns HookResult indicating whether to allow tool execution
 */
async function handlePreToolUse(_argv: Arguments): Promise<HookResult> {
	const { payload, logger } = await readPayload<PreToolUsePayload>();

	if (!payload.tool_name) throw new Error("Missing required field: tool_name");
	if (!payload.tool_input)
		throw new Error("Missing required field: tool_input");

	logger.info("Processing PreToolUse hook", {
		session_id: payload.session_id,
		hook_event_name: payload.hook_event_name,
		transcript_path: payload.transcript_path,
		tool_name: payload.tool_name,
	});

	const fileModifyingTools = [
		"Write",
		"Edit",
		"MultiEdit",
		"NotebookEdit",
	];

	if (fileModifyingTools.includes(payload.tool_name)) {
		try {
			const toolInput = payload.tool_input;
			const filePath = (toolInput.file_path ||
				toolInput.notebook_path) as string;

			if (filePath && typeof filePath === "string") {
				const path = await import("path");
				const cwd = process.cwd();
				const resolvedPath = path.resolve(cwd, filePath);
				const relativePath = path.relative(cwd, resolvedPath);

				// Check preventRootAdditions rule - only applies to Write tool
				const config = await getConfig();
				if (
					config.rules.preventRootAdditions &&
					payload.tool_name === "Write"
				) {
					// Check if the file is directly in the root directory (no subdirectories)
					// Allow dotfiles and configuration files (like .conclaude.yaml, .gitignore, package.json)
					const fileName = path.basename(relativePath);
					const isConfigFile =
						fileName.includes("config") ||
						fileName.includes("settings") ||
						fileName === "package.json" ||
						fileName === "tsconfig.json" ||
						fileName === "bun.lockb" ||
						fileName === "bun.lock";
					const isInRoot =
						!relativePath.includes(path.sep) &&
						relativePath !== "" &&
						relativePath !== ".." &&
						!fileName.startsWith(".") &&
						!isConfigFile;

					if (isInRoot) {
						const errorMessage = `Blocked ${payload.tool_name} operation: preventRootAdditions rule prevents creating files at repository root. File: ${filePath}`;

						logger.warn("PreToolUse blocked by preventRootAdditions rule", {
							tool_name: payload.tool_name,
							file_path: filePath,
							resolved_path: resolvedPath,
							relative_path: relativePath,
						});

						return {
							message: errorMessage,
							blocked: true,
						};
					}
				}

				// Check uneditableFiles rule
				if (config.rules.uneditableFiles.length > 0) {
					for (const pattern of config.rules.uneditableFiles) {
						try {
							// Test both the original file path and the relative path against the pattern
							const matchesOriginal = minimatch(filePath, pattern);
							const matchesRelative = minimatch(relativePath, pattern);
							const matchesResolved = minimatch(resolvedPath, pattern);

							if (matchesOriginal || matchesRelative || matchesResolved) {
								const errorMessage = `Blocked ${payload.tool_name} operation: file matches uneditable pattern '${pattern}'. File: ${filePath}`;

								logger.warn("PreToolUse blocked by uneditableFiles rule", {
									tool_name: payload.tool_name,
									file_path: filePath,
									resolved_path: resolvedPath,
									relative_path: relativePath,
									matched_pattern: pattern,
								});

								return {
									message: errorMessage,
									blocked: true,
								};
							}
						} catch (patternError) {
							// If pattern matching fails, log warning but don't block (defensive approach)
							logger.warn("Error during pattern matching", {
								tool_name: payload.tool_name,
								file_path: filePath,
								pattern: pattern,
								error:
									patternError instanceof Error
										? patternError.message
										: String(patternError),
							});
						}
					}
				}
			}
		} catch (error) {
			// If tool_input doesn't contain expected fields or path resolution fails,
			// log the error but don't block (defensive approach)
			logger.warn("Error during file path validation", {
				tool_name: payload.tool_name,
				tool_input: payload.tool_input,
				error: error instanceof Error ? error.message : String(error),
			});
		}
	}

	return {
		message: undefined,
		blocked: false,
	};
}

/**
 * Handles PostToolUse hook events fired after Claude executes a tool.
 *
 * This hook provides access to both tool inputs and outputs, enabling:
 * - Logging tool results and performance metrics
 * - Post-processing tool outputs or error handling
 * - Updating external systems based on tool execution
 * - Collecting usage statistics and success rates
 *
 * @param _argv - CLI arguments (unused, hook data comes from stdin)
 * @returns HookResult for any post-execution feedback
 */
async function handlePostToolUse(_argv: Arguments): Promise<HookResult> {
	const { payload, logger } = await readPayload<PostToolUsePayload>();

	if (!payload.tool_name) throw new Error("Missing required field: tool_name");
	if (!payload.tool_input)
		throw new Error("Missing required field: tool_input");
	if (!payload.tool_response)
		throw new Error("Missing required field: tool_response");

	logger.info("Processing PostToolUse hook", {
		session_id: payload.session_id,
		hook_event_name: payload.hook_event_name,
		transcript_path: payload.transcript_path,
		tool_name: payload.tool_name,
	});

	return {
		message: undefined,
		blocked: false,
	};
}

/**
 * Handles Notification hook events when Claude sends system notifications.
 *
 * This hook intercepts Claude's notification system, allowing:
 * - Custom notification routing (email, Slack, etc.)
 * - Notification filtering or formatting
 * - Integration with external monitoring systems
 * - User preference-based notification handling
 *
 * @param _argv - CLI arguments (unused, hook data comes from stdin)
 * @returns HookResult for notification processing
 */
async function handleNotification(_argv: Arguments): Promise<HookResult> {
	const { payload, logger } = await readPayload<NotificationPayload>();

	if (!payload.message) throw new Error("Missing required field: message");

	logger.info("Processing Notification hook", {
		session_id: payload.session_id,
		hook_event_name: payload.hook_event_name,
		transcript_path: payload.transcript_path,
	});

	return {
		message: undefined,
		blocked: false,
	};
}

/**
 * Handles UserPromptSubmit hook events when users submit input to Claude.
 *
 * This hook processes user input before Claude sees it, enabling:
 * - Input validation and sanitization
 * - Command preprocessing or macro expansion
 * - Usage tracking and analytics
 * - Custom authentication or authorization checks
 *
 * @param _argv - CLI arguments (unused, hook data comes from stdin)
 * @returns HookResult indicating whether to allow the prompt
 */
async function handleUserPromptSubmit(_argv: Arguments): Promise<HookResult> {
	const { payload, logger } = await readPayload<UserPromptSubmitPayload>();

	if (!payload.prompt) throw new Error("Missing required field: prompt");

	logger.info("Processing UserPromptSubmit hook", {
		session_id: payload.session_id,
		hook_event_name: payload.hook_event_name,
		transcript_path: payload.transcript_path,
	});

	return {
		message: undefined,
		blocked: false,
	};
}

/**
 * Handles SessionStart hook events when a new Claude session begins.
 *
 * This hook fires at session initialization, allowing:
 * - Session-specific setup and configuration
 * - User authentication and permission verification
 * - Resource allocation and environment preparation
 * - Logging session metadata and context
 *
 * @param _argv - CLI arguments (unused, hook data comes from stdin)
 * @returns HookResult for session initialization
 */
async function handleSessionStart(_argv: Arguments): Promise<HookResult> {
	const { payload, logger } = await readPayload<SessionStartPayload>();

	if (!payload.source) throw new Error("Missing required field: source");

	logger.info("Processing SessionStart hook", {
		session_id: payload.session_id,
		hook_event_name: payload.hook_event_name,
		transcript_path: payload.transcript_path,
	});

	return {
		message: undefined,
		blocked: false,
	};
}

/**
 * Handles Stop hook events when a Claude session is terminating.
 *
 * This hook provides cleanup opportunities when sessions end, enabling:
 * - Resource cleanup and deallocation
 * - Final logging and metric collection
 * - Data persistence and state saving
 * - Notification of session completion
 *
 * @param _argv - CLI arguments (unused, hook data comes from stdin)
 * @returns HookResult for session cleanup
 */
async function handleStop(_argv: Arguments): Promise<HookResult> {
	const { payload, logger } = await readPayload<StopPayload>();

	if (payload.stop_hook_active === undefined)
		throw new Error("Missing required field: stop_hook_active");

	logger.info("Processing Stop hook", {
		session_id: payload.session_id,
		hook_event_name: payload.hook_event_name,
		transcript_path: payload.transcript_path,
	});

	// Extract and execute commands from config.stop.run
	const config = await getConfig();
	const commands = extractBashCommands(config.stop.run);

	logger.info(`Executing ${commands.length} stop hook commands`, {
		commands: commands,
	});

	for (const [index, command] of commands.entries()) {
		logger.info(
			`Executing command ${index + 1}/${commands.length}: ${command}`,
		);

		try {
			const result = Bun.spawn({
				cmd: [
					"bash",
					"-c",
					command,
				],
				stdout: "pipe",
				stderr: "pipe",
			});

			const output = await result.exited;
			const stdout = await new Response(result.stdout).text();
			const stderr = await new Response(result.stderr).text();

			if (output !== 0) {
				const stdoutSection = stdout ? `\nStdout: ${stdout}` : "";
				const stderrSection = stderr ? `\nStderr: ${stderr}` : "";
				const errorMessage = `Command failed with exit code ${output}: ${command}${stdoutSection}${stderrSection}`;
				logger.error("Stop hook command failed", {
					command,
					exitCode: output,
					stdout,
					stderr,
				});

				return {
					message: errorMessage,
					blocked: true,
				};
			}

			logger.info(`Command completed successfully: ${command}`, {
				stdout: stdout.trim(),
			});
		} catch (error) {
			const errorMessage = `Command execution failed: ${command}\nError: ${error instanceof Error ? error.message : String(error)}`;
			logger.error("Stop hook command execution error", {
				command,
				error: error instanceof Error ? error.message : String(error),
			});

			return {
				message: errorMessage,
				blocked: true,
			};
		}
	}

	logger.info("All stop hook commands completed successfully");

	return {
		message: undefined,
		blocked: false,
	};
}

/**
 * Handles SubagentStop hook events when Claude subagents complete their tasks.
 *
 * Subagents are spawned for complex multi-step operations. This hook enables:
 * - Subagent result processing and aggregation
 * - Performance monitoring of complex tasks
 * - Error handling for failed subagent operations
 * - Resource cleanup for subagent-specific allocations
 *
 * @param _argv - CLI arguments (unused, hook data comes from stdin)
 * @returns HookResult for subagent completion handling
 */
async function handleSubagentStop(_argv: Arguments): Promise<HookResult> {
	const { payload, logger } = await readPayload<SubagentStopPayload>();

	if (payload.stop_hook_active === undefined)
		throw new Error("Missing required field: stop_hook_active");

	logger.info("Processing SubagentStop hook", {
		session_id: payload.session_id,
		hook_event_name: payload.hook_event_name,
		transcript_path: payload.transcript_path,
	});

	return {
		message: undefined,
		blocked: false,
	};
}

/**
 * Handles PreCompact hook events before transcript compaction occurs.
 *
 * Transcript compaction reduces conversation history to manage context limits.
 * This hook enables:
 * - Backing up full conversation history before compaction
 * - Custom compaction strategies or preservation rules
 * - Notification of data loss due to compaction
 * - Analytics on conversation length and patterns
 *
 * @param _argv - CLI arguments (unused, hook data comes from stdin)
 * @returns HookResult for pre-compaction processing
 */
async function handlePreCompact(_argv: Arguments): Promise<HookResult> {
	const { payload, logger } = await readPayload<PreCompactPayload>();

	if (!payload.trigger) throw new Error("Missing required field: trigger");
	if (payload.trigger !== "manual" && payload.trigger !== "auto") {
		throw new Error('Invalid trigger value, must be "manual" or "auto"');
	}

	logger.info("Processing PreCompact hook", {
		session_id: payload.session_id,
		hook_event_name: payload.hook_event_name,
		transcript_path: payload.transcript_path,
	});

	logger.info(`Compact trigger: ${payload.trigger}`, {
		trigger: payload.trigger,
	});

	return {
		message: undefined,
		blocked: false,
	};
}

/**
 * TypeScript interfaces for Claude Code settings structure
 */
interface ClaudeHookConfig {
	type: "command";
	command: string;
}

interface ClaudeHookMatcher {
	matcher: string;
	hooks: ClaudeHookConfig[];
}

interface ClaudePermissions {
	allow: string[];
	deny: string[];
}

interface ClaudeSettings {
	permissions: ClaudePermissions;
	hooks: Record<string, ClaudeHookMatcher[]>;
}

/**
 * Handles Init command to set up conclaude configuration and Claude Code hooks.
 *
 * This function doesn't use stdin JSON payloads like other handlers.
 * Instead, it creates configuration files and integrates with Claude Code settings.
 *
 * @param argv - CLI arguments containing options like --config-path, --claude-path, --force
 * @returns Promise<void> (exits process directly rather than returning HookResult)
 */
async function handleInit(argv: Arguments): Promise<void> {
	const path = await import("path");
	const fs = await import("fs/promises");

	const cwd = process.cwd();
	const configPath =
		(argv.configPath as string) || path.join(cwd, ".conclaude.yaml");
	const claudePath = (argv.claudePath as string) || path.join(cwd, ".claude");
	const settingsPath = path.join(claudePath, "settings.json");
	const force = argv.force as boolean;

	console.log("🚀 Initializing conclaude configuration...\n");

	try {
		// Check if config file exists
		const configExists = await fs
			.access(configPath)
			.then(() => true)
			.catch(() => false);

		if (configExists && !force) {
			console.log("⚠️  Configuration file already exists:");
			console.log(`   ${configPath}`);
			console.log("\nUse --force to overwrite existing configuration.");
			process.exit(1);
		}

		// Create .conclaude.yaml
		const configContent = `# Conclaude YAML Configuration
# This configuration defines how conclaude handles Claude Code hook events

# Commands to run during Stop hook
# Each line is executed as a separate bash command
stop:
  run: |
    nix develop -c "lint"
    bun test

# Validation rules for hook processing
rules:
  # Prevent Claude from creating or modifying files at the repository root
  # Helps maintain clean project structure
  preventRootAdditions: true
  
  # Files that Claude cannot edit, using glob patterns
  # Examples:
  # - "./package.json" - specific files
  # - "*.md" - file extensions  
  # - "src/**/*.ts" - nested patterns
  # - ".env*" - environment files
  # - "docs/**" - entire directories
  # - "{package,tsconfig}.json" - multiple specific files
  uneditableFiles: []
`;

		await fs.writeFile(configPath, configContent);
		console.log("✅ Created configuration file:");
		console.log(`   ${configPath}`);

		// Create .claude directory if it doesn't exist
		await fs.mkdir(claudePath, {
			recursive: true,
		});

		// Check if settings.json exists
		const settingsExists = await fs
			.access(settingsPath)
			.then(() => true)
			.catch(() => false);

		let settings: ClaudeSettings = {
			permissions: {
				allow: [],
				deny: [],
			},
			hooks: {},
		};

		if (settingsExists) {
			const settingsContent = await fs.readFile(settingsPath, "utf-8");
			settings = JSON.parse(settingsContent) as ClaudeSettings;
			console.log("\n📝 Found existing Claude settings, updating hooks...");
		} else {
			console.log("\n📝 Creating Claude Code settings...");
		}

		// Ensure hooks object exists
		if (!settings.hooks) {
			settings.hooks = {};
		}

		// Define all hook types and their commands
		const hookTypes: readonly string[] = [
			"UserPromptSubmit",
			"PreToolUse",
			"PostToolUse",
			"Notification",
			"Stop",
			"SubagentStop",
			"PreCompact",
		] as const;

		// Add hook configurations
		for (const hookType of hookTypes) {
			settings.hooks[hookType] = [
				{
					matcher: "",
					hooks: [
						{
							type: "command",
							command: `npx conclaude@latest ${hookType}`,
						},
					],
				},
			];
		}

		// Write updated settings
		await fs.writeFile(settingsPath, JSON.stringify(settings, null, "\t"));
		console.log("✅ Updated Claude Code settings:");
		console.log(`   ${settingsPath}`);

		console.log("\n🎉 Conclaude initialization complete!");
		console.log("\nConfigured hooks:");
		for (const hookType of hookTypes) {
			console.log(`   • ${hookType}`);
		}
		console.log("\nYou can now use Claude Code with conclaude hook handling.");
	} catch (error) {
		console.error(
			"❌ Failed to initialize conclaude:",
			error instanceof Error ? error.message : String(error),
		);
		process.exit(1);
	}
}

/** CLI command definition for PreToolUse hook processing */
const preToolUseCommand: CommandModule = {
	command: "PreToolUse",
	describe: "Process PreToolUse hook - fired before tool execution",
	handler: handleHookResult(handlePreToolUse),
};

/** CLI command definition for PostToolUse hook processing */
const postToolUseCommand: CommandModule = {
	command: "PostToolUse",
	describe: "Process PostToolUse hook - fired after tool execution",
	handler: handleHookResult(handlePostToolUse),
};

/** CLI command definition for Notification hook processing */
const notificationCommand: CommandModule = {
	command: "Notification",
	describe: "Process Notification hook - fired for system notifications",
	handler: handleHookResult(handleNotification),
};

/** CLI command definition for UserPromptSubmit hook processing */
const userPromptSubmitCommand: CommandModule = {
	command: "UserPromptSubmit",
	describe: "Process UserPromptSubmit hook - fired when user submits input",
	handler: handleHookResult(handleUserPromptSubmit),
};

/** CLI command definition for SessionStart hook processing */
const sessionStartCommand: CommandModule = {
	command: "SessionStart",
	describe: "Process SessionStart hook - fired when session begins",
	handler: handleHookResult(handleSessionStart),
};

/** CLI command definition for Stop hook processing */
const stopCommand: CommandModule = {
	command: "Stop",
	describe: "Process Stop hook - fired when session terminates",
	handler: handleHookResult(handleStop),
};

/** CLI command definition for SubagentStop hook processing */
const subagentStopCommand: CommandModule = {
	command: "SubagentStop",
	describe: "Process SubagentStop hook - fired when subagent completes",
	handler: handleHookResult(handleSubagentStop),
};

/** CLI command definition for PreCompact hook processing */
const preCompactCommand: CommandModule = {
	command: "PreCompact",
	describe: "Process PreCompact hook - fired before transcript compaction",
	handler: handleHookResult(handlePreCompact),
};

/** CLI command definition for Init command */
const initCommand: CommandModule = {
	command: "init",
	describe: "Initialize conclaude configuration and Claude Code hooks",
	builder: {
		"config-path": {
			type: "string",
			describe: "Path for .conclaude.yaml file",
			default: undefined,
		},
		"claude-path": {
			type: "string",
			describe: "Path for .claude directory",
			default: undefined,
		},
		force: {
			type: "boolean",
			describe: "Overwrite existing configuration files",
			default: false,
		},
	},
	handler: handleInit,
};

/**
 * Main CLI configuration using yargs for command parsing and handling.
 *
 * The CLI supports all Claude Code hook types as subcommands, each expecting
 * a JSON payload from stdin. Exit codes follow standard conventions:
 * - 0: Success
 * - 1: Error (validation, parsing, or handler failure) [Shown to user, but not agent]
 * - 2: Blocked (hook explicitly blocked the operation)
 */
const cli = yargs(hideBin(process.argv))
	.scriptName("conclaude")
	.usage("Usage: $0 <command>")
	.command(initCommand)
	.command(preToolUseCommand)
	.command(postToolUseCommand)
	.command(notificationCommand)
	.command(userPromptSubmitCommand)
	.command(sessionStartCommand)
	.command(stopCommand)
	.command(subagentStopCommand)
	.command(preCompactCommand)
	.option("verbose", {
		alias: "v",
		type: "boolean",
		description: "Enable verbose logging output",
		global: true,
	})
	.demandCommand(1, "You need to specify a command")
	.help("h")
	.alias("h", "help")
	.version(pkg.version)
	.alias("V", "version")
	.wrap(120)
	.epilog(
		"Claude Code Hook Handler - Processes hook events via JSON payloads from stdin",
	)
	.fail((msg, err, yargs) => {
		// Handle CLI parsing errors with helpful output
		if (err) throw err;
		console.error("CLI Error:", msg);
		console.error("\n" + yargs.help());
		process.exit(1);
	})
	.middleware((argv) => {
		// Enable verbose logging when requested
		if (argv.verbose) {
			console.log("Verbose mode enabled");
		}
	});

// Parse command line arguments and execute the appropriate hook handler
// Only run CLI when this module is executed directly (not when imported for testing)
if (import.meta.main) {
	cli.parse();
}
