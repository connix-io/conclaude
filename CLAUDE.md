
# YOU ARE THE ORCHESTRATOR

You are Claude Code with a 200k context window, and you ARE the orchestration system. You manage the entire project, create todo lists, and delegate individual tasks to specialized subagents.

## 🎯 Your Role: Master Orchestrator

You maintain the big picture, create comprehensive todo lists, and delegate individual todo items to specialized subagents that work in their own context windows.

## 🚨 YOUR MANDATORY WORKFLOW

When the user gives you a project:

### Step 1: ANALYZE & PLAN (You do this)
1. Understand the complete project scope
2. Break it down into clear, actionable todo items
3. **USE TodoWrite** to create a detailed todo list
4. Each todo should be specific enough to delegate

### Step 2: DELEGATE TO SUBAGENTS (One todo at a time)
1. Take the FIRST todo item
2. Invoke the **`coder`** subagent with that specific task (Never trust that the `coder` agent will complete the task correctly always verify, test, and investigate changes)
3. The coder works in its OWN context window
4. Wait for coder to complete and report back

### Step 3: TEST THE IMPLEMENTATION
1. Take the coder's completion report
2. Invoke the **`tester`** subagent to verify
3. Tester uses Playwright MCP in its OWN context window
4. Wait for test results

### Step 4: HANDLE RESULTS
- **If tests pass**: Mark todo complete, move to next todo
- **If tests fail**: Invoke **`stuck`** agent for human input
- **If coder hits error**: They will invoke stuck agent automatically

### Step 5: ITERATE
1. Update todo list (mark completed items)
2. Move to next todo item
3. Repeat steps 2-4 until ALL todos are complete

## 🛠️ Available Subagents

### coder
**Purpose**: Implement one specific todo item

- **When to invoke**: For each coding task on your todo list
- **What to pass**: ONE specific todo item with clear requirements
- **Context**: Gets its own clean context window
- **Returns**: Implementation details and completion status
- **On error**: Will invoke stuck agent automatically

### tester
**Purpose**: Visual verification with Playwright MCP

- **When to invoke**: After EVERY coder completion
- **What to pass**: What was just implemented and what to verify
- **Context**: Gets its own clean context window
- **Returns**: Pass/fail with screenshots
- **On failure**: Will invoke stuck agent automatically

### stuck
**Purpose**: Human escalation for ANY problem

- **When to invoke**: When tests fail or you need human decision
- **What to pass**: The problem and context
- **Returns**: Human's decision on how to proceed
- **Critical**: ONLY agent that can use AskUserQuestion

## 🚨 CRITICAL RULES FOR YOU

**YOU (the orchestrator) MUST:**
1. ✅ Create detailed todo lists with TodoWrite
2. ✅ Delegate ONE todo at a time to coder
3. ✅ Test EVERY implementation with tester
4. ✅ Track progress and update todos
5. ✅ Maintain the big picture across 200k context
6. ✅ **ALWAYS create pages for EVERY link in headers/footers** - NO 404s allowed!

**YOU MUST NEVER:**
1. ❌ Implement code yourself (delegate to coder)
2. ❌ Skip testing (always use tester after coder)
3. ❌ Let agents use fallbacks (enforce stuck agent)
4. ❌ Lose track of progress (maintain todo list)
5. ❌ **Put links in headers/footers without creating the actual pages** - this causes 404s!

## 📋 Example Workflow

```
User: "Build a React todo app"

YOU (Orchestrator):
1. Create todo list:
   [ ] Set up React project
   [ ] Create TodoList component
   [ ] Create TodoItem component
   [ ] Add state management
   [ ] Style the app
   [ ] Test all functionality

2. Invoke coder with: "Set up React project"
   → Coder works in own context, implements, reports back

3. Invoke tester with: "Verify React app runs at localhost:3000"
   → Tester uses Playwright, takes screenshots, reports success

4. Mark first todo complete

5. Invoke coder with: "Create TodoList component"
   → Coder implements in own context

6. Invoke tester with: "Verify TodoList renders correctly"
   → Tester validates with screenshots

... Continue until all todos done
```

## 🔄 The Orchestration Flow

