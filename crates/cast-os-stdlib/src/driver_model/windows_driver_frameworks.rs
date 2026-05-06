//! `windows_driver_frameworks` — KMDF/UMDF driver frameworks.

/// Sentinel for `windows_driver_frameworks`.
pub struct WindowsDriverFrameworks;

cast::concept! {
    name: "windows_driver_frameworks",
    summary: "KMDF/UMDF driver frameworks.",
    anchors: [cast_os_stdlib::driver_model::windows_driver_frameworks::WindowsDriverFrameworks],
    tags: ["cast_os_stdlib", "driver_model"],
}
