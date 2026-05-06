//! `line_oriented_protocol` — newline-delimited message framing on a byte stream.

/// Sentinel for `line_oriented_protocol`.
pub struct LineOrientedProtocol;

cast::concept! {
    name: "line_oriented_protocol",
    summary: "One logical message per newline-terminated line on a \
              byte stream. The receiver `read_line()`s; the sender \
              writes the message followed by `\\n`. No length prefix, \
              no framing header — the delimiter IS the framing. JSON \
              Lines (NDJSON) is the canonical instance: each line is \
              a complete JSON value. Trades self-describing length \
              for line-by-line greppability and trivial `tail -f` \
              consumption; doesn't work for payloads that may contain \
              raw `\\n`.",
    anchors: [cast_stdlib::messaging::line_oriented_protocol::LineOrientedProtocol],
    tags: ["cast_stdlib", "messaging"],
}
