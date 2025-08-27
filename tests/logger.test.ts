import { expect, test, describe, beforeEach, afterEach } from "bun:test";
import { createLogger } from "../src/logger.ts";
import type { LoggingConfig } from "../src/types.ts";

describe("Logger configuration", () => {
	let originalEnv: string | undefined;

	beforeEach(() => {
		// Save original environment variable
		originalEnv = process.env.CONCLAUDE_DISABLE_FILE_LOGGING;
	});

	afterEach(() => {
		// Restore original environment variable
		if (originalEnv !== undefined) {
			process.env.CONCLAUDE_DISABLE_FILE_LOGGING = originalEnv;
		} else {
			delete process.env.CONCLAUDE_DISABLE_FILE_LOGGING;
		}
	});

	test("creates logger with file logging disabled by default", () => {
		// Remove environment variable to test default behavior
		delete process.env.CONCLAUDE_DISABLE_FILE_LOGGING;

		const logger = createLogger("test-session", "test-project");

		// Check that logger was created successfully
		expect(logger).toBeDefined();
		
		// Logger should have console transport only (file logging disabled by default)
		expect(logger.transports).toHaveLength(1);
	});

	test("enables file logging when CONCLAUDE_DISABLE_FILE_LOGGING=false", () => {
		process.env.CONCLAUDE_DISABLE_FILE_LOGGING = "false";

		const logger = createLogger("test-session", "test-project");

		// Check that logger was created successfully
		expect(logger).toBeDefined();
		
		// Logger should have both console and file transports
		expect(logger.transports).toHaveLength(2);
	});

	test("disables file logging when CONCLAUDE_DISABLE_FILE_LOGGING=true", () => {
		process.env.CONCLAUDE_DISABLE_FILE_LOGGING = "true";

		const logger = createLogger("test-session", "test-project");

		// Check that logger was created successfully
		expect(logger).toBeDefined();
		
		// Logger should have console transport only
		expect(logger.transports).toHaveLength(1);
	});

	test("config parameter overrides environment variable (enable file logging)", () => {
		process.env.CONCLAUDE_DISABLE_FILE_LOGGING = "true";

		const config: Partial<LoggingConfig> = { fileLogging: true };
		const logger = createLogger("test-session", "test-project", config);

		// Check that logger was created successfully
		expect(logger).toBeDefined();
		
		// Logger should have both console and file transports (config overrides env)
		expect(logger.transports).toHaveLength(2);
	});

	test("config parameter overrides environment variable (disable file logging)", () => {
		process.env.CONCLAUDE_DISABLE_FILE_LOGGING = "false";

		const config: Partial<LoggingConfig> = { fileLogging: false };
		const logger = createLogger("test-session", "test-project", config);

		// Check that logger was created successfully
		expect(logger).toBeDefined();
		
		// Logger should have console transport only (config overrides env)
		expect(logger.transports).toHaveLength(1);
	});

	test("creates logger with custom session ID and project name", () => {
		const config: Partial<LoggingConfig> = { fileLogging: true };
		const logger = createLogger("custom-session", "custom-project", config);

		// Check that logger was created successfully
		expect(logger).toBeDefined();
		
		// Logger should have both transports
		expect(logger.transports).toHaveLength(2);
	});

	test("handles undefined/null config parameter gracefully", () => {
		delete process.env.CONCLAUDE_DISABLE_FILE_LOGGING;

		const logger1 = createLogger("test-session", "test-project", undefined);
		const logger2 = createLogger("test-session", "test-project", {});

		// Both should work and have default behavior (file logging disabled)
		expect(logger1).toBeDefined();
		expect(logger2).toBeDefined();
		expect(logger1.transports).toHaveLength(1);
		expect(logger2.transports).toHaveLength(1);
	});

	test("logger includes correct metadata and format", () => {
		const config: Partial<LoggingConfig> = { fileLogging: true };
		const logger = createLogger("test-session", "test-project", config);

		// Verify logger has correct level and format
		expect(logger.level).toBe("info"); // Default level
		expect(logger).toBeDefined();
		
		// Test that logger can log without errors
		expect(() => {
			logger.info("Test log message");
		}).not.toThrow();
	});
});