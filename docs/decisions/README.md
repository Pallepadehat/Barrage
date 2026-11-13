# Architecture decision records

Decisions made after the PRD, one file each: `NNNN-short-title.md` (e.g. `0001-sidecar-protocol-versioning.md`). Copy `0000-template.md`.

Write an ADR when a choice is hard to reverse, affects more than one crate/package, or a future contributor would ask "why is it like this?". Decisions already recorded in the PRD's **Decisions** table don't need one.

An ADR is never edited after it's accepted. To change a decision, write a new ADR that supersedes it and set the old one's status to `Superseded by NNNN`.

| ADR | Title | Status |
| --- | ----- | ------ |
| [0001](0001-yaml-library.md) | YAML library: serde-saphyr | Accepted |
| [0002](0002-ts-type-generation.md) | TS type generation: ts-rs behind a feature, run by `barrage-typegen` | Accepted |
| [0003](0003-light-brand-contrast.md) | Light-theme brand lightness for AA contrast | Accepted |
