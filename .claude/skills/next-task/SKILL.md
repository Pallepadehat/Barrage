---
name: next-task
description: Pick the next startable task from docs/ROADMAP.md (or the task ID given as an argument), implement it on its own branch, verify it with `just check`, update the roadmap, and commit. Use when the user says "next task", "work on M0-03", "continue the roadmap" or similar.
---

# Work one roadmap task end to end

Argument: optional task ID (e.g. `M0-05`). Without one, pick the lowest-numbered `todo` task in the current milestone whose dependencies are all `done`. If none is startable, say which task blocks progress and stop.

## 1. Understand

- Read `AGENTS.md`, the task's section in `docs/ROADMAP.md`, and the PRD/DESIGN sections it touches.
- If the acceptance criteria are ambiguous or conflict with the PRD/DESIGN, stop and ask, or propose an ADR in `docs/decisions/`. Don't guess on product decisions.

## 2. Branch

- Start from an up-to-date `main`: `git switch main && git switch -c <id>-<slug>` (lowercase, e.g. `m0-05-plan-model`).
- Set the task's status to `in progress` in `docs/ROADMAP.md`.

## 3. Implement

- Stay inside the task's deliverables. Note follow-ups you discover in your final report instead of doing them.
- Write tests alongside the code. Engine logic always gets unit tests.
- Keep files small (~300 lines), add SPDX headers and doc comments.
- If Rust types crossing the boundary changed, run `just types`.

## 4. Verify

- Run `just check` (or the underlying commands if the recipe doesn't exist yet). Fix until green.
- Check every acceptance criterion explicitly and note how you verified it (command, test name, or manual step).
- Run the `rust-reviewer` subagent if Rust changed, and the `design-reviewer` subagent if UI changed. Fix what they find or explain why not.

## 5. Finish

- Set the task's status to `done` in `docs/ROADMAP.md` (it lands with the merge). If `AGENTS.md` commands/layout became real or changed, update it.
- Commit with Conventional Commits, scoped to the area (`feat(plan): …`). Split into a few logical commits if it helps review.
- Don't push or open a PR unless the user asked.

## Report

Reply with: task ID and title, what changed (files/areas), each acceptance criterion with ✓/✗ and its evidence, the `just check` result, reviewer verdicts, and follow-ups.
