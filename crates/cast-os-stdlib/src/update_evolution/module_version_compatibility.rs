//! `module_version_compatibility` — ensure kernel module ABI compatibility.

/// Sentinel for `module_version_compatibility`.
pub struct ModuleVersionCompatibility;

cast::concept! {
    name: "module_version_compatibility",
    summary: "ensure kernel module ABI compatibility.",
    anchors: [cast_os_stdlib::update_evolution::module_version_compatibility::ModuleVersionCompatibility],
    tags: ["cast_os_stdlib", "update_evolution"],
}
