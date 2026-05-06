//! `dependency_graph` — explicit relationships between modules.

/// Sentinel for `dependency_graph`.
pub struct DependencyGraph;

cast::concept! {
    name: "dependency_graph",
    summary: "Explicit map of which modules depend on which. The \
              graph is what catches cycles and unintended fan-out; \
              when it goes unwatched, modules find each other \
              through three layers of re-export.",
    anchors: [cast_stdlib::project_layout::dependency_graph::DependencyGraph],
    tags: ["cast_stdlib", "project_layout"],
}
