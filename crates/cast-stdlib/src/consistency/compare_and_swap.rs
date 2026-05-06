//! `compare_and_swap` — update if current matches expectation.

/// Sentinel for `compare_and_swap`.
pub struct CompareAndSwap;

cast::concept! {
    name: "compare_and_swap",
    summary: "Update only if current value matches expectation. The \
              atomic primitive most lock-free algorithms reduce to.",
    anchors: [cast_stdlib::consistency::compare_and_swap::CompareAndSwap],
    tags: ["cast_stdlib", "consistency"],
}
