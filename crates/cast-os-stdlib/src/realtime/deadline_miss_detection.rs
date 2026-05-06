//! `deadline_miss_detection` — observe missed timing constraints.

/// Sentinel for `deadline_miss_detection`.
pub struct DeadlineMissDetection;

cast::concept! {
    name: "deadline_miss_detection",
    summary: "observe missed timing constraints.",
    anchors: [cast_os_stdlib::realtime::deadline_miss_detection::DeadlineMissDetection],
    tags: ["cast_os_stdlib", "realtime"],
}
