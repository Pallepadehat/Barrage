// SPDX-License-Identifier: MIT

//! The `barrage` command-line tool: runs Barrage plans headless, e.g. in CI.

fn main() {
    println!("barrage {}", env!("CARGO_PKG_VERSION"));
}
