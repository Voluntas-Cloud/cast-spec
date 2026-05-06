//! `windows_driver_model` — Windows framework for kernel drivers.

/// Sentinel for `windows_driver_model`.
pub struct WindowsDriverModel;

cast::concept! {
    name: "windows_driver_model",
    summary: "Windows framework for kernel drivers.",
    anchors: [cast_os_stdlib::driver_model::windows_driver_model::WindowsDriverModel],
    tags: ["cast_os_stdlib", "driver_model"],
}
