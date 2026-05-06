//! `tracepoint_static_probe` — predefined instrumentation point.

/// Sentinel for `tracepoint_static_probe`.
pub struct TracepointStaticProbe;

cast::concept! {
    name: "tracepoint_static_probe",
    summary: "predefined instrumentation point.",
    anchors: [cast_os_stdlib::observability::tracepoint_static_probe::TracepointStaticProbe],
    tags: ["cast_os_stdlib", "observability"],
}
