//! `compaction` — reduce historical log/state into smaller current form.

/// Sentinel for `compaction`.
pub struct Compaction;

cast::concept! {
    name: "compaction",
    summary: "Reduce accumulated historical/log data into smaller \
              current form. Discards data no live reader can still \
              reach; preserves the live working set.",
    anchors: [cast_stdlib::storage::compaction::Compaction],
    tags: ["cast_stdlib", "storage"],
}
