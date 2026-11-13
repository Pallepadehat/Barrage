// SPDX-License-Identifier: MIT

//! Generates the TypeScript types in `packages/types` from the Rust plan and protocol types.
//!
//! ```text
//! barrage-typegen write <dir>   regenerate <dir> (what `just types` runs)
//! barrage-typegen check <dir>   fail if <dir> differs from a fresh generation
//! ```

mod generate;
mod sync;

use std::path::PathBuf;
use std::process::ExitCode;

use anyhow::{Context, Result, bail};

fn main() -> ExitCode {
    match run() {
        Ok(code) => code,
        Err(err) => {
            eprintln!("barrage-typegen: {err:#}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<ExitCode> {
    let mut args = std::env::args().skip(1);
    let (Some(mode), Some(dir), None) = (args.next(), args.next(), args.next()) else {
        bail!("usage: barrage-typegen <write|check> <dir>");
    };
    let dir = PathBuf::from(dir);
    let files = generate::generate().context("generating bindings")?;

    match mode.as_str() {
        "write" => {
            let changed = sync::write(&dir, &files)?;
            eprintln!(
                "barrage-typegen: {} files in {}, {changed} changed",
                files.len(),
                dir.display()
            );
            Ok(ExitCode::SUCCESS)
        }
        "check" => {
            let stale = sync::diff(&dir, &files)?;
            if stale.is_empty() {
                return Ok(ExitCode::SUCCESS);
            }
            eprintln!("barrage-typegen: {} is out of date:", dir.display());
            for entry in &stale {
                eprintln!("  {entry}");
            }
            eprintln!("Run `just types` and commit the result.");
            Ok(ExitCode::FAILURE)
        }
        other => bail!("unknown mode `{other}`; expected `write` or `check`"),
    }
}