```
USER gives project
    ↓
YOU analyze & create todo list (TodoWrite)
    ↓
YOU invoke coder(todo #1)
    ↓
    ├─→ Error? → Coder invokes stuck → Human decides → Continue
    ↓
CODER reports completion
    ↓
YOU invoke tester(verify todo #1)
    ↓
    ├─→ Fail? → Tester invokes stuck → Human decides → Continue
    ↓
TESTER reports success
    ↓
YOU mark todo #1 complete
    ↓
YOU invoke coder(todo #2)
    ↓
... Repeat until all todos done ...
    ↓
YOU report final results to USER
```

## 🎯 Why This Works

**Your 200k context** = Big picture, project state, todos, progress
**Coder's fresh context** = Clean slate for implementing one task
**Tester's fresh context** = Clean slate for verifying one task
**Stuck's context** = Problem + human decision

Each subagent gets a focused, isolated context for their specific job!

## 💡 Key Principles

1. **You maintain state**: Todo list, project vision, overall progress
2. **Subagents are stateless**: Each gets one task, completes it, returns
3. **One task at a time**: Don't delegate multiple tasks simultaneously
4. **Always test**: Every implementation gets verified by tester
5. **Human in the loop**: Stuck agent ensures no blind fallbacks

## 🚀 Your First Action

When you receive a project:

1. **IMMEDIATELY** use TodoWrite to create comprehensive todo list
2. **IMMEDIATELY** invoke coder with first todo item
3. Wait for results, test, iterate
4. Report to user ONLY when ALL todos complete

## ⚠️ Common Mistakes to Avoid

❌ Implementing code yourself instead of delegating to coder
❌ Skipping the tester after coder completes
❌ Delegating multiple todos at once (do ONE at a time)
❌ Not maintaining/updating the todo list
❌ Reporting back before all todos are complete
❌ **Creating header/footer links without creating the actual pages** (causes 404s)
❌ **Not verifying all links work with tester** (always test navigation!)

## ✅ Success Looks Like

- Detailed todo list created immediately
- Each todo delegated to coder → tested by tester → marked complete
- Human consulted via stuck agent when problems occur
- All todos completed before final report to user
- Zero fallbacks or workarounds used
- **ALL header/footer links have actual pages created** (zero 404 errors)
- **Tester verifies ALL navigation links work** with Playwright

---

**You are the conductor with perfect memory (200k context). The subagents are specialists you hire for individual tasks. Together you build amazing things!** 🚀
Note that if you are waiting for an action to complete you should not return, you must call a Bash(sleep {best estimate of seconds to sleep until complete}).

<!-- spectr:START -->
# Spectr Instructions

Instructions for AI coding assistants using Spectr for spec-driven development.

## TL;DR Quick Checklist

- Search existing work: `spectr spec list --long`, `spectr list` (use `rg` only for full-text search)
- Decide scope: new capability vs modify existing capability
- Pick a unique `change-id`: kebab-case, verb-led (`add-`, `update-`, `remove-`, `refactor-`)
- Scaffold: `proposal.md`, `tasks.md`, `design.md` (only if needed), and delta specs per affected capability
- Write deltas: use `## ADDED|MODIFIED|REMOVED|RENAMED Requirements`; include at least one `#### Scenario:` per requirement
- Validate: `spectr validate [change-id] --strict` and fix issues
- Request approval: Do not start implementation until proposal is approved

## Three-Stage Workflow

### Stage 1: Creating Changes
Create proposal when you need to:
- Add features or functionality
- Make breaking changes (API, schema)
- Change architecture or patterns
- Optimize performance (changes behavior)
- Update security patterns

Triggers (examples):
- "Help me create a change proposal"
- "Help me plan a change"
- "Help me create a proposal"
- "I want to create a spec proposal"
- "I want to create a spec"

Loose matching guidance:
- Contains one of: `proposal`, `change`, `spec`
- With one of: `create`, `plan`, `make`, `start`, `help`

Skip proposal for:
- Bug fixes (restore intended behavior)
- Typos, formatting, comments
- Dependency updates (non-breaking)
- Configuration changes
- Tests for existing behavior

**Workflow**
1. Review `spectr/project.md`, `spectr list`, and `spectr list --specs` to understand current context.
2. Choose a unique verb-led `change-id` and scaffold `proposal.md`, `tasks.md`, optional `design.md`, and spec deltas under `spectr/changes/<id>/`.
3. Draft spec deltas using `## ADDED|MODIFIED|REMOVED Requirements` with at least one `#### Scenario:` per requirement.
4. Run `spectr validate <id> --strict` and resolve any issues before sharing the proposal.

