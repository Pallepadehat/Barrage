#!/usr/bin/env bash
# SPDX-License-Identifier: MIT
#
# Builds the engine sidecar (crate `barrage-sidecar`) and copies it to
# src-tauri/binaries/barrage-engine-<target triple>, where Tauri's
# `bundle.externalBin` expects it. `just dev` runs this before `tauri dev`;
# `tauri build` runs it as part of `beforeBuildCommand`.
#
# The bundled name differs from the crate's so Tauri's copy next to the app
# binary never overwrites Cargo's own `target/<profile>/barrage-sidecar`.
#
# Profile and target follow the Tauri CLI when it runs this (TAURI_ENV_DEBUG,
# TAURI_ENV_TARGET_TRIPLE); otherwise a debug build for the host.

set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
host="$(rustc -vV | sed -n 's/^host: //p')"
triple="${TAURI_ENV_TARGET_TRIPLE:-$host}"

args=(build --locked --manifest-path "$root/Cargo.toml" -p barrage-sidecar)
profile=debug
if [[ "${TAURI_ENV_DEBUG:-true}" != "true" ]]; then
  args+=(--release)
  profile=release
fi
out="$root/target/$profile"
if [[ "$triple" != "$host" ]]; then
  args+=(--target "$triple")
  out="$root/target/$triple/$profile"
fi

cargo "${args[@]}"
dest="$root/apps/desktop/src-tauri/binaries"
mkdir -p "$dest"
# Replace rather than overwrite in place: macOS caches code signatures per
# inode and kills a binary whose contents changed under it.
cp "$out/barrage-sidecar" "$dest/.barrage-engine-$triple.tmp"
mv -f "$dest/.barrage-engine-$triple.tmp" "$dest/barrage-engine-$triple"
echo "sidecar: $dest/barrage-engine-$triple ($profile)"
