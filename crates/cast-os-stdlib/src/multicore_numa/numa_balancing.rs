//! `numa_balancing` — move memory/tasks for locality.

/// Sentinel for `numa_balancing`.
pub struct NumaBalancing;

cast::concept! {
    name: "numa_balancing",
    summary: "move memory/tasks for locality.",
    anchors: [cast_os_stdlib::multicore_numa::numa_balancing::NumaBalancing],
    tags: ["cast_os_stdlib", "multicore_numa"],
}