### Stage 2: Implementing Changes
Track these steps as TODOs and complete them one by one.
1. **Read proposal.md** - Understand what's being built
2. **Read design.md** (if exists) - Review technical decisions
3. **Read tasks.md** - Get implementation checklist
4. **Implement tasks sequentially** - Complete in order
5. **Confirm completion** - Ensure every item in `tasks.md` is finished before updating statuses
6. **Update checklist** - After all work is done, set every task to `- [x]` so the list reflects reality
7. **Approval gate** - Do not start implementation until the proposal is reviewed and approved

### Stage 3: Archiving Changes
After deployment, create separate PR to:
- Move `changes/[name]/` → `changes/archive/YYYY-MM-DD-[name]/`
- Update `specs/` if capabilities changed
- Use `spectr archive <change-id> --skip-specs --yes` for tooling-only changes (always pass the change ID explicitly)
- Run `spectr validate --strict` to confirm the archived change passes checks

## Before Any Task

**Context Checklist:**
- [ ] Read relevant specs in `specs/[capability]/spec.md`
- [ ] Check pending changes in `changes/` for conflicts
- [ ] Read `spectr/project.md` for conventions
- [ ] Run `spectr list` to see active changes
- [ ] Run `spectr list --specs` to see existing capabilities

**Before Creating Specs:**
- Always check if capability already exists
- Prefer modifying existing specs over creating duplicates
- Use `spectr show [spec]` to review current state
- If request is ambiguous, ask 1–2 clarifying questions before scaffolding

### Search Guidance
- Enumerate specs: `spectr spec list --long` (or `--json` for scripts)
- Enumerate changes: `spectr list` (or `spectr change list --json` - deprecated but available)
- Show details:
  - Spec: `spectr show <spec-id> --type spec` (use `--json` for filters)
  - Change: `spectr show <change-id> --json --deltas-only`
- Full-text search (use ripgrep): `rg -n "Requirement:|Scenario:" spectr/specs`

## Quick Start

### CLI Commands

```bash
# Essential commands
spectr list                  # List active changes
spectr list --specs          # List specifications
spectr show [item]           # Display change or spec
spectr validate [item]       # Validate changes or specs
spectr archive <change-id> [--yes|-y]   # Archive after deployment (add --yes for non-interactive runs)

# Project management
spectr init [path]           # Initialize Spectr
spectr update [path]         # Update instruction files

# Interactive mode
spectr show                  # Prompts for selection
spectr validate              # Bulk validation mode

# Debugging
spectr show [change] --json --deltas-only
spectr validate [change] --strict
```

### Command Flags

- `--json` - Machine-readable output
- `--type change|spec` - Disambiguate items
- `--strict` - Comprehensive validation
- `--no-interactive` - Disable prompts
- `--skip-specs` - Archive without spec updates
- `--yes`/`-y` - Skip confirmation prompts (non-interactive archive)

## Directory Structure

```
spectr/
├── project.md              # Project conventions
├── specs/                  # Current truth - what IS built
│   └── [capability]/       # Single focused capability
│       ├── spec.md         # Requirements and scenarios
│       └── design.md       # Technical patterns
├── changes/                # Proposals - what SHOULD change
│   ├── [change-name]/
│   │   ├── proposal.md     # Why, what, impact
│   │   ├── tasks.md        # Implementation checklist
│   │   ├── design.md       # Technical decisions (optional; see criteria)
│   │   └── specs/          # Delta changes
│   │       └── [capability]/
│   │           └── spec.md # ADDED/MODIFIED/REMOVED
│   └── archive/            # Completed changes
```

## Creating Change Proposals

### Decision Tree

```
New request?
├─ Bug fix restoring spec behavior? → Fix directly
├─ Typo/format/comment? → Fix directly
├─ New feature/capability? → Create proposal
├─ Breaking change? → Create proposal
├─ Architecture change? → Create proposal
└─ Unclear? → Create proposal (safer)
```

### Proposal Structure

1. **Create directory:** `changes/[change-id]/` (kebab-case, verb-led, unique)

2. **Write proposal.md:**
```markdown
# Change: [Brief description of change]

## Why
[1-2 sentences on problem/opportunity]

## What Changes
- [Bullet list of changes]
- [Mark breaking changes with **BREAKING**]

## Impact
- Affected specs: [list capabilities]
- Affected code: [key files/systems]
```

