// SPDX-License-Identifier: MIT

//! Writes generated files to the output directory, or compares them against it.

use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

use crate::generate::Files;

/// Makes `dir` hold exactly `files`: writes new and changed files and removes
/// `.ts` files that are no longer generated. Returns how many files changed.
pub(crate) fn write(dir: &Path, files: &Files) -> Result<usize> {
    fs::create_dir_all(dir).with_context(|| format!("creating {}", dir.display()))?;
    let existing = read_ts_files(dir)?;
    let mut changed = 0;

    for (name, content) in files {
        if existing.get(name) != Some(content) {
            let path = dir.join(name);
            fs::write(&path, content).with_context(|| format!("writing {}", path.display()))?;
            changed += 1;
        }
    }
    for name in existing.keys().filter(|name| !files.contains_key(*name)) {
        let path = dir.join(name);
        fs::remove_file(&path).with_context(|| format!("removing {}", path.display()))?;
        changed += 1;
    }
    Ok(changed)
}

/// Lists the differences between `dir` and `files`, one line per file.
/// Empty means `dir` is up to date.
pub(crate) fn diff(dir: &Path, files: &Files) -> Result<Vec<String>> {
    let existing = read_ts_files(dir)?;
    let mut stale = Vec::new();

    for (name, content) in files {
        match existing.get(name) {
            None => stale.push(format!("missing: {name}")),
            Some(old) if old != content => stale.push(format!("changed: {name}")),
            Some(_) => {}
        }
    }
    for name in existing.keys().filter(|name| !files.contains_key(*name)) {
        stale.push(format!("extra:   {name}"));
    }
    Ok(stale)
}

/// Reads the `.ts` files in `dir` (not recursive); a missing directory is empty.
pub(crate) fn read_ts_files(dir: &Path) -> Result<Files> {
    let mut files = Files::new();
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(files),
        Err(err) => return Err(err).with_context(|| format!("reading {}", dir.display())),
    };
    for entry in entries {
        let path = entry?.path();
        let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        if name.ends_with(".ts") {
            let content =
                fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
            files.insert(name.to_owned(), content);
        }
    }
    Ok(files)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    /// A fresh directory under the system temp dir, removed on drop.
    struct TempDir(PathBuf);

    impl TempDir {
        fn new(name: &str) -> Self {
            let path = std::env::temp_dir().join(format!(
                "barrage-typegen-test-{name}-{}",
                std::process::id()
            ));
            let _ = fs::remove_dir_all(&path);
            Self(path)
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn files(entries: &[(&str, &str)]) -> Files {
        entries
            .iter()
            .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
            .collect()
    }

    #[test]
    fn missing_dir_reports_every_file_missing() {
        let dir = TempDir::new("missing");
        let stale = diff(&dir.0, &files(&[("A.ts", "a")])).unwrap();
        assert_eq!(stale, ["missing: A.ts"]);
    }

    #[test]
    fn write_then_diff_is_clean_and_idempotent() {
        let dir = TempDir::new("clean");
        let want = files(&[("A.ts", "a"), ("index.ts", "i")]);
        assert_eq!(write(&dir.0, &want).unwrap(), 2);
        assert!(diff(&dir.0, &want).unwrap().is_empty());
        assert_eq!(write(&dir.0, &want).unwrap(), 0);
    }

    #[test]
    fn diff_reports_changed_and_extra_files() {
        let dir = TempDir::new("stale");
        write(&dir.0, &files(&[("A.ts", "old"), ("Gone.ts", "g")])).unwrap();
        let stale = diff(&dir.0, &files(&[("A.ts", "new")])).unwrap();
        assert_eq!(stale, ["changed: A.ts", "extra:   Gone.ts"]);
    }

    #[test]
    fn write_removes_types_no_longer_generated_but_keeps_other_files() {
        let dir = TempDir::new("prune");
        write(&dir.0, &files(&[("A.ts", "a"), ("Gone.ts", "g")])).unwrap();
        fs::write(dir.0.join("README.md"), "keep").unwrap();
        write(&dir.0, &files(&[("A.ts", "a")])).unwrap();
        assert!(!dir.0.join("Gone.ts").exists());
        assert!(dir.0.join("README.md").exists());
    }
}
