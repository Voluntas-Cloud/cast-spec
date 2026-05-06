//! `work_queue` — deferred work structure.

/// Sentinel for `work_queue`.
pub struct WorkQueue;

cast::concept! {
    name: "work_queue",
    summary: "deferred work structure.",
    anchors: [cast_os_stdlib::kernel_data_structures::work_queue::WorkQueue],
    tags: ["cast_os_stdlib", "kernel_data_structures"],
}
