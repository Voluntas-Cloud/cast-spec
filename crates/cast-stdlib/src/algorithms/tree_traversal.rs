//! `tree_traversal` — visit every node of a tree in a defined order.

/// Sentinel for `tree_traversal`.
pub struct TreeTraversal;

cast::concept! {
    name: "tree_traversal",
    summary: "Visit every node of a tree once in a defined order \
              (pre / in / post / level). Pure traversals read; \
              transforming traversals fold or rebuild. Foundational \
              for parsers, type-checkers, AST visitors.",
    anchors: [cast_stdlib::algorithms::tree_traversal::TreeTraversal],
    tags: ["cast_stdlib", "algorithms"],
}
