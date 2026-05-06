//! `configuration_layering` — default/vendor/admin/user override model.

/// Sentinel for `configuration_layering`.
pub struct ConfigurationLayering;

cast::concept! {
    name: "configuration_layering",
    summary: "default/vendor/admin/user override model.",
    anchors: [cast_os_stdlib::configuration::configuration_layering::ConfigurationLayering],
    tags: ["cast_os_stdlib", "configuration"],
}
