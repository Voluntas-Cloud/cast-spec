//! `namespaced_ipc` — IPC visibility isolated by namespace.

/// Sentinel for `namespaced_ipc`.
pub struct NamespacedIpc;

cast::concept! {
    name: "namespaced_ipc",
    summary: "IPC visibility isolated by namespace.",
    anchors: [cast_os_stdlib::ipc::namespaced_ipc::NamespacedIpc],
    tags: ["cast_os_stdlib", "ipc"],
}
