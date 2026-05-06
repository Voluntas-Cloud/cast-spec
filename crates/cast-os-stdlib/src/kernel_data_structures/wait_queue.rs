//! `wait_queue` — tasks waiting for event.

/// Sentinel for `wait_queue`.
pub struct WaitQueue;

cast::concept! {
    name: "wait_queue",
    summary: "tasks waiting for event.",
    anchors: [cast_os_stdlib::kernel_data_structures::wait_queue::WaitQueue],
    tags: ["cast_os_stdlib", "kernel_data_structures"],
}
