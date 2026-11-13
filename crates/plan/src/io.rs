// SPDX-License-Identifier: MIT

//! Loading and saving plan files.

use std::path::Path;

use crate::error::PlanError;
use crate::format::Format;
use crate::model::Plan;

/// Reads a plan from a `.barrage.yaml` or `.barrage.json` file.
///
/// The format comes from the file extension. Parse errors carry the file
/// and, where the parser knows it, the line and column.
pub fn load(path: impl AsRef<Path>) -> Result<Plan, PlanError> {
    let path = path.as_ref();
    let format = Format::from_path(path)?;
    let text = std::fs::read_to_string(path).map_err(|source| PlanError::Io {
        path: path.to_path_buf(),
        source,
    })?;
    format
        .parse(&text)
        .map_err(|e| e.with_path(path.to_path_buf()))
}

/// Writes a plan to a `.barrage.yaml` or `.barrage.json` file.
///
/// The format comes from the file extension. Keys are written in a stable
/// order, and settings left at their defaults are omitted.
pub fn save(path: impl AsRef<Path>, plan: &Plan) -> Result<(), PlanError> {
    let path = path.as_ref();
    let text = Format::from_path(path)?.to_string(plan)?;
    std::fs::write(path, text).map_err(|source| PlanError::Io {
        path: path.to_path_buf(),
        source,
    })
}
