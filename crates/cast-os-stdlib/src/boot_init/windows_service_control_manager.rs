//! `windows_service_control_manager` — Windows service manager.

/// Sentinel for `windows_service_control_manager`.
pub struct WindowsServiceControlManager;

cast::concept! {
    name: "windows_service_control_manager",
    summary: "Windows service manager.",
    anchors: [cast_os_stdlib::boot_init::windows_service_control_manager::WindowsServiceControlManager],
    tags: ["cast_os_stdlib", "boot_init"],
}
