//! `memory_compaction` — reduce fragmentation.

/// Sentinel for `memory_compaction`.
pub struct MemoryCompaction;

cast::concept! {
    name: "memory_compaction",
    summary: "reduce fragmentation.",
    anchors: [cast_os_stdlib::memory_management::memory_compaction::MemoryCompaction],
    tags: ["cast_os_stdlib", "memory_management"],
}
