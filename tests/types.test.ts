import { afterEach, beforeEach, describe, expect, test } from "bun:test";
import { mkdir, rm, writeFile } from "fs/promises";
import { join } from "path";
import type { BasePayloadType } from "../src/types.ts";
import {
	getAllMessages,
	getConversationHistory,
	getInitialMessage,
	validateBasePayload,
} from "../src/types.ts";

describe("validateBasePayload", () => {
	test("validates valid payload", () => {
		const payload: BasePayloadType = {
			session_id: "test-session",
			transcript_path: "/path/to/transcript.jsonl",
			hook_event_name: "PreToolUse",
			tool_name: "Edit",
			tool_input: {},
		};

		expect(() => validateBasePayload(payload, "PreToolUse")).not.toThrow();
	});

	test("throws on missing session_id", () => {
		const payload = {
			transcript_path: "/path/to/transcript.jsonl",
			hook_event_name: "PreToolUse",
			tool_name: "Edit",
			tool_input: {},
		} as BasePayloadType;

		expect(() => validateBasePayload(payload, "PreToolUse")).toThrow(
			"Missing required field: session_id",
		);
	});

	test("throws on missing transcript_path", () => {
		const payload = {
			session_id: "test-session",
			hook_event_name: "PreToolUse",
			tool_name: "Edit",
			tool_input: {},
		} as BasePayloadType;

		expect(() => validateBasePayload(payload, "PreToolUse")).toThrow(
			"Missing required field: transcript_path",
		);
	});

	test("throws on missing hook_event_name", () => {
		const payload = {
			session_id: "test-session",
			transcript_path: "/path/to/transcript.jsonl",
			tool_name: "Edit",
			tool_input: {},
		} as BasePayloadType;

		expect(() => validateBasePayload(payload, "PreToolUse")).toThrow(
			"Missing required field: hook_event_name",
		);
	});

	test("throws on wrong hook_event_name", () => {
		const payload: BasePayloadType = {
			session_id: "test-session",
			transcript_path: "/path/to/transcript.jsonl",
			hook_event_name: "PostToolUse",
			tool_name: "Edit",
			tool_input: {},
			tool_response: {
				success: true,
			},
		};

		expect(() => validateBasePayload(payload, "PreToolUse")).toThrow(
			"Expected hook_event_name to be PreToolUse, got PostToolUse",
		);
	});
});

describe("transcript parsing functions", () => {
	const tempDir = "/tmp/conclaude-test";
	const transcriptPath = join(tempDir, "test-transcript.jsonl");

	beforeEach(async () => {
		await mkdir(tempDir, {
			recursive: true,
		});
	});

	afterEach(async () => {
		await rm(tempDir, {
			recursive: true,
			force: true,
		});
	});

	describe("getInitialMessage", () => {
		test("extracts first user message with string content", async () => {
			const transcript = `{"type":"user","message":{"role":"user","content":"Hello, Claude!"},"uuid":"msg-1","timestamp":"2024-01-01T00:00:00Z","session_id":"test","transcript_path":"/path","hook_event_name":"test"}
{"type":"assistant","message":{"role":"assistant","content":[{"type":"text","text":"Hello! How can I help?"}]},"uuid":"msg-2","timestamp":"2024-01-01T00:01:00Z"}`;

			await writeFile(transcriptPath, transcript);
			const result = await getInitialMessage(transcriptPath);
			expect(result).toBe("Hello, Claude!");
		});

		test("extracts first user message with array content", async () => {
			const transcript = `{"type":"user","message":{"role":"user","content":[{"type":"text","content":"What files are here?"},{"type":"tool_result","content":"file1.ts\\nfile2.ts"}]},"uuid":"msg-1","timestamp":"2024-01-01T00:00:00Z"}`;

			await writeFile(transcriptPath, transcript);
			const result = await getInitialMessage(transcriptPath);
			expect(result).toBe("What files are here?");
		});

		test("returns null for empty transcript", async () => {
			await writeFile(transcriptPath, "");
			const result = await getInitialMessage(transcriptPath);
			expect(result).toBeNull();
		});

		test("skips summary messages", async () => {
			const transcript = `{"type":"summary","summary":"Previous conversation","leafUuid":"uuid-1"}
{"type":"user","message":{"role":"user","content":"Hello!"},"uuid":"msg-1","timestamp":"2024-01-01T00:00:00Z"}`;

			await writeFile(transcriptPath, transcript);
			const result = await getInitialMessage(transcriptPath);
			expect(result).toBe("Hello!");
		});
	});

	describe("getAllMessages", () => {
		test("parses all valid messages", async () => {
			const transcript = `{"type":"user","message":{"role":"user","content":"Hello"},"uuid":"msg-1","timestamp":"2024-01-01T00:00:00Z"}
{"type":"assistant","message":{"role":"assistant","content":[{"type":"text","text":"Hi!"}]},"uuid":"msg-2","timestamp":"2024-01-01T00:01:00Z"}
{"type":"summary","summary":"Test summary","leafUuid":"uuid-1"}`;

			await writeFile(transcriptPath, transcript);
			const messages = await getAllMessages(transcriptPath);
			expect(messages).toHaveLength(3);
			expect(messages[0]?.type).toBe("user");
			expect(messages[1]?.type).toBe("assistant");
			expect(messages[2]?.type).toBe("summary");
		});

		test("ignores invalid JSON lines", async () => {
			const transcript = `{"type":"user","message":{"role":"user","content":"Hello"},"uuid":"msg-1","timestamp":"2024-01-01T00:00:00Z"}
invalid json line
{"type":"summary","summary":"Test summary","leafUuid":"uuid-1"}`;

			await writeFile(transcriptPath, transcript);
			const messages = await getAllMessages(transcriptPath);
			expect(messages).toHaveLength(2);
		});
	});

	describe("getConversationHistory", () => {
		test("extracts conversation between user and assistant", async () => {
			const transcript = `{"type":"user","message":{"role":"user","content":"What is 2+2?"},"uuid":"msg-1","timestamp":"2024-01-01T00:00:00Z"}
{"type":"assistant","message":{"role":"assistant","content":[{"type":"text","text":"The answer is 4."}]},"uuid":"msg-2","timestamp":"2024-01-01T00:01:00Z"}
{"type":"user","message":{"role":"user","content":"Thanks!"},"uuid":"msg-3","timestamp":"2024-01-01T00:02:00Z"}`;

			await writeFile(transcriptPath, transcript);
			const conversation = await getConversationHistory(transcriptPath);

			expect(conversation).toEqual([
				{
					role: "user",
					content: "What is 2+2?",
				},
				{
					role: "assistant",
					content: "The answer is 4.",
				},
				{
					role: "user",
					content: "Thanks!",
				},
			]);
		});

		test("filters out tool usage content", async () => {
			const transcript = `{"type":"user","message":{"role":"user","content":"Hello"},"uuid":"msg-1","timestamp":"2024-01-01T00:00:00Z"}
{"type":"assistant","message":{"role":"assistant","content":[{"type":"text","text":"Hi there!"},{"type":"tool_use","name":"Read","input":{"file_path":"test.txt"}}]},"uuid":"msg-2","timestamp":"2024-01-01T00:01:00Z"}`;

			await writeFile(transcriptPath, transcript);
			const conversation = await getConversationHistory(transcriptPath);

			expect(conversation).toEqual([
				{
					role: "user",
					content: "Hello",
				},
				{
					role: "assistant",
					content: "Hi there!",
				},
			]);
		});
	});
});
