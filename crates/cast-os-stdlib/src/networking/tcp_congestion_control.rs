//! `tcp_congestion_control` — regulate sender rate.

/// Sentinel for `tcp_congestion_control`.
pub struct TcpCongestionControl;

cast::concept! {
    name: "tcp_congestion_control",
    summary: "regulate sender rate.",
    anchors: [cast_os_stdlib::networking::tcp_congestion_control::TcpCongestionControl],
    tags: ["cast_os_stdlib", "networking"],
}