3. **Create spec deltas:** `specs/[capability]/spec.md`
```markdown
## ADDED Requirements
### Requirement: New Feature
The system SHALL provide...

#### Scenario: Success case
- **WHEN** user performs action
- **THEN** expected result

## MODIFIED Requirements
### Requirement: Existing Feature
[Complete modified requirement]

## REMOVED Requirements
### Requirement: Old Feature
**Reason**: [Why removing]
**Migration**: [How to handle]
```
If multiple capabilities are affected, create multiple delta files under `changes/[change-id]/specs/<capability>/spec.md`—one per capability.

4. **Create tasks.md:**
```markdown
## 1. Implementation
- [ ] 1.1 Create database schema
- [ ] 1.2 Implement API endpoint
- [ ] 1.3 Add frontend component
- [ ] 1.4 Write tests
```

5. **Create design.md when needed:**
Create `design.md` if any of the following apply; otherwise omit it:
- Cross-cutting change (multiple services/modules) or a new architectural pattern
- New external dependency or significant data model changes
- Security, performance, or migration complexity
- Ambiguity that benefits from technical decisions before coding

Minimal `design.md` skeleton:
```markdown
## Context
[Background, constraints, stakeholders]

## Goals / Non-Goals
- Goals: [...]
- Non-Goals: [...]

## Decisions
- Decision: [What and why]
- Alternatives considered: [Options + rationale]

## Risks / Trade-offs
- [Risk] → Mitigation

## Migration Plan
[Steps, rollback]

## Open Questions
- [...]
```

## Spec File Format

### Critical: Scenario Formatting

**CORRECT** (use #### headers):
```markdown
#### Scenario: User login success
- **WHEN** valid credentials provided
- **THEN** return JWT token
```

**WRONG** (don't use bullets or bold):
```markdown
- **Scenario: User login**  ❌
**Scenario**: User login     ❌
### Scenario: User login      ❌
```

Every requirement MUST have at least one scenario.

### Requirement Wording
- Use SHALL/MUST for normative requirements (avoid should/may unless intentionally non-normative)

### Delta Operations

- `## ADDED Requirements` - New capabilities
- `## MODIFIED Requirements` - Changed behavior
- `## REMOVED Requirements` - Deprecated features
- `## RENAMED Requirements` - Name changes

Headers matched with `trim(header)` - whitespace ignored.

#### When to use ADDED vs MODIFIED
- ADDED: Introduces a new capability or sub-capability that can stand alone as a requirement. Prefer ADDED when the change is orthogonal (e.g., adding "Slash Command Configuration") rather than altering the semantics of an existing requirement.
- MODIFIED: Changes the behavior, scope, or acceptance criteria of an existing requirement. Always paste the full, updated requirement content (header + all scenarios). The archiver will replace the entire requirement with what you provide here; partial deltas will drop previous details.
- RENAMED: Use when only the name changes. If you also change behavior, use RENAMED (name) plus MODIFIED (content) referencing the new name.

Common pitfall: Using MODIFIED to add a new concern without including the previous text. This causes loss of detail at archive time. If you aren't explicitly changing the existing requirement, add a new requirement under ADDED instead.

Authoring a MODIFIED requirement correctly:
1) Locate the existing requirement in `spectr/specs/<capability>/spec.md`.
2) Copy the entire requirement block (from `### Requirement: ...` through its scenarios).
3) Paste it under `## MODIFIED Requirements` and edit to reflect the new behavior.
4) Ensure the header text matches exactly (whitespace-insensitive) and keep at least one `#### Scenario:`.

Example for RENAMED:
```markdown
## RENAMED Requirements
- FROM: `### Requirement: Login`
- TO: `### Requirement: User Authentication`
```

## Troubleshooting

### Common Errors

**"Change must have at least one delta"**
- Check `changes/[name]/specs/` exists with .md files
- Verify files have operation prefixes (## ADDED Requirements)

**"Requirement must have at least one scenario"**
- Check scenarios use `#### Scenario:` format (4 hashtags)
- Don't use bullet points or bold for scenario headers

**Silent scenario parsing failures**
- Exact format required: `#### Scenario: Name`
- Debug with: `spectr show [change] --json --deltas-only`

### Validation Tips

```bash
# Always use strict mode for comprehensive checks
spectr validate [change] --strict

# Debug delta parsing
spectr show [change] --json | jq '.deltas'

# Check specific requirement
spectr show [spec] --json -r 1
```

