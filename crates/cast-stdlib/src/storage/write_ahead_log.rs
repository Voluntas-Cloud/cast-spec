//! `write_ahead_log` — record intent before applying mutation.

/// Sentinel for `write_ahead_log`.
pub struct WriteAheadLog;

cast::concept! {
    name: "write_ahead_log",
    summary: "Record intent before applying mutation. A crash mid-apply \
              is recoverable by replaying the log; the log is the \
              source of truth until the mutation is durably applied.",
    anchors: [cast_stdlib::storage::write_ahead_log::WriteAheadLog],
    tags: ["cast_stdlib", "storage"],
}
