// SPDX-License-Identifier: MIT

//! Barrage desktop app: a Tauri 2 window hosting the React UI.
//!
//! The window starts hidden and is shown once the page has loaded, so it never
//! flashes empty. Time from process start to that point is logged to stderr as
//! the cold-start figure (target: under 1 s on Apple silicon).

use std::sync::Once;
use std::time::Instant;

use tauri::webview::PageLoadEvent;

fn main() -> anyhow::Result<()> {
    let started = Instant::now();
    let shown = Once::new();

    tauri::Builder::default()
        .on_page_load(move |webview, payload| {
            if payload.event() != PageLoadEvent::Finished {
                return;
            }
            // Reloads (e.g. dev hot reload) fire this again; show and time only once.
            shown.call_once(|| {
                if let Err(err) = webview.window().show() {
                    eprintln!("barrage: failed to show the main window: {err}");
                }
                eprintln!(
                    "barrage: window ready in {} ms",
                    started.elapsed().as_millis()
                );
            });
        })
        .run(tauri::generate_context!())?;
    Ok(())
}
