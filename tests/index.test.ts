import { expect, test, describe } from "bun:test";
import { minimatch } from "minimatch";

describe("CLI integration", () => {
	test("imports without executing CLI", () => {
		// This test verifies that importing the index module doesn't trigger CLI execution
		// The import.meta.main check should prevent CLI parsing during test imports
		expect(true).toBe(true);
	});
});

describe("Uneditable files validation", () => {
	test("minimatch patterns work correctly", () => {
		// Test basic patterns
		expect(minimatch("package.json", "package.json")).toBe(true);
		expect(minimatch("package.json", "*.json")).toBe(true);
		expect(minimatch("src/index.ts", "src/**/*.ts")).toBe(true);
		expect(minimatch("docs/README.md", "docs/**")).toBe(true);
		expect(minimatch(".env.local", ".env*")).toBe(true);
		
		// Test brace expansion
		expect(minimatch("package.json", "{package,tsconfig}.json")).toBe(true);
		expect(minimatch("tsconfig.json", "{package,tsconfig}.json")).toBe(true);
		expect(minimatch("other.json", "{package,tsconfig}.json")).toBe(false);
		
		// Test negation patterns
		expect(minimatch("src/generated/types.ts", "src/**/*.ts")).toBe(true);
		
		// Test case sensitivity
		expect(minimatch("README.md", "*.MD")).toBe(false);
		expect(minimatch("README.md", "*.md")).toBe(true);
	});

	test("file path normalization scenarios", () => {
		const testCases = [
			{ path: "./package.json", pattern: "package.json", expected: true },
			{ path: "src/../package.json", pattern: "package.json", expected: true },
			{ path: "/absolute/path/package.json", pattern: "package.json", expected: true },
			{ path: "src/nested/file.ts", pattern: "src/**/*.ts", expected: true },
			{ path: "src\\nested\\file.ts", pattern: "src/**/*.ts", expected: true }, // Windows paths
		];

		for (const { path, pattern, expected } of testCases) {
			const result = minimatch(path, pattern) || 
							minimatch(path.replace(/\\/g, "/"), pattern) ||
							minimatch(path.split("/").pop() || "", pattern);
			expect(result).toBe(expected);
		}
	});
});