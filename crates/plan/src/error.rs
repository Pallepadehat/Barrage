// SPDX-License-Identifier: MIT

//! Errors from reading and writing plan files.

use std::fmt;
use std::path::PathBuf;

/// An error from loading, parsing or saving a plan.
#[derive(Debug, thiserror::Error)]
pub enum PlanError {
    /// The file name ends in neither `.yaml`/`.yml` nor `.json`.
    #[error("{}: unsupported plan file extension (expected .barrage.yaml or .barrage.json)", path.display())]
    UnsupportedExtension {
        /// The file that was given.
        path: PathBuf,
    },
    /// The file couldn't be read or written.
    #[error("{}: {source}", path.display())]
    Io {
        /// The file that failed.
        path: PathBuf,
        /// The underlying I/O error.
        source: std::io::Error,
    },
    /// The text isn't a valid plan: bad syntax, a wrong type, or an unknown field.
    #[error("{}{location}: {message}", display_path(path.as_ref()))]
    Parse {
        /// The file being parsed, when parsing came from a file.
        path: Option<PathBuf>,
        /// Where in the text the error is, when the parser knows.
        location: Location,
        /// What went wrong.
        message: String,
    },
    /// The plan couldn't be written as text.
    #[error("couldn't serialize plan: {message}")]
    Serialize {
        /// What went wrong.
        message: String,
    },
}

impl PlanError {
    /// Attaches the file a parse error came from.
    pub(crate) fn with_path(self, file: PathBuf) -> Self {
        match self {
            Self::Parse {
                location, message, ..
            } => Self::Parse {
                path: Some(file),
                location,
                message,
            },
            other => other,
        }
    }
}

/// A 1-based line and column in a plan file, if known.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Location {
    /// 1-based line number.
    pub line: Option<u64>,
    /// 1-based column number.
    pub column: Option<u64>,
}

impl fmt::Display for Location {
    /// Formats as `:line:column`, `:line`, or nothing.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.line, self.column) {
            (Some(line), Some(column)) => write!(f, ":{line}:{column}"),
            (Some(line), None) => write!(f, ":{line}"),
            _ => Ok(()),
        }
    }
}

fn display_path(path: Option<&PathBuf>) -> String {
    path.map_or_else(|| "<plan>".to_owned(), |p| p.display().to_string())
}
