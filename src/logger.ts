import { tmpdir } from "node:os";
import { basename, join } from "node:path";
import {
	createLogger as createWinstonLogger,
	format,
	type Logger,
	transports,
} from "winston";
import type { LoggingConfig } from "./types.ts";

const { combine, timestamp, label, printf } = format;

function sanitizeProjectName(name: string): string {
	return name
		.toLowerCase()
		.replace(/[^a-z0-9-]/g, "-")
		.replace(/-+/g, "-")
		.replace(/^-|-$/g, "");
}

function getProjectName(): string {
	try {
		const cwd = process.cwd();
		return sanitizeProjectName(basename(cwd));
	} catch {
		return "unknown";
	}
}

/**
 * Resolves logging configuration from environment variables and optional overrides.
 *
 * @param config - Optional logging configuration to override defaults
 * @returns Resolved logging configuration
 */
function resolveLoggingConfig(config?: Partial<LoggingConfig>): LoggingConfig {
	// Check environment variable CONCLAUDE_DISABLE_FILE_LOGGING
	// - If "true", disable file logging
	// - If "false", enable file logging
	// - If unset, default to disabled (breaking change)
	const envVar = process.env.CONCLAUDE_DISABLE_FILE_LOGGING;
	const defaultFileLogging = envVar === "false"; // Only enable if explicitly set to "false"

	return {
		fileLogging: config?.fileLogging ?? defaultFileLogging,
	};
}

export function createLogger(
	sessionId?: string,
	projectName?: string,
	config?: Partial<LoggingConfig>,
): Logger {
	const project = projectName || getProjectName();
	const session = sessionId || Date.now().toString();
	const filename = `conclaude-${project}-sess-${session}.jsonl`;
	const loggingConfig = resolveLoggingConfig(config);

	const loggerTransports: Array<
		| InstanceType<typeof transports.Console>
		| InstanceType<typeof transports.File>
	> = [
		new transports.Console({
			format: format.combine(format.colorize(), format.simple()),
		}),
	];

	// Conditionally add file transport based on configuration
	if (loggingConfig.fileLogging) {
		loggerTransports.push(
			new transports.File({
				filename: join(tmpdir(), filename),
				format: format.json(),
			}),
		);
	}

	return createWinstonLogger({
		level: process.env.CONCLAUDE_LOG_LEVEL || "info",
		format: combine(
			label({
				label: "conclaude",
			}),
			timestamp(),
			printf(({ level, message, label, timestamp }) => {
				return `${timestamp} [${label}] ${level}: ${message}`;
			}),
		),
		transports: loggerTransports,
	});
}

export const logger = createLogger();
