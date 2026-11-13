// SPDX-License-Identifier: MIT

//! Engine process spawned by the Barrage desktop app.
//!
//! Speaks the `barrage-protocol` NDJSON protocol on stdin/stdout and logs to
//! stderr. Exits with status 0 on a `shutdown` command or when stdin closes.

mod handler;
mod lines;
mod server;

use std::process::ExitCode;

use tokio::io::BufReader;

/// Engine version reported in the `ready` event.
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() -> ExitCode {
    eprintln!("barrage-sidecar {VERSION}: started");
    // tokio reads stdin on a blocking thread that can't be cancelled, so the
    // runtime can only shut down promptly while no read is pending. That holds
    // because `serve` reads one line at a time; once commands are read
    // alongside a running test (M1), move stdin to a dedicated thread.
    let stdin = BufReader::new(tokio::io::stdin());
    match server::serve(stdin, tokio::io::stdout()).await {
        Ok(()) => {
            eprintln!("barrage-sidecar: exiting");
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("barrage-sidecar: {err:#}");
            ExitCode::FAILURE
        }
    }
}
