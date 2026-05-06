//! `system_extension_model` — extend base OS through controlled mechanisms.

/// Sentinel for `system_extension_model`.
pub struct SystemExtensionModel;

cast::concept! {
    name: "system_extension_model",
    summary: "extend base OS through controlled mechanisms.",
    anchors: [cast_os_stdlib::update_evolution::system_extension_model::SystemExtensionModel],
    tags: ["cast_os_stdlib", "update_evolution"],
}
