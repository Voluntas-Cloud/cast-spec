//! `semaphore_ipc` — counting synchronization primitive.

/// Sentinel for `semaphore_ipc`.
pub struct SemaphoreIpc;

cast::concept! {
    name: "semaphore_ipc",
    summary: "counting synchronization primitive.",
    anchors: [cast_os_stdlib::ipc::semaphore_ipc::SemaphoreIpc],
    tags: ["cast_os_stdlib", "ipc"],
}
