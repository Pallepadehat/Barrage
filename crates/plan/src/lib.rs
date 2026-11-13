// SPDX-License-Identifier: MIT

//! Barrage test plan model.
//!
//! Defines the plan types (plan, thread groups, steps, assertions, extractors),
//! reads and writes them as YAML (`.barrage.yaml`) or JSON (`.barrage.json`),
//! and validates them before a run.

mod error;
mod format;
mod io;
mod model;

pub use error::{Location, PlanError};
pub use format::Format;
pub use io::{load, save};
pub use model::*;
