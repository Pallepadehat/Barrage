# Contributing to Barrage

Thanks for helping. This guide gets you from a clone to a merged PR. The rules for code live in [`AGENTS.md`](AGENTS.md) (it's written for people and coding agents alike); this file doesn't repeat them, it points to them.

## Setup

Follow the [quick start in the README](README.md#quick-start): prerequisites, `just setup`, then `just dev`. Toolchain versions are pinned in the repo:

| Tool | Pinned in                                  |
| ---- | ------------------------------------------ |
| Rust | [`rust-toolchain.toml`](rust-toolchain.toml) (stable, with rustfmt and clippy) |
| Node | [`.node-version`](.node-version)           |
| pnpm | `packageManager` in [`package.json`](package.json) |

Editor: `.vscode/extensions.json` recommends rust-analyzer, Biome, Tauri, Tailwind and just extensions.

## Commands

Everything common is a [`just`](https://just.systems) recipe. Run `just` to list them.

| Command      | Does                                                                                 |
| ------------ | ------------------------------------------------------------------------------------ |
| `just setup` | Install JS deps and the pinned Rust toolchain                                        |
| `just dev`   | Build the engine sidecar, then run the desktop app with hot reload                   |
| `just check` | **The gate.** fmt check, clippy, Biome, typecheck, all tests, generated types up to date, contrast check |
| `just fmt`   | Format Rust and TS                                                                   |
| `just test`  | Rust + TS tests                                                                      |
| `just types` | Regenerate `packages/types` from Rust (ts-rs)                                        |
| `just bench` | Engine throughput benchmark (a stub until M1)                                        |

CI runs `just setup && just check` on macOS, plus a debug build of the desktop app. If `just check` passes locally, CI's gate should too.

Two handy extras:

- `pnpm --filter @barrage/desktop dev` serves the UI alone at <http://localhost:1420> in a browser. Outside Tauri, `lib/ipc.ts` falls back to an in-memory mock engine, so you can work on UI without the Rust side.
- `cargo run -p barrage-sidecar` starts the engine on its own. Type `{"v":0,"id":1,"type":"hello"}` and it answers with `ready`.

## Architecture tour

```text
apps/desktop/          Tauri 2 app
  src/                   React UI: app/ (shell), features/* (one folder per feature), lib/ (ipc, shortcuts)
  src-tauri/             Rust shell: owns the window and spawns the engine sidecar
  scripts/sidecar.sh     builds the sidecar and puts it where Tauri bundles it
crates/
  plan/                barrage-plan: plan model, YAML/JSON I/O, validation
  engine/              barrage-engine: runner, assertions, extractors, metrics (M1)
  protocol/            barrage-protocol: NDJSON messages between app and sidecar
  sidecar/             the engine process the app spawns
  cli/                 `barrage` command-line tool (M1)
  typegen/             dev tool behind `just types`
packages/
  ui/                  shadcn/ui components + tokens.css (no app logic)
  types/               TS types generated from Rust — never edit by hand
examples/              sample plans (.barrage.yaml / .barrage.json)
docs/                  PRD, DESIGN, ROADMAP, plan format, ADRs
```

How the pieces fit:

- **The engine runs in its own process.** The desktop app spawns `barrage-sidecar` and talks to it over stdin/stdout, one JSON message per line (`barrage-protocol`). A crashed or saturated engine never freezes the UI. The CLI links the same engine, so a plan behaves the same on a laptop and in CI.
- **Dependencies point one way:** `plan` ← `engine` ← `sidecar`/`cli`, and `protocol` depends on `plan` only. UI code never imports engine internals.
- **Types are defined once, in Rust.** Plan and protocol types derive `ts-rs`; `just types` writes them into `packages/types`, and CI fails if they're stale ([ADR 0002](docs/decisions/0002-ts-type-generation.md)).
- **One door to Tauri.** UI components never call Tauri directly; everything goes through `apps/desktop/src/lib/ipc.ts`.
- **The UI sees aggregates, not samples.** The engine sends a snapshot every 500 ms, so UI cost stays flat at any request rate.

For the reasoning behind these choices, read the Architecture section of [`docs/PRD.md`](docs/PRD.md) and the [ADRs](docs/decisions/).

## Picking work

Work comes from [`docs/ROADMAP.md`](docs/ROADMAP.md). Each task has an ID (e.g. `M1-03`), dependencies and acceptance criteria. Pick a `todo` task whose dependencies are `done`, set it to `in progress`, and work on a branch named `<id>-<slug>` (e.g. `m1-03-status-specs`). One task per branch and PR.

Bugs and ideas outside the roadmap: open an issue first. Features outside the PRD's MVP scope (distributed load, non-HTTP protocols, JMX import, scripting) won't be merged without a decision in the PRD or an [ADR](docs/decisions/README.md).

## Commits

[Conventional Commits](https://www.conventionalcommits.org): `type(scope): summary`.

- **Types:** `feat`, `fix`, `refactor`, `perf`, `test`, `docs`, `chore`, `ci`.
- **Scopes:** `plan`, `engine`, `protocol`, `sidecar`, `cli`, `desktop`, `ui`, `types`, `ci`, `docs`.
- Examples: `feat(engine): parse status specs like 2xx`, `fix(desktop): keep the toolbar visible when the engine exits`, `docs: start M1-03`.

Split a task into a few logical commits if it helps review.

## Pull requests

PRs target `develop`. Keep them reviewable: past ~600 changed lines (generated files excluded), split the task. The [PR template](.github/pull_request_template.md) carries this checklist:

- [ ] `just check` passes
- [ ] Roadmap status updated in `docs/ROADMAP.md`
- [ ] Tests added or updated next to the code (engine logic always gets unit tests)
- [ ] `just types` run if Rust types crossing the boundary changed, output committed
- [ ] New dependencies justified, one line each
- [ ] UI only: meets [`docs/DESIGN.md`](docs/DESIGN.md) §12, with light and dark screenshots
- [ ] New source files start with `// SPDX-License-Identifier: MIT`

If code and a doc disagree, fix whichever is wrong in the same PR. Don't change decisions in `docs/PRD.md` or `docs/DESIGN.md` on your own; propose them in the PR description or an ADR.

## Testing against servers

Never point a load test at a host you don't own. Tests and benchmarks use the in-process mock server or `localhost`, and example plans use `https://api.example.com`.

## License

By contributing you agree that your contributions are licensed under the [MIT License](LICENSE).
