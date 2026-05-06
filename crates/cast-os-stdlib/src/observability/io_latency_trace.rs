//! `io_latency_trace` — inspect storage/network I/O delay.

/// Sentinel for `io_latency_trace`.
pub struct IoLatencyTrace;

cast::concept! {
    name: "io_latency_trace",
    summary: "inspect storage/network I/O delay.",
    anchors: [cast_os_stdlib::observability::io_latency_trace::IoLatencyTrace],
    tags: ["cast_os_stdlib", "observability"],
}
