// SPDX-License-Identifier: MIT OR Apache-2.0

//! Messages exchanged between the Barrage desktop app and the engine sidecar.
//!
//! Commands go to the sidecar on stdin and events come back on stdout, one JSON
//! object per line (NDJSON). Logs go to stderr and are never part of the protocol.
