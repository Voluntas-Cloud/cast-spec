//! `time_sharing_scheduler` — CPU shared among interactive tasks.

/// Sentinel for `time_sharing_scheduler`.
pub struct TimeSharingScheduler;

cast::concept! {
    name: "time_sharing_scheduler",
    summary: "CPU shared among interactive tasks.",
    anchors: [cast_os_stdlib::scheduling::time_sharing_scheduler::TimeSharingScheduler],
    tags: ["cast_os_stdlib", "scheduling"],
}
