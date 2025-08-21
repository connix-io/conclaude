/**
 * TypeScript definitions for the Claude Code hook system.
 *
 * This module defines all payload types for different hook events that occur
 * during Claude Code execution, along with utilities for parsing transcript files
 * and extracting conversation data.
 */

import * as fs from "fs";
import * as readline from "readline";

/**
 * Response structure returned by hook handlers to control execution flow.
 */
export interface HookResult {
	/** Custom message to display to the user */
	message: string | undefined;
	/** Whether to block the current operation from proceeding */
	blocked: boolean | undefined;
}

/**
 * Union type of all possible hook event payloads.
 */
export type BasePayloadType =
	| PreToolUsePayload
	| PostToolUsePayload
	| NotificationPayload
	| StopPayload
	| SubagentStopPayload
	| UserPromptSubmitPayload
	| PreCompactPayload
	| SessionStartPayload;

/**
 * Payload for PreToolUse hook - fired before Claude executes a tool.
 * Allows blocking or modifying tool execution before it occurs.
 */
export interface PreToolUsePayload {
	/** Unique identifier for the current Claude session */
	session_id: string;
	/** Path to the JSONL transcript file containing conversation history */
	transcript_path: string;
	/** Hook event type identifier */
	hook_event_name: "PreToolUse";
	/** Name of the tool about to be executed (e.g., "Edit", "Bash", "Read") */
	tool_name: string;
	/** Input parameters that will be passed to the tool */
	tool_input: Record<string, unknown>;
}

/**
 * Payload for PostToolUse hook - fired after Claude executes a tool.
 * Contains both the input and response data for analysis or logging.
 */
export interface PostToolUsePayload {
	/** Unique identifier for the current Claude session */
	session_id: string;
	/** Path to the JSONL transcript file containing conversation history */
	transcript_path: string;
	/** Hook event type identifier */
	hook_event_name: "PostToolUse";
	/** Name of the tool that was executed */
	tool_name: string;
	/** Input parameters that were passed to the tool */
	tool_input: Record<string, unknown>;
	/** Response data returned by the tool execution */
	tool_response: Record<string, unknown> & {
		/** Whether the tool execution completed successfully */
		success?: boolean;
	};
}

/**
 * Payload for Notification hook - fired when Claude sends system notifications.
 * Used for displaying messages or alerts to the user.
 */
export interface NotificationPayload {
	/** Unique identifier for the current Claude session */
	session_id: string;
	/** Path to the JSONL transcript file containing conversation history */
	transcript_path: string;
	/** Hook event type identifier */
	hook_event_name: "Notification";
	/** The notification message content */
	message: string;
	/** Optional title for the notification */
	title?: string;
}

/**
 * Payload for Stop hook - fired when a Claude session is terminating.
 * Allows for cleanup operations or final processing before session ends.
 */
export interface StopPayload {
	/** Unique identifier for the current Claude session */
	session_id: string;
	/** Path to the JSONL transcript file containing conversation history */
	transcript_path: string;
	/** Hook event type identifier */
	hook_event_name: "Stop";
	/** Whether stop hooks are currently active for this session */
	stop_hook_active: boolean;
}

/**
 * Payload for SubagentStop hook - fired when a Claude subagent terminates.
 * Subagents are spawned for complex tasks and this fires when they complete.
 */
export interface SubagentStopPayload {
	/** Unique identifier for the current Claude session */
	session_id: string;
	/** Path to the JSONL transcript file containing conversation history */
	transcript_path: string;
	/** Hook event type identifier */
	hook_event_name: "SubagentStop";
	/** Whether stop hooks are currently active for this session */
	stop_hook_active: boolean;
}

/**
 * Payload for UserPromptSubmit hook - fired when user submits input to Claude.
 * Allows processing or validation of user input before Claude processes it.
 */
