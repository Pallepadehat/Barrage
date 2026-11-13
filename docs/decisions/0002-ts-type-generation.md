# 0002 — TS type generation: ts-rs behind a feature, run by `barrage-typegen`

- **Status:** Accepted
- **Date:** 2026-11-13

## Context

The PRD makes Rust the single source of truth for plan and protocol types and generates `packages/types` with ts-rs; CI must fail when the generated files are stale (M0-06). It leaves open how generation runs and how Rust types map to TypeScript where the defaults don't match the wire format.

## Decision

- Library crates derive `ts_rs::TS` only behind a `ts` cargo feature (`#[cfg_attr(feature = "ts", derive(ts_rs::TS))]`), so per-package and release builds of the engine, sidecar and CLI don't compile ts-rs. Workspace builds unify features and do, so `just check` also lints `barrage-plan` on its own with `ts` off.
- A dev-only binary, `crates/typegen` (`publish = false`), enables those features and exports every type reachable from the root types (`Plan` today; protocol `Command`/`Event` join in M0-07) into `packages/types/src/generated/`, plus an `index.ts` barrel. `just types` runs `barrage-typegen write`; `just check` runs `barrage-typegen check`, which compares a fresh generation to the directory without touching it.
- Mapping rules: 64-bit integers are `number` (plan values are milliseconds and counts, far below 2^53, and `serde_json` writes them as numbers). `Option` fields that are skipped when `None` get field-level `#[ts(optional)]`, so they are `field?: T` and never `null`: the plan reader rejects `null` for text fields, and the writer never emits it.

## Consequences

- Adding a root type means adding one `export_all` call in `crates/typegen/src/generate.rs`; its dependencies follow automatically.
- Every new `Option` field on a boundary type needs `#[cfg_attr(feature = "ts", ts(optional))]`. A generated `null` fails the typegen unit test, so a missed one is caught.
- The staleness check needs no git and works on uncommitted changes.

## Alternatives considered

- **`#[ts(export)]` test hooks** (ts-rs default): generation becomes a side effect of `cargo test`, which then writes into the source tree during the gate; rejected.
- **Struct-level `#[ts(optional_fields)]`**: ts-rs 12 then ignores serde `default` on non-`Option` fields and marks them required (e.g. `enabled`); rejected.
- **`bigint` for `u64`**: matches the Rust range but not the JSON the sidecar sends; rejected.
