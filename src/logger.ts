import { tmpdir } from "node:os";
import { basename, join } from "node:path";
import {
	createLogger as createWinstonLogger,
	format,
	type Logger,
	transports,
} from "winston";

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

export function createLogger(
	sessionId?: string,
	projectName?: string,
): Logger {
	const project = projectName || getProjectName();
	const session = sessionId || Date.now().toString();
	const filename = `conclaude-${project}-sess-${session}.jsonl`;

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
		transports: [
			new transports.Console({
				format: format.combine(format.colorize(), format.simple()),
			}),
			new transports.File({
				filename: join(tmpdir(), filename),
				format: format.json(),
			}),
		],
	});
}

export const logger = createLogger();
