//! `throughput_oriented_scheduling` — optimize total work completed.

/// Sentinel for `throughput_oriented_scheduling`.
pub struct ThroughputOrientedScheduling;

cast::concept! {
    name: "throughput_oriented_scheduling",
    summary: "optimize total work completed.",
    anchors: [cast_os_stdlib::scheduling::throughput_oriented_scheduling::ThroughputOrientedScheduling],
    tags: ["cast_os_stdlib", "scheduling"],
}
