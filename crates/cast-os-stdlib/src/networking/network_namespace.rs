//! `network_namespace` — isolated network stack view.

/// Sentinel for `network_namespace`.
pub struct NetworkNamespace;

cast::concept! {
    name: "network_namespace",
    summary: "isolated network stack view.",
    anchors: [cast_os_stdlib::networking::network_namespace::NetworkNamespace],
    tags: ["cast_os_stdlib", "networking"],
}
