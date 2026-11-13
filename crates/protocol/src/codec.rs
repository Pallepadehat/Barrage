// SPDX-License-Identifier: MIT

//! Turning messages into NDJSON lines and back.

use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::{CommandMessage, EventMessage, PROTOCOL_VERSION, RequestId};

/// Why a line could not be decoded into a message.
#[derive(Debug, thiserror::Error)]
pub enum DecodeError {
    /// The line is not a JSON value.
    #[error("not valid JSON: {source}")]
    Json {
        /// The parser's complaint.
        #[source]
        source: serde_json::Error,
    },
    /// The message speaks another protocol version.
    #[error("unsupported protocol version {got} (expected {PROTOCOL_VERSION})")]
    UnsupportedVersion {
        /// The version the message claims.
        got: u64,
        /// The message's id, if it had a readable one.
        id: Option<RequestId>,
    },
    /// Valid JSON, but not a valid message (missing field, unknown `type`, …).
    #[error("invalid message: {source}")]
    Invalid {
        /// The message's id, if it had a readable one.
        id: Option<RequestId>,
        /// What didn't match.
        #[source]
        source: serde_json::Error,
    },
}

impl DecodeError {
    /// The id of the offending message, when it had one, so a reply can name it.
    pub fn id(&self) -> Option<RequestId> {
        match self {
            Self::Json { .. } => None,
            Self::UnsupportedVersion { id, .. } | Self::Invalid { id, .. } => *id,
        }
    }
}

/// Decodes one line (without or with its trailing newline) into a command.
pub fn decode_command(line: &[u8]) -> Result<CommandMessage, DecodeError> {
    decode(line)
}

/// Decodes one line (without or with its trailing newline) into an event.
pub fn decode_event(line: &[u8]) -> Result<EventMessage, DecodeError> {
    decode(line)
}

/// Encodes a message as one NDJSON line, including the trailing `\n`.
///
/// JSON escapes newlines inside strings, so the result is always a single line.
pub fn encode<T: Serialize>(message: &T) -> Result<String, serde_json::Error> {
    let mut line = serde_json::to_string(message)?;
    line.push('\n');
    Ok(line)
}

/// Parses to a JSON value first so the version can be checked, and the id
/// recovered, before the shape of the message is.
fn decode<T: DeserializeOwned>(line: &[u8]) -> Result<T, DecodeError> {
    let value: Value =
        serde_json::from_slice(line).map_err(|source| DecodeError::Json { source })?;
    let id = value.get("id").and_then(Value::as_u64);
    if let Some(got) = value.get("v").and_then(Value::as_u64)
        && got != u64::from(PROTOCOL_VERSION)
    {
        return Err(DecodeError::UnsupportedVersion { got, id });
    }
    serde_json::from_value(value).map_err(|source| DecodeError::Invalid { id, source })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Command, Event};

    #[test]
    fn commands_have_the_documented_wire_shape() {
        let hello = encode(&CommandMessage::new(1, Command::Hello)).unwrap();
        assert_eq!(hello, "{\"v\":0,\"id\":1,\"type\":\"hello\"}\n");
        let shutdown = encode(&CommandMessage::new(2, Command::Shutdown)).unwrap();
        assert_eq!(shutdown, "{\"v\":0,\"id\":2,\"type\":\"shutdown\"}\n");
    }

    #[test]
    fn events_have_the_documented_wire_shape() {
        let ready = EventMessage::new(
            Some(1),
            Event::Ready {
                version: "1.2.3".into(),
            },
        );
        assert_eq!(
            encode(&ready).unwrap(),
            "{\"v\":0,\"id\":1,\"type\":\"ready\",\"version\":\"1.2.3\"}\n"
        );
        let error = EventMessage::new(
            None,
            Event::Error {
                message: "a\nb".into(),
            },
        );
        assert_eq!(
            encode(&error).unwrap(),
            "{\"v\":0,\"type\":\"error\",\"message\":\"a\\nb\"}\n"
        );
    }

    #[test]
    fn messages_round_trip() {
        let command = CommandMessage::new(7, Command::Hello);
        assert_eq!(
            decode_command(encode(&command).unwrap().as_bytes()).unwrap(),
            command
        );
        let event = EventMessage::new(
            Some(7),
            Event::Ready {
                version: "0.0.0".into(),
            },
        );
        assert_eq!(
            decode_event(encode(&event).unwrap().as_bytes()).unwrap(),
            event
        );
    }

    #[test]
    fn garbage_is_a_json_error_without_id() {
        for line in [&b"not json"[..], b"", b"\xff\xfe", b"{\"v\":0,"] {
            let err = decode_command(line).unwrap_err();
            assert!(matches!(err, DecodeError::Json { .. }), "{line:?}: {err}");
            assert_eq!(err.id(), None);
        }
    }

    #[test]
    fn invalid_messages_keep_their_id() {
        let err = decode_command(br#"{"v":0,"id":5,"type":"launch"}"#).unwrap_err();
        assert!(matches!(err, DecodeError::Invalid { .. }), "{err}");
        assert_eq!(err.id(), Some(5));

        let err = decode_command(br#"{"id":6,"type":"hello"}"#).unwrap_err();
        assert!(err.to_string().contains("missing field `v`"), "{err}");
        assert_eq!(err.id(), Some(6));

        let err = decode_command(br#"{"v":0,"type":"hello"}"#).unwrap_err();
        assert!(err.to_string().contains("missing field `id`"), "{err}");
    }

    #[test]
    fn other_versions_are_rejected() {
        let err = decode_command(br#"{"v":1,"id":3,"type":"hello"}"#).unwrap_err();
        assert!(
            matches!(err, DecodeError::UnsupportedVersion { got: 1, .. }),
            "{err}"
        );
        assert_eq!(err.id(), Some(3));
        assert_eq!(
            err.to_string(),
            "unsupported protocol version 1 (expected 0)"
        );
    }

    #[test]
    fn a_trailing_newline_is_accepted() {
        let command = decode_command(b"{\"v\":0,\"id\":1,\"type\":\"shutdown\"}\n").unwrap();
        assert_eq!(command.command, Command::Shutdown);
    }
}
