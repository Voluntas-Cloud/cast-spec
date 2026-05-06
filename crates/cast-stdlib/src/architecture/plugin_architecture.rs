//! `plugin_architecture` — extensibility through stable plugin contract.

/// Sentinel for `plugin_architecture`.
pub struct PluginArchitecture;

cast::concept! {
    name: "plugin_architecture",
    summary: "Extensibility through loaded modules. Core defines a \
              stable plugin contract; plugins add capability without \
              modifying the core.",
    anchors: [cast_stdlib::architecture::plugin_architecture::PluginArchitecture],
    tags: ["cast_stdlib", "architecture"],
}
