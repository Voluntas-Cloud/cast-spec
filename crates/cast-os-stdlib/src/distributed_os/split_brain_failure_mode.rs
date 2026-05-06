//! `split_brain_failure_mode` — partitions create competing authorities.

/// Sentinel for `split_brain_failure_mode`.
pub struct SplitBrainFailureMode;

cast::concept! {
    name: "split_brain_failure_mode",
    summary: "partitions create competing authorities.",
    anchors: [cast_os_stdlib::distributed_os::split_brain_failure_mode::SplitBrainFailureMode],
    tags: ["cast_os_stdlib", "distributed_os"],
}
