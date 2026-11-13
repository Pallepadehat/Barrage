// SPDX-License-Identifier: MIT

//! Barrage desktop app: a Tauri 2 window hosting the React UI.
//!
//! The window starts hidden and is shown once the page has loaded, so it never
//! flashes empty. If the page hasn't loaded after [`SHOW_FALLBACK`] (dev server
//! down, webview crash), the window is shown anyway so the app is never
//! invisible. Time from process start to showing the window is logged to stderr
//! as the cold-start figure (target: under 1 s on Apple silicon). It is taken
//! at page load, before React's first render, so it is a lower bound.

use std::sync::{Arc, Once};
use std::time::{Duration, Instant};

use tauri::webview::PageLoadEvent;
use tauri::{AppHandle, Manager};

/// Label of the only window, as configured in `tauri.conf.json`.
const MAIN_WINDOW: &str = "main";

/// How long to wait for the first page load before showing the window anyway.
const SHOW_FALLBACK: Duration = Duration::from_secs(2);

fn main() -> anyhow::Result<()> {
    let started = Instant::now();
    let shown = Arc::new(Once::new());
    let shown_on_load = Arc::clone(&shown);

    tauri::Builder::default()
        .setup(move |app| {
            let handle = app.handle().clone();
            std::thread::spawn(move || {
                std::thread::sleep(SHOW_FALLBACK);
                show_once(&handle, &shown, started, "page load timed out");
            });
            Ok(())
        })
        .on_page_load(move |webview, payload| {
            // Reloads (e.g. dev hot reload) fire this again; `show_once` ignores them.
            if payload.event() == PageLoadEvent::Finished {
                show_once(webview.app_handle(), &shown_on_load, started, "page loaded");
            }
        })
        .run(tauri::generate_context!())?;
    Ok(())
}

/// Shows the main window and logs the elapsed time, the first time it's called.
fn show_once(app: &AppHandle, shown: &Once, started: Instant, reason: &str) {
    shown.call_once(|| {
        match app.get_webview_window(MAIN_WINDOW) {
            Some(window) => {
                if let Err(err) = window.show() {
                    eprintln!("barrage: failed to show the main window: {err}");
                }
            }
            None => eprintln!("barrage: main window not found"),
        }
        eprintln!(
            "barrage: window ready in {} ms ({reason})",
            started.elapsed().as_millis()
        );
    });
}
