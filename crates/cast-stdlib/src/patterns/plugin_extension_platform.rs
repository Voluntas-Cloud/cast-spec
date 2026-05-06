//! `plugin_extension_platform` — the core can be extended safely by external or optional components.

/// Sentinel for `plugin_extension_platform`.
pub struct PluginExtensionPlatform;

cast::concept! {
    name: "plugin_extension_platform",
    summary: "The core system can be extended safely by external or \
              optional components. Composes plugin_architecture, \
              typed_interface, capability_discovery, sandboxing, \
              agent_permission_boundary, versioned_api_endpoint, and \
              compatibility_test_suite. Used for app ecosystems, \
              Voluntas service modules, AI tools, connectors, and \
              custom workflows.",
    anchors: [cast_stdlib::patterns::plugin_extension_platform::PluginExtensionPlatform],
    tags: ["cast_stdlib", "patterns"],
}
