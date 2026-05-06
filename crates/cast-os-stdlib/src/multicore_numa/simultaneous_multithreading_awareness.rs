//! `simultaneous_multithreading_awareness` — scheduler knows sibling threads.

/// Sentinel for `simultaneous_multithreading_awareness`.
pub struct SimultaneousMultithreadingAwareness;

cast::concept! {
    name: "simultaneous_multithreading_awareness",
    summary: "scheduler knows sibling threads.",
    anchors: [cast_os_stdlib::multicore_numa::simultaneous_multithreading_awareness::SimultaneousMultithreadingAwareness],
    tags: ["cast_os_stdlib", "multicore_numa"],
}
