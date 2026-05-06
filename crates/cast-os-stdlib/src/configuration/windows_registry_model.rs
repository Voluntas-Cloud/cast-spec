//! `windows_registry_model` — Windows central configuration database.

/// Sentinel for `windows_registry_model`.
pub struct WindowsRegistryModel;

cast::concept! {
    name: "windows_registry_model",
    summary: "Windows central configuration database.",
    anchors: [cast_os_stdlib::configuration::windows_registry_model::WindowsRegistryModel],
    tags: ["cast_os_stdlib", "configuration"],
}
