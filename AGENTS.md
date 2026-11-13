# Barrage — agent guide

Barrage is a native-feeling Mac app for building and running HTTP load tests: a JMeter-style visual test plan, a Rust load engine, and a Tauri 2 + React UI. A headless CLI runs the same plans in CI.

This file is the entry point for every coding agent (Claude Code reads it through `CLAUDE.md`). Keep it short and current. If something here is wrong, fix it in the same PR.

## Sources of truth

Read the relevant doc **before** you change code in its area. Don't restate them here; link to them.

| Doc                                              | Owns                                                                        |
| ------------------------------------------------ | --------------------------------------------------------------------------- |
| [`docs/PRD.md`](docs/PRD.md)                     | What we build and why: scope, priorities (P0/P1/P2), performance targets    |
| [`docs/DESIGN.md`](docs/DESIGN.md)               | How the UI looks and behaves: tokens, components, screens, shortcuts        |
| [`docs/ROADMAP.md`](docs/ROADMAP.md)             | Milestones and the task list agents work from, with status                  |
| [`docs/decisions/`](docs/decisions/)             | Architecture decision records (ADRs) for choices made after the PRD         |

If code and a doc disagree, one of them is a bug. Fix whichever is wrong, and never silently diverge. If a task needs a decision the docs don't cover, write an ADR (see `docs/decisions/README.md`) or ask.

## Status

The repo is pre-foundation: only docs and tooling exist. The layout and commands below are the **target** that milestone M0 in `docs/ROADMAP.md` builds. When a task creates something listed here, it's real; until then, don't assume it exists.

## Repository layout (target)

```text
apps/desktop/          Tauri 2 app: React UI (src/) + Rust shell (src-tauri/)
crates/plan/           barrage-plan: plan model, YAML/JSON I/O, validation
crates/engine/         barrage-engine: runner, assertions, extractors, metrics
crates/protocol/       barrage-protocol: sidecar stdio messages (NDJSON)
crates/sidecar/        engine binary the app spawns
crates/cli/            `barrage` command-line tool
packages/ui/           shadcn/ui components + tokens.css (no app logic)
packages/types/        TS types generated from Rust by ts-rs — never edit by hand
examples/              sample plans (.barrage.yaml / .barrage.json)
```

Dependency direction: `plan` ← `engine` ← `sidecar`/`cli`; `protocol` depends on `plan` only. The desktop app talks to the engine **only** through the sidecar over stdio. UI code never imports engine internals.

## Commands

Toolchain: Rust stable (see `rust-toolchain.toml`), Node (see `.node-version`), pnpm, just.

| Command          | Does                                                             |
| ---------------- | ---------------------------------------------------------------- |
| `just`           | List all recipes                                                 |
| `just setup`     | Install JS deps and Rust components                              |
| `just dev`       | Run the desktop app with hot reload                              |
| `just check`     | **The gate.** fmt check, clippy, biome, typecheck, all tests, types-up-to-date check |
| `just fmt`       | Format Rust and TS                                               |
| `just test`      | Rust + TS tests                                                  |
| `just types`     | Regenerate `packages/types` from Rust (ts-rs)                    |
| `just bench`     | Engine throughput benchmark                                      |

Before you say a change is done, run `just check` and make sure it passes. If a recipe doesn't exist yet, run the underlying `cargo`/`pnpm` commands directly and say so.

## How to work

1. **Pick work from `docs/ROADMAP.md`.** Each task has an ID (e.g. `M0-03`), deliverables and acceptance criteria. Mark it `in progress` when you start and `done` when it's merged. Use the `/next-task` skill for the full loop.
2. **One task per branch and PR.** Branch name `<id>-<slug>` (e.g. `m0-03-plan-model`). Keep diffs reviewable; split a task if it grows past ~600 changed lines (generated files excluded).
3. **Tests first where it's cheap.** Engine logic (substitution, status specs, assertions, extractors, CSV cycling, percentiles) always gets unit tests.
4. **Verify, then report honestly.** Run `just check`. Report what you ran and what failed. Never claim a test passes that you didn't run.
5. **Conventional Commits** (`feat(engine): …`, `fix(ui): …`, `chore: …`, `docs: …`). Scopes: `plan`, `engine`, `protocol`, `sidecar`, `cli`, `desktop`, `ui`, `types`, `ci`, `docs`.

## Code standards

**General**
- One concept per file, ~300 lines max. Prefer small modules over clever ones.
- No new dependency without a one-line reason in the PR description. Prefer the crates/packages named in the PRD stack table.
- SPDX header on every source file: `// SPDX-License-Identifier: MIT`.

**Rust**
- `rustfmt`; `clippy -D warnings` (workspace lints in the root `Cargo.toml`).
- `thiserror` in libraries, `anyhow` only in binaries (`sidecar`, `cli`, `src-tauri`).
- No `unwrap()` / `expect()` / `panic!` outside tests. A bad response (invalid UTF-8, huge body, broken connection) is a failed sample, never a panic.
- Doc comments on every public item.
- Hot path (per-sample code) must not allocate needlessly: intern labels, compile regex/JSONPath once per run, zero-alloc substitution when a string has no `${`.
- Latency is measured from send to last byte; the timer starts after the request is built. In paced mode, measure from the scheduled send time (coordinated omission).

**Types across the boundary**
- Plan and protocol types are defined once in Rust with `serde` + `ts-rs`. Run `just types` after changing them and commit the output. CI fails if `packages/types` is stale.
- Plan files keep a stable key order and round-trip YAML ⇄ JSON losslessly. Changing the plan format means updating `examples/` and `docs/plan-format.md`.

**TypeScript / UI**
- `strict`, no `any`, Biome for lint + format.
- Follow `docs/DESIGN.md` §11: feature folders under `apps/desktop/src/features/*` with an `index.ts` public surface; features don't import each other's internals.
- Colours, spacing, radius and motion come from tokens in `packages/ui/tokens.css` only. No raw hex/OKLCH in components.
- Numbers are formatted only through `lib/format.ts`. Shortcuts are registered only in `lib/shortcuts.ts`.
- Components never call Tauri directly — everything goes through `lib/ipc.ts`.
- Live data reaches charts via refs (`uPlot.setData`), not React state. Long lists are virtualised.
- UI changes meet the Definition of Done in `docs/DESIGN.md` §12. Use the `design-reviewer` subagent before opening a UI PR.

## Guardrails

- Don't hand-edit `packages/types/` or `LICENSE`. Don't change `docs/PRD.md` or `docs/DESIGN.md` decisions unless asked; propose the change in the PR description (or an ADR) instead. Fixing a doc that's factually out of date with the code is fine.
- Don't add features outside the PRD MVP scope or listed as non-goals (distributed load, non-HTTP protocols, JMX import, scripting) unless the task says so.
- Don't run load tests against hosts you don't own. Tests and benchmarks use the in-process mock server or `localhost`.
- Never commit secrets, `.env` files, or real customer URLs. Example plans use `https://api.example.com` or a local mock.
- Ask before destructive git operations (force push, history rewrites, deleting branches).