export interface UserPromptSubmitPayload {
	/** Unique identifier for the current Claude session */
	session_id: string;
	/** Path to the JSONL transcript file containing conversation history */
	transcript_path: string;
	/** Hook event type identifier */
	hook_event_name: "UserPromptSubmit";
	/** The user's input prompt text */
	prompt: string;
}

/**
 * Payload for PreCompact hook - fired before transcript compaction occurs.
 * Transcript compaction reduces conversation history size to manage context limits.
 */
export interface PreCompactPayload {
	/** Unique identifier for the current Claude session */
	session_id: string;
	/** Path to the JSONL transcript file containing conversation history */
	transcript_path: string;
	/** Hook event type identifier */
	hook_event_name: "PreCompact";
	/** Whether compaction was triggered manually by user or automatically by system */
	trigger: "manual" | "auto";
}

/**
 * Payload for SessionStart hook - fired when a new Claude session begins.
 * Allows initialization or setup operations at the start of a conversation.
 */
export interface SessionStartPayload {
	/** Unique identifier for the current Claude session */
	session_id: string;
	/** Path to the JSONL transcript file containing conversation history */
	transcript_path: string;
	/** Hook event type identifier */
	hook_event_name: "SessionStart";
	/** Source that initiated the session (e.g., CLI, IDE integration) */
	source: string;
}

/**
 * Discriminated union of all hook payloads with additional hook_type field.
 * This allows type-safe handling of different hook events in a single function.
 * The hook_type field enables TypeScript to narrow the payload type automatically.
 */
export type HookPayload =
	| (PreToolUsePayload & {
			hook_type: "PreToolUse";
	  })
	| (PostToolUsePayload & {
			hook_type: "PostToolUse";
	  })
	| (NotificationPayload & {
			hook_type: "Notification";
	  })
	| (StopPayload & {
			hook_type: "Stop";
	  })
	| (SubagentStopPayload & {
			hook_type: "SubagentStop";
	  })
	| (UserPromptSubmitPayload & {
			hook_type: "UserPromptSubmit";
	  })
	| (PreCompactPayload & {
			hook_type: "PreCompact";
	  })
	| (SessionStartPayload & {
			hook_type: "SessionStart";
	  });

/**
 * Validates that a payload contains all required base fields and matches expected hook type.
 * Throws descriptive errors for missing or invalid fields.
 *
 * @param payload - The hook payload to validate
 * @param expectedHookEvent - The expected hook_event_name value
 * @throws {Error} When required fields are missing or hook type doesn't match
 */
export function validateBasePayload(
	payload: BasePayloadType,
	expectedHookEvent: string,
): void {
	const p = payload;
	if (!p.session_id) throw new Error("Missing required field: session_id");
	if (!p.transcript_path)
		throw new Error("Missing required field: transcript_path");
	if (!p.hook_event_name)
		throw new Error("Missing required field: hook_event_name");
	if (p.hook_event_name !== expectedHookEvent) {
		throw new Error(
			`Expected hook_event_name to be ${expectedHookEvent}, got ${p.hook_event_name}`,
		);
	}
}

/**
 * Transcript message types represent the structure of messages stored in JSONL transcript files.
 * Each line in the transcript file contains one of these message types as JSON.
 */
/**
 * Summary message generated during transcript compaction.
 * Contains condensed information about previous conversation segments.
 */
export interface TranscriptSummary {
	/** Message type discriminator */
	type: "summary";
	/** Condensed summary of the conversation segment */
	summary: string;
	/** UUID of the last message included in this summary */
	leafUuid: string;
}

/**
 * User message in the transcript containing user input and context information.
 * Can contain either plain text or structured tool result content.
 */
