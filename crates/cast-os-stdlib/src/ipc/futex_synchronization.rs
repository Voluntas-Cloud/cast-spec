//! `futex_synchronization` — fast user-space mutex primitive.

/// Sentinel for `futex_synchronization`.
pub struct FutexSynchronization;

cast::concept! {
    name: "futex_synchronization",
    summary: "fast user-space mutex primitive.",
    anchors: [cast_os_stdlib::ipc::futex_synchronization::FutexSynchronization],
    tags: ["cast_os_stdlib", "ipc"],
}
