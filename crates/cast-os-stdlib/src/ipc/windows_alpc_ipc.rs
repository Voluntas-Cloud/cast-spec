//! `windows_alpc_ipc` — Windows local procedure-call IPC.

/// Sentinel for `windows_alpc_ipc`.
pub struct WindowsAlpcIpc;

cast::concept! {
    name: "windows_alpc_ipc",
    summary: "Windows local procedure-call IPC.",
    anchors: [cast_os_stdlib::ipc::windows_alpc_ipc::WindowsAlpcIpc],
    tags: ["cast_os_stdlib", "ipc"],
}
