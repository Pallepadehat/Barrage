// SPDX-License-Identifier: MIT

//! Reads newline-delimited input with a cap on line length, so a runaway
//! writer can't make the sidecar buffer without bound.

use std::io;

use tokio::io::{AsyncBufRead, AsyncBufReadExt};

/// Outcome of [`read_line`].
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Read {
    /// A line is in the buffer, without its `\n`. The last line may lack one.
    Line,
    /// The line was longer than the cap and was skipped; the buffer is empty.
    TooLong,
    /// End of input; nothing was read.
    Eof,
}

/// Reads the next line into `buf` (cleared first), keeping at most `max` bytes.
///
/// A longer line is consumed up to and including its newline and reported as
/// [`Read::TooLong`], so the next call starts on the following line.
pub(crate) async fn read_line<R>(reader: &mut R, buf: &mut Vec<u8>, max: usize) -> io::Result<Read>
where
    R: AsyncBufRead + Unpin,
{
    buf.clear();
    let mut seen_any = false;
    let mut too_long = false;
    loop {
        let available = reader.fill_buf().await?;
        if available.is_empty() {
            return Ok(match (seen_any, too_long) {
                (false, _) => Read::Eof,
                (true, true) => Read::TooLong,
                (true, false) => Read::Line,
            });
        }
        seen_any = true;
        let newline = available.iter().position(|&b| b == b'\n');
        let chunk = &available[..newline.unwrap_or(available.len())];
        if !too_long {
            if buf.len() + chunk.len() > max {
                too_long = true;
                buf.clear();
            } else {
                buf.extend_from_slice(chunk);
            }
        }
        let used = chunk.len() + usize::from(newline.is_some());
        reader.consume(used);
        if newline.is_some() {
            return Ok(if too_long { Read::TooLong } else { Read::Line });
        }
    }
}

#[cfg(test)]
mod tests {
    use tokio::io::BufReader;

    use super::*;

    /// Reads every line with the given cap; `None` marks a skipped line.
    async fn all_lines(input: &[u8], max: usize, capacity: usize) -> Vec<Option<String>> {
        let mut reader = BufReader::with_capacity(capacity, input);
        let mut buf = Vec::new();
        let mut out = Vec::new();
        loop {
            match read_line(&mut reader, &mut buf, max).await.unwrap() {
                Read::Line => out.push(Some(String::from_utf8(buf.clone()).unwrap())),
                Read::TooLong => out.push(None),
                Read::Eof => return out,
            }
        }
    }

    #[tokio::test]
    async fn splits_on_newlines_and_keeps_a_final_unterminated_line() {
        let lines = all_lines(b"a\n\nbc\nd", 10, 64).await;
        assert_eq!(
            lines,
            [
                Some("a".into()),
                Some(String::new()),
                Some("bc".into()),
                Some("d".into())
            ]
        );
    }

    #[tokio::test]
    async fn empty_input_is_eof() {
        assert!(all_lines(b"", 10, 64).await.is_empty());
    }

    #[tokio::test]
    async fn skips_long_lines_and_resyncs_across_small_buffers() {
        // A capacity of 2 forces lines to span many reads.
        let lines = all_lines(b"abcd\nok\nabcdefgh\nxyz", 3, 2).await;
        assert_eq!(lines, [None, Some("ok".into()), None, Some("xyz".into())]);
    }

    #[tokio::test]
    async fn a_line_exactly_at_the_cap_is_kept() {
        assert_eq!(
            all_lines(b"abc\nabcd", 3, 64).await,
            [Some("abc".into()), None]
        );
    }
}
