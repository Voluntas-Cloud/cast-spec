//! `ipc_namespace` — isolate IPC resources.

/// Sentinel for `ipc_namespace`.
pub struct IpcNamespace;

cast::concept! {
    name: "ipc_namespace",
    summary: "isolate IPC resources.",
    anchors: [cast_os_stdlib::isolation::ipc_namespace::IpcNamespace],
    tags: ["cast_os_stdlib", "isolation"],
}
