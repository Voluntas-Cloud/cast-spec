//! `append_only_log` — durability via record-without-overwrite.

/// Sentinel for `append_only_log`.
pub struct AppendOnlyLog;

cast::concept! {
    name: "append_only_log",
    summary: "Durability by recording facts/events without overwriting. \
              Every write is sequential; concurrent reads of \
              historical records are inherently safe. Foundational \
              under MVCC, event sourcing, write-ahead logs.",
    anchors: [cast_stdlib::storage::append_only_log::AppendOnlyLog],
    tags: ["cast_stdlib", "storage"],
}
