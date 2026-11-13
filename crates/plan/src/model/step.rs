// SPDX-License-Identifier: MIT

//! Steps: the things a virtual user does in each iteration.

use serde::{Deserialize, Serialize};

use super::http::HttpRequest;
use super::serde_util::{default_true, is_true};

/// One step in a thread group, tagged by `type` on the wire.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase", deny_unknown_fields)]
pub enum Step {
    /// Send an HTTP request.
    Http(HttpRequest),
    /// Wait for a fixed time.
    Pause(Pause),
}

/// A fixed delay between requests.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Pause {
    /// Display name.
    pub name: String,
    /// Whether the step runs. Disabled steps stay in the file.
    #[serde(default = "default_true", skip_serializing_if = "is_true")]
    pub enabled: bool,
    /// How long to wait, in milliseconds.
    pub duration_ms: u64,
}
