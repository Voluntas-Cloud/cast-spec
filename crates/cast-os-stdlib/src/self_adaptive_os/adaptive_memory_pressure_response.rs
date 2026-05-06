//! `adaptive_memory_pressure_response` — OS changes behavior under memory pressure.

/// Sentinel for `adaptive_memory_pressure_response`.
pub struct AdaptiveMemoryPressureResponse;

cast::concept! {
    name: "adaptive_memory_pressure_response",
    summary: "OS changes behavior under memory pressure.",
    anchors: [cast_os_stdlib::self_adaptive_os::adaptive_memory_pressure_response::AdaptiveMemoryPressureResponse],
    tags: ["cast_os_stdlib", "self_adaptive_os"],
}
