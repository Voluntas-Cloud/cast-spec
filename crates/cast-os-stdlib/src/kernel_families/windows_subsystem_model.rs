//! `windows_subsystem_model` — Windows architecture exposing multiple subsystem personalities over NT primitives.

/// Sentinel for `windows_subsystem_model`.
pub struct WindowsSubsystemModel;

cast::concept! {
    name: "windows_subsystem_model",
    summary: "Windows architecture exposing multiple subsystem \
               personalities over NT primitives.",
    anchors: [cast_os_stdlib::kernel_families::windows_subsystem_model::WindowsSubsystemModel],
    tags: ["cast_os_stdlib", "kernel_families"],
}
