//! `session_and_process_group` — Unix-style job-control grouping.

/// Sentinel for `session_and_process_group`.
pub struct SessionAndProcessGroup;

cast::concept! {
    name: "session_and_process_group",
    summary: "Unix-style job-control grouping.",
    anchors: [cast_os_stdlib::execution_model::session_and_process_group::SessionAndProcessGroup],
    tags: ["cast_os_stdlib", "execution_model"],
}
