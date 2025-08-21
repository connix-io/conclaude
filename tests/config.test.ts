import { expect, test, describe } from "bun:test";
import { extractBashCommands } from "../src/config.ts";

describe("extractBashCommands", () => {
	test("extracts single command", () => {
		const script = "echo hello";
		const commands = extractBashCommands(script);
		expect(commands).toEqual(["echo hello"]);
	});

	test("extracts multiple commands", () => {
		const script = `echo hello
npm install
npm test`;
		const commands = extractBashCommands(script);
		expect(commands).toEqual(["echo hello", "npm install", "npm test"]);
	});

	test("ignores empty lines", () => {
		const script = `echo hello

npm test
`;
		const commands = extractBashCommands(script);
		expect(commands).toEqual(["echo hello", "npm test"]);
	});

	test("ignores comments", () => {
		const script = `# This is a comment
echo hello
# Another comment
npm test`;
		const commands = extractBashCommands(script);
		expect(commands).toEqual(["echo hello", "npm test"]);
	});

	test("handles mixed whitespace and comments", () => {
		const script = `   # Comment with leading spaces
echo hello
   
	# Tab-indented comment
npm test
   echo world   `;
		const commands = extractBashCommands(script);
		expect(commands).toEqual(["echo hello", "npm test", "   echo world   "]);
	});

	test("handles complex bash commands", () => {
		const script = `nix develop -c "lint"
bun x tsc --noEmit
cd /tmp && echo "test"`;
		const commands = extractBashCommands(script);
		expect(commands).toEqual([
			'nix develop -c "lint"',
			"bun x tsc --noEmit",
			'cd /tmp && echo "test"'
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