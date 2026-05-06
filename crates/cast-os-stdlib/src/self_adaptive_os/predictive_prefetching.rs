//! `predictive_prefetching` — anticipate future data needs.

/// Sentinel for `predictive_prefetching`.
pub struct PredictivePrefetching;

cast::concept! {
    name: "predictive_prefetching",
    summary: "anticipate future data needs.",
    anchors: [cast_os_stdlib::self_adaptive_os::predictive_prefetching::PredictivePrefetching],
    tags: ["cast_os_stdlib", "self_adaptive_os"],
}