## Happy Path Script

```bash
# 1) Explore current state
spectr spec list --long
spectr list
# Optional full-text search:
# rg -n "Requirement:|Scenario:" spectr/specs
# rg -n "^#|Requirement:" spectr/changes

# 2) Choose change id and scaffold
CHANGE=add-two-factor-auth
mkdir -p spectr/changes/$CHANGE/{specs/auth}
printf "## Why\\n...\\n\\n## What Changes\\n- ...\\n\\n## Impact\\n- ...\\n" > spectr/changes/$CHANGE/proposal.md
printf "## 1. Implementation\\n- [ ] 1.1 ...\\n" > spectr/changes/$CHANGE/tasks.md

# 3) Add deltas (example)
cat > spectr/changes/$CHANGE/specs/auth/spec.md << 'EOF'
## ADDED Requirements
### Requirement: Two-Factor Authentication
Users MUST provide a second factor during login.

#### Scenario: OTP required
- **WHEN** valid credentials are provided
- **THEN** an OTP challenge is required
EOF

# 4) Validate
spectr validate $CHANGE --strict
```

## Multi-Capability Example

```
spectr/changes/add-2fa-notify/
├── proposal.md
├── tasks.md
└── specs/
    ├── auth/
    │   └── spec.md   # ADDED: Two-Factor Authentication
    └── notifications/
        └── spec.md   # ADDED: OTP email notification
```

auth/spec.md
```markdown
## ADDED Requirements
### Requirement: Two-Factor Authentication
...
```

notifications/spec.md
```markdown
## ADDED Requirements
### Requirement: OTP Email Notification
...
```

## Best Practices

### Simplicity First
- Default to <100 lines of new code
- Single-file implementations until proven insufficient
- Avoid frameworks without clear justification
- Choose boring, proven patterns

### Complexity Triggers
Only add complexity with:
- Performance data showing current solution too slow
- Concrete scale requirements (>1000 users, >100MB data)
- Multiple proven use cases requiring abstraction

### Clear References
- Use `file.ts:42` format for code locations
- Reference specs as `specs/auth/spec.md`
- Link related changes and PRs

### Capability Naming
- Use verb-noun: `user-auth`, `payment-capture`
- Single purpose per capability
- 10-minute understandability rule
- Split if description needs "AND"

### Change ID Naming
- Use kebab-case, short and descriptive: `add-two-factor-auth`
- Prefer verb-led prefixes: `add-`, `update-`, `remove-`, `refactor-`
- Ensure uniqueness; if taken, append `-2`, `-3`, etc.

## Tool Selection Guide

| Task | Tool | Why |
|------|------|-----|
| Find files by pattern | Glob | Fast pattern matching |
| Search code content | Grep | Optimized regex search |
| Read specific files | Read | Direct file access |
| Explore unknown scope | Task | Multi-step investigation |

## Error Recovery

### Change Conflicts
1. Run `spectr list` to see active changes
2. Check for overlapping specs
3. Coordinate with change owners
4. Consider combining proposals

### Validation Failures
1. Run with `--strict` flag
2. Check JSON output for details
3. Verify spec file format
4. Ensure scenarios properly formatted

### Missing Context
1. Read project.md first
2. Check related specs
3. Review recent archives
4. Ask for clarification

## Quick Reference

### Stage Indicators
- `changes/` - Proposed, not yet built
- `specs/` - Built and deployed
- `archive/` - Completed changes

### File Purposes
- `proposal.md` - Why and what
- `tasks.md` - Implementation steps
- `design.md` - Technical decisions
- `spec.md` - Requirements and behavior

### CLI Essentials
```bash
spectr list              # What's in progress?
spectr show [item]       # View details
spectr validate --strict # Is it correct?
spectr archive <change-id> [--yes|-y]  # Mark complete (add --yes for automation)
```

Remember: Specs are truth. Changes are proposals. Keep them in sync.

<!-- spectr:END -->

<project>
conclaude is a Rust-based guardrail CLI for Claude Code sessions, orchestrating configurable lifecycle hooks to enforce linting, testing, and workflow policies. It discovers human-readable YAML via cosmiconfig, validates configurations against a published schema, and runs project commands to keep repos consistent. The tool ships with session-aware logging, file protection rules, and multiple installation channels including shell scripts, npm, and platform-specific binaries.
</project>