export interface TranscriptUserMessage {
	/** UUID of the parent message, null for root messages */
	parentUuid: string | null;
	/** Whether this message is part of a sidechain conversation */
	isSidechain: boolean;
	/** Type of user (always "external" for CLI usage) */
	userType: "external";
	/** Current working directory when message was created */
	cwd: string;
	/** Claude session identifier */
	sessionId: string;
	/** Claude Code version */
	version: string;
	/** Git branch name if in a git repository */
	gitBranch?: string;
	/** Message type discriminator */
	type: "user";
	/** The actual message content */
	message: {
		/** Message role (always "user") */
		role: "user";
		/** Message content - either plain text or structured tool results */
		content:
			| string
			| Array<{
					/** ID of the tool use this result corresponds to */
					tool_use_id?: string;
					/** Type of content block */
					type: "tool_result" | "text";
					/** Text content of the block */
					content?: string;
					/** Whether this represents an error result */
					is_error?: boolean;
			  }>;
	};
	/** Unique identifier for this message */
	uuid: string;
	/** ISO timestamp when message was created */
	timestamp: string;
	/** Tool execution results if this message contains tool output */
	toolUseResult?: {
		/** Standard output from tool execution */
		stdout: string;
		/** Standard error from tool execution */
		stderr: string;
		/** Whether tool execution was interrupted */
		interrupted: boolean;
		/** Whether the result contains image data */
		isImage: boolean;
	};
}

/**
 * Assistant message in the transcript containing Claude's response and usage metrics.
 * Contains both text responses and tool use commands with detailed token usage information.
 */
export interface TranscriptAssistantMessage {
	/** UUID of the parent message */
	parentUuid: string;
	/** Whether this message is part of a sidechain conversation */
	isSidechain: boolean;
	/** Type of user (always "external" for CLI usage) */
	userType: "external";
	/** Current working directory when message was created */
	cwd: string;
	/** Claude session identifier */
	sessionId: string;
	/** Claude Code version */
	version: string;
	/** Git branch name if in a git repository */
	gitBranch?: string;
	/** The actual message content and metadata */
	message: {
		/** Unique message identifier */
		id: string;
		/** Message type (always "message") */
		type: "message";
		/** Message role (always "assistant") */
		role: "assistant";
		/** Claude model used for this response */
		model: string;
		/** Array of content blocks (text and tool uses) */
		content: Array<{
			/** Type of content block */
			type: "text" | "tool_use";
			/** Text content for text blocks */
			text?: string;
			/** Unique ID for tool use blocks */
			id?: string;
			/** Tool name for tool use blocks */
			name?: string;
			/** Tool input parameters for tool use blocks */
			input?: Record<string, unknown>;
		}>;
		/** Reason the response stopped (e.g., "end_turn", "tool_use") */
		stop_reason: string | null;
		/** Stop sequence that ended the response, if any */
		stop_sequence: string | null;
		/** Token usage statistics for this response */
		usage: {
			/** Input tokens consumed */
			input_tokens: number;
			/** Tokens used for cache creation */
			cache_creation_input_tokens: number;
			/** Tokens read from cache */
			cache_read_input_tokens: number;
			/** Output tokens generated */
			output_tokens: number;
			/** Service tier used for this request */
			service_tier: string;
		};
	};
	/** API request identifier */
	requestId: string;
	/** Message type discriminator */
	type: "assistant";
	/** Unique identifier for this message */
	uuid: string;
	/** ISO timestamp when message was created */
	timestamp: string;
}

/**
 * Union type representing all possible message types found in transcript files.
 * Each line in a JSONL transcript file contains one of these message types.
 */
export type TranscriptMessage =
	| TranscriptSummary
	| TranscriptUserMessage
	| TranscriptAssistantMessage;

/**
 * Extracts the first user message from a transcript file.
 * Useful for understanding what started the conversation.
 *
 * @param transcriptPath - Path to the JSONL transcript file
 * @returns The first user message content, or null if not found
 */
