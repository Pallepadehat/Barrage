# Roadmap and task list

This is the working plan agents pick tasks from. The PRD says **what**; this file says **in what order** and **when a piece is done**.

**Status values:** `todo` · `in progress` · `blocked` · `done`. Update the status in the same PR that does the work. Put the PR link in the Notes column once it exists.

**Rules for tasks**

- A task is sized for one PR (roughly ≤ 600 changed lines, excluding generated files and lockfiles).
- Work tasks in dependency order. A task whose dependencies aren't `done` is not startable.
- Acceptance criteria are checkable. "Done" means `just check` passes and every criterion is met.
- Later milestones are deliberately coarse. Break a milestone into numbered tasks (in a docs-only PR) before anyone starts it.

---

## M0 — Foundation

Goal: an empty-but-real monorepo where every crate and package builds, `just check` is green locally and in CI, the app opens a window, and the app ↔ sidecar pipe round-trips one message. No product features yet.

| ID    | Task                                         | Depends on          | Status | Notes |
| ----- | -------------------------------------------- | ------------------- | ------ | ----- |
| M0-01 | Cargo workspace + crate stubs                | —                   | done   |       |
| M0-02 | pnpm workspace + Biome + TS config           | —                   | done   |       |
| M0-03 | justfile with the gate                       | M0-01, M0-02        | done   |       |
| M0-04 | CI workflow                                  | M0-03               | done   |       |
| M0-05 | Plan model v0 + YAML/JSON I/O                | M0-01               | done   |       |
| M0-06 | ts-rs type generation + staleness check      | M0-03, M0-05        | todo   |       |
| M0-07 | Protocol v0 + sidecar echo                   | M0-05               | todo   |       |
| M0-08 | Tauri app shell + window chrome              | M0-02               | todo   |       |
| M0-09 | Design tokens + shadcn base in `packages/ui` | M0-02               | todo   |       |
| M0-10 | App ↔ sidecar wiring through `lib/ipc.ts`    | M0-06, M0-07, M0-08 | todo   |       |
| M0-11 | Contributor docs (README, CONTRIBUTING)      | M0-03               | todo   |       |

### M0-01 — Cargo workspace + crate stubs

- Root `Cargo.toml` workspace with members `crates/{plan,engine,protocol,sidecar,cli}` (desktop `src-tauri` joins in M0-08), `resolver = "3"`, shared `[workspace.package]` (edition 2024, license `MIT`, rust-version) and `[workspace.dependencies]`.
- `[workspace.lints]`: `unsafe_code = "forbid"`, clippy `unwrap_used`, `expect_used`, `panic` = deny (allowed in tests), `missing_docs = "warn"` for libraries.
- Each lib crate has a `lib.rs` with a crate doc comment; `sidecar` and `cli` have a `main.rs` that prints the version.
- **Acceptance:** `cargo build --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --check` and `cargo test --workspace` pass.

### M0-02 — pnpm workspace + Biome + TS config

- Root `package.json` (private, `packageManager` pinned), `pnpm-workspace.yaml` covering `apps/*` and `packages/*`.
- Root `biome.json` (formatter + linter, `noExplicitAny` error) and `tsconfig.base.json` (`strict`, `noUncheckedIndexedAccess`, `verbatimModuleSyntax`).
- Empty `packages/ui` and `packages/types` with their own `package.json` and `tsconfig.json`.
- **Acceptance:** `pnpm install`, `pnpm biome ci .` and `pnpm -r typecheck` pass.

### M0-03 — justfile with the gate

- Recipes listed in `AGENTS.md`: `setup`, `dev`, `check`, `fmt`, `test`, `types`, `bench` (stubs may print "not yet implemented" and exit 0 until their task lands, except `check`, which must be real).
- `just check` runs: `cargo fmt --check`, clippy `-D warnings`, `cargo test`, `biome ci`, TS typecheck, Vitest.
- **Acceptance:** `just check` passes on a clean clone after `just setup`.

### M0-04 — CI workflow

- `.github/workflows/ci.yml` on PR and push to `main` and `develop`: one job on `macos-15` (`macos-14` runners are retired) running `just setup && just check`, with Rust and pnpm caches.
- A separate job building the desktop app (`tauri build --debug` or equivalent) once M0-08 lands; until then it's absent, not failing.
- **Acceptance:** CI is green on the PR that adds it.

### M0-05 — Plan model v0 + YAML/JSON I/O

- `barrage-plan` types for the plan in the PRD example: `Plan`, `Variable`, `HttpDefaults`, `CsvDataSet`, `ThreadGroup`, `Step` (tagged enum: `http`, `pause`), `HttpRequest`, `Assertion` and `Extractor` (tagged enums). camelCase on the wire; stable field order.
- `load(path)` / `save(path, plan)` choose YAML or JSON from the extension (`.barrage.yaml`, `.barrage.json`); errors via `thiserror` with file and line where available.
- `examples/checkout.barrage.yaml` (the PRD example) and its JSON twin.
- **Acceptance:** round-trip tests: YAML → model → YAML is byte-identical for the example; YAML → JSON → YAML is lossless; unknown fields are rejected with a clear error.

