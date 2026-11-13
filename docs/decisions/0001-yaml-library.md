# 0001 — YAML library: serde-saphyr

- **Status:** Accepted
- **Date:** 2026-11-13

## Context

Plans are saved as YAML (the default) or JSON (PRD, Files and CLI). The PRD stack table names no YAML crate. `serde_yaml`, the usual choice, is archived and unmaintained. The plan crate needs a serde-compatible YAML reader and writer that never panics on bad input, reports line and column on errors, and writes stable block-style output so YAML → model → YAML is byte-identical.

## Decision

Use [`serde-saphyr`](https://crates.io/crates/serde-saphyr) 2.x for YAML in `barrage-plan`. Plans are written with a two-space indent, list items indented under their key, multi-line strings as literal blocks (`|`), and no line folding. To get 2.x, the workspace `rust-version` moves from 1.88 to 1.89.

## Consequences

- Pure Rust, no `unsafe-libyaml`; parsing is designed to be panic-free and has resource budgets against hostile input.
- Errors carry a location and a readable message; `barrage-plan` turns them into `file:line:column: message`.
- The writer has no flow style (`{ key: value }`), so checked-in example plans use block style. Hand-written flow style still loads fine; it's rewritten as block style on save.
- Errors inside internally tagged enums (steps, assertions, extractors) point at the start of the item, not the exact field.

## Alternatives considered

- **serde_yaml** — archived; no fixes.
- **serde_yaml_ng / serde_norway** — maintained forks of serde_yaml, but still built on `unsafe-libyaml`.
- **serde_yml** — fork with a history of unsound code; avoided.
- **serde-saphyr 1.x** — would keep MSRV 1.88, but is the older line; raising MSRV by one release costs nothing for a new project.
