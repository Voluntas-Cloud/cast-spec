//! `real_time_embedded_os` — deterministic embedded OS.

/// Sentinel for `real_time_embedded_os`.
pub struct RealTimeEmbeddedOs;

cast::concept! {
    name: "real_time_embedded_os",
    summary: "deterministic embedded OS.",
    anchors: [cast_os_stdlib::os_use_cases::real_time_embedded_os::RealTimeEmbeddedOs],
    tags: ["cast_os_stdlib", "os_use_cases"],
}
