//! `tcp_congestion_algorithm` — regulate network sending.

/// Sentinel for `tcp_congestion_algorithm`.
pub struct TcpCongestionAlgorithm;

cast::concept! {
    name: "tcp_congestion_algorithm",
    summary: "regulate network sending.",
    anchors: [cast_os_stdlib::os_algorithms::tcp_congestion_algorithm::TcpCongestionAlgorithm],
    tags: ["cast_os_stdlib", "os_algorithms"],
}
