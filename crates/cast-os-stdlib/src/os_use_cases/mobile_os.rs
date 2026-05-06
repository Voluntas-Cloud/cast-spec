//! `mobile_os` — power-managed, sandboxed, sensor-rich OS.

/// Sentinel for `mobile_os`.
pub struct MobileOs;

cast::concept! {
    name: "mobile_os",
    summary: "power-managed, sandboxed, sensor-rich OS.",
    anchors: [cast_os_stdlib::os_use_cases::mobile_os::MobileOs],
    tags: ["cast_os_stdlib", "os_use_cases"],
}