### M0-06 — ts-rs type generation + staleness check

- `ts-rs` derives on plan (and later protocol) types, exported to `packages/types/src/generated/` with an `index.ts` barrel.
- `just types` regenerates; `just check` fails if regenerating produces a diff.
- **Acceptance:** generated types typecheck; changing a Rust field without `just types` makes `just check` fail.

### M0-07 — Protocol v0 + sidecar echo

- `barrage-protocol`: `Command` (stdin) and `Event` (stdout) enums as newline-delimited JSON, each with a `v` protocol version and a request `id`. v0 commands: `Hello`, `Shutdown`. v0 events: `Ready { version }`, `Error { message }`.
- `sidecar` binary: reads commands line by line on a tokio runtime, writes events, logs to stderr only, exits cleanly on `Shutdown` or stdin EOF.
- **Acceptance:** an integration test spawns the binary, sends `Hello`, gets `Ready`, sends `Shutdown`, and sees exit code 0. Malformed input yields an `Error` event, not a crash.

### M0-08 — Tauri app shell + window chrome

- `apps/desktop` via Tauri 2 + React 19 + Vite + TS; `src-tauri` joins the Cargo workspace.
- Window config from `docs/DESIGN.md` §8: overlay title bar, hidden title, traffic lights at `{16, 20}`, min size 960 × 640, sidebar vibrancy.
- Placeholder three-pane layout (sidebar 248 px, editor, collapsed results drawer) and a 52 px drag-region toolbar.
- **Acceptance:** `just dev` opens the window on macOS 14+; light and dark both render; cold start < 1 s on Apple silicon (measured and noted in the PR).

### M0-09 — Design tokens + shadcn base in `packages/ui`

- `tokens.css` with every token from `docs/DESIGN.md` §2 (colour light/dark, fonts, radius, motion), wired into Tailwind v4 `@theme`.
- shadcn CLI initialised in `packages/ui` with the neutral base; add `Button` (with `brand` variant and `sm/default/lg` sizes), `Kbd`, `Tooltip`.
- A token contrast script in `packages/ui/scripts` that checks WCAG AA for text tokens in both themes.
- **Acceptance:** the desktop app renders a Run button and a Kbd from `packages/ui` in both themes; contrast script passes and runs in `just check`.

### M0-10 — App ↔ sidecar wiring through `lib/ipc.ts`

- `src-tauri` spawns the sidecar via Tauri's sidecar support, forwards stdout events to the webview, and sends commands from the webview to stdin. Engine stderr goes to the app log.
- `apps/desktop/src/lib/ipc.ts`: the only module that touches Tauri, typed with `packages/types`, with an in-memory mock used in tests and the browser preview.
- **Acceptance:** on launch the toolbar shows the engine version from `Ready`; killing the sidecar shows an error state without freezing the UI; a Vitest test drives the UI through the mock.

### M0-11 — Contributor docs

- `README.md` (what, status, quick start), `CONTRIBUTING.md` (setup, `just` recipes, architecture tour, PR checklist, Conventional Commits).
- **Acceptance:** a fresh clone reaches a running app by following README alone.

---

## M1 — Engine core (coarse)

Plan validation and compile step · variable substitution and built-in functions · virtual-user loop on tokio with ramp-up, duration/iteration stop, think time + jitter · HTTP via reqwest/rustls with per-user cookie jars · assertions (status, body, JSONPath, response time) · extractors (JSONPath, regex, header) · CSV data sets · HdrHistogram aggregation and 500 ms snapshots · cancellation within 1 s · in-process mock server and integration tests · `barrage run` CLI with live line and summary table · throughput benchmark in CI.

## M2 — Desktop editors (coarse)

Plan store (Zustand + Immer + zundo) · plan tree with keyboard nav, drag reorder, context menu · plan, thread group and request editors · `KeyValueEditor`, `VariableInput`, `CodeEditor` · Send once with inline results · file new/open/save/recent with native dialogs · `lib/shortcuts.ts` feeding native menu, command palette and tooltips · welcome and empty states.

## M3 — Live results (coarse)

Run/Stop wired to the sidecar · dashboard KPI tiles and uPlot charts · summary table · results tree with `SampleList`/`SampleDetail` · saturation warning · errors breakdown (P1) · export (P1).

## M4 — Polish and release (coarse)

Accessibility pass (VoiceOver, keyboard-only, contrast) · performance targets from the PRD verified · CLI thresholds (P1) · signing, notarisation, release-please, tauri-action, cargo-dist, Homebrew · open-source readiness checklist from the PRD.
