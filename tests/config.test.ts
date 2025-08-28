import { afterAll, beforeAll, describe, expect, test } from "bun:test";
import * as fs from "fs";
import * as os from "os";
import * as path from "path";
import {
	checkFileGrepRules,
	executeGrepRules,
	extractBashCommands,
	type GrepRule,
} from "../src/config.ts";

describe("extractBashCommands", () => {
	test("extracts single command", () => {
		const script = "echo hello";
		const commands = extractBashCommands(script);
		expect(commands).toEqual([
			"echo hello",
		]);
	});

	test("extracts multiple commands", () => {
		const script = `echo hello
npm install
npm test`;
		const commands = extractBashCommands(script);
		expect(commands).toEqual([
			"echo hello",
			"npm install",
			"npm test",
		]);
	});

	test("ignores empty lines", () => {
		const script = `echo hello

npm test
`;
		const commands = extractBashCommands(script);
		expect(commands).toEqual([
			"echo hello",
			"npm test",
		]);
	});

	test("ignores comments", () => {
		const script = `# This is a comment
echo hello
# Another comment
npm test`;
		const commands = extractBashCommands(script);
		expect(commands).toEqual([
			"echo hello",
			"npm test",
		]);
	});

	test("handles mixed whitespace and comments", () => {
		const script = `   # Comment with leading spaces
echo hello
   
	# Tab-indented comment
npm test
   echo world   `;
		const commands = extractBashCommands(script);
		expect(commands).toEqual([
			"echo hello",
			"npm test",
			"   echo world   ",
		]);
	});

	test("handles complex bash commands", () => {
		const script = `nix develop -c "lint"
bun x tsc --noEmit
cd /tmp && echo "test"`;
		const commands = extractBashCommands(script);
		expect(commands).toEqual([
			'nix develop -c "lint"',
			"bun x tsc --noEmit",
			'cd /tmp && echo "test"',
		]);
	});

	test("returns empty array for empty script", () => {
		const commands = extractBashCommands("");
		expect(commands).toEqual([]);
	});

	test("returns empty array for script with only comments", () => {
		const script = `# Comment 1
# Comment 2
   # Comment 3`;
		const commands = extractBashCommands(script);
		expect(commands).toEqual([]);
	});
});

