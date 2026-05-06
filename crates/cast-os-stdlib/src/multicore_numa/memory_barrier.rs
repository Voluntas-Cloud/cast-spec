//! `memory_barrier` — enforce ordering of memory operations.

/// Sentinel for `memory_barrier`.
pub struct MemoryBarrier;

cast::concept! {
    name: "memory_barrier",
    summary: "enforce ordering of memory operations.",
    anchors: [cast_os_stdlib::multicore_numa::memory_barrier::MemoryBarrier],
    tags: ["cast_os_stdlib", "multicore_numa"],
}
