---
name: Spectr: Archive
description: Archive a deployed Spectr change and update specs.
category: Spectr
tags: [spectr, archive]
---
<!-- spectr:START -->
**Guardrails**
- Favor straightforward, minimal implementations first and add complexity only when it is requested or clearly required.
- Keep changes tightly scoped to the requested outcome.
- Refer to `spectr/AGENTS.md` (located inside the `spectr/` directory—run `ls spectr` or `spectr update` if you don't see it) if you need additional Spectr conventions or clarifications.

**Steps**
1. Determine the change ID to archive:
   - If this prompt already includes a specific change ID (for example inside a `<ChangeId>` block populated by slash-command arguments), use that value after trimming whitespace.
   - If the conversation references a change loosely (for example by title or summary), run `spectr list` to surface likely IDs, share the relevant candidates, and confirm which one the user intends.
   - Otherwise, review the conversation, run `spectr list`, and ask the user which change to archive; wait for a confirmed change ID before proceeding.
   - If you still cannot identify a single change ID, stop and tell the user you cannot archive anything yet.
2. Validate the change ID by running `spectr list` (or `spectr show <id>`) and stop if the change is missing, already archived, or otherwise not ready to archive.
3. Run `spectr archive <id> --yes` so the CLI moves the change and applies spec updates without prompts.
4. Review the command output to confirm the target specs were updated and the change landed in `spectr/changes/archive/`.
5. Validate with `spectr validate --strict` and inspect with `spectr show <id>` if anything looks off.

**Reference**
- Use `spectr list` to confirm change IDs before archiving.
- Inspect refreshed specs with `spectr list --specs` and address any validation issues before handing off.

<!-- spectr:END -->
