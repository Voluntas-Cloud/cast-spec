//! `fail_stop_design` — stop rather than continue unsafely.

/// Sentinel for `fail_stop_design`.
pub struct FailStopDesign;

cast::concept! {
    name: "fail_stop_design",
    summary: "stop rather than continue unsafely.",
    anchors: [cast_os_stdlib::fault_recovery::fail_stop_design::FailStopDesign],
    tags: ["cast_os_stdlib", "fault_recovery"],
}
