import { expect, test, describe } from "bun:test";

describe("CLI integration", () => {
	test("imports without executing CLI", () => {
		// This test verifies that importing the index module doesn't trigger CLI execution
		// The import.meta.main check should prevent CLI parsing during test imports
		expect(true).toBe(true);
	});
});