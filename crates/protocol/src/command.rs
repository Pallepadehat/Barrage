// SPDX-License-Identifier: MIT

//! Commands: app → sidecar, on stdin.

use serde::{Deserialize, Serialize};

use crate::{PROTOCOL_VERSION, RequestId};

/// One line on the sidecar's stdin: a command plus its envelope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct CommandMessage {
    /// Protocol version; see [`PROTOCOL_VERSION`].
    pub v: u32,
    /// Request id, echoed by the events that answer this command.
    pub id: RequestId,
    /// The command itself, flattened into the envelope and tagged by `type`.
    #[serde(flatten)]
    pub command: Command,
}

impl CommandMessage {
    /// Wraps `command` in an envelope for the current protocol version.
    pub fn new(id: RequestId, command: Command) -> Self {
        Self {
            v: PROTOCOL_VERSION,
            id,
            command,
        }
    }
}

/// What the app asks the sidecar to do, tagged by `type` on the wire.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Command {
    /// Handshake. The sidecar answers with [`crate::Event::Ready`].
    Hello,
    /// Stop and exit with status 0. No event answers it.
    Shutdown,
}
