//! `lockfile_dependency_graph` — resolved dependency graph committed.

/// Sentinel for `lockfile_dependency_graph`.
pub struct LockfileDependencyGraph;

cast::concept! {
    name: "lockfile_dependency_graph",
    summary: "Resolved dependency graph committed. Captures the \
              transitive closure that was actually selected; future \
              builds reproduce the same graph rather than re-resolving.",
    anchors: [cast_stdlib::supply_chain::lockfile_dependency_graph::LockfileDependencyGraph],
    tags: ["cast_stdlib", "supply_chain"],
}
