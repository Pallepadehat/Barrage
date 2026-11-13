// SPDX-License-Identifier: MIT

//! Engine process spawned by the Barrage desktop app.
//!
//! Speaks the `barrage-protocol` NDJSON protocol on stdin/stdout and logs to stderr.

fn main() {
    // Placeholder until the protocol loop lands (M0-07). Version goes to stderr:
    // stdout is reserved for protocol messages.
    eprintln!("barrage-sidecar {}", env!("CARGO_PKG_VERSION"));
}
