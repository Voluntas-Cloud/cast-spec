//! `recursive_descent_parser` — top-down parser; mutually recursive functions, one per grammar rule.

/// Sentinel for `recursive_descent_parser`.
pub struct RecursiveDescentParser;

cast::concept! {
    name: "recursive_descent_parser",
    summary: "Top-down parser implemented as mutually recursive \
              functions, one per grammar production. Recursion \
              mirrors the grammar's tree. Composes tree_traversal at \
              the call-stack level.",
    anchors: [cast_stdlib::algorithms::recursive_descent_parser::RecursiveDescentParser],
    tags: ["cast_stdlib", "algorithms"],
}
