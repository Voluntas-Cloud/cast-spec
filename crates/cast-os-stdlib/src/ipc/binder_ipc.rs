//! `binder_ipc` — Android IPC mechanism.

/// Sentinel for `binder_ipc`.
pub struct BinderIpc;

cast::concept! {
    name: "binder_ipc",
    summary: "Android IPC mechanism.",
    anchors: [cast_os_stdlib::ipc::binder_ipc::BinderIpc],
    tags: ["cast_os_stdlib", "ipc"],
}
