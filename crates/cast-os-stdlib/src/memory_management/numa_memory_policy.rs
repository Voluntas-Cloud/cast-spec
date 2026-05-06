//! `numa_memory_policy` — memory placement across NUMA nodes.

/// Sentinel for `numa_memory_policy`.
pub struct NumaMemoryPolicy;

cast::concept! {
    name: "numa_memory_policy",
    summary: "memory placement across NUMA nodes.",
    anchors: [cast_os_stdlib::memory_management::numa_memory_policy::NumaMemoryPolicy],
    tags: ["cast_os_stdlib", "memory_management"],
}
