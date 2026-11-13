// SPDX-License-Identifier: MIT

//! Messages exchanged between the Barrage desktop app and the engine sidecar.
//!
//! Commands go to the sidecar on stdin and events come back on stdout, one JSON
//! object per line (NDJSON). Logs go to stderr and are never part of the protocol.
//!
//! Every message carries the protocol version `v` and a request `id`:
//!
//! ```text
//! → {"v":0,"id":1,"type":"hello"}
//! ← {"v":0,"id":1,"type":"ready","version":"0.0.0"}
//! → {"v":0,"id":2,"type":"shutdown"}
//! ```

mod codec;
mod command;
mod event;

pub use codec::{DecodeError, decode_command, decode_event, encode};
pub use command::{Command, CommandMessage};
pub use event::{Event, EventMessage};

/// The protocol version this crate speaks. Messages with another `v` are rejected.
pub const PROTOCOL_VERSION: u32 = 0;

/// Identifies a command; the events that answer it carry the same id.
///
/// The sender picks ids; they only need to be unique among its in-flight commands.
pub type RequestId = u64;
