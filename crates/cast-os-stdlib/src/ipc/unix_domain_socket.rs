//! `unix_domain_socket` — local socket IPC.

/// Sentinel for `unix_domain_socket`.
pub struct UnixDomainSocket;

cast::concept! {
    name: "unix_domain_socket",
    summary: "local socket IPC.",
    anchors: [cast_os_stdlib::ipc::unix_domain_socket::UnixDomainSocket],
    tags: ["cast_os_stdlib", "ipc"],
}
