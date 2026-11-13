// SPDX-License-Identifier: MIT OR Apache-2.0

//! The `barrage` command-line tool: runs Barrage plans headless, e.g. in CI.

fn main() {
    println!("barrage {}", env!("CARGO_PKG_VERSION"));
}
