//! `scheduler_class` — pluggable scheduling policy family.

/// Sentinel for `scheduler_class`.
pub struct SchedulerClass;

cast::concept! {
    name: "scheduler_class",
    summary: "pluggable scheduling policy family.",
    anchors: [cast_os_stdlib::scheduling::scheduler_class::SchedulerClass],
    tags: ["cast_os_stdlib", "scheduling"],
}
