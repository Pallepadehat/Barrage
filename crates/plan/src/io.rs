// SPDX-License-Identifier: MIT

//! Loading and saving plan files.

use std::io::Write;
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
/// order, and settings left at their defaults are omitted. The file is
/// replaced atomically.
pub fn save(path: impl AsRef<Path>, plan: &Plan) -> Result<(), PlanError> {
    let path = path.as_ref();
    let text = Format::from_path(path)?.to_string(plan)?;
    write_atomically(path, text.as_bytes()).map_err(|source| PlanError::Io {
        path: path.to_path_buf(),
        source,
    })
}

/// Writes to a temporary file next to `path`, then renames it over `path`,
/// so a crash mid-write never leaves a truncated plan.
fn write_atomically(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    let mut tmp_name = path.file_name().unwrap_or_default().to_os_string();
    tmp_name.push(format!(".{}.tmp", std::process::id()));
    let tmp = path.with_file_name(tmp_name);
    let result = std::fs::File::create(&tmp)
        .and_then(|mut file| {
            file.write_all(bytes)?;
            file.sync_all()
        })
        .and_then(|()| std::fs::rename(&tmp, path));
    if result.is_err() {
        // Best effort: the original error is what matters.
        let _ = std::fs::remove_file(&tmp);
    }
    result
}
