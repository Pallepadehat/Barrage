# SPDX-License-Identifier: MIT
#
# Task runner for Barrage. `just` lists recipes; `just check` is the gate.

set shell := ["bash", "-euo", "pipefail", "-c"]

# List all recipes
default:
    @just --list --unsorted

# Install JS deps and Rust components
setup:
    pnpm install --frozen-lockfile
    rustup toolchain install
    rustup component add rustfmt clippy

# Run the desktop app with hot reload
dev:
    pnpm --filter @barrage/desktop tauri dev

# Where `just types` writes the generated TS types
types_dir := "packages/types/src/generated"

# The gate: formatting, lints, typecheck, all tests and up-to-date generated types
check: check-rust check-types check-ts

# Rust half of the gate
[private]
check-rust:
    cargo fmt --all --check
    cargo clippy --workspace --all-targets --locked -- -D warnings
    # The workspace build unifies features, so also lint the library crates without `ts`.
    cargo clippy -p barrage-plan -p barrage-protocol --all-targets --locked -- -D warnings
    cargo test --workspace --locked

# Fails if packages/types is stale (run `just types`)
[private]
check-types:
    cargo run --quiet --locked -p barrage-typegen -- check {{types_dir}}

# TypeScript half of the gate
[private]
check-ts:
    pnpm biome ci .
    pnpm -r typecheck
    pnpm vitest run
    pnpm --filter @barrage/ui contrast

# Format Rust and TS
fmt:
    cargo fmt --all
    pnpm biome check --write .

# Rust + TS tests
test:
    cargo test --workspace --locked
    pnpm vitest run

# Regenerate packages/types from Rust (ts-rs)
types:
    cargo run --quiet --locked -p barrage-typegen -- write {{types_dir}}

# Engine throughput benchmark
bench:
    @echo "just bench: not yet implemented (lands with M1)"
