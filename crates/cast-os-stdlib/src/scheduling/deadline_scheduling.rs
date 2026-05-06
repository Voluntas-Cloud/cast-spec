//! `deadline_scheduling` — tasks scheduled by deadline constraints.

/// Sentinel for `deadline_scheduling`.
pub struct DeadlineScheduling;

cast::concept! {
    name: "deadline_scheduling",
    summary: "tasks scheduled by deadline constraints.",
    anchors: [cast_os_stdlib::scheduling::deadline_scheduling::DeadlineScheduling],
    tags: ["cast_os_stdlib", "scheduling"],
}
