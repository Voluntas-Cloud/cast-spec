//! `socket_activation` — start service when socket receives traffic.

/// Sentinel for `socket_activation`.
pub struct SocketActivation;

cast::concept! {
    name: "socket_activation",
    summary: "start service when socket receives traffic.",
    anchors: [cast_os_stdlib::service_management::socket_activation::SocketActivation],
    tags: ["cast_os_stdlib", "service_management"],
}
