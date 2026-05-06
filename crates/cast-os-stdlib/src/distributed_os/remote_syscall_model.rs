//! `remote_syscall_model` — OS calls forwarded across nodes.

/// Sentinel for `remote_syscall_model`.
pub struct RemoteSyscallModel;

cast::concept! {
    name: "remote_syscall_model",
    summary: "OS calls forwarded across nodes.",
    anchors: [cast_os_stdlib::distributed_os::remote_syscall_model::RemoteSyscallModel],
    tags: ["cast_os_stdlib", "distributed_os"],
}
