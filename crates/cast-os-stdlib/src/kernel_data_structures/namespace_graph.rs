//! `namespace_graph` — relationships among isolated namespaces.

/// Sentinel for `namespace_graph`.
pub struct NamespaceGraph;

cast::concept! {
    name: "namespace_graph",
    summary: "relationships among isolated namespaces.",
    anchors: [cast_os_stdlib::kernel_data_structures::namespace_graph::NamespaceGraph],
    tags: ["cast_os_stdlib", "kernel_data_structures"],
}
