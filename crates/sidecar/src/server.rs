// SPDX-License-Identifier: MIT

//! The serve loop: read commands, write events, until `shutdown` or EOF.

use anyhow::{Context, Result};
use tokio::io::{AsyncBufRead, AsyncWrite, AsyncWriteExt};

use barrage_protocol::{EventMessage, encode};

use crate::handler::{Action, error, handle};
use crate::lines::{Read, read_line};

/// Longest accepted input line. Plans will travel inline in commands, so this
/// is generous; anything longer is skipped with an `error` event.
const MAX_LINE: usize = 16 * 1024 * 1024;

/// Serves the protocol on `input`/`output` until a `shutdown` command or the
/// end of input. Bad input is answered with `error` events; only I/O failures
/// (e.g. the app closed our stdout) end the loop with an error.
pub(crate) async fn serve<R, W>(mut input: R, mut output: W) -> Result<()>
where
    R: AsyncBufRead + Unpin,
    W: AsyncWrite + Unpin,
{
    let mut line = Vec::new();
    loop {
        let read = read_line(&mut input, &mut line, MAX_LINE)
            .await
            .context("reading stdin")?;
        let action = match read {
            Read::Eof => return Ok(()),
            Read::TooLong => {
                let message = format!("line longer than {MAX_LINE} bytes ignored");
                Action::Reply(error(None, message))
            }
            // Blank lines are harmless; skip them quietly.
            Read::Line if line.trim_ascii().is_empty() => continue,
            Read::Line => handle(&line),
        };
        match action {
            Action::Reply(event) => send(&mut output, &event).await?,
            Action::Exit => return Ok(()),
        }
    }
}

/// Writes one event line and flushes, so the app sees it immediately.
async fn send<W: AsyncWrite + Unpin>(output: &mut W, event: &EventMessage) -> Result<()> {
    let line = encode(event).context("encoding event")?;
    output
        .write_all(line.as_bytes())
        .await
        .context("writing stdout")?;
    output.flush().await.context("flushing stdout")
}

#[cfg(test)]
mod tests {
    use barrage_protocol::{Event, decode_event};

    use super::*;

    async fn run(input: &[u8]) -> Vec<EventMessage> {
        let mut output = Vec::new();
        serve(input, &mut output).await.unwrap();
        output
            .split(|&b| b == b'\n')
            .filter(|l| !l.is_empty())
            .map(|l| decode_event(l).unwrap())
            .collect()
    }

    #[tokio::test]
    async fn answers_each_command_and_stops_at_shutdown() {
        let input = b"{\"v\":0,\"id\":1,\"type\":\"hello\"}\n\n  \nnope\n{\"v\":0,\"id\":2,\"type\":\"shutdown\"}\n{\"v\":0,\"id\":3,\"type\":\"hello\"}\n";
        let events = run(input).await;
        assert_eq!(events.len(), 2, "{events:?}");
        assert!(matches!(events[0].event, Event::Ready { .. }));
        assert_eq!(events[0].id, Some(1));
        assert!(matches!(events[1].event, Event::Error { .. }));
        assert_eq!(events[1].id, None);
    }

    #[tokio::test]
    async fn eof_ends_cleanly() {
        assert!(run(b"").await.is_empty());
        assert_eq!(run(b"{\"v\":0,\"id\":1,\"type\":\"hello\"}").await.len(), 1);
    }
}
