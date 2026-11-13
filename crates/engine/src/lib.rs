// SPDX-License-Identifier: MIT

//! The Barrage load engine.
//!
//! Compiles a validated plan, runs one tokio task per virtual user, applies
//! assertions and extractors, and aggregates samples into periodic snapshots.
//! Shared by the desktop sidecar and the CLI so a plan behaves the same in both.
