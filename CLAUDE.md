@AGENTS.md

## Claude Code specifics

- Project subagents live in `.claude/agents/`: `design-reviewer` (UI changes vs `docs/DESIGN.md`) and `rust-reviewer` (Rust changes vs the code standards above). Run the matching one before opening a PR.
- `/next-task` (`.claude/skills/next-task`) runs the full loop: pick the next task from `docs/ROADMAP.md`, implement, verify, update the roadmap, commit.
- A PostToolUse hook (`.claude/hooks/format.sh`) formats edited `.rs`/`.ts`/`.tsx` files when the formatter is installed. Don't fight it; don't reformat by hand.
