// SPDX-License-Identifier: MIT

//! Decides what to do with one input line. Pure: no I/O, so it's easy to test.

use barrage_protocol::{Command, Event, EventMessage, decode_command};

use crate::VERSION;

/// What the serve loop does after a line.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Action {
    /// Write this event and keep reading.
    Reply(EventMessage),
    /// Stop reading and exit cleanly.
    Exit,
}

/// Handles one line from stdin (without its newline).
///
/// Anything that isn't a valid command becomes an `error` event, never a crash.
pub(crate) fn handle(line: &[u8]) -> Action {
    let message = match decode_command(line) {
        Ok(message) => message,
        Err(err) => return Action::Reply(error(err.id(), err.to_string())),
    };
    match message.command {
        Command::Hello => Action::Reply(EventMessage::new(
            Some(message.id),
            Event::Ready {
                version: VERSION.to_owned(),
            },
        )),
        Command::Shutdown => Action::Exit,
    }
}

/// An `error` event, optionally answering a command.
pub(crate) fn error(id: Option<u64>, message: String) -> EventMessage {
    EventMessage::new(id, Event::Error { message })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn error_message(action: Action) -> (Option<u64>, String) {
        match action {
            Action::Reply(EventMessage {
                id,
                event: Event::Error { message },
                ..
            }) => (id, message),
            other => panic!("expected an error event, got {other:?}"),
        }
    }

    #[test]
    fn hello_gets_ready_with_the_engine_version() {
        let action = handle(br#"{"v":0,"id":4,"type":"hello"}"#);
        let ready = EventMessage::new(
            Some(4),
            Event::Ready {
                version: VERSION.to_owned(),
            },
        );
        assert_eq!(action, Action::Reply(ready));
    }

    #[test]
    fn shutdown_exits() {
        assert_eq!(handle(br#"{"v":0,"id":1,"type":"shutdown"}"#), Action::Exit);
    }

    #[test]
    fn malformed_json_is_an_error_without_id() {
        let (id, message) = error_message(handle(b"{oops"));
        assert_eq!(id, None);
        assert!(message.starts_with("not valid JSON"), "{message}");
    }

    #[test]
    fn unknown_command_is_an_error_answering_its_id() {
        let (id, message) = error_message(handle(br#"{"v":0,"id":9,"type":"fly"}"#));
        assert_eq!(id, Some(9));
        assert!(message.contains("unknown variant `fly`"), "{message}");
    }

    #[test]
    fn wrong_version_is_an_error_answering_its_id() {
        let (id, message) = error_message(handle(br#"{"v":2,"id":3,"type":"hello"}"#));
        assert_eq!(id, Some(3));
        assert!(message.contains("protocol version 2"), "{message}");
    }
}
