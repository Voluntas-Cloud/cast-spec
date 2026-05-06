//! `wait_free_structure` — bounded progress for every participant.

/// Sentinel for `wait_free_structure`.
pub struct WaitFreeStructure;

cast::concept! {
    name: "wait_free_structure",
    summary: "bounded progress for every participant.",
    anchors: [cast_os_stdlib::multicore_numa::wait_free_structure::WaitFreeStructure],
    tags: ["cast_os_stdlib", "multicore_numa"],
}
