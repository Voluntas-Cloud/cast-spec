//! `tcp_stack` — reliable stream protocol implementation.

/// Sentinel for `tcp_stack`.
pub struct TcpStack;

cast::concept! {
    name: "tcp_stack",
    summary: "reliable stream protocol implementation.",
    anchors: [cast_os_stdlib::networking::tcp_stack::TcpStack],
    tags: ["cast_os_stdlib", "networking"],
}