describe("grep rules functionality", () => {
	let tempDir: string;

	beforeAll(() => {
		// Create temporary directory for test files
		tempDir = fs.mkdtempSync(path.join(os.tmpdir(), "conclaude-test-"));
	});

	afterAll(() => {
		// Clean up temporary directory
		if (tempDir && fs.existsSync(tempDir)) {
			fs.rmSync(tempDir, {
				recursive: true,
				force: true,
			});
		}
	});

	describe("executeGrepRules", () => {
		test("finds violations across multiple files", () => {
			// Create test files
			const testFile1 = path.join(tempDir, "test1.ts");
			const testFile2 = path.join(tempDir, "test2.js");

			fs.writeFileSync(
				testFile1,
				`
// TODO: Fix this later
console.log("Hello world");
function test() {
	// FIXME: This needs work
	return true;
}
			`,
			);

			fs.writeFileSync(
				testFile2,
				`
console.log("Debug message");
const x = 42;
			`,
			);

			const rules: GrepRule[] = [
				{
					filePattern: "**/*.ts",
					forbiddenPattern: "TODO|FIXME",
					description: "No TODO/FIXME comments allowed",
				},
				{
					filePattern: "**/*.{ts,js}",
					forbiddenPattern: "console\\.log",
					description: "No console.log statements allowed",
				},
			];

			const violations = executeGrepRules(rules, tempDir);

			expect(violations.length).toBeGreaterThan(0);

			// Check that we found TODO/FIXME violations in TypeScript file
			const todoViolations = violations.filter(
				(v) =>
					v.rule.description === "No TODO/FIXME comments allowed" &&
					v.file.includes("test1.ts"),
			);
			expect(todoViolations.length).toBe(2); // TODO and FIXME

			// Check that we found console.log violations in both files
			const consoleViolations = violations.filter(
				(v) => v.rule.description === "No console.log statements allowed",
			);
			expect(consoleViolations.length).toBe(2); // One in each file
		});

		test("returns empty array when no violations found", () => {
			// Create test file without violations
			const testFile = path.join(tempDir, "clean.ts");
			fs.writeFileSync(
				testFile,
				`
export function add(a: number, b: number): number {
	return a + b;
}
			`,
			);

			const rules: GrepRule[] = [
				{
					filePattern: "**/*.ts",
					forbiddenPattern: "TODO|FIXME",
					description: "No TODO/FIXME comments allowed",
				},
			];

			const violations = executeGrepRules(rules, tempDir);

			// Filter to only violations in our test file
			const relevantViolations = violations.filter((v) =>
				v.file.includes("clean.ts"),
			);
			expect(relevantViolations).toEqual([]);
		});

		test("handles empty rules array", () => {
			const violations = executeGrepRules([], tempDir);
			expect(violations).toEqual([]);
		});
	});

	describe("checkFileGrepRules", () => {
		test("finds violations in specific file", () => {
			// Create test file
			const testFile = path.join(tempDir, "single-test.ts");
			fs.writeFileSync(
				testFile,
				`
// TODO: Implement this
function test() {
	console.log("Debug");
	return 42;
}
			`,
			);

			const rules: GrepRule[] = [
				{
					filePattern: "*.ts",
					forbiddenPattern: "TODO",
					description: "No TODO comments",
				},
				{
					filePattern: "*.ts",
					forbiddenPattern: "console\\.log",
					description: "No console.log statements",
				},
			];

			const violations = checkFileGrepRules(testFile, rules, tempDir);

			expect(violations.length).toBe(2);
			expect(violations[0]?.rule.description).toBe("No TODO comments");
			expect(violations[0]?.lineContent).toContain("TODO: Implement this");
			expect(violations[1]?.rule.description).toBe(
				"No console.log statements",
			);
			expect(violations[1]?.lineContent).toContain('console.log("Debug")');
		});

		test("returns empty array for file that doesn't match patterns", () => {
			// Create test file
			const testFile = path.join(tempDir, "no-match.txt");
			fs.writeFileSync(
				testFile,
				`
TODO: This should not be found
console.log("This should not be found");
			`,
			);

			const rules: GrepRule[] = [
				{
					filePattern: "*.ts",
					forbiddenPattern: "TODO",
					description: "No TODO comments",
				},
			];

			const violations = checkFileGrepRules(testFile, rules, tempDir);
			expect(violations).toEqual([]);
		});

		test("returns empty array for non-existent file", () => {
			const nonExistentFile = path.join(tempDir, "does-not-exist.ts");

			const rules: GrepRule[] = [
				{
					filePattern: "*.ts",
					forbiddenPattern: "TODO",
					description: "No TODO comments",
				},
			];

			const violations = checkFileGrepRules(nonExistentFile, rules, tempDir);
			expect(violations).toEqual([]);
		});

		test("handles file path variations correctly", () => {
			// Create test file in subdirectory
			const subDir = path.join(tempDir, "src");
			fs.mkdirSync(subDir, {
				recursive: true,
			});
			const testFile = path.join(subDir, "nested.ts");
			fs.writeFileSync(
				testFile,
				`
// FIXME: This needs attention
			`,
			);

			const rules: GrepRule[] = [
				{
					filePattern: "src/*.ts",
					forbiddenPattern: "FIXME",
					description: "No FIXME comments",
				},
			];

			// Test with relative path
			const relativePath = path.relative(tempDir, testFile);
			const violations = checkFileGrepRules(relativePath, rules, tempDir);
			expect(violations.length).toBe(1);
			expect(violations[0]?.lineContent).toContain(
				"FIXME: This needs attention",
			);
		});

		test("handles empty rules array", () => {
			const testFile = path.join(tempDir, "any-file.ts");
			fs.writeFileSync(testFile, "TODO: test");

			const violations = checkFileGrepRules(testFile, [], tempDir);
			expect(violations).toEqual([]);
		});
	});

	describe("pattern matching", () => {
		test("supports complex glob patterns", () => {
			// Create nested directory structure
			const srcDir = path.join(tempDir, "src");
			const testDir = path.join(srcDir, "test");
			fs.mkdirSync(testDir, {
				recursive: true,
			});

			const file1 = path.join(srcDir, "main.js");
			const file2 = path.join(testDir, "spec.js");

			fs.writeFileSync(file1, 'console.log("main");');
			fs.writeFileSync(file2, 'console.log("test");');

			const rules: GrepRule[] = [
				{
					filePattern: "src/**/*.js",
					forbiddenPattern: "console\\.log",
					description: "No console.log in src files",
				},
			];

			const violations = executeGrepRules(rules, tempDir);
			const relevantViolations = violations.filter(
				(v) => v.file.includes("main.js") || v.file.includes("spec.js"),
			);

			expect(relevantViolations.length).toBe(2); // Found in both files
		});

		test("supports regex patterns for forbidden content", () => {
			const testFile = path.join(tempDir, "regex-test.js");
			fs.writeFileSync(
				testFile,
				`
var oldVar = 1;
let newLet = 2;
const newConst = 3;
			`,
			);

			const rules: GrepRule[] = [
				{
					filePattern: "*.js",
					forbiddenPattern: "\\bvar\\b",
					description: "Use let/const instead of var",
				},
			];

			const violations = checkFileGrepRules(testFile, rules, tempDir);
			expect(violations.length).toBe(1);
			expect(violations[0]?.lineContent).toContain("var oldVar = 1;");
		});
	});
});
