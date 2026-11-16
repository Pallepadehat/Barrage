// SPDX-License-Identifier: MIT

//! The engine sidecar: spawning it, piping commands to its stdin and
//! forwarding its stdout events to the webview.
//!
//! The webview drives the lifecycle through two commands, [`engine_start`] and
//! [`engine_send`], and listens for two events: [`EVENT`] carries each protocol
//! event from the engine, and [`EXIT`] says the process stopped, with a
//! human-readable reason. Engine stderr goes to the app log (our stderr).

use std::sync::{Mutex, MutexGuard};

use barrage_protocol::{Command, CommandMessage, Event, EventMessage, decode_event, encode};
use tauri::async_runtime::Receiver;
use tauri::{AppHandle, Emitter, Manager, State};
use tauri_plugin_shell::ShellExt;
use tauri_plugin_shell::process::{CommandChild, CommandEvent, TerminatedPayload};

/// The sidecar's name in `bundle.externalBin`, without directory or target triple.
const SIDECAR: &str = "barrage-engine";

/// Webview event carrying one `EventMessage` from the engine.
pub(crate) const EVENT: &str = "engine:event";

/// Webview event saying the engine process stopped; the payload is the reason.
pub(crate) const EXIT: &str = "engine:exit";

/// Id of the `shutdown` command sent when the app quits. Nothing answers it.
const SHUTDOWN_ID: u64 = 0;

/// The running engine, if any. Managed as Tauri state.
#[derive(Default)]
pub(crate) struct Engine {
    inner: Mutex<Inner>,
}

#[derive(Default)]
struct Inner {
    child: Option<CommandChild>,
    /// Bumped on every spawn, so a late exit from a replaced process is ignored.
    generation: u64,
}

impl Engine {
    fn lock(&self) -> MutexGuard<'_, Inner> {
        // A panic while holding the lock can't leave `Inner` half-updated, so
        // a poisoned lock is still safe to use.
        self.inner
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    /// Spawns the sidecar unless it's already running.
    fn start(&self, app: &AppHandle) -> Result<(), String> {
        let mut inner = self.lock();
        if inner.child.is_some() {
            return Ok(());
        }
        let (events, child) = app
            .shell()
            .sidecar(SIDECAR)
            .and_then(|command| command.spawn())
            .map_err(|err| {
                format!("Couldn't start the engine ({err}). Restart it to try again.")
            })?;
        inner.generation += 1;
        inner.child = Some(child);
        let generation = inner.generation;
        tauri::async_runtime::spawn(forward(app.clone(), events, generation));
        Ok(())
    }

    /// Writes one command line to the sidecar's stdin.
    fn send(&self, message: &CommandMessage) -> Result<(), String> {
        let line = encode(message).map_err(|err| format!("Couldn't encode the command: {err}"))?;
        let mut inner = self.lock();
        let child = inner
            .child
            .as_mut()
            .ok_or_else(|| "The engine isn't running. Restart it to continue.".to_owned())?;
        child
            .write(line.as_bytes())
            .map_err(|err| format!("Couldn't reach the engine: {err}"))
    }

    /// Forgets the process if it's still the current one. Returns whether it was.
    fn exited(&self, generation: u64) -> bool {
        let mut inner = self.lock();
        let current = inner.generation == generation;
        if current {
            inner.child = None;
        }
        current
    }

    /// Asks the engine to exit when the app quits; kills it if it can't be asked.
    pub(crate) fn shutdown(&self) {
        let mut inner = self.lock();
        // A new generation, so the exit this causes isn't reported as a crash.
        inner.generation += 1;
        let Some(mut child) = inner.child.take() else {
            return;
        };
        drop(inner);
        let message = CommandMessage::new(SHUTDOWN_ID, Command::Shutdown);
        let asked = encode(&message)
            .ok()
            .is_some_and(|line| child.write(line.as_bytes()).is_ok());
        if !asked && let Err(err) = child.kill() {
            eprintln!("barrage: couldn't stop the engine: {err}");
        }
    }
}

