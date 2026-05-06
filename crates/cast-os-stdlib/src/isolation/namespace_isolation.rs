//! `namespace_isolation` — isolate views of OS resources.

/// Sentinel for `namespace_isolation`.
pub struct NamespaceIsolation;

cast::concept! {
    name: "namespace_isolation",
    summary: "isolate views of OS resources.",
    anchors: [cast_os_stdlib::isolation::namespace_isolation::NamespaceIsolation],
    tags: ["cast_os_stdlib", "isolation"],
}
