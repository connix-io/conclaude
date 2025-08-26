import { expect, test, describe, beforeEach, afterEach } from "bun:test";
import { extractBashCommands, loadConclaudeConfig } from "../src/config.ts";
import * as fs from "fs";
import * as path from "path";
import * as os from "os";

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

describe("loadConclaudeConfig", () => {
	let tempDir: string;
	let originalCwd: string;

	beforeEach(() => {
		// Create a temporary directory for testing
		tempDir = fs.mkdtempSync(path.join(os.tmpdir(), "conclaude-test-"));
		originalCwd = process.cwd();
	});

	afterEach(() => {
		// Clean up temporary directory
		fs.rmSync(tempDir, { recursive: true, force: true });
		process.chdir(originalCwd);
	});

	const validConfigContent = `
stop:
  run: "echo test"
rules:
  preventRootAdditions: true
  uneditableFiles: []
`;

	test("finds config in current directory", async () => {
		const configPath = path.join(tempDir, ".conclaude.yaml");
		fs.writeFileSync(configPath, validConfigContent);

		const config = await loadConclaudeConfig(tempDir);
		expect(config.stop.run).toBe("echo test");
		expect(config.rules.preventRootAdditions).toBe(true);
	});

	test("finds .yml config file", async () => {
		const configPath = path.join(tempDir, ".conclaude.yml");
		fs.writeFileSync(configPath, validConfigContent);

		const config = await loadConclaudeConfig(tempDir);
		expect(config.stop.run).toBe("echo test");
	});

	test("searches parent directories", async () => {
		// Create nested directory structure
		const subDir = path.join(tempDir, "subdir");
		fs.mkdirSync(subDir);
		
		// Place config in parent directory
		const configPath = path.join(tempDir, ".conclaude.yaml");
		fs.writeFileSync(configPath, validConfigContent);

		const config = await loadConclaudeConfig(subDir);
		expect(config.stop.run).toBe("echo test");
	});

	test("searches up to 2 parent directories", async () => {
		// Create nested directory structure: tempDir/level1/level2/level3
		const level1Dir = path.join(tempDir, "level1");
		const level2Dir = path.join(level1Dir, "level2");
		const level3Dir = path.join(level2Dir, "level3");
		fs.mkdirSync(level1Dir);
		fs.mkdirSync(level2Dir);
		fs.mkdirSync(level3Dir);
		
		// Place config in level1 directory (2 levels up from level3)
		const configPath = path.join(level1Dir, ".conclaude.yaml");
		fs.writeFileSync(configPath, validConfigContent);

		const config = await loadConclaudeConfig(level3Dir);
		expect(config.stop.run).toBe("echo test");
	});

	test("does not search beyond 2 parent directories", async () => {
		// Create nested directory structure: tempDir/level1/level2/level3/level4
		const level1Dir = path.join(tempDir, "level1");
		const level2Dir = path.join(level1Dir, "level2");
		const level3Dir = path.join(level2Dir, "level3");
		const level4Dir = path.join(level3Dir, "level4");
		fs.mkdirSync(level1Dir);
		fs.mkdirSync(level2Dir);
		fs.mkdirSync(level3Dir);
		fs.mkdirSync(level4Dir);
		
		// Place config in tempDir (3 levels up from level4) - should NOT be found
		const configPath = path.join(tempDir, ".conclaude.yaml");
		fs.writeFileSync(configPath, validConfigContent);

		await expect(loadConclaudeConfig(level4Dir)).rejects.toThrow("No .conclaude.yaml or .conclaude.yml configuration file found");
	});

	test("stops at git root", async () => {
		// Create git repository in tempDir
		const gitDir = path.join(tempDir, ".git");
		fs.mkdirSync(gitDir);
		
		// Create nested directory structure beyond git root
		const subDir = path.join(tempDir, "subdir");
		const subSubDir = path.join(subDir, "subsubdir");
		fs.mkdirSync(subDir);
		fs.mkdirSync(subSubDir);
		
		// Place config at git root
		const configPath = path.join(tempDir, ".conclaude.yaml");
		fs.writeFileSync(configPath, validConfigContent);

		const config = await loadConclaudeConfig(subSubDir);
		expect(config.stop.run).toBe("echo test");
	});

	test("provides detailed error message when config not found", async () => {
		await expect(loadConclaudeConfig(tempDir)).rejects.toThrow(/No \.conclaude\.yaml or \.conclaude\.yml configuration file found/);
		await expect(loadConclaudeConfig(tempDir)).rejects.toThrow(/Searched locations:/);
	});

	test("handles invalid YAML content", async () => {
		const configPath = path.join(tempDir, ".conclaude.yaml");
		fs.writeFileSync(configPath, "invalid: yaml: content: [");

		await expect(loadConclaudeConfig(tempDir)).rejects.toThrow(/failed to parse YAML content/);
	});

	test("prefers .yaml over .yml when both exist", async () => {
		const yamlConfig = `
stop:
  run: "echo yaml"
rules:
  preventRootAdditions: true
  uneditableFiles: []
`;
		const ymlConfig = `
stop:
  run: "echo yml"
rules:
  preventRootAdditions: true
  uneditableFiles: []
`;
		
		fs.writeFileSync(path.join(tempDir, ".conclaude.yaml"), yamlConfig);
		fs.writeFileSync(path.join(tempDir, ".conclaude.yml"), ymlConfig);

		const config = await loadConclaudeConfig(tempDir);
		expect(config.stop.run).toBe("echo yaml");
	});

	test("error message includes git root information", async () => {
		// Create git repository in tempDir
		const gitDir = path.join(tempDir, ".git");
		fs.mkdirSync(gitDir);
		
		const subDir = path.join(tempDir, "subdir");
		fs.mkdirSync(subDir);

		try {
			await loadConclaudeConfig(subDir);
		} catch (error) {
			expect((error as Error).message).toContain("Search was limited to git repository root:");
			expect((error as Error).message).toContain(tempDir);
		}
	});

	test("error message when no git repository found", async () => {
		try {
			await loadConclaudeConfig(tempDir);
		} catch (error) {
			expect((error as Error).message).toContain("No git repository found - searched up to filesystem root or maximum 2 parent directories");
		}
	});
});