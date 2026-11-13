// SPDX-License-Identifier: MIT

//! Events: sidecar → app, on stdout.

use serde::{Deserialize, Serialize};

use crate::{PROTOCOL_VERSION, RequestId};

/// One line on the sidecar's stdout: an event plus its envelope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct EventMessage {
    /// Protocol version; see [`PROTOCOL_VERSION`].
    pub v: u32,
    /// Id of the command this event answers. Absent when it answers none,
    /// e.g. an error for a line that isn't a command at all.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts", ts(optional))]
    pub id: Option<RequestId>,
    /// The event itself, flattened into the envelope and tagged by `type`.
    #[serde(flatten)]
    pub event: Event,
}

impl EventMessage {
    /// Wraps `event` in an envelope for the current protocol version.
    pub fn new(id: Option<RequestId>, event: Event) -> Self {
        Self {
            v: PROTOCOL_VERSION,
            id,
            event,
        }
    }
}

/// What the sidecar reports, tagged by `type` on the wire.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Event {
    /// Answer to [`crate::Command::Hello`]: the sidecar is ready for commands.
    Ready {
        /// Engine version, e.g. `0.1.0`.
        version: String,
    },
    /// A command could not be read or carried out. The sidecar keeps running.
    Error {
        /// Human-readable description, safe to show in the UI.
        message: String,
    },
}
