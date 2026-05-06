//! Classical computer-science algorithms.
//!
//! Each algorithm is a named, well-known computation pattern (recursive
//! descent, tree traversal, hash function, …) with a sentinel struct
//! anchored at `cast_stdlib::algorithms::<algorithm>::<Sentinel>`.
//! Concepts in downstream code use `cast::continues_in!` to declare
//! "this function is an instance of *this* algorithm" without
//! re-explaining what the algorithm is.

pub mod recursive_descent_parser;
pub mod tree_traversal;
