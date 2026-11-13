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
    @echo "just dev: not yet implemented (lands with M0-08)"

# The gate: formatting, lints, typecheck and all tests
check: check-rust check-ts

# Rust half of the gate
[private]
check-rust:
    cargo fmt --all --check
    cargo clippy --workspace --all-targets --locked -- -D warnings
    cargo test --workspace --locked

# TypeScript half of the gate
[private]
check-ts:
    pnpm biome ci .
    pnpm -r typecheck
    pnpm vitest run

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
    @echo "just types: not yet implemented (lands with M0-06)"

# Engine throughput benchmark
bench:
    @echo "just bench: not yet implemented (lands with M1)"
