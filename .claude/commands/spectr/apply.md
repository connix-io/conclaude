---
name: Spectr: Apply
description: Implement an approved Spectr change and keep tasks in sync.
category: Spectr
tags: [spectr, apply]
---
<!-- spectr:START -->
**Guardrails**
- Favor straightforward, minimal implementations first and add complexity only when it is requested or clearly required.
- Keep changes tightly scoped to the requested outcome.
- Refer to `spectr/AGENTS.md` (located inside the `spectr/` directory—run `ls spectr` or `spectr update` if you don't see it) if you need additional Spectr conventions or clarifications.

**Steps**
Track these steps as TODOs and complete them one by one.
1. Read `spectr/changes/<id>/proposal.md`, `design.md` (if present), and `tasks.md` to confirm scope and acceptance criteria.
2. Work through tasks sequentially, keeping edits minimal and focused on the requested change.
3. Confirm completion before updating statuses—make sure every item in `tasks.md` is finished.
4. Update the checklist after all work is done so each task is marked `- [x]` and reflects reality.
5. Reference `spectr list` or `spectr show <item>` when additional context is required.

**Reference**
- Use `spectr show <id> --json --deltas-only` if you need additional context from the proposal while implementing.

<!-- spectr:END -->