export async function getInitialMessage(
	transcriptPath: string,
): Promise<string | null> {
	try {
		const fileStream = fs.createReadStream(transcriptPath);
		const rl = readline.createInterface({
			input: fileStream,
			crlfDelay: Infinity,
		});

		for await (const line of rl) {
			if (!line.trim()) continue;

			try {
				const message = JSON.parse(line) as TranscriptMessage;

				// Skip summary messages
				if (message.type === "summary") continue;

				// Find the first user message
				if (message.type === "user" && message.message.role === "user") {
					// Handle string content
					if (typeof message.message.content === "string") {
						return message.message.content;
					}

					// Handle array content (tool results)
					if (Array.isArray(message.message.content)) {
						const textContent = message.message.content
							.filter((item) => item.type === "text" && item.content)
							.map((item) => item.content)
							.join("\n");

						if (textContent) return textContent;
					}
				}
			} catch {
				// Ignore parsing errors for individual lines
			}
		}

		return null;
	} catch (error) {
		console.error("Error reading transcript:", error);
		return null;
	}
}

/**
 * Loads all messages from a transcript file into memory.
 * Parses the JSONL format and returns an array of typed messages.
 *
 * @param transcriptPath - Path to the JSONL transcript file
 * @returns Array of all transcript messages
 */
export async function getAllMessages(
	transcriptPath: string,
): Promise<TranscriptMessage[]> {
	const messages: TranscriptMessage[] = [];

	try {
		const fileStream = fs.createReadStream(transcriptPath);
		const rl = readline.createInterface({
			input: fileStream,
			crlfDelay: Infinity,
		});

		for await (const line of rl) {
			if (!line.trim()) continue;

			try {
				const message = JSON.parse(line) as TranscriptMessage;
				messages.push(message);
			} catch {
				// Ignore parsing errors for individual lines
			}
		}
	} catch (error) {
		console.error("Error reading transcript:", error);
	}

	return messages;
}

/**
 * Extracts a simplified conversation history from a transcript.
 * Returns only the text content of user and assistant messages, filtered from tool results.
 *
 * @param transcriptPath - Path to the JSONL transcript file
 * @returns Array of conversation messages with role and content
 */
export async function getConversationHistory(transcriptPath: string): Promise<
	Array<{
		role: "user" | "assistant";
		content: string;
	}>
> {
	const messages = await getAllMessages(transcriptPath);
	const conversation: Array<{
		role: "user" | "assistant";
		content: string;
	}> = [];

	for (const message of messages) {
		if (message.type === "summary") continue;

		if (message.type === "user" && message.message.role === "user") {
			let content = "";

			if (typeof message.message.content === "string") {
				content = message.message.content;
			} else if (Array.isArray(message.message.content)) {
				content = message.message.content
					.filter((item) => item.type === "text" && item.content)
					.map((item) => item.content)
					.join("\n");
			}

			if (content) {
				conversation.push({
					role: "user",
					content,
				});
			}
		} else if (message.type === "assistant") {
			const textContent = message.message.content
				.filter((item) => item.type === "text" && item.text)
				.map((item) => item.text)
				.join("");

			if (textContent) {
				conversation.push({
					role: "assistant",
					content: textContent,
				});
			}
		}
	}

	return conversation;
}

/**
 * Extracts all tool usage events from a transcript file.
 * Useful for analyzing what tools Claude used during the conversation.
 *
 * @param transcriptPath - Path to the JSONL transcript file
 * @returns Array of tool usage events with tool name, input, and timestamp
 */
export async function getToolUsage(transcriptPath: string): Promise<
	Array<{
		tool: string;
		input: Record<string, unknown>;
		timestamp: string;
	}>
> {
	const messages = await getAllMessages(transcriptPath);
	const toolUsage: Array<{
		tool: string;
		input: Record<string, unknown>;
		timestamp: string;
	}> = [];

	for (const message of messages) {
		if (message.type === "assistant") {
			const toolUses = message.message.content.filter(
				(item) => item.type === "tool_use",
			);

			for (const toolUse of toolUses) {
				if (toolUse.name && toolUse.input) {
					toolUsage.push({
						tool: toolUse.name,
						input: toolUse.input,
						timestamp: message.timestamp,
					});
				}
			}
		}
	}

	return toolUsage;
}
