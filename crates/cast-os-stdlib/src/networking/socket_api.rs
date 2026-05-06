//! `socket_api` — application network endpoint abstraction.

/// Sentinel for `socket_api`.
pub struct SocketApi;

cast::concept! {
    name: "socket_api",
    summary: "application network endpoint abstraction.",
    anchors: [cast_os_stdlib::networking::socket_api::SocketApi],
    tags: ["cast_os_stdlib", "networking"],
}
