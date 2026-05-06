//! `lazy_evaluation` — defer work until needed.

/// Sentinel for `lazy_evaluation`.
pub struct LazyEvaluation;

cast::concept! {
    name: "lazy_evaluation",
    summary: "Defer work until needed. The result of a computation is \
              produced on demand, not eagerly; if no one ever asks, \
              the work is never done.",
    anchors: [cast_stdlib::performance::lazy_evaluation::LazyEvaluation],
    tags: ["cast_stdlib", "performance"],
}
