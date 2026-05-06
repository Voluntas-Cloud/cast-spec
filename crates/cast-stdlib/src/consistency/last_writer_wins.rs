//! `last_writer_wins` — newest write overwrites older conflict.

/// Sentinel for `last_writer_wins`.
pub struct LastWriterWins;

cast::concept! {
    name: "last_writer_wins",
    summary: "Newest write overwrites older conflict. Simple, fast, \
              lossy — older writes are silently discarded.",
    anchors: [cast_stdlib::consistency::last_writer_wins::LastWriterWins],
    tags: ["cast_stdlib", "consistency"],
}
