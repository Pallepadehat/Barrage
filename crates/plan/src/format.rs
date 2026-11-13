// SPDX-License-Identifier: MIT

//! Plan text formats: YAML and JSON, in memory.

use std::path::Path;

use serde_saphyr::SerializerOptions;

use crate::error::{Location, PlanError};
use crate::model::Plan;

/// A plan file format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Format {
    /// YAML (`.barrage.yaml`), the default for hand-editing.
    Yaml,
    /// JSON (`.barrage.json`).
    Json,
}

impl Format {
    /// Picks the format from a file's extension: `.yaml`/`.yml` or `.json`.
    pub fn from_path(path: &Path) -> Result<Self, PlanError> {
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        match ext.to_ascii_lowercase().as_str() {
            "yaml" | "yml" => Ok(Self::Yaml),
            "json" => Ok(Self::Json),
            _ => Err(PlanError::UnsupportedExtension {
                path: path.to_path_buf(),
            }),
        }
    }

    /// Parses a plan from text in this format.
    pub fn parse(self, text: &str) -> Result<Plan, PlanError> {
        match self {
            Self::Yaml => serde_saphyr::from_str(text).map_err(|e| {
                let message = e.without_snippet().to_string();
                match e.location() {
                    Some(l) => {
                        let suffix = format!(" at line {}, column {}", l.line(), l.column());
                        parse_error(Some((l.line(), l.column())), &message, &suffix)
                    }
                    None => parse_error(None, &message, ""),
                }
            }),
            Self::Json => serde_json::from_str(text).map_err(|e| {
                let (line, column) = (e.line() as u64, e.column() as u64);
                let suffix = format!(" at line {line} column {column}");
                // serde_json reports line 0 when it has no position.
                let position = (line > 0).then_some((line, column));
                parse_error(position, &e.to_string(), &suffix)
            }),
        }
    }

    /// Writes a plan as text in this format, ending with a newline.
    pub fn to_string(self, plan: &Plan) -> Result<String, PlanError> {
        let mut text = match self {
            Self::Yaml => serde_saphyr::to_string_with_options(plan, yaml_options())
                .map_err(|e| serialize_error(&e))?,
            Self::Json => serde_json::to_string_pretty(plan).map_err(|e| serialize_error(&e))?,
        };
        if !text.ends_with('\n') {
            text.push('\n');
        }
        Ok(text)
    }
}

/// The YAML style plans are saved in: two-space indent, list items indented
/// under their key, multi-line strings as literal blocks, no line folding.
fn yaml_options() -> SerializerOptions {
    serde_saphyr::ser_options! {
        indent_step: 2,
        compact_list_indent: false,
        folded_wrap_chars: usize::MAX,
    }
}

/// Builds a parse error, dropping the parser's own location suffix from the
/// message because [`PlanError`]'s `Display` prints the location up front.
fn parse_error(position: Option<(u64, u64)>, message: &str, suffix: &str) -> PlanError {
    let message = message.strip_suffix(suffix).unwrap_or(message);
    PlanError::Parse {
        path: None,
        location: Location {
            line: position.map(|(line, _)| line),
            column: position.map(|(_, column)| column),
        },
        message: message.to_owned(),
    }
}

fn serialize_error(err: &dyn std::fmt::Display) -> PlanError {
    PlanError::Serialize {
        message: err.to_string(),
    }
}