/// Starts the engine if it isn't running. Safe to call again, e.g. after a reload.
///
/// Commands are `async` so they run off the main thread: a stdin write can
/// block while the engine is busy, and that must never freeze the window.
#[tauri::command(async)]
pub(crate) fn engine_start(app: AppHandle, engine: State<'_, Engine>) -> Result<(), String> {
    engine.start(&app)
}

/// Sends one command to the engine's stdin.
#[tauri::command(async)]
pub(crate) fn engine_send(
    message: CommandMessage,
    engine: State<'_, Engine>,
) -> Result<(), String> {
    engine.send(&message)
}

/// Pumps one process's output until it exits: stdout events to the webview,
/// stderr to the app log, and the exit as an [`EXIT`] event.
async fn forward(app: AppHandle, mut events: Receiver<CommandEvent>, generation: u64) {
    while let Some(event) = events.recv().await {
        match event {
            CommandEvent::Stdout(line) => match decode_event(&line) {
                Ok(message) => emit(&app, EVENT, message),
                Err(err) => {
                    eprintln!("barrage: ignoring engine output: {err}");
                    // Answer the request it was meant for, so nothing waits forever.
                    if let Some(id) = err.id() {
                        let message =
                            format!("The engine sent an answer the app can't read ({err}).");
                        emit(
                            &app,
                            EVENT,
                            EventMessage::new(Some(id), Event::Error { message }),
                        );
                    }
                }
            },
            CommandEvent::Stderr(line) => {
                eprintln!("engine: {}", String::from_utf8_lossy(line.trim_ascii_end()));
            }
            CommandEvent::Error(err) => eprintln!("barrage: engine I/O error: {err}"),
            CommandEvent::Terminated(status) => {
                eprintln!("barrage: engine exited ({})", describe(&status));
                if app.state::<Engine>().exited(generation) {
                    emit(&app, EXIT, exit_reason(&status));
                }
            }
            other => eprintln!("barrage: unexpected engine event {other:?}"),
        }
    }
}

fn emit<P: serde::Serialize + Clone>(app: &AppHandle, name: &str, payload: P) {
    if let Err(err) = app.emit(name, payload) {
        eprintln!("barrage: couldn't forward {name} to the window: {err}");
    }
}

/// Exit status in a few words, e.g. `exit code 1` or `signal 9`.
fn describe(status: &TerminatedPayload) -> String {
    match (status.code, status.signal) {
        (Some(code), _) => format!("exit code {code}"),
        (None, Some(signal)) => format!("signal {signal}"),
        (None, None) => "unknown status".to_owned(),
    }
}

/// The reason shown in the UI when the engine stops (DESIGN §10 microcopy).
fn exit_reason(status: &TerminatedPayload) -> String {
    format!(
        "The engine stopped unexpectedly ({}). Restart it to continue.",
        describe(status)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn status(code: Option<i32>, signal: Option<i32>) -> TerminatedPayload {
        TerminatedPayload { code, signal }
    }

    #[test]
    fn exit_reasons_name_the_code_or_signal() {
        assert_eq!(
            exit_reason(&status(Some(1), None)),
            "The engine stopped unexpectedly (exit code 1). Restart it to continue."
        );
        assert_eq!(
            exit_reason(&status(None, Some(9))),
            "The engine stopped unexpectedly (signal 9). Restart it to continue."
        );
        assert_eq!(
            exit_reason(&status(None, None)),
            "The engine stopped unexpectedly (unknown status). Restart it to continue."
        );
    }

    #[test]
    fn a_late_exit_from_a_replaced_process_is_ignored() {
        let engine = Engine::default();
        engine.lock().generation = 2;
        assert!(!engine.exited(1));
        assert!(engine.exited(2));
    }

    #[test]
    fn the_exit_caused_by_shutdown_is_not_reported() {
        let engine = Engine::default();
        engine.lock().generation = 1;
        engine.shutdown();
        assert!(!engine.exited(1));
    }

    #[test]
    fn sending_without_a_running_engine_is_an_error() {
        let engine = Engine::default();
        let err = engine
            .send(&CommandMessage::new(1, Command::Hello))
            .unwrap_err();
        assert_eq!(err, "The engine isn't running. Restart it to continue.");
    }
}
