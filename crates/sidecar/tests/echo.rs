// SPDX-License-Identifier: MIT

//! Spawns the real sidecar binary and talks to it over its pipes.

// Integration-test helpers fall outside clippy.toml's test exemptions.
#![allow(clippy::unwrap_used, clippy::panic)]

use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command as Process, ExitStatus, Stdio};
use std::time::{Duration, Instant};

use barrage_protocol::{Command, CommandMessage, Event, EventMessage, decode_event, encode};

struct Sidecar {
    child: Child,
    stdin: Option<ChildStdin>,
    stdout: BufReader<ChildStdout>,
}

impl Sidecar {
    fn spawn() -> Self {
        let mut child = Process::new(env!("CARGO_BIN_EXE_barrage-sidecar"))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        let stdin = child.stdin.take();
        let stdout = BufReader::new(child.stdout.take().unwrap());
        Self {
            child,
            stdin,
            stdout,
        }
    }

    fn send_raw(&mut self, line: &str) {
        let stdin = self.stdin.as_mut().unwrap();
        stdin.write_all(line.as_bytes()).unwrap();
        stdin.flush().unwrap();
    }

    fn send(&mut self, id: u64, command: Command) {
        self.send_raw(&encode(&CommandMessage::new(id, command)).unwrap());
    }

    fn recv(&mut self) -> EventMessage {
        let mut line = String::new();
        assert!(
            self.stdout.read_line(&mut line).unwrap() > 0,
            "sidecar closed stdout"
        );
        decode_event(line.as_bytes()).unwrap()
    }

    /// Waits for exit, killing the process if it takes longer than a few seconds.
    fn wait(&mut self) -> ExitStatus {
        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            if let Some(status) = self.child.try_wait().unwrap() {
                return status;
            }
            if Instant::now() > deadline {
                let _ = self.child.kill();
                panic!("sidecar did not exit in time");
            }
            std::thread::sleep(Duration::from_millis(10));
        }
    }
}

#[test]
fn hello_ready_shutdown_exits_zero() {
    let mut sidecar = Sidecar::spawn();
    sidecar.send(1, Command::Hello);
    let ready = sidecar.recv();
    assert_eq!(ready.id, Some(1));
    assert_eq!(
        ready.event,
        Event::Ready {
            version: env!("CARGO_PKG_VERSION").to_owned()
        }
    );

    // Stdin stays open: shutdown alone must end the process.
    sidecar.send(2, Command::Shutdown);
    assert!(sidecar.wait().success());
}

#[test]
fn malformed_input_yields_error_events_and_the_sidecar_keeps_going() {
    let mut sidecar = Sidecar::spawn();
    sidecar.send_raw("this is not json\n");
    let error = sidecar.recv();
    assert_eq!(error.id, None);
    assert!(matches!(error.event, Event::Error { .. }), "{error:?}");

    sidecar.send_raw("{\"v\":0,\"id\":7,\"type\":\"explode\"}\n");
    let error = sidecar.recv();
    assert_eq!(error.id, Some(7));
    assert!(matches!(error.event, Event::Error { .. }), "{error:?}");

    sidecar.send(8, Command::Hello);
    assert!(matches!(sidecar.recv().event, Event::Ready { .. }));
    sidecar.send(9, Command::Shutdown);
    assert!(sidecar.wait().success());
}

#[test]
fn stdin_eof_exits_zero() {
    let mut sidecar = Sidecar::spawn();
    sidecar.stdin = None; // drop closes the pipe
    assert!(sidecar.wait().success());
}
