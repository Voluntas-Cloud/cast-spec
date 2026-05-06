//! `hierarchical_configuration_database` — registry-like configuration store.

/// Sentinel for `hierarchical_configuration_database`.
pub struct HierarchicalConfigurationDatabase;

cast::concept! {
    name: "hierarchical_configuration_database",
    summary: "registry-like configuration store.",
    anchors: [cast_os_stdlib::configuration::hierarchical_configuration_database::HierarchicalConfigurationDatabase],
    tags: ["cast_os_stdlib", "configuration"],
}
