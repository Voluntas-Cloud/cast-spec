//! `numa_aware_scheduling` — schedule near memory locality.

/// Sentinel for `numa_aware_scheduling`.
pub struct NumaAwareScheduling;

cast::concept! {
    name: "numa_aware_scheduling",
    summary: "schedule near memory locality.",
    anchors: [cast_os_stdlib::scheduling::numa_aware_scheduling::NumaAwareScheduling],
    tags: ["cast_os_stdlib", "scheduling"],
}
