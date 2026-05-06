//! `configuration_drop_in` — layered config override.

/// Sentinel for `configuration_drop_in`.
pub struct ConfigurationDropIn;

cast::concept! {
    name: "configuration_drop_in",
    summary: "layered config override.",
    anchors: [cast_os_stdlib::service_management::configuration_drop_in::ConfigurationDropIn],
    tags: ["cast_os_stdlib", "service_management"],
}
