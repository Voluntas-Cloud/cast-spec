//! `network_namespace` — isolate network stack.

/// Sentinel for `network_namespace`.
pub struct NetworkNamespace;

cast::concept! {
    name: "network_namespace",
    summary: "isolate network stack.",
    anchors: [cast_os_stdlib::isolation::network_namespace::NetworkNamespace],
    tags: ["cast_os_stdlib", "isolation"],
}
