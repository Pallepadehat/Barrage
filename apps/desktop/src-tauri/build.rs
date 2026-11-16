// SPDX-License-Identifier: MIT

//! Generates the Tauri context (config, capabilities, icons) at build time.
//!
//! Tauri copies the engine sidecar (`bundle.externalBin`) next to the app
//! binary and fails the build if it's missing. `scripts/sidecar.sh` puts the
//! real one there (`just dev` and `tauri build` run it first). For builds that
//! don't, such as clippy and tests in `just check`, this writes a stand-in that
//! exits with an explanation, so the app shows the engine error state instead
//! of failing to compile.

use std::fs;
use std::io;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

/// The sidecar's path as configured in `tauri.conf.json`, without the target triple.
const SIDECAR: &str = "binaries/barrage-engine";

/// The stand-in's contents: report the problem on stderr (the app log) and fail.
const STAND_IN: &str = "#!/bin/sh\n\
echo 'barrage-engine is a stand-in: run scripts/sidecar.sh (just dev does) to build the engine' >&2\n\
exit 1\n";

fn main() -> io::Result<()> {
    write_stand_in_if_missing()?;
    tauri_build::build();
    Ok(())
}

/// Writes an executable stand-in sidecar unless a real one is already there.
fn write_stand_in_if_missing() -> io::Result<()> {
    let triple = std::env::var("TARGET").map_err(io::Error::other)?;
    let path = format!("{SIDECAR}-{triple}");
    let path = Path::new(&path);
    if path.exists() {
        return Ok(());
    }
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }
    fs::write(path, STAND_IN)?;
    fs::set_permissions(path, fs::Permissions::from_mode(0o755))
}
