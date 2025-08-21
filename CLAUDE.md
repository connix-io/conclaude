
# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`conclaude` is a Claude Code hook handler CLI tool that processes hook events from Claude Code by reading JSON payloads from stdin and executing handlers for each event type. The tool provides lifecycle hooks for tool usage, session management, and transcript processing.

## Commands

- **Build**: `bun build` or `bun build src/index.ts --target=bun`
- **Test**: `bun test`
- **Lint**: `bun x tsc --noEmit` (type checking)
- **Run**: `bun src/index.ts <hook-type>` or `bun run index.ts`

## Architecture

### Core Components

- **CLI Handler** (`src/index.ts`): Main entry point with yargs-based command routing for all hook types
- **Type Definitions** (`src/types.ts`): Complete TypeScript definitions for hook payloads and transcript parsing utilities
- **Logger** (`src/logger.ts`): Winston-based logging with session-specific file output to tmpdir

### Hook System Architecture

The application implements a comprehensive hook system for Claude Code lifecycle events:

1. **Input Processing**: JSON payloads received via stdin containing session metadata and event-specific data
2. **Validation Layer**: Each handler validates required fields using typed interfaces
3. **Session Logging**: Winston logger tagged with session ID for tracking across hook events
4. **Result Handling**: Standardized `HookResult` type controls whether operations are blocked or allowed

### Hook Types

- **PreToolUse/PostToolUse**: Tool execution lifecycle with input/output logging
- **UserPromptSubmit**: User input validation and preprocessing
- **SessionStart/Stop**: Session initialization and cleanup with lint/test execution on stop
- **SubagentStop**: Subagent completion handling
- **Notification**: System notification processing
- **PreCompact**: Transcript compaction preprocessing

### Stop Hook Implementation

The Stop hook executes validation checks:
- Runs `nix develop -c lint` 
- Runs `nix develop -c tests`
- Blocks if either fails, returning error details

## Development Environment

- **TypeScript**: Strict configuration with ESNext target
- **Bun Runtime**: Default runtime (prefer over Node.js)
- **Biome**: Linting and formatting with tab indentation
- **Winston**: Structured logging to console and files

## Bun Usage

- Use `bun <file>` instead of `node <file>` or `ts-node <file>`
- Use `bun test` instead of `jest` or `vitest`
- Use `bun build <file.html|file.ts|file.css>` instead of `webpack` or `esbuild`
- Use `bun install` instead of `npm install` or `yarn install` or `pnpm install`
- Use `bun run <script>` instead of `npm run <script>` or `yarn run <script>` or `pnpm run <script>`
- Bun automatically loads .env, so don't use dotenv
- `Bun.spawn()` for subprocess execution (used in Stop hook)
- Prefer `Bun.file` over `node:fs`'s readFile/writeFile
