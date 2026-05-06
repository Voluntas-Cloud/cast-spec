//! `real_time_priority_class` — tasks with deterministic priority treatment.

/// Sentinel for `real_time_priority_class`.
pub struct RealTimePriorityClass;

cast::concept! {
    name: "real_time_priority_class",
    summary: "tasks with deterministic priority treatment.",
    anchors: [cast_os_stdlib::scheduling::real_time_priority_class::RealTimePriorityClass],
    tags: ["cast_os_stdlib", "scheduling"],
}
