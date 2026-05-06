//! `namespace_based_resource_model` — resource views are scoped by namespace.

/// Sentinel for `namespace_based_resource_model`.
pub struct NamespaceBasedResourceModel;

cast::concept! {
    name: "namespace_based_resource_model",
    summary: "resource views are scoped by namespace.",
    anchors: [cast_os_stdlib::architectural_patterns::namespace_based_resource_model::NamespaceBasedResourceModel],
    tags: ["cast_os_stdlib", "architectural_patterns"],
}
