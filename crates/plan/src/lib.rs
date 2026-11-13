// SPDX-License-Identifier: MIT

//! Barrage test plan model.
//!
//! Defines the plan types (plan, thread groups, steps, assertions, extractors),
//! and reads and writes them as YAML (`.barrage.yaml`) or JSON (`.barrage.json`).
//! Validation before a run lands with the engine (M1).

mod error;
mod format;
mod io;
mod model;

pub use error::{Location, PlanError};
pub use format::Format;
pub use io::{load, save};
pub use model::*;
