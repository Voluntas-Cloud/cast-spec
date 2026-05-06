//! `log_structured_storage` — sequential-append substrate, compacted later.

/// Sentinel for `log_structured_storage`.
pub struct LogStructuredStorage;

cast::concept! {
    name: "log_structured_storage",
    summary: "Writes optimized as sequential appends, later compacted. \
              Trades read amplification for write throughput; pairs \
              naturally with append-only log and compaction.",
    anchors: [cast_stdlib::storage::log_structured_storage::LogStructuredStorage],
    tags: ["cast_stdlib", "storage"],
}
