//! `pipe_ipc` — byte stream between processes.

/// Sentinel for `pipe_ipc`.
pub struct PipeIpc;

cast::concept! {
    name: "pipe_ipc",
    summary: "byte stream between processes.",
    anchors: [cast_os_stdlib::ipc::pipe_ipc::PipeIpc],
    tags: ["cast_os_stdlib", "ipc"],
}
