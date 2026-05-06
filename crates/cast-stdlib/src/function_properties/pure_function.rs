//! `pure_function` — output depends only on input; no observable side effects.

/// Sentinel for `pure_function`.
pub struct PureFunction;

cast::concept! {
    name: "pure_function",
    summary: "Output depends only on inputs; no I/O, no caller-visible \
              mutation, no time / RNG / global state. Always \
              deterministic. Safe to memoise, reorder, parallelise, \
              eliminate-when-unused.",
    anchors: [cast_stdlib::function_properties::pure_function::PureFunction],
    tags: ["cast_stdlib", "function_properties"],
}
