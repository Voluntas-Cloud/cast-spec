//! `cyclictest_latency_measurement` — common Linux RT latency measurement concept.

/// Sentinel for `cyclictest_latency_measurement`.
pub struct CyclictestLatencyMeasurement;

cast::concept! {
    name: "cyclictest_latency_measurement",
    summary: "common Linux RT latency measurement concept.",
    anchors: [cast_os_stdlib::realtime::cyclictest_latency_measurement::CyclictestLatencyMeasurement],
    tags: ["cast_os_stdlib", "realtime"],
}
