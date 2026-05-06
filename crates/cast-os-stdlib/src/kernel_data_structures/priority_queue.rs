//! `priority_queue` — ordered work selection.

/// Sentinel for `priority_queue`.
pub struct PriorityQueue;

cast::concept! {
    name: "priority_queue",
    summary: "ordered work selection.",
    anchors: [cast_os_stdlib::kernel_data_structures::priority_queue::PriorityQueue],
    tags: ["cast_os_stdlib", "kernel_data_structures"],
}
