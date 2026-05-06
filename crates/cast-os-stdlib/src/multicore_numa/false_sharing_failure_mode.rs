//! `false_sharing_failure_mode` — cores contend over cache lines.

/// Sentinel for `false_sharing_failure_mode`.
pub struct FalseSharingFailureMode;

cast::concept! {
    name: "false_sharing_failure_mode",
    summary: "cores contend over cache lines.",
    anchors: [cast_os_stdlib::multicore_numa::false_sharing_failure_mode::FalseSharingFailureMode],
    tags: ["cast_os_stdlib", "multicore_numa"],
}
