//! `pinned_dependency` — exact dependency version fixed.

/// Sentinel for `pinned_dependency`.
pub struct PinnedDependency;

cast::concept! {
    name: "pinned_dependency",
    summary: "Exact dependency version fixed. Not a version range, \
              not 'latest' — a single point. Upgrading is a deliberate \
              action, not something that happens to you in CI.",
    anchors: [cast_stdlib::supply_chain::pinned_dependency::PinnedDependency],
    tags: ["cast_stdlib", "supply_chain"],
}
