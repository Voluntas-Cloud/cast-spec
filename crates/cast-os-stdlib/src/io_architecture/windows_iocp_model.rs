//! `windows_iocp_model` — Windows I/O completion port model.

/// Sentinel for `windows_iocp_model`.
pub struct WindowsIocpModel;

cast::concept! {
    name: "windows_iocp_model",
    summary: "Windows I/O completion port model.",
    anchors: [cast_os_stdlib::io_architecture::windows_iocp_model::WindowsIocpModel],
    tags: ["cast_os_stdlib", "io_architecture"],
}
